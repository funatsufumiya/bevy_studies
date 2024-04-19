use bevy::prelude::*;
use bevy::window::WindowMode;

const FULLSCREEN_MODE : WindowMode = WindowMode::BorderlessFullscreen;
// const FULLSCREEN_MODE : WindowMode = WindowMode::Fullscreen;
// const FULLSCREEN_MODE : WindowMode = WindowMode::SizedFullscreen;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "fullscreen test".to_string(),
                    mode: FULLSCREEN_MODE, // set initial window mode here
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, key_handler)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // mut window: Query<&mut Window>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        root.spawn(
            TextBundle::from_section(
                "Fullscreen Test",
                TextStyle {
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..default()
                })
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    ..default()
                })
        );
        root.spawn(
            TextBundle::from_section(
                "* Press F to toggle fullscreen",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                })
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    ..default()
                })
        );
        root.spawn(
            TextBundle::from_section(
                "* Press Esc to close",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                })
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    ..default()
                })
        );
    });
}

fn key_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // keyboard_input: Res<ButtonInput<KeyCode>>, // for bevy 0.13
    mut window: Query<&mut Window>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
    // if keyboard_input.just_pressed(KeyCode::KeyF) { // for bevy 0.13
        println!("F key pressed");
        let mut w = window.single_mut();

        // toggle fullscreen and windowed mode
        if w.mode != FULLSCREEN_MODE {
            w.mode = FULLSCREEN_MODE
        } else {
            w.mode = WindowMode::Windowed;
        }
        
    }
}