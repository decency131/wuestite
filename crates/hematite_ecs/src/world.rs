use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};

use crate::event::Event;
use crate::Entity;

pub struct World {
    pub entities: Vec<Entity>,
    components: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    next_entity_id: u64,
    events: HashMap<TypeId, VecDeque<Box<dyn Any>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            next_entity_id: 0,
            events: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let entity = Entity::new(self.next_entity_id);
        self.next_entity_id += 1;
        self.entities.push(entity);
        entity
    }

    pub fn add_component<T: 'static + Any>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let components = self.components.entry(type_id).or_default();

        while entity.id() >= components.len() as u64 {
            components.push(None);
        }

        components[entity.id() as usize] = Some(Box::new(component));
    }

    pub fn get_component<T: 'static + Any>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .get(entity.id() as usize)?
            .as_ref()
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn get_component_mut<T: 'static + Any>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)?
            .get_mut(entity.id() as usize)?
            .as_mut()
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    pub fn remove_component<T: 'static + Any>(&mut self, entity: Entity) -> Option<Box<dyn Any>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)?
            .get_mut(entity.id() as usize)?
            .take()
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.entities.retain(|&e| e != entity);
        for components in self.components.values_mut() {
            if entity.id() < components.len() as u64 {
                components[entity.id() as usize] = None;
            }
        }
    }

    pub fn send_event<E: Event + 'static>(&mut self, event: E) {
        let type_id = TypeId::of::<E>();
        self.events
            .entry(type_id)
            .or_default()
            .push_back(Box::new(event));
    }

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

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
