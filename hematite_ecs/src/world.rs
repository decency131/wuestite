use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::entity::Entity;
use crate::component::Component;

pub trait World {
    fn new() -> Self;
    fn spawn(&mut self) -> Entity;
    fn add_component<T: Component>(&mut self, entity: Entity, component: T);
    fn get_component<T: Component>(&self, entity: Entity) -> Option<&T>;
}

pub struct BasicWorld {
    pub entities: Vec<Entity>,
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
}

impl World for BasicWorld {
    fn new() -> Self {
        BasicWorld {
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    fn spawn(&mut self) -> Entity {
        let entity = Entity::new(self.entities.len());
        self.entities.push(entity);
        entity
    }

    fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        self.components
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity, Box::new(component));
    }

    fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .get(&entity)
            .and_then(|c| c.downcast_ref::<T>())
    }
}