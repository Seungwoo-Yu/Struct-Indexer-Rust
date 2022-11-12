use struct_indexer_macro::Indexed;

#[derive(Indexed, Clone)]
pub struct B {}

impl Default for B {
    fn default() -> Self {
        B {}
    }
}