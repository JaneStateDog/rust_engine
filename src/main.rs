use ecs::{
    component::Component,
    //system::System,
    world::World,
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

    let entity = world
        .spawn_entity()
            .with(Position { x: 100, y: 100 })
            .with(Player { move_speed: 5 })
        .build();

    let mut players = world.borrow_storage::<Player>().unwrap();
    let mut positions = world.borrow_storage::<Position>().unwrap();
    let zip = players.iter_mut().zip(positions.iter_mut());
    let iter = zip.filter_map(
        |(player, pos)| { 
            Some((Some(player.1)?, Some(pos.1)?))
        }
    );

    for (player, pos) in iter {
        if pos.x > 0 {
            pos.x = 5;
            println!("Woah, poggies");
        }
    }

    let zip = players.iter_mut().zip(positions.iter_mut());
    let iter = zip.filter_map(
        |(player, pos)| { 
            Some((Some(player.1)?, Some(pos.1)?))
        }
    );
    println!("{:?}", iter);
}