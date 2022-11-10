use struct_indexer_macro::{Indexed, ToAnyTrait};
use crate::tests::trait_based::models::base_trait::BaseTrait;

#[derive(Indexed, ToAnyTrait, Clone)]
pub struct A {}

impl BaseTrait for A {
    fn hello_world(&self) -> &str {
        "Hello A world!"
    }
}

impl Default for A {
    fn default() -> Self {
        A {}
    }
}