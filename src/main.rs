pub(crate) mod constants;
pub(crate) mod systems;
pub(crate) mod helpers;
pub(crate) mod components;

pub(crate) mod obstacles;
pub(crate) mod player;

use bevy::prelude::*;
use obstacles::ObstaclesPlugin;
use player::PlayerPlugin;

use crate::systems::spawn_camera;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, PlayerPlugin, ObstaclesPlugin))
    .add_systems(Startup, spawn_camera)
    .run();
}