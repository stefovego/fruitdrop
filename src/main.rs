use bevy::{
    prelude::*, 
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(300.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(BallPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WallsPlugin)
        .add_plugins(DropperPlugin)
        .run();
}

