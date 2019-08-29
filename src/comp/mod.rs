use specs::prelude::*;

pub mod ai;
pub mod name;
pub mod threat_table;

pub use ai::*;
pub use name::*;
pub use threat_table::*;

#[derive(Debug,Default)]
pub struct PlayerControl;

impl Component for PlayerControl {
    type Storage = NullStorage<Self>;
}

#[derive(Debug)]
pub struct Position(pub i32, pub i32);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Health(pub u32);

impl Component for Health {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Target(pub Option<Entity>);

impl Component for Target {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Cooldown(pub u32);

impl Component for Cooldown {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Model(pub u32);

impl Component for Model {
    type Storage = DenseVecStorage<Self>;
}

pub fn register_components(world: &mut World) {
    world.register::<PlayerControl>();
    world.register::<AIControl>();
    world.register::<Model>();
    world.register::<Name>();
}
