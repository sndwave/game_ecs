use specs::prelude::*;

use rand::prelude::*;
use rand::distributions::Uniform;

use crate::comp::*;
use crate::events::*;

pub type TargetData<'a> = (
    &'a Entities<'a>,
    &'a ReadStorage<'a, Health>,
    &'a ReadStorage<'a, Position>
);

pub type AttackData<'a> = (
    &'a Entities<'a>,
    &'a WriteStorage<'a, Health>,
    &'a ReadStorage<'a, Target>,
    &'a ReadStorage<'a, Position>
);

pub trait MonsterAI {
    fn target(
        &self,
        monster: &Entity,
        target: &mut Target,
        data: TargetData
    ) {
        let (entities, healths, _) = data;
        let target_health = if let Some(target) = target.0 {
            healths.get(target).unwrap_or_else(|| &Health(0)).0
        } else {
            0
        };
        if target_health == 0 {
            // try target something new
            let possible_targets = (entities, healths).par_join().filter(|(e, h)| {
                e.id() != monster.id() && h.0 > 0
            }).map(|(e, _)| e).collect::<Vec<_>>();
            // choose randomly
            let mut rng = rand::thread_rng();
            if let Some(&new_target) = possible_targets.choose(&mut rng) {
                *target = Target(Some(new_target));
            }
        }
    }
    fn attack(
        &self,
        monster: &Entity,
        data: AttackData
    ) -> Option<(Cooldown, Vec<AttackEvent>)> {
        let mut rng = rand::thread_rng();
        let damage_range = Uniform::new(0, 10);

        let (_, healths, targets, positions) = data;
        let position = positions.get(*monster);
        if position.is_none() {
            return None;
        }
        let position = position.unwrap();

        let monster_health = healths.get(*monster).unwrap_or_else(|| &Health(0)).0;
        if monster_health == 0 {
            return None;
        }

        let target = if let Some(target) = targets.get(*monster) {
            if let Some(target) = target.0 {
                target
            } else {
                return None;
            }
        } else {
            return None;
        };
        // make sure the target is in range
        let target_position = positions.get(target);
        if target_position.is_none() {
            // target doesn't have a position! :sob:
            return None;
        }
        let (ex, ey) = (position.0, position.1);
        let (tx, ty) = { let p = target_position.unwrap(); (p.0, p.1) };

        let xdiff = ex - tx;
        let ydiff = ey - ty;
        if xdiff.abs() > 1 || ydiff.abs() > 1 {
            return None;
        }

        // attack!!
        let damage_done = damage_range.sample(&mut rng);

        Some((Cooldown(10), vec![AttackEvent {
            target: target,
            attack_type: AttackType::Damage(damage_done)
        }]))
    }
}

pub struct AIControl(pub Box<dyn MonsterAI + Send + Sync>);

impl Component for AIControl {
    type Storage = DenseVecStorage<Self>;
}

impl AIControl {
    pub fn target(&self, monster: &Entity, target: &mut Target, data: TargetData) {
        self.0.target(monster, target, data)
    }
    pub fn attack(
        &self, monster: &Entity, data: AttackData
    ) -> Option<(Cooldown, Vec<AttackEvent>)> {
        self.0.attack(monster, data)
    }
}
