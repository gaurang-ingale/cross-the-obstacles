use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use rand::Rng;

use super::components::Obstacle;
use crate::components::{Background, Lane};
use crate::helpers::row_to_y_pos;

use crate::constants::*;
use crate::obstacles::components::ObstacleDirection;

use super::super::player::components::Player;
use super::events::PlayerHitEvent;
use crate::components::Row;

pub fn on_resize_window(
    mut lane_query: Query<(&mut Sprite, &mut Transform, &Lane), (With<Lane>, Without<Background>)>,
    mut background_query: Query<&mut Sprite, (With<Background>, Without<Lane>)>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        for (mut sprite, mut transform, lane) in &mut lane_query {
            sprite.rect = Some(Rect {
                min: Vec2::new(0.0, 0.0),
                max: Vec2::new(e.width, 64.0),
            });
            *transform.as_mut() = Transform::from_xyz(0.0, row_to_y_pos(lane.index, e.height), 0.0)
                .with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 1.0));
        }
        for mut sprite in &mut background_query {
            sprite.rect = Some(Rect {
                min: Vec2::new(0.0, 0.0),
                max: Vec2::new(e.width, e.height),
            })
        }
    }
}

pub fn spawn_lanes(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if let Ok(window) = window_query.get_single() {
        for i in 2..NUMBER_OF_ROWS {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        rect: Some(Rect {
                            min: Vec2::new(0.0, 0.0),
                            max: Vec2::new(window.width(), window.height()),
                        }),
                        ..default()
                    },
                    texture: asset_server.load(
                        "sprites/kenney_topdown-roads/PNG/Default size/tileGrass1.png"
                    ),
                    transform: Transform::from_xyz(0.0, 0.0, -1.0),
                    ..default()
                },
                Background
            ));

            let lane_y = row_to_y_pos(i, window.height());
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        rect: Some(Rect {
                            min: Vec2::new(0.0, 0.0),
                            max: Vec2::new(window.width(), 64.0),
                        }),
                        ..default()
                    },
                    texture: asset_server.load(
                        "sprites/kenney_topdown-roads/PNG/Default size/tileGrass_roadEast.png",
                    ),
                    transform: Transform::from_xyz(0.0, lane_y, 0.0)
                        .with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 1.0)),
                    ..default()
                },
                Lane { index: i },
            ));

            const NUM_OBSTACLES: u8 = 4;
            let mut rng = rand::thread_rng();

            let speed_offset: f32 = rand::random::<f32>() % 1.0;
            let random_direction = rng.gen::<bool>();

            let mut flip_x: bool = false;

            let obstacle_direction: ObstacleDirection = if random_direction{
                flip_x = true;
                ObstacleDirection::Left
            }else {
                ObstacleDirection::Right
            };

            for j in 0..NUM_OBSTACLES {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            flip_x,
                            ..default()
                        },
                        texture: asset_server
                            .load("sprites/kenney_pixel-vehicle-pack/PNG/Cars/bus.png"),
                        ..default()
                    },
                    Obstacle {
                        //x_index: j, //TODO:
                        speed: 0.0003 * (i as f32 + speed_offset),
                        progress: j as f32 / NUM_OBSTACLES as f32,
                        direction: obstacle_direction
                    },
                    Row(i),
                ));
            }
        }
    }
}

pub fn obstacle_move(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut obstacle_query: Query<(&mut Transform, &mut Obstacle, &Row), With<Obstacle>>,
) {
    if let Ok(window) = window_query.get_single() {
        for (mut transform, mut obstacle, row) in &mut obstacle_query {
            obstacle.progress = (obstacle.progress + obstacle.speed) % 1.0;
            
            if obstacle.direction == ObstacleDirection::Right{
                *transform.as_mut() = Transform::from_xyz(
                    obstacle.progress * window.width() - window.width() / 2.0,
                    row_to_y_pos(row.0, window.height()),
                    1.0,
                );
            }else{
                *transform.as_mut() = Transform::from_xyz(
                    window.width() / 2.0 - obstacle.progress * window.width(),
                    row_to_y_pos(row.0, window.height()),
                    1.0,
                );
            }

        }
    }
}

pub fn on_player_hit(
    mut event_writer_player_hit: EventWriter<PlayerHitEvent>,
    obstacle_query: Query<(&Transform, &Handle<Image>, &Row), (With<Obstacle>, Without<Player>)>,
    player_query: Query<(&Transform, &Handle<Image>, &Row, Entity), (With<Player>, Without<Obstacle>)>,
    assets: Res<Assets<Image>>
) {
    for (obstacle_transform, obstacle_image_handle, obstacle_row) in obstacle_query.iter() {
        for (player_transform, player_image_handle, player_row, player_entity) in
            player_query.iter()
        {
            if obstacle_row != player_row {
                continue;
            }

            if let (Some(obstacle_image), Some(player_image)) = (
                assets.get(obstacle_image_handle),
                assets.get(player_image_handle),
            ) {
                let player_dimensions_uvec = player_image.size();
                let obstacle_dimensions_uvec = obstacle_image.size();

                let obstacle_dimensions = Vec2::new(
                    obstacle_dimensions_uvec.x as f32,
                    obstacle_dimensions_uvec.y as f32,
                );
                let player_dimensions = Vec2::new(
                    player_dimensions_uvec.x as f32,
                    player_dimensions_uvec.y as f32,
                );

                let obstacle_bounding_box = Rect::from_center_size(
                    obstacle_transform.translation.truncate(),
                    obstacle_dimensions * obstacle_transform.scale.truncate(),
                );
                let player_bounding_box = Rect::from_center_size(
                    player_transform.translation.truncate(),
                    player_dimensions * player_transform.scale.truncate(),
                );

                if !player_bounding_box
                    .intersect(obstacle_bounding_box)
                    .is_empty()
                {
                    event_writer_player_hit.send(PlayerHitEvent { player_entity });
                }
            } else {
                debug!("Could not get a hold of Obstacle Image or of Player Image");
            }
        }
    }
}
