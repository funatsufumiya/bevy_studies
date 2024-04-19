use bevy::{
    // input::mouse::{MouseButtonInput, MouseWheel},
    sprite::MaterialMesh2dBundle,
    // input::ButtonState,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_tracking)
        .run();
}

#[derive(Component)]
struct MouseTrackingSprite;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    
    // Circle
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(50.)).into(),
        material: materials.add(ColorMaterial::from(Color::YELLOW)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }, MouseTrackingSprite));
}


fn mouse_tracking(
    mut cursor_moved_events: EventReader<CursorMoved>,
    // mut mouse_button_input_events: EventReader<MouseButtonInput>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
    mut transforms: Query<(&mut Transform, &MouseTrackingSprite)>,
    windows: Query<&mut Window>,
) {
    let window = windows.single();
    let mut transform = transforms.single_mut().0;

    // for event in mouse_button_input_events.read() {
    //     info!("{:?}", event);
    // }

    // for event in mouse_wheel_events.read() {
    //     info!("{:?}", event);
    // }

    for event in cursor_moved_events.read() {
        // info!("{:?}", event);
        transform.translation.x = event.position.x - window.width() / 2.;
        transform.translation.y = -event.position.y + window.height() / 2.;
    }
}
