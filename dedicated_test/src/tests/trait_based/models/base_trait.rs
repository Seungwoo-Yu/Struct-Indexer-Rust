use struct_indexer_core::{Indexed, ToAnyTrait};

pub trait BaseTrait : Indexed + ToAnyTrait {
    fn hello_world(&self) -> &str {
        "Hello world"
    }
}
