use crate::{Component, Entity};
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct World {
    pub entities: Vec<Entity>,
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
    next_entity_id: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            next_entity_id: 0,
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let entity = Entity::new(self.next_entity_id);
        self.next_entity_id += 1;
        self.entities.push(entity);
        entity
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        self.components
            .entry(TypeId::of::<T>())
            .or_default()
            .insert(entity, Box::new(component));
    }

    pub fn get_component<T: Component + 'static>(&self, entity: Entity) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())?
            .get(&entity)
            .and_then(|c| c.downcast_ref())
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
