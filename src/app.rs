pub use crate::{Event, EventHandler, System, SystemRegistry, World};

pub struct App {
    running: bool,
    wolrd: World,
    system: SystemRegistry,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            wolrd: World::new(),
            system: SystemRegistry::new(),
        }
    }

    pub fn run(&mut self) {
        self.system.run(&mut self.wolrd);
        while self.running {
            self.system.update(&mut self.wolrd);
            if self.wolrd.get_events::<AppExit>().is_some() {
                self.quit();
            }
        }
    }

    pub fn quit(&mut self) {
        self.system.shutdown(&mut self.wolrd);
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
