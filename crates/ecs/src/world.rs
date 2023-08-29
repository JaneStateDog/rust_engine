use crate::{
    component::*,
    entity::*,
    storage::*,
};

use std::{
    cell::{
        RefCell,
        RefMut,
    },
    collections::HashMap,
};

pub struct World {
    pub entity_count: usize,
    pub storages: Vec<Box<dyn Storage>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            storages: Vec::new(),
        }
    }

    pub fn spawn_entity(&mut self) -> EntityBuilder {
        let entity_id = self.entity_count;
        self.entity_count += 1;

        EntityBuilder::new(self, entity_id)
    }

    pub fn borrow_storage<T: 'static + Component>(&self) -> Option<RefMut<HashMap<EntityID, T>>> {
        for storage in self.storages.iter() {
            if let Some(i) = storage.as_any().downcast_ref::<RefCell<HashMap<EntityID, T>>>() {
                return Some(i.borrow_mut());
            }
        }

        None
    }
}