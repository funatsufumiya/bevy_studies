use bevy::{
    input::mouse::MouseButtonInput,
    sprite::MaterialMesh2dBundle,
    input::ButtonState,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MyMaterial {
            material: None,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_tracking)
        .run();
}

#[derive(Component)]
struct MouseTrackingSprite;

#[derive(Resource)]
struct MyMaterial {
    material: Option<Handle<ColorMaterial>>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut my_material: ResMut<MyMaterial>,
) {
    commands.spawn(Camera2dBundle::default());

    my_material.material = Some(materials.add(ColorMaterial::from(Color::YELLOW)));
    
    // Circle
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(50.)).into(),
        material: my_material.material.clone().unwrap(),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }, MouseTrackingSprite));
}


fn mouse_tracking(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
    mut transforms: Query<(&mut Transform, &MouseTrackingSprite)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&mut Window>,
    my_material: ResMut<MyMaterial>,
) {
    let window = windows.single();

    let mut transform = transforms.single_mut().0;

    for event in mouse_button_input_events.read() {
        // info!("{:?}", event);
        let mat = materials.get_mut(my_material.material.as_ref().unwrap()).unwrap();

        if event.button == MouseButton::Left {
            if event.state == ButtonState::Pressed {
                info!("Mouse Pressed");
                mat.color = Color::RED;
            } else {
                info!("Mouse Released");
                mat.color = Color::YELLOW;
            }
        }
    }

    // for event in mouse_wheel_events.read() {
    //     info!("{:?}", event);
    // }

    for event in cursor_moved_events.read() {
        // info!("{:?}", event);
        transform.translation.x = event.position.x - window.width() / 2.;
        transform.translation.y = -event.position.y + window.height() / 2.;
    }
}
