use bevy::{
    input::Input,
    math::Vec3,
    prelude::{
        App, Color, Commands, Component, Entity, KeyCode, OrthographicCameraBundle, Query, Res,
        Transform, UiCameraBundle, SystemSet,
    },
    sprite::{Sprite, SpriteBundle},
    DefaultPlugins, core::FixedTimestep,
};

#[derive(Debug, Component)]
struct Player {
    name: String,
}

fn player_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform)>) {
    println!("System...");
    for (player, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Right) {
            println!("{:?} : RIGHT", player);
            transform.translation.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            println!("{:?} : LEFT", player);
            transform.translation.x -= 1.0;
        }
    }
}

fn create_entities(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(255.0, 255.0, 255.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(50.0, 50.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_entities)
        //.add_system(player_input)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                //.with_system(paddle_movement_system)
                //.with_system(ball_collision_system)
                .with_system(player_input),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
