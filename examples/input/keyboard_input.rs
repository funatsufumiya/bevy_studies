use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, key_handler)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct MouseTrackingSprite;

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}

fn key_handler(
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Space key pressed");
    }
    if keyboard_input.just_pressed(KeyCode::Return) {
        println!("Return key pressed");
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        println!("Right key pressed");
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        println!("Left key pressed");
    }
}