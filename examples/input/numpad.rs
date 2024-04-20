use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, key_handler)
        .run();
}
fn key_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Numpad1) {
        println!("Numpad 1 key pressed");
    }

    if keyboard_input.just_pressed(KeyCode::Numpad2) {
        println!("Numpad 2 key pressed");
    }
}