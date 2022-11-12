use std::rc::Rc;
use std::sync::Arc;
use struct_indexer_core::{ContainerConvert, exclude_container, InstanceOf, SameTypeWith};
use crate::tests::container::models::a::A;
use crate::tests::container::models::b::B;

pub mod models;

#[test]
fn test1() {
    let nested_container_a = Box::new(Box::new(Box::new(A {})));
    let quite_nested_container_b = Box::new(Arc::new(Box::new(Rc::new(B {}))));

    // Exclude nested containers for A
    let a = exclude_container(
        exclude_container(
            exclude_container(
                nested_container_a.to_container()
            ).to_container()
        ).to_container()
    );

    // Exclude nested containers for B
    let b = exclude_container(
        exclude_container(
            exclude_container(
                exclude_container(
                    quite_nested_container_b.to_container()
                ).to_container()
            ).to_container()
        ).to_container()
    );

    assert_eq!(a.same_type_with(&b).unwrap(), false);
    assert_eq!(b.instance_of::<A>().unwrap(), false);
}