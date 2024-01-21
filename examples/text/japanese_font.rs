use bevy::prelude::*;

fn main() {   
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, check_font)
        .add_systems(Startup, setup_ui)
        .run();
}

fn check_font(
    // asset_server: Res<AssetServer>,
) {
    // show warning if assets/fonts/NotoSansJP-Regular.otf is not found
    // (show site: https://fonts.google.com/noto/specimen/Noto+Sans+JP)

    // let font_handle: Handle<Font> = asset_server.load("fonts/NotoSansJP-Regular.otf")
    // let is_font_exists = asset_server.get_load_state(font_handle) == LoadState::Loaded;

    if !std::path::Path::new("assets/fonts/NotoSansJP-Regular.otf").exists() {
        warn!("assets/fonts/NotoSansJP-Regular.otf is not found.");
        warn!("Please download NotoSansJP-Regular.otf from https://fonts.google.com/noto/specimen/Noto+Sans+JP");
        warn!("and put it in assets/fonts/NotoSansJP-Regular.otf");
    }
}

fn setup_ui(
    mut cmd: Commands,
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>,
) {
    let mut window = windows.single_mut();
    window.resolution.set(500.0, 300.0);
    window.title = "Japanese Font with Bevy".to_string();

    let font_handle: Handle<Font> = asset_server.load("fonts/NotoSansJP-Regular.otf");

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
                "こんにちは世界",
                TextStyle {
                    font: font_handle,
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..default()
                })
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    ..default()
                })
        );
    });
}