use std::any::Any;

pub trait Event: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub struct EventHandler {
    events: Vec<Box<dyn Event>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push<E: Event + 'static>(&mut self, event: E) {
        self.events.push(Box::new(event));
    }

    pub fn get_events<E: Event + 'static>(&self) -> Option<Vec<&E>> {
        let events: Vec<&E> = self
            .events
            .iter()
            .filter_map(|e| e.as_any().downcast_ref::<E>())
            .collect();

        if events.is_empty() {
            None
        } else {
            Some(events)
        }
    }

    pub fn get_all(&self) -> Vec<&dyn Event> {
        self.events.iter().map(|e| e.as_ref()).collect()
    }
}
