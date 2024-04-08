use std::f64::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    
    // Circle
    // commands.spawn((MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::YELLOW)),
    //     transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    //     ..default()
    // }, MouseTrackingSprite));
}

fn update(
    mut gizmos: Gizmos,
    time: Res<Time>,
    // mut query: Query<(&mut Transform,)>,
) {
    // sin wave
    let s = time.elapsed_seconds_f64();
    let pos = Vec2::new(
        s.sin() as f32 * 100.0, 
        s.cos() as f32 * 100.0
    );
    gizmos.circle_2d(pos, 50.0, Color::YELLOW);

    let pos2 = Vec2::new(
        (s + PI).sin() as f32 * 100.0, 
        (s + PI).cos() as f32 * 100.0
    );
    gizmos.circle_2d(pos2, 50.0, Color::RED);
}