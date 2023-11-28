use bevy::{
    prelude::*, 
    render::camera::ScalingMode,
    window::PrimaryWindow
};
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod ball;
use ball::*;

mod walls;
use walls::*;

mod dropper;
use dropper::*;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(300.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        //.add_systems(Startup, setup_graphics)
        .add_plugins(BallPlugin)
        .add_plugins(WallsPlugin)
        .add_plugins(DropperPlugin)
        .add_systems(PreStartup, setup_graphics)
        //.add_systems(Update, print_ball_altitude)
        //.add_systems(Update, print_mouse_position)
        .run();
}

fn setup_graphics(mut commands: Commands){ 
    //commands.spawn(Camera2dBundle::default());
    println!("setup_graphics");
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(1000.0),
            near: -2.0,
            ..default()
        },
        //transform: Transform::from_xyz(1000.0, -1000.0, 0.0),
        ..default()
    }, MainCamera));
}


fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn print_mouse_position(q_windows: Query<&Window, With<PrimaryWindow>>) {
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not inside the primary window");
    }
}

