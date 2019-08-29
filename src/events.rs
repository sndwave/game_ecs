use specs::prelude::*;

pub enum AttackType {
    Damage(u32)
}

pub struct AttackEvent {
    pub target: Entity,
    pub attack_type: AttackType
}
