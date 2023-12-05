use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod ball;
use ball::*;

mod camera;
use camera::*;

mod walls;
use walls::*;

mod dropper;
use dropper::*;

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

mod bundles;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(40.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F9)))
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
        .run();
}
