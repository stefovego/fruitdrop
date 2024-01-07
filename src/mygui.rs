use crate::ball::resources::GrowStats;
use crate::dropper::resources::DropperStats;
use crate::game_state::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct MyGuiPlugin;

impl Plugin for MyGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, gui_setup.run_if(in_state(AppState::InGame)));
        // app.add_systems(Update, gui_setup.run_if(in_state(AppState::InGame)));
    }
}

fn gui_setup(
    mut contexts: EguiContexts,
    mut grow_stats: ResMut<GrowStats>,
    mut dropper_stats: ResMut<DropperStats>,
) {
    egui::Window::new("Whatup").show(contexts.ctx_mut(), |ui| {
        ui.add(egui::Label::new("Ball Stats"));
        ui.add(
            egui::Slider::new(&mut grow_stats.initial_multiplier, 0.0..=1.0).text("Initial Size"),
        );
        ui.add(egui::Slider::new(&mut grow_stats.grow_speed, 0.0..=2.0).text("Grow Speed"));
        ui.add(egui::Separator::default());
        ui.add(egui::Label::new("Dropper Stats"));
        ui.add(egui::Slider::new(&mut dropper_stats.speed, 200.0..=1000.0).text("Speed"));
        ui.add(egui::Slider::new(&mut dropper_stats.delay_time, 0.0..=5.0).text("Delay"));
    });
}
