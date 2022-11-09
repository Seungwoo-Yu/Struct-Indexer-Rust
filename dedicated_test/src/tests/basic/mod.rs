use std::any::Any;
use std::borrow::Borrow;
use struct_indexer_core::{InstanceOf, SameTypeWith};
use crate::tests::basic::models::a::A;
use crate::tests::basic::models::b::B;

pub mod models;

#[test]
fn test1() {
    let a = A {};
    let b = B {};
    let cloned_b = b.clone();

    let vector: Vec<Box<dyn Any>> = vec![Box::new(a), Box::new(b), Box::new(a)];

    // Borrow value
    let vec_a: &dyn Any = vector.get(0).unwrap().borrow();

    // Downcast to A
    let vec_a_casted = vec_a.downcast_ref::<A>().unwrap();

    // TODO: Find out reason why instance_of and same_type_with are not shown up automatically in IDE code completion
    assert_eq!(vec_a_casted.same_type_with(&cloned_b).unwrap(), false);
    assert_eq!(vec_a_casted.instance_of::<A>().unwrap(), true);
}