use specs::prelude::{
    Entities,
    System,
    ReadStorage,
    WriteStorage,
    Join
};

use crate::comp::*;
use crate::events::*;

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Target>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Cooldown>,
        ReadStorage<'a, PlayerControl>,
        ReadStorage<'a, AIControl>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
             names,
             positions,
             targets,
             mut healths,
             mut cooldowns,
             player_controlled,
             ai_controlled) = data;

        for (entity, cooldown, name) in (&entities, &mut cooldowns, &names).join() {
            // make sure the cooldown is ready
            if cooldown.0 > 0 {
                *cooldown = Cooldown(cooldown.0 - 1);
                continue;
            }

            // make sure this entity has a health component
            let entity_health = if let Some(health) = healths.get(entity) {
                if health.0 == 0 {
                    continue;
                }
                health.0
            } else {
                continue;
            };

            if let Some(ai) = ai_controlled.get(entity) {
                let result = ai.attack(&entity, (&entities, &healths, &targets, &positions));
                if let Some((cd, attacks)) = result {
                    *cooldown = cd;
                    for attack in attacks {
                        if let Some(health) = healths.get_mut(attack.target) {
                            let target_name = if let Some(name) = names.get(attack.target) {
                                &name.0
                            } else {
                                "UNNAMED"
                            };
                            let target_health = health.0;
                            match attack.attack_type {
                                AttackType::Damage(damage_done) => {
                                    println!("X {}({}) -> {}({}) damage: {} !",
                                             name, entity_health,
                                             target_name, target_health,
                                             damage_done);
                                    if damage_done >= target_health {
                                        *health = Health(0);
                                        println!("X {} Defeated!", target_name);
                                    } else {
                                        *health = Health(target_health - damage_done);
                                    }
                                }
                            }
                        }
                    }
                }
                continue;
            } else if let Some(_) = player_controlled.get(entity) {

            }
        }
    }
}
