use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};

const NUMBER_OF_ROWS: u8 = 9;
const TILE_SIZE: f32 = 1.28;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (spawn_camera, spawn_player, spawn_lanes))
    .add_systems(FixedUpdate, (player_input, on_row_updated).chain())
    .add_systems(Update, on_resize_system)
    .run();
}

fn spawn_camera(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands
)
{
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    );
}

#[derive(Component)]
struct Player;

fn spawn_player(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
)
{
    if let Ok(window) = window_query.get_single(){
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/kenney_animal-pack/PNG/Round/penguin.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.2, 0.2, 1.0)),
                ..default()
            },
            Player,
            Row(4)
        ));
    }
}

fn player_input(
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

fn row_to_y_pos(row: u8, height: f32) -> f32 {
    (height / NUMBER_OF_ROWS as f32) * row as f32 - height / 2.0
}

fn on_row_updated(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &Row), With<Player>>
)
{
    if let (Ok((mut transform, row)), Ok(window)) 
        = (player_query.get_single_mut(), window_query.get_single()){
        transform.translation = Vec3::new(0.0, row_to_y_pos(row.0, window.height()), 1.0);
    }
}

#[derive(Component)]
struct Row(u8);

#[derive(Component)]
struct Lane {
    index: u8,
    num_obstacles: u8,
    obstacle_speed: f32,
}

fn spawn_lanes(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    if let Ok(window) = window_query.get_single() {
        for i in 2..NUMBER_OF_ROWS {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        rect: Some(Rect { min: Vec2::new(0.0, 0.0), max: Vec2::new(window.width(), 64.0)}),
                        ..default()
                    },
                    texture: asset_server.load("sprites/kenney_topdown-roads/PNG/Default size/tileGrass_roadEast.png"),
                    transform: Transform::from_xyz(0.0, row_to_y_pos(i, window.height()), 0.0).with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 1.0)),
                    ..default()
                },
                Lane {
                    index: i,
                    num_obstacles: 4,
                    obstacle_speed: 5.0 * i as f32,
                },
            ));
        }
    }
}

fn on_resize_system(
    mut lane_query: Query<(&mut Sprite, &mut Transform, &Lane), With<Lane>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        if let Ok((mut sprite, mut transform, lane)) = lane_query.get_single_mut() {
            sprite.rect = Some(Rect { min: Vec2::new(0.0, 0.0), max: Vec2::new(e.width, 64.0)});
            *transform.as_mut() = Transform::from_xyz(0.0, row_to_y_pos(lane.index, e.height), 0.0).with_scale(Vec3::new(TILE_SIZE, TILE_SIZE, 1.0));
        }
    }
}

//Todo: (Milestone 1)
// Camera system
// Player sprite spawning system
//  - Player component
//  - Sprite component
// Player movement system