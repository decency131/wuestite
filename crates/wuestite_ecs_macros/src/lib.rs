extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for automatically implementing the `Component` trait.
#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    quote! {
        impl Component for #name {}
    }
    .into()
}

/// Derive macro for automatically implementing the `System` trait.
#[proc_macro_derive(System)]
pub fn system_derive(_input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    quote! {
        impl Event for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
    .into()
}
