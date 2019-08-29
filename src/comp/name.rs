use specs::prelude::*;

#[derive(Debug)]
pub struct Name(pub String);

impl Component for Name {
    type Storage = VecStorage<Self>;
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
