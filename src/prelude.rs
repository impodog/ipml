pub use std::cell::{Cell, Ref, RefCell, RefMut};
pub use std::collections::{HashMap, VecDeque};
pub use std::error::Error;
pub use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
pub use std::rc::{Rc, Weak};

pub use crate::error::*;
pub use crate::parser::*;
pub use crate::scope::*;
pub use crate::value::*;

#[inline(always)]
pub(crate) fn cell<T>(v: T) -> RefCell<T> {
    RefCell::new(v)
}

#[inline(always)]
pub(crate) fn rc_cell<T>(v: T) -> Rc<RefCell<T>> {
    Rc::new(cell(v))
}
