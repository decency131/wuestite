pub use crate::{Event, EventHandler, System, SystemRegistry, World};

pub struct App {
    running: bool,
    world: World,
    system: SystemRegistry,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            world: World::new(),
            system: SystemRegistry::new(),
        }
    }

    pub fn run(&mut self) {
        self.system.run(&mut self.world);
        while self.running {
            self.system.update(&mut self.world);
            if self.world.get_events::<AppExit>().is_some() {
                self.quit();
            }
        }
    }

    pub fn quit(&mut self) {
        self.system.shutdown(&mut self.world);
        self.running = false;
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) -> &mut App {
        self.system.add_system(system);
        self
    }
}

#[derive(Event)]
pub struct AppExit {
    pub error: Option<String>,
}
