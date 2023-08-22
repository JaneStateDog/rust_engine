use ecs::{
    entity::*,
    component::*,
    component,
};
use ecs_derive::*;

use std::{
    rc::Rc,
    cell::RefCell,
};

fn main() {
    #[derive(ComponentTrait, Debug)]
    struct Player { move_speed: i32 }
    #[derive(ComponentTrait, Debug)]
    struct Light { intensity: i32 }

    let mut entity = Entity::new(vec![
        component!(Player { move_speed: 10 }),
        component!(Light { intensity: 100 }),
    ]);

    println!("{:?}", entity.get_components_from_name("Player"));

    {
        let start = entity.get_components_from_name("Player").unwrap().get(0).unwrap();
        let mut b = start.borrow_mut();
        let mut c = b.as_any().downcast_mut::<Player>().unwrap();

        println!("{:?}", c);
        c.move_speed = 3;
        println!("{:?}", c);
    }

    println!("{:?}", entity.get_components_from_name("Player"));
}