use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_graphics);
    }
}

fn setup_graphics(mut commands: Commands) {
    //commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(1000.0),
                near: -10.0,
                far: 10.,
                ..default()
            },
            //transform: Transform::from_xyz(1000.0, -1000.0, 0.0),
            ..default()
        },
        MainCamera,
    ));
}
