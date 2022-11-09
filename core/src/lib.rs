pub mod errors;

use std::any::Any;
use std::borrow::BorrowMut;
use std::cell::Cell;
use std::sync::{Mutex, MutexGuard, PoisonError};
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