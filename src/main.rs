use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const NUMBER_OF_ROWS: u8 = 9;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (spawn_camera, spawn_player))
    .add_systems(FixedUpdate, (player_input, on_row_updated).chain())
    .run();
}

fn spawn_camera(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands
)
{
    if let Ok(window) = window_query.get_single(){
        commands.spawn(
            Camera2dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            }
        );
    }
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
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
            row.0 = if row.0 < NUMBER_OF_ROWS { row.0 + 1 } else { row.0 };
        }else if button_input.just_pressed(KeyCode::ArrowDown)
        {
            row.0 = if row.0 > 0 { row.0 - 1} else { 0 };
        }
    }
}

fn on_row_updated(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &Row), With<Player>>
)
{
    if let (Ok((mut transform, row)), Ok(window)) 
        = (player_query.get_single_mut(), window_query.get_single()){
        println!("Row number: {}", row.0);
        println!("Translation before: {}", transform.translation);
        transform.translation = Vec3::new(0.0, (window.height() / NUMBER_OF_ROWS as f32) * row.0 as f32 - window.height() / 2.0, 0.0);
        println!("Translation now: {}", transform.translation);
    }
}

#[derive(Component)]
struct Row(u8);

//Todo: (Milestone 1)
// Camera system
// Player sprite spawning system
//  - Player component
//  - Sprite component
// Player movement system