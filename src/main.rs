use bevy::{prelude::{App, Commands, Component, Entity, Query, Transform, Color, OrthographicCameraBundle, UiCameraBundle}, DefaultPlugins, sprite::{SpriteBundle, Sprite}, math::Vec3};

#[derive(Debug, Component)]
struct Player {
    name: String,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn print_position_system(mut query: Query<(Option<&Player>, &mut Position)>) {
    for (player, mut pos) in query.iter_mut() {
        pos.y += 1.0;
        println!("{:?}: {}:{}", player, pos.x, pos.y);
    }
}

fn create_entities(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(255.0, 255.0, 255.0),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(10.0, 10.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
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
