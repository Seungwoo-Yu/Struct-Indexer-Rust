use struct_indexer_macro::{Indexed, ToAnyTrait};
use crate::tests::trait_based::models::base_trait::BaseTrait;

#[derive(Indexed, ToAnyTrait, Clone)]
pub struct B {}

impl BaseTrait for B {
    fn hello_world(&self) -> &str {
        "Hello B world!"
    }
}

impl Default for B {
    fn default() -> Self {
        B {}
    }
}