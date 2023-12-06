use ecs::{world::World, component::Component};

struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {}

struct Empty {
    i: i32,
}

impl Component for Empty {}

fn main() {
    let mut world = World::new();

    let entity_1 = world.new_entity();
    world.add_component_to_entity(entity_1, Position{ x: 10, y: 12 });
    world.add_component_to_entity(entity_1, Empty{ i: 30 });

    let mut positions = world.borrow_storage::<Position>().unwrap();
    let mut empties = world.borrow_storage::<Empty>().unwrap();
    let zip = positions.iter_mut().zip(empties.iter_mut());
    let iter = zip.filter_map(|(position, empty)| Some((position.1, empty.1)));
    for (position, empty) in iter {
        //println!("{}: {}, {}", entity_id, position.x, position.y);
        println!("{}, {}", position.x, empty.i);
    }
}