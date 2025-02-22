extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl hematite_ecs::Component for #name {}
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(System)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl hematite_ecs::System for #name {
            fn run(&self, world: &hematite_ecs::World) {
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(World)]
pub fn derive_world(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl hematite_ecs::World for #name {
            fn new() -> Self {
                #name {
                    entities: Vec::new(),
                    components: std::collections::HashMap::new(),
                }
            }

            fn spawn(&mut self) -> hematite_ecs::Entity {
                let entity = hematite_ecs::Entity::new(self.entities.len());
                self.entities.push(entity);
                entity
            }

            fn add_component<T: hematite_ecs::Component>(&mut self, entity: hematite_ecs::Entity, component: T) {
                let type_id = std::any::TypeId::of::<T>();
                self.components
                    .entry(type_id)
                    .or_insert_with(std::collections::HashMap::new)
                    .insert(entity, Box::new(component));
            }

            fn get_component<T: hematite_ecs::Component>(&self, entity: hematite_ecs::Entity) -> Option<&T> {
                let type_id = std::any::TypeId::of::<T>();
                self.components
                    .get(&type_id)?
                    .get(&entity)
                    .and_then(|c| c.downcast_ref::<T>())
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl hematite_ecs::Entity for #name {
            fn new(id: usize) -> Self {
                #name(id)
            }
        }
    };
    TokenStream::from(expanded)
}