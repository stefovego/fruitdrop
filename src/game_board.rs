use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::ball::plugin::{BallPlugin, InitBallSet};
use crate::dropper::plugin::*;
use crate::game_state::AppState;
use crate::loserbox::*;
use crate::walls::*;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
pub struct GameBoard;

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Reflect, Resource, Clone)]
#[reflect(Resource)]
pub struct GameBoardResource {
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize,
}
pub struct GameBoardPlugins;

impl PluginGroup for GameBoardPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GameBoardPlugin)
            .add_before::<GameBoardPlugin>(WallsPlugin)
            .add_before::<GameBoardPlugin>(DropperPlugin)
            .add_after::<GameBoardPlugin>(BallPlugin)
            .add_after::<GameBoardPlugin>(LoserBoxPlugin)
    }
}

pub struct GameBoardPlugin;

impl Plugin for GameBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameBoardResource {
            x: 0,
            y: -55,
            width: 700,
            height: 700,
        })
        .add_systems(
            OnEnter(AppState::InGame),
            setup
                .before(InitWallSet)
                .before(InitDropperSet)
                .before(InitBallSet)
                .before(InitLoserBoxSet),
        )
        .add_systems(OnExit(AppState::InGame), tear_down);
    }
}

fn setup(mut commands: Commands, game_board: Res<GameBoardResource>) {
    commands.spawn((
        Transform::from_xyz(0., -55., 0.),
        Size {
            width: game_board.width as f32,
            height: game_board.height as f32,
        },
        GameBoard,
        Name::new("Game Board"),
        InheritedVisibility::VISIBLE,
    ));
}

fn tear_down(mut commands: Commands, game_board_query: Query<Entity, With<GameBoard>>) {
    for game_board_entity in &game_board_query {
        commands.entity(game_board_entity).despawn();
    }
}
