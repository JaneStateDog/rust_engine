use ecs::{world::World, component::Component};

struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {}

fn main() {
    let mut world = World::new();

    let entity_1 = world.new_entity();
    world.add_component_to_entity(entity_1, Position{ x: 10, y: 12 });

    let mut borrowed = world.borrow_storage::<Position>().unwrap();
    for (entity_id, position) in borrowed.iter_mut() {
        println!("{}: {}, {}", entity_id, position.x, position.y);
    }
}