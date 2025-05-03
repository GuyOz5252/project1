mod player;
mod components;

use bevy::prelude::*;
use crate::components::Velocity;
use crate::player::PlayerPlugin;

const SPRITE_SCALE: f32 = 0.2;
const PLAYER_MOVEMENT_SPEED: f32 = 300.0;
const PLAYER_SPRINT_MULTIPLIER: f32 = 2.0;
const PLAYER_ACCELERATION: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, movement_system)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d {},
        Transform {
            translation: Vec3::ZERO,
            ..Default::default()
        },
    ));
}

fn movement_system(
    mut query: Query<(&Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.velocity * time.delta_secs();
    }
}
