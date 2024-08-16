use bevy::prelude::*;
use bevy::app::Plugin;

pub(super) mod components;
pub(super) mod systems;

use systems::{spawn_player, player_input, on_row_updated};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_input);
        app.add_systems(FixedUpdate, on_row_updated);
    }
}