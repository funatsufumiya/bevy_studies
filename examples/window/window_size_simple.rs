use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut windows: Query<&mut Window>
) {
    let mut window = windows.single_mut();
    window.resolution.set(500.0, 500.0);
}