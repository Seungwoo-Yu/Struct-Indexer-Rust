use std::ops::Deref;
use struct_indexer_core::{InstanceOf, SameTypeWith, ToNamedStruct};
use crate::tests::trait_based::models::a::A;
use crate::tests::trait_based::models::b::B;
use crate::tests::trait_based::models::base_trait::BaseTrait;

pub mod models;

#[test]
fn test1() {
    // Add A and B instances into vector as BaseTrait
    let mut container: Vec<Box<dyn BaseTrait>> = vec![Box::new(A {}), Box::new(B {})];

    // To get reference of each instance from vector,
    // vector will have to pop instances so they can be owned value.

    // Remember that only single Box indicates single instance (using smart pointer).
    // Therefore, Box and data inside Box cannot be copied.

    // Get B instance
    let b_box = container.pop().unwrap().to_named_struct::<B>().unwrap();
    let b = b_box.deref();

    // Get A instance
    let a_box = container.pop().unwrap().to_named_struct::<A>().unwrap();
    let a = a_box.deref();

    println!("{}", a.hello_world());
    println!("{}", b.hello_world());

    assert_eq!(a.same_type_with(b).unwrap(), false);
    assert_eq!(b.instance_of::<A>().unwrap(), false);
}