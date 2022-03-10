use bevy::{
    core::FixedTimestep,
    input::Input,
    math::Vec3,
    prelude::{
        App, Commands, Component, KeyCode, OrthographicCameraBundle, Query, Res, SystemSet,
        Transform, UiCameraBundle,
    },
    DefaultPlugins,
};
use bevy_spicy_aseprite::{
    AsepriteAnimation, AsepriteAnimationState, AsepriteBundle, AsepritePlugin,
};

mod sprites {
    use bevy_spicy_aseprite::aseprite;
    aseprite!(pub Player, "assets/player-base.aseprite");
}

const TIME_STEP: f32 = 1.0 / 60.0;

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
    mut query: Query<(&Player, &mut Transform, &mut AsepriteAnimation)>,
) {
    for (player, mut transform, mut animation) in query.iter_mut() {
        if keyboard_input.pressed(player.key_left) {
            if !animation.is_tag(sprites::Player::tags::WALK_RIGHT) {
                *animation = AsepriteAnimation::from(sprites::Player::tags::WALK_RIGHT);
            }
            transform.translation.x -= 5.0;
        }
        if keyboard_input.pressed(player.key_right) {
            if !animation.is_tag(sprites::Player::tags::WALK_RIGHT) {
                *animation = AsepriteAnimation::from(sprites::Player::tags::WALK_RIGHT);
            }
            transform.translation.x += 5.0;
        }
        if keyboard_input.pressed(player.key_down) {
            if !animation.is_tag(sprites::Player::tags::WALK_DOWN) {
                *animation = AsepriteAnimation::from(sprites::Player::tags::WALK_DOWN);
            }
            transform.translation.y -= 5.0;
        }
        if keyboard_input.pressed(player.key_up) {
            if !animation.is_tag(sprites::Player::tags::WALK_UP) {
                *animation = AsepriteAnimation::from(sprites::Player::tags::WALK_UP);
            }
            transform.translation.y += 5.0;
        }
    }
}

fn create_entities(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(AsepriteBundle {
            aseprite: sprites::Player::sprite(),
            animation: AsepriteAnimation::from(sprites::Player::tags::WALK_UP),
            transform: Transform {
                scale: Vec3::splat(4.0),
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AsepritePlugin)
        .add_startup_system(create_entities)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_input),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
