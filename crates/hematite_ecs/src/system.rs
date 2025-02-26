use crate::world::World;

pub trait System {
    fn run(&self, world: &World);
}