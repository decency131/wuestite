use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};

use crate::{event::Event, sparse_set::SparseSet, Entity};

/// Represents the game world, containing [Entity], [`Component`](crate::Component), and [Event].
pub struct World {
    pub components: HashMap<TypeId, Box<dyn Any>>,
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
            components: HashMap::new(),
            next_entity_id: 0,
            events: HashMap::new(),
        }
    }

    /// Spawns a new [Entity] and returns it.
    pub fn spawn(&mut self) -> Entity {
        let entity = Entity::new(self.next_entity_id);
        self.next_entity_id += 1;
        entity
    }

    /// Adds a [`Component`](crate::Component) to the given [Entity].
    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let entry = self
            .components
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<T>::new()));

        if let Some(sparse_set) = entry.downcast_mut::<SparseSet<T>>() {
            sparse_set.insert(entity.id(), component);
        }
    }

    /// Returns a reference to the [`Component`](crate::Component) of the given type for the given [Entity].
    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .downcast_ref::<SparseSet<T>>()?
            .get(entity.id())
    }

    /// Returns a mutable reference to the [`Component`](crate::Component) of the given type for the given [Entity].
    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)?
            .downcast_mut::<SparseSet<T>>()?
            .get_mut(entity.id())
    }

    /// Removes the [`Component`](crate::Component) of the given type from the given [Entity] and returns it.
    pub fn remove_component<T: 'static>(&mut self, entity: Entity) -> Option<T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)?
            .downcast_mut::<SparseSet<T>>()?
            .remove(entity.id())
    }

    /// Despawns the given [Entity].
    pub fn despawn(&mut self, entity: Entity) {
        for components in self.components.values_mut() {
            if let Some(sparse_set) = components.downcast_mut::<SparseSet<()>>() {
                sparse_set.remove(entity.id());
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

    pub fn next_entity_id(&self) -> u64 {
        self.next_entity_id
    }
}
