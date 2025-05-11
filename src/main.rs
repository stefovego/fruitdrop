use avian2d::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game_board::GameBoardPlugins;
//use noisy_bevy::NoisyShaderPlugin;
use web_sys;

mod ball;
mod camera;
mod dropper;
mod fps_display;
mod game_board;
mod game_state;
mod gameover;
mod handle_input;
mod ingame_ui;
mod loserbox;
mod material_color_animation;
mod menu;
mod menus;
mod ondeck;
mod pause;
mod physics;
mod score;
mod walls;

use crate::camera::*;
use crate::fps_display::*;
use crate::game_state::*;
use crate::gameover::*;
use crate::ingame_ui::*;
use crate::material_color_animation::*;
use crate::menu::*;
use crate::menus::*;
use crate::ondeck::*;
use crate::pause::*;
use crate::score::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#game-canvas".into()),
                ..default()
            }),
            ..default()
        }))
        .init_state::<game_state::AppState>()
        .init_state::<game_state::GameState>()
        //.add_plugins(NoisyShaderPlugin)
        .add_plugins(PhysicsPlugins::default().with_length_unit(10.0))
        .add_plugins(handle_input::InputPlugin)
        // .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: false,
        })
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F9)))
        .init_state::<AppState>()
        .add_plugins(CameraPlugin)
        .add_plugins(MaterialColorAnimation)
        .add_plugins(GameBoardPlugins)
        .add_plugins(OnDeckPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(FpsDisplayPlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(pause_menu::PauseMenuPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(PausePlugin)
        .add_plugins(InGameUiPlugin)
        .add_systems(Startup, update_canvas_size)
        //.add_systems(Startup, update_canvas_size.run_if(cfg!(target_arch = "wasm32")))
        .run();
}

//#[cfg(target_arch = "wasm32")]
fn update_canvas_size(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    (|| {
        if cfg!(target_arch = "wasm32") {
            let mut window = window.single_mut().ok()?;
            let browser_window = web_sys::window()?;
            let width = browser_window.inner_width().ok()?.as_f64()?;
            let height = browser_window.inner_height().ok()?.as_f64()?;
            window.resolution.set(width as f32, height as f32);
        }
        Some(())
    })();
}
