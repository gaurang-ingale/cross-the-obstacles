use bevy::prelude::*;

#[derive(PartialEq, Eq, Clone ,Copy)]
pub enum ObstacleDirection{
    Left,
    Right
}


#[derive(Component)]
pub struct Obstacle {
    //pub x_index: u8, //TODO:
    pub speed: f32,
    pub progress: f32,
    pub direction: ObstacleDirection
}