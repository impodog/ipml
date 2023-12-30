pub use std::cell::{Cell, Ref, RefCell, RefMut};
pub use std::collections::{HashMap, VecDeque};
pub use std::error::Error;
pub use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
pub use std::rc::{Rc, Weak};

pub use crate::error::*;
pub use crate::init::*;
pub use crate::parser::*;
pub use crate::scope::*;
pub use crate::value::*;

pub static ANONYMOUS: &'static str = "[anonymous]";
pub static RETURN: &'static str = "ret";

pub(crate) type RcCell<T> = Rc<RefCell<T>>;

#[inline(always)]
pub(crate) fn cell<T>(v: T) -> RefCell<T> {
    RefCell::new(v)
}

#[inline(always)]
pub fn rc_cell<T>(v: T) -> RcCell<T> {
    Rc::new(cell(v))
}

#[inline(always)]
pub(crate) fn rc_clone<T>(v: Rc<T>) -> Rc<T>
where
    T: Clone,
{
    Rc::new((*v).clone())
}
