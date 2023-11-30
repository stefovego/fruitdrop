use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use crate::dropper::Dropper;
use rand::Rng;
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_graphics);
    }
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
