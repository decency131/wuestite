use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};

use crate::sparse_set::SparseSet;
use crate::{event::Event, Entity};

/// Trait for types that can erase an [Entity] from the [World].
pub trait EraseEntity {
    fn erase_entity(&mut self, id: u64);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Trait for types that can be used as components in the [World].
impl<T: 'static> EraseEntity for SparseSet<T> {
    fn erase_entity(&mut self, id: u64) {
        self.remove(id);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Represents the game world, containing [Entity], [`Component`](crate::Component), and [Event].
pub struct World {
    pub components: HashMap<TypeId, Box<dyn EraseEntity>>,
    next_entity_id: u64,
    pub events: HashMap<TypeId, VecDeque<Box<dyn Any>>>,
}

/// Represents a default implementation for [World].
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
    pub fn add_component<T: 'static + Any>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let entry = self
            .components
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<T>::new()));
        let set = entry.as_any_mut().downcast_mut::<SparseSet<T>>().unwrap();
        set.insert(entity.id(), component);
    }

    /// Returns a reference to the [`Component`](crate::Component) of the given type for the given [Entity].
    pub fn get_component<T: 'static + Any>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .and_then(|boxed| boxed.as_any().downcast_ref::<SparseSet<T>>())
            .and_then(|set| set.get(entity.id()))
    }

    /// Returns a mutable reference to the [`Component`](crate::Component) of the given type for the given [Entity].
    pub fn get_component_mut<T: 'static + Any>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)
            .and_then(|boxed| boxed.as_any_mut().downcast_mut::<SparseSet<T>>())
            .and_then(|set| set.get_mut(entity.id()))
    }

    /// Removes the [`Component`](crate::Component) of the given type from the given [Entity] and returns it.
    pub fn remove_component<T: 'static + Any>(&mut self, entity: Entity) -> Option<T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)
            .and_then(|boxed| boxed.as_any_mut().downcast_mut::<SparseSet<T>>())
            .and_then(|set| set.remove(entity.id()))
    }

    /// Despawns the given [Entity].
    pub fn despawn(&mut self, entity: Entity) {
        for set in self.components.values_mut() {
            set.erase_entity(entity.id());
        }
    }

    /// Iterates over all [Entity]s that have the given [`Component`](crate::Component) type.
    pub fn iter_component_entities<T: 'static>(&self) -> impl Iterator<Item = u64> + '_ {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .and_then(|boxed| boxed.as_any().downcast_ref::<SparseSet<T>>())
            .map(|set| set.keys().copied())
            .into_iter()
            .flatten()
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
