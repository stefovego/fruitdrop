use bevy::{
    prelude::*, input::common_conditions::input_toggle_active, 
};
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(300.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F9)))
        .add_plugins(BallPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WallsPlugin)
        .add_plugins(OnDeckPlugin)
        .add_plugins(DropperPlugin)
        .add_plugins(ScorePlugin)
        .run();
}

