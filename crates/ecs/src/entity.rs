use crate::{
    world::World,
    component::Component,
    storage::Storage,
};

use std::{
    cell::RefCell,
    any::TypeId,
};

// -- ENTITY BUILDER --
pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity_id: usize,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World, entity_id: usize) -> Self {
        Self {
            world,
            entity_id,
        }
    }

    pub fn with<C: Component>(self, component: C) -> Self {
        // Get the storage for the component we're given,
        if let Some(i) = self.world.storages.get_mut(&TypeId::of::<C>()) {
            // and if we're able to store it there,
            if let Some(storage) = i.as_any_mut().downcast_mut::<RefCell<Storage<C>>>() {
                // then store it.
                storage.get_mut()[self.entity_id] = Some(component);
                return self;
            }
        }

        // No matching component storage exists yet, so we'll make a new one
        // and fill it with None for all other entities.
        let mut new_storage: Storage<C> = Vec::new();
        for _ in 0..self.world.entity_capacity { new_storage.push(None); }

        // Store the component.
        new_storage[self.entity_id] = Some(component);

        // Give world the new component storage.
        self.world.storages.insert(TypeId::of::<C>(), Box::new(RefCell::new(new_storage)));
        
        self
    }

    pub fn build(&self) -> usize {
        self.entity_id
    }
}