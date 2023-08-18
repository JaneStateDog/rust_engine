use std::{
    rc::Rc,
    cell::RefCell,
    any::Any,
};

// Components can not be unit or tuple structs. 
// They can only be normal structs or enums.
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

#[macro_export]
macro_rules! as_mut {
    ($e:expr, $t:ty) => {
        $e.as_any().downcast_mut::<$t>()
    };
}