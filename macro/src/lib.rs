#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput};
use struct_indexer_core::__index;

#[proc_macro_derive(Indexed)]
pub fn indexed(_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as DeriveInput);
    let name = input.clone().ident;
    let index = __index().expect("index not found");

    let output = quote! {
        impl struct_indexer_core::Indexed for #name {
            fn __id(&self) -> Result<usize, struct_indexer_core::errors::Error> {
                Ok(#index)
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(ToAnyTrait)]
pub fn to_any_trait(_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as DeriveInput);
    let name = input.clone().ident;

    let output = quote! {
        impl struct_indexer_core::ToAnyTrait for #name {
            fn __struct_indexer_to_box_any(self: Box<Self>) ->
                Box<dyn std::any::Any> {
                self
            }

            fn __struct_indexer_to_rc_any(self: std::rc::Rc<Self>) ->
                std::rc::Rc<dyn std::any::Any> {
                self
            }

            fn __struct_indexer_to_arc_any(self: std::sync::Arc<Self>) ->
                std::sync::Arc<dyn std::any::Any + std::marker::Send + std::marker::Sync> {
                self
            }
        }
    };

    TokenStream::from(output)
}