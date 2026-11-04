use std::any::Any;

/// Events are used to transfer information between systems.
/// [EventHandler] is used to store Events trait objects.
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

    /// Adds an [Event] to the [EventHandler].
    pub fn push<E: Event + 'static>(&mut self, event: E) {
        self.events.push(Box::new(event));
    }

    /// Removes an [Event] by index
    pub fn remove_event(&mut self, index: usize) {
        self.events.remove(index);
    }

    /// Get all specific event of type.
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

    /// Get all [Event]
    pub fn get_all(&self) -> Vec<&dyn Event> {
        self.events.iter().map(|e| e.as_ref()).collect()
    }
}
