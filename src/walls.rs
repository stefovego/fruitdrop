use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::MainCamera;


pub const LEVEL_WIDTH: f32 = 1000.;
pub const LEVEL_HEIGHT: f32 = 700.;
pub const WALL_THICKNESS: f32 = 5.;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_walls);
    }
}

#[derive(Component)]
struct Wall;


fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut projection_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    
    let projection = projection_query.single_mut();

    let screen_width = projection.area.max.x - projection.area.min.x;
    let screen_height = projection.area.max.y - projection.area.min.y;

    // Spawn Bottom Wall
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(LEVEL_WIDTH, WALL_THICKNESS * 2.0)),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(LEVEL_WIDTH / 2., WALL_THICKNESS))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -400.0, 0.0)))
        .insert(Wall);

    // Spawn Left Wall
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(WALL_THICKNESS * 2.0, LEVEL_HEIGHT)),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(WALL_THICKNESS, LEVEL_HEIGHT / 2.))
        .insert(TransformBundle::from(Transform::from_xyz(-(LEVEL_WIDTH / 2. + WALL_THICKNESS), -55.0, 0.0)))
        .insert(Wall);

    // Spawn Right Wall
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(WALL_THICKNESS * 2.0, LEVEL_HEIGHT)),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(WALL_THICKNESS, LEVEL_HEIGHT / 2.))
        .insert(TransformBundle::from(Transform::from_xyz(LEVEL_WIDTH / 2. + WALL_THICKNESS, -55.0, 0.0)))
        .insert(Wall);
    }

