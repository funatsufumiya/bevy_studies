use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;

use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.75, 0.25, 0.25),
        ..default()
    });

    let shapes = [
        meshes.add(shape::Torus::default().into()),
        meshes.add(shape::Box::default().into()),
        meshes.add(shape::Cylinder::default().into()),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    };

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // FIXME: currently below 2 cameras shows warning:
    // (related forum post: https://www.reddit.com/r/bevy/comments/y8mtpw/2d_and_3d_at_the_same_time/)
    //
    // WARN bevy_render::camera::camera:
    //  Camera order ambiguities detected for active cameras
    //  with the following priorities:
    //  {(0, Some(Window(NormalizedWindowRef(0v0))))}. 
    //  To fix this, ensure there is exactly one Camera entity
    //  spawned with a given order for a given RenderTarget.
    //  Ambiguities should be resolved because either
    //  (1) multiple active cameras were spawned accidentally,
    //    which will result in rendering multiple instances
    //    of the scene or
    //  (2) for cases where multiple active cameras is intentional,
    //    ambiguities could result in unpredictable render results.

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        camera: Camera {
            order: 0,
            ..default()
        },
        ..default()
    });

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            // renders after / on top of the main camera
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., -150., 0.)),
        ..default()
    });
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}