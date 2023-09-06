use crate::{
    component::*,
    entity::*,
    storage::*,
    query::*,
};

use std::{
    cell::{ RefCell, RefMut, },
    any::TypeId,
    collections::HashMap,
    marker::PhantomData,
};

// -- WORLD --
pub struct World {
    pub entity_count: usize,
    pub entity_capacity: usize,
    pub open_entity_ids: Vec<usize>,

    pub storages: HashMap<TypeId, Box<dyn AnyStorage>>,
    pub component_types: Vec<TypeId>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            entity_capacity: 0,
            open_entity_ids: Vec::new(),

            storages: HashMap::new(),
            component_types: Vec::new(),
        }
    }

    pub fn register<C: Component>(&mut self) -> &mut Self {
        self.component_types.push(TypeId::of::<C>());
        self
    }

    pub fn spawn_entity(&mut self) -> EntityBuilder {
        // The entity id we use will prefer to be a dropped entity id
        // but if there are none, it will just get a new one.
        let mut entity_id = self.entity_count;
        if let Some(i) = self.open_entity_ids.pop() {
            entity_id = i;
        } else {
            self.entity_capacity += 1;

            // Make room for the new entity in all the storages.
            for (_, storage) in self.storages.iter_mut() {
                storage.push_none();
            }
        }

        self.entity_count += 1;

        EntityBuilder::new(self, entity_id)
    }

    pub fn drop_entity(&mut self, entity_id: usize) {
        for (_, storage) in self.storages.iter_mut() {
            storage.drop_entity(entity_id);
        }

        self.open_entity_ids.push(entity_id)
    }

    pub fn get_component_from_entity<C: Component>(&mut self, entity_id: usize) {
        if let Some(i) = self.storages.get(&TypeId::of::<C>()) {
            if let Some(storage) = i.as_any().downcast_ref::<RefCell<Storage<C>>>() {
                
            }
        }
    }

    pub fn borrow_storage<'a, C: Component>(&'a self) -> Option<RefMut<'a, Storage<C>>> {
        if let Some(i) = self.storages.get(&TypeId::of::<C>()) {
            if let Some(storage) = i.as_any().downcast_ref::<RefCell<Storage<C>>>() {
                return Some(storage.borrow_mut());
            }
        }

        None
    }

    /*pub fn new_query<'a, C: Component, F: QueryFilter>(&'a self) -> Query<'a, C, F> {
        if let Some(i) = self.storages.get(&TypeId::of::<C>()) {
            if let Some(storage) = i.as_any().downcast_ref::<RefCell<Storage<C>>>() {
                return Query {
                    storage: storage.borrow_mut(),
        
                    f: PhantomData,
                    c: PhantomData,
                };
            }
        }

        panic!("oops");
    }*/
}