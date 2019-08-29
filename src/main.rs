extern crate rand;
extern crate rayon;
extern crate specs;

use rand::prelude::*;
use rand::distributions::Uniform;

use specs::{prelude::*, WorldExt};

mod comp;
mod system;
mod entity;
mod ai;
mod events;

use comp::*;
use system::*;
use entity::MonsterBuilderExt;
use ai::*;

fn main() {
    let mut world = World::new();

    register_components(&mut world);

    let mut dispatcher = DispatcherBuilder::new()
        .with(TargetingSystem, "targeting", &[])
        .with(MovementSystem, "movement", &["targeting"])
        .with(AttackSystem, "attack", &["movement"])
        .build();

    dispatcher.setup(&mut world);

    let mut rng = rand::thread_rng();
    let position_range = Uniform::new(0, 20);
    // create enemies
    let mut rats = 0;
    let mut goblins = 0;
    for _ in 0..10 {
        let (x, y) = (position_range.sample(&mut rng),
                      position_range.sample(&mut rng));
        let (name, aicontrol) = {
            if position_range.sample(&mut rng) < 10 {
                rats += 1;
                (Name(format!("Rat #{}", rats).to_owned()),
                 AIControl(Box::new(Rat)))
            } else {
                goblins += 1;
                (Name(format!("Goblin #{}", goblins).to_owned()),
                 AIControl(Box::new(Goblin)))
            }
        };
        world
            .create_monster()
            .with(name)
            .with(aicontrol)
            .with(Position(x, y))
            .with(Health(100))
            .with(Model(0))
            .build();
    }
    // create player characters
    // for i in 0..2 {
    //     let (x, y) = (position_range.sample(&mut rng),
    //                   position_range.sample(&mut rng));
    //     world
    //         .create_entity()
    //         .with(Name(format!("Player {}", i).to_owned()))
    //         .with(PlayerControl)
    //         .with(Position(x, y))
    //         .with(Health(100))
    //         .with(Target(None))
    //         .with(Cooldown(0))
    //         .build();
    // }

    loop {

        // {
        //     let cooldowns = world.read_storage::<Cooldown>();
        //     let player_controlled = world.read_storage::<PlayerControl>();
        //     let healths = world.read_storage::<Health>();
        //     let ai_controlled = world.read_storage::<AIControl>();
        //     for (entity, cooldown, health, _) in (&world.entities(), &cooldowns, &healths, &player_controlled).join() {
        //         if cooldown.0 == 0 && health.0 > 0 {
        //             println!("PLAYER {} TURN", entity.id())
        //         }
        //     }
        //     for (entity, cooldown, health, ai) in (&world.entities(), &cooldowns, &healths, &ai_controlled).join() {
        //         if cooldown.0 == 0 && health.0 > 0 {
        //             ai.run(&entity)
        //         }
        //     }
        // }

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
                    let names = world.read_storage::<Name>();
                    if let Some(name) = names.get(entity) {
                        println!("THE WINNER IS {}", name.0);
                    } else {
                        println!("THE WINNER IS Entity: #{}", entity.id());
                    }
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
