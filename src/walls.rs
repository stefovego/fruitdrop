use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game_board::{GameBoard, GameBoardResource};
use crate::game_state::AppState;
use crate::physics::Layer;

pub const LEVEL_WIDTH: f32 = 700.;
pub const WALL_THICKNESS: f32 = 40.;

pub struct WallsPlugin;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct InitWallSet;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_walls.in_set(InitWallSet))
            .add_systems(OnExit(AppState::InGame), tear_down);
    }
}

#[derive(Component)]
struct Wall;

fn tear_down(mut commands: Commands, wall_query: Query<Entity, With<Wall>>) {
    for wall_entity in &wall_query {
        commands.entity(wall_entity).despawn_recursive();
    }
}

fn spawn_walls(
    mut commands: Commands,
    game_board: Single<Entity, With<GameBoard>>,
    game_board_resource: Res<GameBoardResource>,
) {
    commands.entity(*game_board).with_children(|parent| {
        // Spawn Bottom Wall
        parent.spawn((
            Sprite {
                color: Color::linear_rgb(0.502, 0.502, 0.502), //was gray
                custom_size: Some(Vec2::new(game_board_resource.width as f32, WALL_THICKNESS)),
                ..default()
            },
            Transform::from_xyz(0., -(game_board_resource.height as f32 / 2.), 0.0),
            RigidBody::Static,
            Collider::rectangle(game_board_resource.height as f32, WALL_THICKNESS),
            CollisionLayers::new([Layer::Wall], [Layer::Wall, Layer::Ball]),
            Wall,
            Name::new("Bottom Wall"),
        ));

        // Spawn Left Wall
        parent.spawn((
            Sprite {
                color: Color::linear_rgb(0.502, 0.502, 0.502), //was gray
                custom_size: Some(Vec2::new(
                    WALL_THICKNESS,
                    game_board_resource.height as f32 + WALL_THICKNESS,
                )),
                ..default()
            },
            Transform::from_xyz(
                -(game_board_resource.width as f32 / 2. + WALL_THICKNESS / 2.),
                0.,
                0.0,
            ),
            RigidBody::Static,
            Collider::rectangle(
                WALL_THICKNESS,
                game_board_resource.height as f32 + WALL_THICKNESS,
            ),
            CollisionLayers::new([Layer::Wall], [Layer::Wall, Layer::Ball]),
            Wall,
            Name::new("Left Wall"),
        ));

        // Spawn Right Wall
        parent.spawn((
            Sprite {
                color: Color::linear_rgb(0.502, 0.502, 0.502), //was gray
                custom_size: Some(Vec2::new(
                    WALL_THICKNESS,
                    game_board_resource.height as f32 + WALL_THICKNESS,
                )),
                ..default()
            },
            Transform::from_xyz(
                game_board_resource.width as f32 / 2. + WALL_THICKNESS / 2.,
                0.,
                0.0,
            ),
            RigidBody::Static,
            Collider::rectangle(
                WALL_THICKNESS,
                game_board_resource.height as f32 + WALL_THICKNESS,
            ),
            CollisionLayers::new([Layer::Wall], [Layer::Wall, Layer::Ball]),
            Wall,
            Name::new("Right Wall"),
        ));
    });
}
