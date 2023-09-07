use crate::component::Component;

use std::{
    any::Any,
    cell::RefCell,
    fmt::{ Debug, Formatter, Result },
};

// -- STORAGE --
pub type Storage<C> = Vec<Option<C>>;

// -- ANY STORAGE --
pub trait AnyStorage {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn push_none(&mut self);

    fn drop_entity(&mut self, entity_id: usize);

    fn debug_text(&self) -> String;
}

impl<C> AnyStorage for RefCell<Storage<C>>
where
    C: Component,
{
    fn as_any(&self) -> &dyn Any { self as &dyn Any }
    fn as_any_mut(&mut self) -> &mut dyn Any { self as &mut dyn Any }

    fn push_none(&mut self) {
        self.borrow_mut().push(None);
    }

    fn drop_entity(&mut self, entity_id: usize) {
        self.borrow_mut()[entity_id] = None;
    }

    fn debug_text(&self) -> String {
        format!("{:?}", self.borrow()).into()
    }
}

impl Debug for dyn AnyStorage {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.debug_text())
    }
}