use bevy::{
    core::FixedTimestep,
    input::Input,
    math::{Vec2, Vec3},
    prelude::{
        App, AssetServer, Assets, Color, Commands, Component, Entity, KeyCode,
        OrthographicCameraBundle, Query, Res, ResMut, SystemSet, Transform, UiCameraBundle,
    },
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    DefaultPlugins,
};

#[derive(Debug, Component)]
struct Player {
    name: String,
    key_left: KeyCode,
    key_right: KeyCode,
    key_up: KeyCode,
    key_down: KeyCode,
}

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &mut TextureAtlasSprite)>,
) {
    for (player, mut transform, mut sprite) in query.iter_mut() {
        sprite.index = (sprite.index + 1) % 4;
        if keyboard_input.pressed(player.key_left) {
            transform.translation.x -= 1.0;
        }
        if keyboard_input.pressed(player.key_right) {
            transform.translation.x += 1.0;
        }
        if keyboard_input.pressed(player.key_down) {
            transform.translation.y -= 1.0;
        }
        if keyboard_input.pressed(player.key_up) {
            transform.translation.y += 1.0;
        }
    }
}

fn create_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let texture_handle = asset_server.load("player-base.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 12, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Player {
            name: "Player 1".into(),
            key_left: KeyCode::Left,
            key_right: KeyCode::Right,
            key_up: KeyCode::Up,
            key_down: KeyCode::Down,
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
