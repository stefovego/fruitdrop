use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use noisy_bevy::NoisyShaderPlugin;
use web_sys;

mod handle_input;

mod camera;
use camera::*;

mod walls;
use walls::*;

mod dropper;
use dropper::plugin::DropperPlugin;

mod ondeck;
use ondeck::*;

mod score;
use score::*;

mod fps_display;
use fps_display::*;

mod loserbox;
use loserbox::*;

mod game_state;
use game_state::*;

mod gameover;
use gameover::*;

mod ball;
mod physics;

//mod mygui;
//use mygui::*;

mod menu;
mod menus;
mod pause;

//mod states;

use crate::menu::*;
use crate::menus::*;
use crate::pause::*;

use crate::ball::plugin::BallPlugin;
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
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        //.add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F9)))
        .init_state::<AppState>()
        .add_plugins(BallPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WallsPlugin)
        .add_plugins(OnDeckPlugin)
        .add_plugins(DropperPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(FpsDisplayPlugin)
        .add_plugins(LoserBoxPlugin)
        .add_plugins(GameOverPlugin)
        //       .add_plugins(MyGuiPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(pause_menu::PauseMenuPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(PausePlugin)
        .add_systems(Startup, update_canvas_size)
        //.add_systems(Startup, update_canvas_size.run_if(cfg!(target_arch = "wasm32")))
        .run();
}

//#[cfg(target_arch = "wasm32")]
fn update_canvas_size(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    (|| {
        if cfg!(target_arch = "wasm32") {
            let mut window = window.get_single_mut().ok()?;
            let browser_window = web_sys::window()?;
            let width = browser_window.inner_width().ok()?.as_f64()?;
            let height = browser_window.inner_height().ok()?.as_f64()?;
            window.resolution.set(width as f32, height as f32);
        }
        Some(())
    })();
}
