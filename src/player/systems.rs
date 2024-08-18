use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

use crate::constants::*;
use crate::obstacles::events::PlayerHitEvent;
use super::components::Player;
use crate::components::Row;

use crate::helpers::row_to_y_pos;

pub fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands
)
{
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/kenney_animal-pack/PNG/Round/penguin.png"),
            transform: Transform::from_xyz(0.0, 0.0, PLAYER_LAYER).with_scale(Vec3::new(0.2, 0.2, 0.0)),
            ..default()
        },
        Player,
        Row(4)
    ));
}

pub fn player_input(
    button_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Row), With<Player>>
){
    if let Ok((mut player_transform, mut row)) = player_query.get_single_mut(){
        if button_input.pressed(KeyCode::ArrowLeft) {
            player_transform.translation += Vec3::new(PLAYER_HORIZONTAL_MOVEMENT_SPEED * -1.0, 0.0, 0.0);
        }else if button_input.pressed(KeyCode::ArrowRight)
        {
            player_transform.translation += Vec3::new(PLAYER_HORIZONTAL_MOVEMENT_SPEED, 0.0, 0.0);
        }else if button_input.just_pressed(KeyCode::ArrowUp) {
            row.0 = if row.0 < NUMBER_OF_ROWS - 1 { row.0 + 1 } else { row.0 };
        }else if button_input.just_pressed(KeyCode::ArrowDown)
        {
            row.0 = if row.0 > 1 { row.0 - 1} else { row.0 };
        }
    }
}

pub fn on_row_updated(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &Row), With<Player>>
)
{
    if let (Ok((mut transform, row)), Ok(window)) 
        = (player_query.get_single_mut(), window_query.get_single()){
        transform.translation = Vec3::new(transform.translation.x, row_to_y_pos(row.0, window.height()), transform.translation.z);
    }
}

pub fn on_player_hit(
    mut player_hit_event_reader: EventReader<PlayerHitEvent>,
    mut player_query: Query<(&mut Transform, &mut Row, Entity), With<Player>>
)
{
    for event in player_hit_event_reader.read(){
        for (mut transform, mut row, player_entity) in player_query.iter_mut(){
            if player_entity == event.player_entity{
                transform.translation = Vec3::new(0.0, transform.translation.y, transform.translation.z);
                row.0 = 1;
            }
        }
    }
}