extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    quote! {
        impl hematite_ecs::Component for #name {}
    }
    .into()
}

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    quote! {
        impl hematite_ecs::Event for #name {}
    }
    .into()
}

#[proc_macro_derive(System)]
pub fn system_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    quote! {
        impl hematite_ecs::System for #name {
            fn run(&self, world: &mut hematite_ecs::World) {}
        }
    }
    .into()
}
