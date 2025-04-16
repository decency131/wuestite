use std::any::{Any, TypeId};
use std::collections::HashMap;

/// [Event] are used to communicate between [`System`](crate::System).
pub trait Event: 'static + Send + Sync {}

type EventListener = Box<dyn Fn(&dyn Any)>;

/// Manages event listeners and dispatches [Event] to them.
pub struct EventDispatcher {
    listeners: HashMap<TypeId, Vec<EventListener>>,
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventDispatcher {
    /// Creates a new [EventDispatcher] with no listeners.
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    /// Registers a listener for a specific [Event] type.
    pub fn add_listener<E: 'static, F: Fn(&E) + 'static>(&mut self, listener: F) {
        let type_id = TypeId::of::<E>();
        self.listeners
            .entry(type_id)
            .or_default()
            .push(Box::new(move |event| {
                listener(event.downcast_ref::<E>().unwrap());
            }));
    }

    /// Dispatches an [Event] to all listeners of its type.
    pub fn dispatch<E: 'static>(&self, event: &E) {
        if let Some(listeners) = self.listeners.get(&TypeId::of::<E>()) {
            for listener in listeners {
                listener(event);
            }
        }
    }
}
