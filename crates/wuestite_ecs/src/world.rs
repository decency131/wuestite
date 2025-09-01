use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::sparse_set::SparseSet;
use crate::Component;

/// The [World] struct manages entities and their associated components in an ECS architecture.
pub struct World {
    components: HashMap<TypeId, Box<dyn Any>>,
    next_entity_id: u64,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Creates a new, empty [World].
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            next_entity_id: 0,
        }
    }
    
    /// Spawns a new entity and returns its ID.
    pub fn spawn(&mut self) -> u64 {
        let eid = self.next_entity_id;
        self.next_entity_id += 1;
        eid
    }
    
    /// Despawns an entity.
    pub fn despawn(&mut self, entity: u64) {
        for storage in self.components.values_mut() {
            let _ = storage
                .downcast_mut::<SparseSet<Box<dyn Any>>>()
                .map(|s| s.remove(entity));
        }
    }

    /// Adds a [Component] to the specified entity.
    pub fn add_component<T: 'static + Component>(&mut self, entity: u64, component: T) {
        let cid = TypeId::of::<T>();
        if let Some(storage) = self.components.get_mut(&cid) {
            let sparse_set = storage.downcast_mut::<SparseSet<T>>().unwrap();
            sparse_set.insert(entity, component);
        } else {
            let mut sparse_set = SparseSet::<T>::new();
            sparse_set.insert(entity, component);
            self.components.insert(cid, Box::new(sparse_set));
        }
    }

    /// Removes a [Component] from the specified entity.
    pub fn remove_component<T: 'static + Component>(&mut self, entity: u64) {
        let cid = TypeId::of::<T>();
        if let Some(storage) = self.components.get_mut(&cid) {
            let sparse_set = storage.downcast_mut::<SparseSet<T>>().unwrap();
            sparse_set.remove(entity);
        }
    }
   
    /// Gets a reference to a [Component] of the specified type for the given entity.
    pub fn get_component<T: 'static + Component>(&self, entity: u64) -> Option<&T> {
        let cid = TypeId::of::<T>();
        self.components.get(&cid).and_then(|storage| {
            storage.downcast_ref::<SparseSet<T>>().unwrap().get(entity)
        })
    }

    /// Gets a mutable reference to a [Component] of the specified type for the given entity.
    pub fn get_component_mut<T: 'static + Component>(&mut self, entity: u64) -> Option<&mut T> {
        let cid = TypeId::of::<T>();
        self.components.get_mut(&cid).and_then(|storage| {
            storage.downcast_mut::<SparseSet<T>>().unwrap().get_mut(entity)
        })
    }
}
