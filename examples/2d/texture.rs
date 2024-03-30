use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            // color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(300.0, 300.0)),
            ..default()
        },
        texture: asset_server.load("images/bevy_logo.png"),
        ..default()
    });
}