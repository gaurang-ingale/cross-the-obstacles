use bevy::prelude::*;

#[derive(Component)]
pub struct Obstacle {
    //pub x_index: u8, //TODO:
    pub speed: f32,
    pub progress: f32,
}