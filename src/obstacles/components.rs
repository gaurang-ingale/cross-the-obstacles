use bevy::prelude::*;

#[derive(Component)]
pub struct Obstacle {
    pub x_index: u8,
    pub y_index: u8,
    pub speed: f32,
    pub progress: f32,
}