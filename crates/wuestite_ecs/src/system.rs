use crate::World;

/// [System] contains logic that operates on [`Entity`](crate::Entity) and their [`Component`](crate::Component).
pub trait System {
    /// Executes the system's logic.
    fn run(&self, world: &mut World);
    /// Updates the system's state.
    fn update(&self, world: &mut World);
}

/// Manages a collection of [System].
pub struct SystemRegistry {
    systems: Vec<Box<dyn System>>,
}

impl Default for SystemRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemRegistry {
    /// Creates a new [SystemRegistry] with no systems.
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Adds a [System] to the registry.
    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    /// Executes all [System] in the registry.
    pub fn run(&self, world: &mut World) {
        for system in &self.systems {
            system.run(world);
        }
    }
}
