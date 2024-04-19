use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(1)]
    elapsed_time: f32,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

#[derive(Component)]
pub struct MyObj;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(50.)).into(),
        material: materials.add(CustomMaterial {
            color: Color::rgba(1., 0., 0., 0.5),
            elapsed_time: 0.,
        }),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
        ..default()
    }, MyObj));
}

fn update(
    time: Res<Time>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut query: Query<(&mut Handle<CustomMaterial>, &MyObj)>,
) {
    for (mut mat_handle, _) in query.iter_mut() {
        let mat = materials.get_mut(mat_handle.as_ref()).unwrap();
        mat.elapsed_time += time.delta_seconds();
    }
}