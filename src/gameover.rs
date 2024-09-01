use crate::game_state::*;
use crate::score::PlayerScore;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const WIDTH: f32 = 900.;
const HEIGHT: f32 = 400.;

#[derive(Component)]
pub struct GameOver;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_gameover)
            .add_systems(OnEnter(GameState::GameOver), setup)
            .add_systems(OnExit(GameState::GameOver), tear_down)
            .add_systems(
                Update,
                exit_gameover_screen.run_if(in_state(GameState::GameOver)),
            );
    }
}

fn tear_down(mut commands: Commands, gameover_query: Query<Entity, With<GameOver>>) {
    for gameover_entity in &gameover_query {
        commands.entity(gameover_entity).despawn_recursive();
    }
}
fn exit_gameover_screen(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}
fn trigger_gameover(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F6) {
        next_state.set(GameState::GameOver);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_score: Res<PlayerScore>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(WIDTH, HEIGHT)).into(),
            material: materials.add(ColorMaterial::from_color(LinearRgba::rgb(
                1.0, 0.388, 0.278,
            ))), //TOMATO
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: -100.,
                    z: 3.,
                },
                ..default()
            },
            ..default()
        },
        GameOver,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                player_score.value.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Black.ttf"),
                    font_size: 60.,
                    ..default()
                },
            ),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: -160.,
                    z: 4.,
                },
                ..default()
            },
            ..default()
        },
        Name::new("gameover score"),
        GameOver,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Game Over!!!",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Black.ttf"),
                    font_size: 80.,
                    ..default()
                },
            ),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 4.,
                },
                ..default()
            },
            ..default()
        },
        Name::new("gameover message"),
        GameOver,
    ));
}
