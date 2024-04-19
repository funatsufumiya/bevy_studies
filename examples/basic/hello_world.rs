use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(
    mut cmd: Commands,
    mut windows: Query<&mut Window>
) {
    let mut window = windows.single_mut();
    window.resolution.set(500.0, 300.0);
    window.title = "Hello World example with Bevy".to_string();

    cmd.spawn(Camera2dBundle::default());
    cmd.spawn(NodeBundle {
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
                "Hello World",
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
    });
}