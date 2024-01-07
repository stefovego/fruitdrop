use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;

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

mod mygui;
use mygui::*;

use crate::ball::plugin::BallPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        //.add_plugins(PhysicsDebugPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F9)))
        .add_state::<AppState>()
        .add_plugins(BallPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WallsPlugin)
        .add_plugins(OnDeckPlugin)
        .add_plugins(DropperPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(FpsDisplayPlugin)
        .add_plugins(LoserBoxPlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(MyGuiPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .run();
}
