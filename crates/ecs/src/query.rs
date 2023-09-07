use crate::{
    component::Component,
    storage::*,
    world::World,
};

use std::{
    marker::PhantomData,
    cell::{ RefCell, RefMut },
    slice::IterMut,
    any::TypeId,
    collections::HashMap,
};

// -- QUERY BUILDER --
pub struct QueryBuilder {
    //components: Vec<(usize, &'a mut dyn Component)>,
    withs: Vec<TypeId>,
    nots: Vec<TypeId>,
}

impl QueryBuilder {
    pub fn with<C: Component>(mut self) -> Self { 
        self.withs.push(TypeId::of::<C>()); 
        self
    }
    pub fn not<C: Component>(mut self) -> Self { 
        self.nots.push(TypeId::of::<C>()); 
        self
    }

    pub fn build(self, world: &World) {
        let mut component_types: Vec<TypeId> = Vec::new();

        for query in self.withs {
            
        }
    }
}

#[macro_export]
macro_rules! type_id_to_ident {
    ($ty:ty) => {
        $ty
    };
}







/*
// -- QUERY FILTER --
pub trait QueryFilter {
    type Component: Component;
    type Filter: QueryFilter;

    fn get_ref_muts<'a, C2: Component>(
        world: &World,
        past: &mut Vec<(TypeId, RefMut<'a, Storage<C2>>)>,
    );
}

// -- WITH --
pub struct With<C: Component, F: QueryFilter> {
    c: PhantomData<C>,
    f: PhantomData<F>,
}

impl<C, F> QueryFilter for With<C, F>
where
    C: Component,
    F: QueryFilter,
{
    type Component = C;
    type Filter = F;

    fn get_ref_muts<'a, C2: Component>(
        world: &World,
        past: &mut Vec<(TypeId, RefMut<'a, Storage<C2>>)>,
    ) {
        if let Some(storage) = world.borrow_storage::<C2>() {
            past.push((TypeId::of::<Self::Filter>(), storage));
            Self::Filter::get_ref_muts::<Self::Filter::Component>(world, past);
        }
    }
}

// -- OR -- I DONT KNOW IF WE CAN DO THIS
pub struct Or<C: Component, F: QueryFilter> {
    c: PhantomData<C>,
    f: PhantomData<F>,
}

impl<C, F> QueryFilter for Or<C, F>
where
    C: Component,
    F: QueryFilter,
{

}

// -- STOP --
pub struct Stop {}

impl QueryFilter for Stop {

} 

// -- QUERY --
pub struct Query<'a, C: Component, F: QueryFilter> {
    pub storage: RefMut<'a, Storage<C>>,

    pub f: PhantomData<&'a F>,
    pub c: PhantomData<C>,
}

impl<'a, C, F> Query<'a, C, F> 
where
    C: Component,
    F: QueryFilter,
{
    pub fn iter<'b>(&'b mut self) -> QueryIter<'b, C, F> {
        QueryIter {
            iter_mut: self.storage.iter_mut(),

            f: PhantomData,
            c: PhantomData,
        }
    }
}

// -- QUERY ITER --
pub struct QueryIter<'a, C: Component, F: QueryFilter> {
    iter_mut: IterMut<'a, Option<C>>,

    f: PhantomData<F>,
    c: PhantomData<C>,
}

impl<'a, C, F> Iterator for QueryIter<'a, C, F> 
where
    C: Component,
    F: QueryFilter,
{
    type Item = &'a mut C;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(component) = self.iter_mut.next() {
            return component.as_mut();
        }

        None
    }
}
*/