use bevy::{
    // input::mouse::{MouseButtonInput, MouseWheel},
    prelude::*, sprite::MaterialMesh2dBundle, window::WindowMode
};

use mouse_position::mouse_position::Mouse;

// Check mouse position in two different systems, and gizmos

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    // present_mode: bevy::window::PresentMode::Mailbox,
                    // present_mode: bevy::window::PresentMode::Immediate,
                    // present_mode: bevy::window::PresentMode::Fifo,
                    // present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
        )
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, mouse_tracking_a)
        .add_systems(PreUpdate, mouse_tracking_b)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct MouseTrackingSpriteA;

#[derive(Component)]
struct MouseTrackingSpriteB;

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    
    // Circle A
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(50.)).into(),
        material: materials.add(ColorMaterial::from(Color::rgba(1., 1., 0., 0.5))),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }, MouseTrackingSpriteA));

    // Circle B
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(30.)).into(),
        material: materials.add(ColorMaterial::from(Color::rgba(1., 0., 0., 0.5))),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
        ..default()
    }, MouseTrackingSpriteB));
}


fn mouse_tracking_a(
    mut transforms: Query<(&mut Transform, &MouseTrackingSpriteA)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&mut Window>,
) {
    let window = windows.single();
    let mut transform = transforms.single_mut().0;
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        transform.translation.x = world_position.x;
        transform.translation.y = world_position.y;
    }
}

fn mouse_tracking_b(
    mut transforms: Query<(&mut Transform, &MouseTrackingSpriteB)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // windows: Query<&mut Window>,
    mut gizmos: Gizmos,
) {
    // let window = windows.single();
    let mut transform = transforms.single_mut().0;
    let (camera, camera_transform) = camera_q.single();

    let pos = Mouse::get_mouse_position();
    if let Mouse::Position { x, y } = pos {
        let cursor = Vec2::new(x as f32, y as f32);
        let pos_world = camera.viewport_to_world_2d(camera_transform, cursor);
        if let Some(world_position) = pos_world {
            // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
            transform.translation.x = world_position.x;
            transform.translation.y = world_position.y;
        }
    }
}