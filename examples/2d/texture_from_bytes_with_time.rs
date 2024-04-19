use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ImageHandle {
            handle: None,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Resource)]
struct ImageHandle {
    handle: Option<Handle<Image>>,
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut image_handle: ResMut<ImageHandle>,
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
        image_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD // for bevy 0.13
    );

    image_handle.handle = Some(images.add(image));
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            // color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(300.0, 300.0)),
            ..default()
        },
        // texture: asset_server.load("images/bevy_logo.png"),
        texture: image_handle.handle.clone().unwrap(),
        ..default()
    });
}

fn update(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut image_handle: ResMut<ImageHandle>,
    time: Res<Time>,
) {
    // get image from handle
    let image = images.get_mut(image_handle.handle.clone().unwrap()).unwrap();

    // update image data
    let (width, height) = (256, 256);

    let mut image_data = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = (time.elapsed_seconds().sin() * 127.0 + 128.0) as u8;
            let a = 255;
            image_data.push(r);
            image_data.push(g);
            image_data.push(b);
            image_data.push(a);
        }
    }

    // println!("Update image data with time: {}", time.elapsed_seconds());

    image.data = image_data;
}