use crate::{
    component::Component,
    entity::EntityID,
};

use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
};

pub trait Storage {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> Storage for RefCell<HashMap<EntityID, T>>
where
    T: Component,
{
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}