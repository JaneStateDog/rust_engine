use crate::component::Component;

use std::{collections::HashMap, any::Any, cell::RefCell};

pub trait Storage {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// The key in this hashmap is an entity_id.
// entity_ids are randomly generated and compared to a list of currently used ids
// to make sure the generated id is new and unique.
// When getting a component from an entity, we don't need the hasmap to store Option<C>,
// because it will simply just fail to give us anything if no component is at that entity_id.
// We can also remove entities or remove components from entities with hashmap's "remove()".
impl<C> Storage for RefCell<HashMap<usize, C>>
where
    C: Component,
{
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}