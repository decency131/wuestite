use crate::World;

pub trait System {
    fn run(&self, world: &mut World);
}

pub struct SystemRegistry {
    systems: Vec<Box<dyn System>>,
}

impl Default for SystemRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemRegistry {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn run(&self, world: &mut World) {
        for system in &self.systems {
            system.run(world);
        }
    }
}
