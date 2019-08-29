use specs::prelude::{
    Entities,
    System,
    ReadStorage,
    WriteStorage
};

use rayon::prelude::*;
use specs::ParJoin;

use crate::comp::*;

pub struct TargetingSystem;

impl<'a> System<'a> for TargetingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, AIControl>,
        WriteStorage<'a, Target>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
             healths,
             positions,
             ais,
             mut targets) = data;

        // parallel!
        (&entities, &ais, &mut targets, &healths)
            .par_join()
            .for_each(|(entity, ai, target, health)| {
                if health.0 == 0 {
                    return;
                }
                ai.target(&entity, target, (&entities, &healths, &positions))
            });
    }
}
