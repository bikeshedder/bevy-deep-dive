use bevy::{prelude::{App, Commands, Component, Entity, Query, Transform}, DefaultPlugins};

#[derive(Debug, Component)]
struct Player {
    name: String,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn print_position_system(query: Query<(Option<&Player>, &Position)>) {
    for (player, pos) in query.iter() {
        println!("{:?}: {}:{}", player, pos.x, pos.y);
    }
}

fn create_entities(mut commands: Commands) {
    commands
        .spawn()
        .insert(Position { x: 42.0, y: 0.0 })
        .insert(Player {
            name: "Benny".into(),
        });
    commands
        .spawn()
        .insert(Position { x: 13.0, y: 0.0 });
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_entities)
        .add_system(print_position_system)
        .run();
}
