/*

Simple AI Battle Royal

All entities have position, health, and an attack cooldown
First, each entity picks a target (currently at random)
Second, the entity will move towards the target (1 "row/column" per turn)
Finally, if the entity is in range of it's target (1 "row/column" away), it will attack

Attacks have a cooldown, so each entity will only attack once per each cooldown duration.
Simulation continues until there is one entity left with > 0 health

*/

extern crate rand;
extern crate rayon;
extern crate specs;

use rayon::prelude::*;
use rand::prelude::*;
use rand::distributions::Uniform;

use specs::{prelude::*, WorldExt};
use specs::ParJoin;

#[derive(Debug)]
struct Position(i32, i32);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Health(u32);

impl Component for Health {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Target(Option<Entity>);

impl Component for Target {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Cooldown(u32);

impl Component for Cooldown {
    type Storage = VecStorage<Self>;
}

struct TargetingSystem;
impl<'a> System<'a> for TargetingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Health>,
        WriteStorage<'a, Target>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
             healths,
             mut targets) = data;

        // parallel!
        (&entities, &mut targets).par_join().for_each(|(entity, target)| {
            if target.0.is_none() || healths.get(target.0.unwrap())
                .unwrap_or_else(|| &Health(0)).0 == 0 {
                // try target something new
                let possible_targets = (&entities, &healths).par_join().filter(|(e, h)| {
                    e.id() != entity.id() && h.0 > 0
                }).map(|(e, _)| e).collect::<Vec<_>>();
                // choose randomly
                let mut rng = rand::thread_rng();
                if let Some(&new_target) = possible_targets.choose(&mut rng) {
                    *target = Target(Some(new_target));
                }
            }
        });
    }
}

struct MovementSystem;
impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Target>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
             targets,
             mut positions) = data;

        for (entity, target) in (&entities, &targets).join() {
            if target.0.is_none() {
                continue;
            }
            let entity_position = positions.get(entity);
            if entity_position.is_none() {
                continue;
            }
            let target_position = positions.get(target.0.unwrap());
            if target_position.is_none() {
                continue;
            }
            let (ex, ey) = { let p = entity_position.unwrap(); (p.0, p.1) };
            let (tx, ty) = { let p = target_position.unwrap(); (p.0, p.1) };

            let xdiff = tx - ex;
            let ydiff = ty - ey;
            if xdiff.abs() > 1 || ydiff.abs() > 1 {
                let ex = ex + if xdiff > 0 {
                    1
                } else if xdiff < 0 {
                    -1
                } else {
                    0
                };
                let ey = ey + if ydiff > 0 {
                    1
                } else if ydiff < 0 {
                    -1
                } else {
                    0
                };
                *positions.get_mut(entity).unwrap() = Position(ex, ey);
            }
        }
    }
}

struct AttackSystem;
impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Target>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Cooldown>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
             positions,
             targets,
             mut healths,
             mut cooldowns) = data;
        let mut rng = rand::thread_rng();

        let damage_range = Uniform::new(0, 10);
        // join components
        let joined_comps = (&entities, &targets,
                            &positions, &mut cooldowns).join();
        for (entity, target, position, cooldown) in joined_comps {
            let entity_health = healths.get(entity);

            // make sure this entity has a health component
            if entity_health.is_none() {
                continue;
            }

            let entity_health = entity_health.unwrap().0;

            // make sure this entity isn't dead
            if entity_health == 0 {
                continue;
            }

            // make sure the cooldown timer is ready
            if cooldown.0 > 0 {
                // otherwise decrese it and skip this entity
                *cooldown = Cooldown(cooldown.0 - 1);
                continue;
            }

            // make sure the target is in range
            let target_position = positions.get(target.0.unwrap());
            if target_position.is_none() {
                // target doesn't have a position! :sob:
                continue;
            }
            let (ex, ey) = (position.0, position.1);
            let (tx, ty) = { let p = target_position.unwrap(); (p.0, p.1) };

            let xdiff = ex - tx;
            let ydiff = ey - ty;
            if xdiff.abs() > 1 || ydiff.abs() > 1 {
                continue;
            }

            // attack!!
            let target_health = healths.get_mut(target.0.unwrap()).unwrap();
            let damage_done = damage_range.sample(&mut rng);
            let target_id = target.0.unwrap().id();
            println!("X {}({}) -> {}({}) damage: {} !",
                     entity.id(), entity_health,
                     target_id, target_health.0,
                     damage_done);
            if damage_done >= target_health.0 {
                *target_health = Health(0);
                println!("X {} Defeated!", target_id);
            } else {
                *target_health = Health(target_health.0 - damage_done);
            }
            *cooldown = Cooldown(10);
        }
    }
}

fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(TargetingSystem, "targeting", &[])
        .with(MovementSystem, "movement", &["targeting"])
        .with(AttackSystem, "attack", &["movement"])
        .build();

    dispatcher.setup(&mut world);

    let mut rng = rand::thread_rng();
    let position_range = Uniform::new(0, 20);
    for _ in 0..10 {
        let (x, y) = (position_range.sample(&mut rng),
                      position_range.sample(&mut rng));
        world
            .create_entity()
            .with(Position(x, y))
            .with(Health(100))
            .with(Target(None))
            .with(Cooldown(0))
            .build();
    }

    loop {

        dispatcher.dispatch(&world);

        let mut alive_entities = 0;
        {
            let healths = world.read_storage::<Health>();
            for entity in world.entities().join() {
                let health = healths.get(entity).unwrap().0;
                if health > 0 {
                    alive_entities += 1;
                }
            }
        }

        if alive_entities == 1 {
            let healths = world.read_storage::<Health>();
            for entity in world.entities().join() {
                let health = healths.get(entity).unwrap().0;
                if health > 0 {
                    println!("THE WINNER IS {}", entity.id());
                    break;
                }
            }
            break;
        } else if alive_entities == 0 {
            println!("NO WINNER (this is an error)")
        }

        world.maintain();
    }
}
