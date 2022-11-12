use struct_indexer_macro::Indexed;

#[derive(Indexed, Clone, Copy)]
pub struct A {}

impl Default for A {
    fn default() -> Self {
        A {}
    }
}