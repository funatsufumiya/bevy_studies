use bevy::prelude::*;
use bevy::window::WindowMode;

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
    // mut window: Query<&mut Window>,
) {
    commands.spawn(Camera2dBundle::default());

    // let mut w = window.single_mut();
    // w
    //     .resolution
    //     .set_scale_factor_override(Some(1.0));

    // show sample sprite
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(300.0, 400.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });
}

fn key_handler(
    keyboard_input: Res<Input<KeyCode>>,
    mut window: Query<&mut Window>,
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
    if keyboard_input.just_pressed(KeyCode::F) {
        println!("F key pressed");
        let mut w = window.single_mut();
        const fs: WindowMode = WindowMode::BorderlessFullscreen;
        // toggle fullscreen
        if w.mode != fs {
            w.mode = fs
        } else {
            w.mode = WindowMode::Windowed;
        }
        
    }
}