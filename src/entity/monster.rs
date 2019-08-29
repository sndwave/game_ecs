use std::any::TypeId;

use specs::prelude::{
    Component,
    World,
    Entity,
    Builder,
    EntityBuilder,
    WorldExt
};

use crate::comp::*;

pub struct MonsterBuilder<'a> {
    has_position: bool,
    has_model: bool,
    has_health: bool,
    has_cooldown: bool,
    has_target: bool,
    has_ai: bool,

    pub builder: EntityBuilder<'a>
}

impl<'a> Builder for MonsterBuilder<'a> {
    #[inline]
    fn with<T: Component + Sync + Send>(mut self, c: T) -> Self {
        if TypeId::of::<Health>() == c.type_id() {
            self.has_health = true;
        } else if TypeId::of::<Position>() == c.type_id() {
            self.has_position = true;
        } else if TypeId::of::<Model>() == c.type_id() {
            self.has_model = true;
        } else if TypeId::of::<Cooldown>() == c.type_id() {
            self.has_cooldown = true;
        } else if TypeId::of::<Target>() == c.type_id() {
            self.has_target = true;
        } else if TypeId::of::<AIControl>() == c.type_id() {
            self.has_ai = true;
        }

        self.builder = self.builder.with(c);
        self
    }

    #[inline]
    fn build(mut self) -> Entity {
        if !self.has_model {
            panic!("Tried to create monster without specifying the Model");
        }
        if !self.has_position {
            panic!("Tried to create monster without specifying the Position");
        }
        if !self.has_health {
            panic!("Tried to create monster without specifying the Health");
        }
        if !self.has_ai {
            panic!("Tried to create monster without specifying the AI");
        }
        if !self.has_target {
            self.builder = self.builder.with(Target(None));
        }
        if !self.has_cooldown {
            self.builder = self.builder.with(Cooldown(0));
        }
        self.builder.build()
    }
}

pub trait MonsterBuilderExt {
    fn create_monster(&mut self) -> MonsterBuilder;
}

impl MonsterBuilderExt for World {
    fn create_monster(&mut self) -> MonsterBuilder {
        MonsterBuilder {
            builder: self.create_entity(),

            has_ai: false,
            has_model: false,
            has_position: false,
            has_health: false,
            has_cooldown: false,
            has_target: false,

        }
    }
}
