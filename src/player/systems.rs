use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

use crate::constants::*;
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
            player_transform.translation += Vec3::new(-10.0, 0.0, 0.0);
        }else if button_input.pressed(KeyCode::ArrowRight)
        {
            player_transform.translation += Vec3::new(10.0, 0.0, 0.0);
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
        println!("Translation before Row Update: {}", transform.translation);
        transform.translation = Vec3::new(transform.translation.x, row_to_y_pos(row.0, window.height()), transform.translation.z);
        println!("Translation after Row Update: {}", transform.translation);
    }
}