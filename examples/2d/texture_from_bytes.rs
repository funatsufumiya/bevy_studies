use bevy::{prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2dBundle::default());

    // texture from bytes
    let (width, height) = (256, 256);

    let mut image_data = Vec::new();

    // create sample gradient image
    for y in 0..height {
        for x in 0..width {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = 0;
            let a = 255;
            image_data.push(r);
            image_data.push(g);
            image_data.push(b);
            image_data.push(a);
        }
    }

    let image = Image::new(
        Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // vec![255; (width * height * 4) as usize],
        image_data,
        TextureFormat::Rgba8UnormSrgb,
    );

    let image_handle = images.add(image);
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            // color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(300.0, 300.0)),
            ..default()
        },
        // texture: asset_server.load("images/bevy_logo.png"),
        texture: image_handle,
        ..default()
    });
}