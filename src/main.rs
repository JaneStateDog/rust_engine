use ecs::{
    component::Component,
    world::World,
    query::*,
};

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}
impl Component for Position {}

#[derive(Debug)]
struct Player {
    move_speed: i32,
}
impl Component for Player {}

fn main() {
    let mut world = World::new();

    world
        .register::<Position>()
        .register::<Player>();

    /*let e1 = world.spawn_entity()
        .with(Position { x: 100, y: 100 })
        .with(Player { move_speed: 5 })
        .build();

    world.spawn_entity()
        .with(Player { move_speed: 2 })
        .build();

    println!("{:?}", world.storages);

    world.drop_entity(e1);

    println!("{:?}", world.storages);

    world.spawn_entity()
        .with(Position { x: 200, y: 300 })
        .with(Player { move_speed: 6 })
        .build();

    println!("{:?}", world.storages);*/

    /*let mut players = world.borrow_storage::<Player>().unwrap();
    let mut positions = world.borrow_storage::<Position>().unwrap();
    let zip = players.iter_mut().zip(positions.iter_mut());
    let iter = zip.filter_map(
        |(player, pos)| { 
            Some((player.as_mut()?, pos.as_mut()?))
        }
    );

    for (player, pos) in iter {
        if pos.x > 0 {
            pos.x = 5;
            println!("Woah, poggies");
        }
    }*/

    world.spawn_entity()
        .with(Position { x: 100, y: 100 })
        .with(Player { move_speed: 5 })
        .build();

    /*for player in world.new_query::<Player, Stop>().iter() {
        println!("{:?}", player)
    }*/
}