use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Event: 'static + Send + Sync {}

pub struct EventDispatcher {
    listeners: HashMap<TypeId, Vec<Box<dyn Fn(&dyn Any)>>>,
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn add_listener<E: 'static, F: Fn(&E) + 'static>(&mut self, listener: F) {
        let type_id = TypeId::of::<E>();
        self.listeners
            .entry(type_id)
            .or_default()
            .push(Box::new(move |event| {
                listener(event.downcast_ref::<E>().unwrap());
            }));
    }

    pub fn dispatch<E: 'static>(&self, event: &E) {
        if let Some(listeners) = self.listeners.get(&TypeId::of::<E>()) {
            for listener in listeners {
                listener(event);
            }
        }
    }
}
