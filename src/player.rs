use crate::components::{Movement, Player, Velocity};
use crate::{CAMERA_SPEED, PLAYER_ACCELERATION, PLAYER_MOVEMENT_SPEED, PLAYER_SPRINT_MULTIPLIER, SPRITE_SCALE};
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const KEY_DIRECTIONS: [(KeyCode, Vec3); 4] = [
    (KeyCode::KeyW, Vec3::Y),
    (KeyCode::KeyS, Vec3::NEG_Y),
    (KeyCode::KeyA, Vec3::NEG_X),
    (KeyCode::KeyD, Vec3::X),
];

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, camera_follow_player_system)
            .add_systems(Update, player_input_system)
            .add_systems(Update, player_sprint_system)
            .add_systems(Update, player_look_system);
    }
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    commands.spawn((
        Sprite {
            image: asset_server.load("images/soldier.png"),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        Player,
        Velocity {
            velocity: Vec3::ZERO,
        },
        Movement {
            speed: PLAYER_MOVEMENT_SPEED,
            acceleration: PLAYER_ACCELERATION,
            ..Default::default()
        },
    ));
}

fn camera_follow_player_system(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = camera_transform.translation.lerp(
        player_transform.translation,
        CAMERA_SPEED * time.delta_secs(),
    );
}

fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Movement), With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    for (key, single_direction) in KEY_DIRECTIONS.iter() {
        if keyboard.pressed(*key) {
            direction += single_direction;
        }
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    if let Ok((mut player_velocity, movement)) = query.get_single_mut() {
        player_velocity.velocity = player_velocity.velocity.lerp(
            direction * movement.speed * movement.speed_multiplier,
            movement.acceleration * time.delta_secs(),
        );
    }
}

fn player_sprint_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Movement, With<Player>>,
) {
    let mut movement = query.single_mut();

    if keyboard.pressed(KeyCode::ShiftLeft) {
        movement.speed_multiplier = PLAYER_SPRINT_MULTIPLIER;
    } else {
        movement.speed_multiplier = 1.0;
    }
}

fn player_look_system(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();
    let mut player_transform = player.single_mut();

    if let Some(curser_position) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, curser_position) {
            let direction = world_position - player_transform.translation.truncate();
            let angle = direction.y.atan2(direction.x);
            player_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
