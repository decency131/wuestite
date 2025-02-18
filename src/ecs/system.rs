use crate::ecs::world::World;

pub trait System {
    fn run(&self, world: &World);
}