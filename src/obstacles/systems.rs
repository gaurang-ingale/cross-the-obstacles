use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};

use crate::helpers::row_to_y_pos;
use super::components::Obstacle;
use crate::components::Lane;

use crate::constants::*;

pub fn on_resize_window(
    mut lane_query: Query<(&mut Sprite, &mut Transform, &Lane), (With<Lane>, Without<Obstacle>)>,
    mut obstacle_query: Query<(&mut Transform, &mut Obstacle), (With<Obstacle>, Without<Lane>)>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        for (mut sprite, mut transform, lane) in &mut lane_query {
            sprite.rect = Some(Rect { min: Vec2::new(0.0, 0.0), max: Vec2::new(e.width, 64.0)});
            *transform.as_mut() = Transform::from_xyz(0.0, row_to_y_pos(lane.index, e.height), 0.0).with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 1.0));
        }
    }
}

pub fn spawn_lanes(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    if let Ok(window) = window_query.get_single() {
        for i in 2..NUMBER_OF_ROWS {
            let lane_y = row_to_y_pos(i, window.height());
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        rect: Some(Rect { min: Vec2::new(0.0, 0.0), max: Vec2::new(window.width(), 64.0)}),
                        ..default()
                    },
                    texture: asset_server.load("sprites/kenney_topdown-roads/PNG/Default size/tileGrass_roadEast.png"),
                    transform: Transform::from_xyz(0.0, lane_y, 0.0).with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 1.0)),
                    ..default()
                },
                Lane {
                    index: i,
                },
            ));

            const NUM_OBSTACLES: u8 = 4;
            let speed_offset: f32 = rand::random::<f32>() % 1.0;
            for j in 0..NUM_OBSTACLES {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("sprites/kenney_pixel-vehicle-pack/PNG/Cars/bus.png"),
                        ..default()
                    },
                    Obstacle {
                        x_index: j,
                        y_index: i,
                        speed: 0.001 * (i as f32 + speed_offset),
                        progress: j as f32 / NUM_OBSTACLES as f32,
                    }
                ));
            }
        }
    }
}

pub fn obstacle_move(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut obstacle_query: Query<(&mut Transform, &mut Obstacle), With<Obstacle>>,
) {
    if let Ok(window) = window_query.get_single() {
        for (mut transform, mut obstacle) in &mut obstacle_query {
            obstacle.progress = (obstacle.progress + obstacle.speed) % 1.0;
            *transform.as_mut() = Transform::from_xyz(obstacle.progress * window.width() - window.width() / 2.0, row_to_y_pos(obstacle.y_index, window.height()), 1.0)
        }
    }
}