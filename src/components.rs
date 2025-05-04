use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec3
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub speed_multiplier: f32,
    pub acceleration: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 1.0,
            speed_multiplier: 1.0,
            acceleration: 1.0,
        }
    }
}

#[derive(Component)]
pub struct Cursor;
