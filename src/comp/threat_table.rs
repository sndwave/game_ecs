use specs::prelude::*;

pub struct ThreatTable(pub Vec<Entity>, pub Vec<u32>);

impl Component for ThreatTable {
    type Storage = DenseVecStorage<Self>;
}
