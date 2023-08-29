use crate::{
    world::World,
    component::Component,
};

use std::{
    collections::HashMap,
    cell::RefCell,
};

pub type EntityID = usize;

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity_id: EntityID,
}
impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World, entity_id: EntityID) -> Self {
        Self {
            world,
            entity_id,
        }
    }

    pub fn with<T: Component>(self, component: T) -> Self {
        // Look through all the component storages
        for storage in self.world.storages.iter_mut() {
            // If we're able to store the component in this storage
            if let Some(i) = storage.as_any_mut().downcast_mut::<RefCell<HashMap<EntityID, T>>>() {
                // Store it
                i.get_mut().insert(self.entity_id, component);
                return self;
            }
        }

        // No matching component storage exists yet, so we'll make a new one.
        let mut new_storage: HashMap<EntityID, T> = HashMap::new();
        new_storage.insert(self.entity_id, component);
        
        self.world.storages.push(Box::new(RefCell::new(new_storage)));
        
        self
    }

    pub fn build(&self) -> EntityID {
        self.entity_id
    }
}