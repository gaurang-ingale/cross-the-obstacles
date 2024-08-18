use bevy::prelude::*;

#[derive(Component, PartialEq, Eq)]
pub struct Row(pub u8);

#[derive(Component)]
pub struct Lane {
    pub index: u8,
}

#[derive(Component)]
pub struct Background;