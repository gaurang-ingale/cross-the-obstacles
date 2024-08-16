use bevy::prelude::*;
use bevy::app::Plugin;

pub(super) mod components;
pub(super) mod systems;

use systems::{spawn_lanes, obstacle_move, on_resize_window};

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_lanes);
        app.add_systems(FixedUpdate, obstacle_move);
        app.add_systems(Update, on_resize_window);
    }
}
