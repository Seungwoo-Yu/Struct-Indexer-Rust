use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use struct_indexer_core::{InstanceOf, SameTypeWith, ToNamedArcStruct, ToNamedBoxStruct, ToNamedRcStruct};
use crate::tests::trait_based::models::a::A;
use crate::tests::trait_based::models::b::B;
use crate::tests::trait_based::models::base_trait::BaseTrait;

pub mod models;

#[test]
fn test1() {
    // Add A and B instances into vector as BaseTrait
    let mut container: Vec<Box<dyn BaseTrait>> = vec![Box::new(A {}), Box::new(B {})];

    // To get each instance from vector,
    // vector will have to pop instances so they can be owned value.

    // Remember that only single Box indicates single instance (using smart pointer).
    // Therefore, Box and data inside Box cannot be copied.

    // Get B instance
    let b_box = container.pop().unwrap().to_named_box_struct::<B>().unwrap();
    let b = b_box.deref();

    // Get A instance
    let a_box = container.pop().unwrap().to_named_box_struct::<A>().unwrap();
    let a = a_box.deref();

    println!("{}", a.hello_world());
    println!("{}", b.hello_world());

    assert_eq!(a.same_type_with(b).unwrap(), false);
    assert_eq!(b.instance_of::<A>().unwrap(), false);
}

#[test]
fn test2() {
    let rc_vec: Vec<Rc<dyn BaseTrait>> = vec![Rc::new(A {}), Rc::new(B {})];

    // Unlike Box, reference of Rc can be cloned!

    let rc_ref_a = rc_vec.get(0).unwrap();
    // Clone new reference for A and change its type to named struct
    let rc_cloned_a = Rc::clone(rc_ref_a).to_named_rc_struct::<A>().unwrap();

    let rc_ref_b = rc_vec.get(1).unwrap();
    // Clone new reference for B and change its type to named struct
    let rc_cloned_b = Rc::clone(rc_ref_b).to_named_rc_struct::<B>().unwrap();

    println!("{}", rc_cloned_a.hello_world());
    println!("{}", rc_cloned_b.hello_world());

    assert_eq!(rc_cloned_a.instance_of::<B>().unwrap(), false);
    assert_eq!(rc_cloned_b.same_type_with(rc_cloned_b.deref()).unwrap(), true);

    let rc_vec: Vec<Arc<dyn BaseTrait>> = vec![Arc::new(A {}), Arc::new(B {})];

    // Unlike Box, reference of Arc can be cloned!

    let rc_ref_a = rc_vec.get(0).unwrap();
    // Clone new reference for A and change its type to named struct
    let rc_cloned_a = Arc::clone(rc_ref_a).to_named_arc_struct::<A>().unwrap();

    let rc_ref_b = rc_vec.get(1).unwrap();
    // Clone new reference for B and change its type to named struct
    let rc_cloned_b = Arc::clone(rc_ref_b).to_named_arc_struct::<B>().unwrap();

    println!("{}", rc_cloned_a.hello_world());
    println!("{}", rc_cloned_b.hello_world());

    assert_eq!(rc_cloned_a.instance_of::<B>().unwrap(), false);
    assert_eq!(rc_cloned_b.same_type_with(rc_cloned_b.deref()).unwrap(), true);
}
