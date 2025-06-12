use std::collections::HashMap;

/// A sparse set is a data structure that allows efficient storage and retrieval of components associated with entities.
pub struct SparseSet<T> {
    sparse: HashMap<u64, usize>,
    dense_entities: Vec<u64>,
    dense_components: Vec<T>,
}

impl<T> SparseSet<T> {
    /// Creates a new empty `SparseSet`.
    pub fn new() -> Self {
        Self {
            sparse: HashMap::new(),
            dense_entities: Vec::new(),
            dense_components: Vec::new(),
        }
    }

    /// Inserts a component associated with an entity into the sparse set.
    pub fn insert(&mut self, entity: u64, component: T) {
        if self.sparse.contains_key(&entity) {
            let idx = self.sparse[&entity];
            self.dense_components[idx] = component;
        } else {
            let idx = self.dense_entities.len();
            self.sparse.insert(entity, idx);
            self.dense_entities.push(entity);
            self.dense_components.push(component);
        }
    }

    /// Retrieves a reference to the component associated with the given entity.
    pub fn get(&self, entity: u64) -> Option<&T> {
        self.sparse
            .get(&entity)
            .map(move |&idx| &self.dense_components[idx])
    }

    /// Retrieves a mutable reference to the component associated with the given entity.
    pub fn get_mut(&mut self, entity: u64) -> Option<&mut T> {
        let idx = *self.sparse.get(&entity)?;
        self.dense_components.get_mut(idx)
    }

    /// Remove the entity from the sparse set, removing its associated component.
    pub fn remove(&mut self, entity: u64) -> Option<T> {
        let idx = self.sparse.remove(&entity)?;
        let last = self.dense_entities.len() - 1;

        if idx != last {
            self.dense_entities.swap(idx, last);
            self.dense_components.swap(idx, last);
            let swapped_entity = self.dense_entities[idx];
            self.sparse.insert(swapped_entity, idx);
        }

        self.dense_entities.pop();
        let removed_component = self.dense_components.pop();
        removed_component
    }

    pub fn keys(&self) -> impl Iterator<Item = &u64> {
        self.dense_entities.iter()
    }
}
