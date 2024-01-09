use crate::ball::resources::{BallScaler, GrowStats};
use crate::dropper::resources::DropperStats;
use crate::game_state::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct MyGuiPlugin;

#[derive(Reflect, Resource, Clone, Default)]
#[reflect(Resource)]
pub struct TuneWindowResources {
    pub window_opened: bool,
}

impl Plugin for MyGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, gui_setup.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                toggle_window_system.run_if(in_state(AppState::InGame)),
            )
            .register_type::<TuneWindowResources>()
            .insert_resource(TuneWindowResources {
                window_opened: false,
            });
        // app.add_systems(Update, gui_setup.run_if(in_state(AppState::InGame)));
    }
}

fn toggle_window_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut tune_window: ResMut<TuneWindowResources>,
) {
    if keyboard_input.just_pressed(KeyCode::T) {
        tune_window.window_opened = !tune_window.window_opened;
    }
}

fn gui_setup(
    mut contexts: EguiContexts,
    tune_window: Res<TuneWindowResources>,
    mut grow_stats: ResMut<GrowStats>,
    mut dropper_stats: ResMut<DropperStats>,
    mut ball_scaler_stats: ResMut<BallScaler>,
) {
    if tune_window.window_opened {
        let mut slider_value = ball_scaler_stats.initial_size;

        egui::Window::new("Tune").show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Label::new("Ball Stats"));
            ui.add(
                egui::Slider::new(&mut grow_stats.initial_multiplier, 0.0..=1.0)
                    .text("Initial Size"),
            );
            ui.add(egui::Slider::new(&mut grow_stats.grow_speed, 0.0..=2.0).text("Grow Speed"));
            ui.add(egui::Slider::new(&mut slider_value, 1.0..=20.0).text("Ball Size"));
            //ui.add(egui::Slider::new(&mut ball_scaler_stats.initial_size, 1.0..=50.0).text("Ball Size"));
            ui.add(egui::Separator::default());
            ui.add(egui::Label::new("Dropper Stats"));
            ui.add(egui::Slider::new(&mut dropper_stats.speed, 200.0..=1000.0).text("Speed"));
            ui.add(egui::Slider::new(&mut dropper_stats.delay_time, 0.0..=5.0).text("Delay"));
        });

        if slider_value != ball_scaler_stats.initial_size {
            ball_scaler_stats.initial_size = slider_value;
        }
    }
}
