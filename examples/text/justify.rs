use bevy::{prelude::*, text::BreakLineOn};

fn main() {   
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, check_font)
        .add_systems(Startup, setup_ui)
        .add_systems(Update, update_text)
        .run();
}

const BASE_TEXT: &str = "山路を登りながら、こう考えた。智に働けば角が立つ。情に棹させば流される。意地を通せば窮屈だ。とかくに人の世は住みにくい。住みにくさが高じると、安い所へ引き越したくなる。どこへ越しても住みにくいと悟った時、詩が生れて、画が出来る。";

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

#[derive(Component)]
pub struct MainText;

trait TextBundleExt {
    fn with_text_wrap(self, wrap: BreakLineOn) -> Self;
}

impl TextBundleExt for TextBundle {
    fn with_text_wrap(mut self, wrap: BreakLineOn) -> Self {
        self.text.linebreak_behavior = wrap;
        self
    }
}

fn setup_ui(
    mut cmd: Commands,
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>,
) {
    let mut window = windows.single_mut();
    window.resolution.set(600.0, 400.0);
    window.title = "Japanese Font with Bevy".to_string();

    let font_handle: Handle<Font> = asset_server.load("fonts/NotoSansJP-Regular.otf");

    cmd.spawn(Camera2dBundle::default());
    cmd.spawn(NodeBundle {
        // background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
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
    .with_children(|r| {
        r.spawn(NodeBundle {
            background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
            style: Style {
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|root| {
            root.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font_handle,
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    })
                    .with_text_wrap(BreakLineOn::WordBoundary)
                    .with_text_justify(JustifyText::Left)
                    .with_style(Style {
                        ..default()
                    })

            , MainText));
        });
    });
}

fn update_text (
    time: Res<Time>,
    mut query: Query<(&mut Text, &MainText)>,
) {
    let n = BASE_TEXT.len();
    let i = ((time.elapsed_seconds() * 6.0 ) as usize) % n;
    let substr = utf8_slice::slice(BASE_TEXT, 0, i);
    for (mut text, _) in query.iter_mut() {
        text.sections[0].value = substr.to_string();
    }
}