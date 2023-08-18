#[macro_use]
extern crate ecs;
use ecs::{
    entity::*,
    component::*,
};
use ecs_derive::ComponentTrait;

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
        component!(Player { move_speed: 15 }),
    ]);

    println!("{:?}", entity.get_components("Player"));

    {
        let a = Rc::clone(entity.get_components("Player").unwrap().get(0).unwrap());
        let mut b = a.borrow_mut();
        let mut component = as_mut!(b, Player).unwrap();

        println!("{:?}", component);
        component.move_speed = 3;
        println!("{:?}", component);
    }

    println!("{:?}", entity.get_components("Player"));
}