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
    window.resolution.set(500.0, 400.0);
    window.title = "Text Layout example with Bevy".to_string();

    cmd.spawn(Camera2dBundle::default());
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        root.spawn(
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.2, 0., 0.)),
                ..default()
        }).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Upper",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    })
                    .with_text_justify(JustifyText::Center)
                    .with_style(Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    })
            );
        });
        
        root.spawn(
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.0, 0.2, 0.)),
                ..default()
        }).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Lower",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    })
                    .with_text_justify(JustifyText::Center)
                    .with_style(Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    })
            );
        });
    });
}