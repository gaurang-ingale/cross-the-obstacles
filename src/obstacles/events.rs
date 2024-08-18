use bevy::prelude::*;


#[derive(Event)]
pub struct PlayerHitEvent{
    pub player_entity: Entity,
}