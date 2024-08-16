use bevy::prelude::*;
use bevy::app::Plugin;

pub(super) mod components;
pub(super) mod systems;

use systems::{on_player_hit, on_row_updated, player_input, spawn_player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_input, on_player_hit));
        app.add_systems(FixedUpdate, on_row_updated);
    }
}