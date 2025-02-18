use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::ecs::entity::Entity;
use crate::ecs::component::Component;

pub struct World {
    pub entities: Vec<Entity>,
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let entity = Entity::new(self.entities.len());
        self.entities.push(entity);
        entity
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        self.components
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity, Box::new(component));
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .get(&entity)
            .and_then(|c| c.downcast_ref::<T>())
    }
}