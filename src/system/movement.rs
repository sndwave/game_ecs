use specs::prelude::{
    Entities,
    System,
    ReadStorage,
    WriteStorage,
    Join
};

use crate::comp::{
    Target,
    Position
};

pub struct MovementSystem;

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
