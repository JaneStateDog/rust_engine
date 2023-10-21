use crate::{storage::Storage, component::Component};

use std::{collections::HashMap, any::TypeId, cell::{RefCell, RefMut}};
use rand::random;

pub struct World {
    // The key is the type of storage and the value is the storage itself
    box_storages: HashMap<TypeId, Box<dyn Storage>>,

    entity_ids: Vec<usize>,
}

impl World {
    pub fn new() -> Self {
        Self {
            box_storages: HashMap::new(),
            entity_ids: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = random::<usize>();
        self.entity_ids.push(entity_id);

        entity_id
    }

    pub fn add_component_to_entity<C: Component>(&mut self, entity_id: usize, component: C) {
        // Get the Box of the storage we are looking for
        if let Some(box_storage) = self.box_storages.get_mut(&TypeId::of::<C>()) {
            // Get the raw storage
            if let Some(storage) = box_storage.as_any_mut().downcast_mut::<RefCell<HashMap<usize, C>>>() {
                storage.get_mut().insert(entity_id, component);
                return;
            }
        }

        // If we failed to insert the component into the storage,
        // that means the correct storage does not exist yet.
        // Let's create it.
        let mut new_storage: HashMap<usize, C> = HashMap::new();
        new_storage.insert(entity_id, component);

        // Insert the newly created storage into the world's box_storages hashmap.
        self.box_storages.insert(TypeId::of::<C>(), Box::new(RefCell::new(new_storage)));
    }

    pub fn borrow_storage<C: Component>(&self) -> Option<RefMut<HashMap<usize, C>>> {
        if let Some(box_storage) = self.box_storages.get(&TypeId::of::<C>()) {
            if let Some(storage) = box_storage.as_any().downcast_ref::<RefCell<HashMap<usize, C>>>() {
                return Some(storage.borrow_mut());
            }
        }

        None
    }
}