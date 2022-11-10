pub mod errors;

use std::any::Any;
use std::borrow::BorrowMut;
use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use crate::errors::Error;
use crate::errors::ErrorCodes::NoIdFound;

static LIST: Mutex<Cell<usize>> = Mutex::new(Cell::new(0));

/**
Is not Public API
 **/
#[doc(hidden)]
pub fn __index() -> Result<usize, PoisonError<MutexGuard<'static, Cell<usize>>>> {
    let mut a = match LIST.lock() {
        Ok(value) => value,
        Err(error) => {
            return Err(error);
        }
    };
    let result = a.get();

    *a.borrow_mut().get_mut() += 1;

    Ok(result)
}

pub trait Indexed {
    fn __id(&self) -> Result<usize, Error> {
        Err(Error { code: NoIdFound })
    }
}

pub trait SameTypeWith
    where
        Self: Any + Indexed {
    fn same_type_with<U: Sized + Any + Indexed>(&self, another: &U) -> Result<bool, Error>;
}

impl<T: Sized + Any + Indexed> SameTypeWith for T {
    fn same_type_with<U: Sized + Any + Indexed>(&self, another: &U) -> Result<bool, Error> {
        equal_to(self, another)
    }
}

pub trait InstanceOf
    where
        Self: Any + Indexed {
    fn instance_of<U: ?Sized + Any + Indexed + Default>(&self) -> Result<bool, Error>;
}

impl<T: Sized + Any + Indexed> InstanceOf for T {
    fn instance_of<U: ?Sized + Any + Indexed + Default>(&self) -> Result<bool, Error> {
        Ok(self.__id()? == U::default().__id()?)
    }
}

pub fn equal_to<T: Sized + Any + Indexed, T2: Sized + Any + Indexed>(
    instance1: &T,
    instance2: &T2,
) -> Result<bool, Error> {
    Ok(instance1.__id()? == instance2.__id()?)
}

pub enum Containers<T> {
    Box(Box<T>),
    Rc(Rc<T>),
    Arc(Arc<T>),
    Value(T)
}

pub trait ContainerConvert<T> {
    fn to_container(self) -> Containers<T>;
}

impl<T> ContainerConvert<T> for Box<T> {
    fn to_container(self) -> Containers<T> {
        Containers::Box(Box::from(self))
    }
}

impl<T> ContainerConvert<T> for Rc<T> {
    fn to_container(self) -> Containers<T> {
        Containers::Rc(self)
    }
}

impl<T> ContainerConvert<T> for Arc<T> {
    fn to_container(self) -> Containers<T> {
        Containers::Arc(self)
    }
}

pub fn exclude_container<T: Clone>(instance: Containers<T>) -> T {
        return match instance {
            Containers::Box(value) => {
                *value
            }
            Containers::Rc(value) => {
                value.deref().clone()
            }
            Containers::Arc(value) => {
                value.deref().clone()
            }
            Containers::Value(value) => {
                value
            }
        };
}

pub trait ToAnyTrait {
    fn __struct_indexer_to_box_any(self: Box<Self>) -> Box<dyn Any>;
    fn __struct_indexer_to_rc_any(self: Rc<Self>) -> Rc<dyn Any>;
    fn __struct_indexer_to_arc_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

pub trait ToNamedBoxStruct
    where
        Self: Any + ToAnyTrait {
    fn to_named_box_struct<U: Any>(self: Box<Self>) -> Result<Box<U>, Box<dyn Any>>;
}

impl<T: ?Sized + Any + ToAnyTrait> ToNamedBoxStruct for T {
    fn to_named_box_struct<U: Any>(self: Box<T>) -> Result<Box<U>, Box<dyn Any>> {
        self.__struct_indexer_to_box_any().downcast::<U>()
    }
}

pub trait ToNamedRcStruct
    where
        Self: Any + ToAnyTrait {
    fn to_named_rc_struct<U: Any>(self: Rc<Self>) -> Result<Rc<U>, Rc<dyn Any>>;
}

impl<T: ?Sized + Any + ToAnyTrait> ToNamedRcStruct for T {
    fn to_named_rc_struct<U: Any>(self: Rc<T>) -> Result<Rc<U>, Rc<dyn Any>> {
        self.__struct_indexer_to_rc_any().downcast::<U>()
    }
}

pub trait ToNamedArcStruct
    where
        Self: Any + ToAnyTrait {
    fn to_named_arc_struct<U: Any + Send + Sync>(self: Arc<Self>) -> Result<Arc<U>, Arc<dyn Any + Send + Sync>>;
}

impl<T: ?Sized + Any + ToAnyTrait> ToNamedArcStruct for T {
    fn to_named_arc_struct<U: Any + Send + Sync>(self: Arc<T>) -> Result<Arc<U>, Arc<dyn Any + Send + Sync>> {
        self.__struct_indexer_to_arc_any().downcast::<U>()
    }
}