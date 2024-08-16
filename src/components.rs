use bevy::prelude::*;

#[derive(Component)]
pub struct Row(pub u8);

#[derive(Component)]
pub struct Lane {
    pub index: u8,
}