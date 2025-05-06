mod components;
mod player;

use bevy::prelude::*;
use crate::components::{Cursor, Velocity};
use crate::player::PlayerPlugin;

const SPRITE_SCALE: f32 = 0.2;
const PLAYER_MOVEMENT_SPEED: f32 = 300.0;
const PLAYER_SPRINT_MULTIPLIER: f32 = 2.0;
const PLAYER_ACCELERATION: f32 = 10.0;
const CAMERA_SPEED: f32 = 3.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_cursor)
        .add_systems(Update, update_cursor)
        .add_systems(Update, movement_system)
        .run();
}

fn spawn_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.visible = false;
    }

    commands.spawn((
        Sprite {
            image: asset_server.load("images/cursor.png"),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 100.0),
            scale: Vec3::splat(0.6),
            ..Default::default()
        },
        Cursor,
    ));
}

fn update_cursor(
    window: Query<&Window>,
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera
            .viewport_to_world(camera_transform, cursor_pos)
            .map(|ray| ray.origin.truncate())
        {
            let mut transform = cursor_query.single_mut();
            transform.translation = world_pos.extend(100.0);
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn movement_system(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.velocity * time.delta_secs();
    }
}
