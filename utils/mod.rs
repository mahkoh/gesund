use std::rc::{Rc};
use std::cell::{RefCell, Cell};

pub type RefMut<T> = Rc<RefCell<T>>;
pub type CopyMut<T> = Rc<Cell<T>>;
