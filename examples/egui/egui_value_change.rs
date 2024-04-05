use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(UiState::default())
        .add_systems(Update, ui_example)
        .run();
}

#[derive(Default, Resource)]
struct UiState {
    value: f32,
}

fn ui_example(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
) {
    egui::Window::new("Test").show(contexts.ctx_mut(), |ui| {
        if ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value")).changed() {
            println!("value changed: {}", ui_state.value);
        }

        if ui.button("Reset").clicked() {
            ui_state.value = 0.0;
            println!("value reset");
        }
    });
}
