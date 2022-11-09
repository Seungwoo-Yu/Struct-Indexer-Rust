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
        use struct_indexer_core::errors::Error;

        impl struct_indexer_core::Indexed for #name {
            fn __id(&self) -> Result<usize, Error> {
                Ok(#index)
            }
        }
    };

    TokenStream::from(output)
}