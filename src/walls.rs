use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::game_state::AppState;
use crate::physics::Layer;

pub const LEVEL_WIDTH: f32 = 700.;
pub const LEVEL_HEIGHT: f32 = 700.;
pub const WALL_THICKNESS: f32 = 40.;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_walls)
            .add_systems(OnExit(AppState::GameOver), tear_down);
    }
}

#[derive(Component)]
struct Wall;

fn tear_down(mut commands: Commands, wall_query: Query<Entity, With<Wall>>) {
    for wall_entity in &wall_query {
        commands.entity(wall_entity).despawn_recursive();
    }
}

fn spawn_walls(mut commands: Commands) {
    // Spawn Bottom Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(LEVEL_WIDTH, WALL_THICKNESS)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -405.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(LEVEL_WIDTH, WALL_THICKNESS),
        CollisionLayers::new([Layer::Wall], [Layer::Wall, Layer::Ball]),
        Wall,
        Name::new("Bottom Wall"),
    ));

    // Spawn Left Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(WALL_THICKNESS, LEVEL_HEIGHT + WALL_THICKNESS)),
                ..default()
            },
            transform: Transform::from_xyz(-(LEVEL_WIDTH / 2. + WALL_THICKNESS / 2.), -55.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(WALL_THICKNESS, LEVEL_HEIGHT + WALL_THICKNESS),
        CollisionLayers::new([Layer::Wall], [Layer::Wall, Layer::Ball]),
        Wall,
        Name::new("Left Wall"),
    ));

    // Spawn Right Wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(WALL_THICKNESS, LEVEL_HEIGHT + WALL_THICKNESS)),
                ..default()
            },
            transform: Transform::from_xyz(LEVEL_WIDTH / 2. + WALL_THICKNESS / 2., -55.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(WALL_THICKNESS, LEVEL_HEIGHT + WALL_THICKNESS),
        CollisionLayers::new([Layer::Wall], [Layer::Wall, Layer::Ball]),
        Wall,
        Name::new("Right Wall"),
    ));
}
