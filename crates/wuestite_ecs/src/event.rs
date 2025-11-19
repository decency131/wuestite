use std::any::Any;

pub trait Event: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub struct EventHandler {
    events: Vec<Box<dyn Event>>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push<E: Event + 'static>(&mut self, event: E) {
        self.events.push(Box::new(event));
    }

    pub fn remove(&mut self, index: usize) {
        self.events.remove(index);
    }

    pub fn get_events<E: Event>(&self) -> Option<Vec<(usize, &E)>> {
        let events: Vec<(usize, &E)> = self
            .events
            .iter()
            .enumerate()
            .filter_map(|(idx, ev)| ev.as_any().downcast_ref::<E>().map(|e| (idx, e)))
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

    pub fn remove_event(&mut self, index: usize) {
        self.events.remove(index);
    }
}
