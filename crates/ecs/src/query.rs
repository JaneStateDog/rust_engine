use crate::component::Component;

pub trait Query {
    //fn join() -> 
}

macro_rules! impl_query {
    ($($name: ident),*) => {
        impl<$($name: Component,)*> Query for ($($name,)*) {
            
        }
    };
}

impl_query!(A);
impl_query!(A, B);
impl_query!(A, B, C);
impl_query!(A, B, C, D);
impl_query!(A, B, C, D, E);
impl_query!(A, B, C, D, E, F);
impl_query!(A, B, C, D, E, F, G);
impl_query!(A, B, C, D, E, F, G, H);
impl_query!(A, B, C, D, E, F, G, H, I);
impl_query!(A, B, C, D, E, F, G, H, I, J);
impl_query!(A, B, C, D, E, F, G, H, I, J, K);
impl_query!(A, B, C, D, E, F, G, H, I, J, K, L);

pub trait QueryIter<'a> {
    type Iter: Iterator;
    fn iter(&'a mut self) -> Self::Iter;
}