use std::{
    rc::Rc,
    cell::RefCell,
    any::Any,
};

// Components can not be unit or tuple structs. They can only be normal structs or enums.
// A component must always be owned/inside an Entity.
pub type Component = Rc<RefCell<dyn ComponentTrait>>;

pub trait ComponentTrait: std::fmt::Debug {
    fn name(&self) -> String;
    fn as_any(&mut self) -> &mut dyn Any;
}

#[macro_export]
macro_rules! component {
    ($e:expr) => {
        Rc::new(RefCell::new($e))
    };
}

/*pub fn get_raw_component<T: 'static>(component: &Rc<RefCell<dyn ComponentTrait>>) -> &mut T {
    let mut b = component.borrow_mut();
    b.as_any().downcast_mut::<T>().unwrap()
}*/