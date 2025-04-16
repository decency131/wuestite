use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};

use crate::{event::Event, Entity};

/// Represents the game world, containing [Entity], [`Component`](crate::Component), and [Event].
pub struct World {
    pub entities: Vec<Entity>,
    pub components: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    next_entity_id: u64,
    pub events: HashMap<TypeId, VecDeque<Box<dyn Any>>>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Creates a new empty [World].
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            next_entity_id: 0,
            events: HashMap::new(),
        }
    }

    /// Spawns a new [Entity] and returns it.
    pub fn spawn(&mut self) -> Entity {
        let entity = Entity::new(self.next_entity_id);
        self.next_entity_id += 1;
        self.entities.push(entity);
        entity
    }

    /// Adds a [`Component`](crate::Component) to the given [Entity].
    pub fn add_component<T: 'static + Any>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let components = self.components.entry(type_id).or_default();

        while entity.id() >= components.len() as u64 {
            components.push(None);
        }

        components[entity.id() as usize] = Some(Box::new(component));
    }

    /// Returns a reference to the [`Component`](crate::Component) of the given type for the given [Entity].
    pub fn get_component<T: 'static + Any>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .get(entity.id() as usize)?
            .as_ref()
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Returns a mutable reference to the [`Component`](crate::Component) of the given type for the given [Entity].
    pub fn get_component_mut<T: 'static + Any>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)?
            .get_mut(entity.id() as usize)?
            .as_mut()
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// Removes the [`Component`](crate::Component) of the given type from the given [Entity] and returns it.
    pub fn remove_component<T: 'static + Any>(&mut self, entity: Entity) -> Option<Box<dyn Any>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)?
            .get_mut(entity.id() as usize)?
            .take()
    }

    /// Despawns the given [Entity].
    pub fn despawn(&mut self, entity: Entity) {
        self.entities.retain(|&e| e != entity);
        for components in self.components.values_mut() {
            if entity.id() < components.len() as u64 {
                components[entity.id() as usize] = None;
            }
        }
    }

    /// Sends an [Event] to the [World].
    pub fn send_event<E: Event + 'static>(&mut self, event: E) {
        let type_id = TypeId::of::<E>();
        self.events
            .entry(type_id)
            .or_default()
            .push_back(Box::new(event));
    }

    /// Returns all the [Event]s of the given type in the [World] and clears them.
    pub fn get_events<E: Event + 'static>(&mut self) -> Vec<E> {
        let type_id = TypeId::of::<E>();
        if let Some(events) = self.events.get_mut(&type_id) {
            events
                .drain(..)
                .map(|boxed| *boxed.downcast::<E>().unwrap())
                .collect()
        } else {
            Vec::new()
        }
    }
}
