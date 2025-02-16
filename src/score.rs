use crate::game_state::{AppState, GameState};
use bevy::prelude::*;

pub struct ScorePlugin;
pub const SCORE_BOARD_X: f32 = -700.;
pub const SCORE_BOARD_Y: f32 = 300.;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerScore { value: 0 })
            .add_systems(OnEnter(AppState::InGame), spawn_scoreboard)
            .add_systems(Update, update_scoreboard.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), tear_down);
    }
}

#[derive(Component)]
struct Score;

#[derive(Resource)]
pub struct PlayerScore {
    pub value: u32,
}

fn tear_down(mut commands: Commands, score_query: Query<Entity, With<Score>>) {
    for score_entity in &score_query {
        commands.entity(score_entity).despawn_recursive();
    }
}

fn update_scoreboard(
    mut score_text_query: Query<&mut Text, With<Score>>,
    player_score: Res<PlayerScore>,
) {
    if player_score.is_changed() {
        for mut text in &mut score_text_query {
            let value = player_score.value;
            text.0 = format!("{value:.2}");
        }
    }
}

fn spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_score: ResMut<PlayerScore>,
) {
    player_score.value = 0;
    commands.spawn((
        Text::new(player_score.value.to_string()),
        TextFont {
            font: asset_server.load("fonts/Roboto-Black.ttf"),
            font_size: 60.,
            ..default()
        },
        Transform::from_translation(Vec3 {
            x: SCORE_BOARD_X,
            y: SCORE_BOARD_Y,
            z: 1.,
        }),
        Score,
        Name::new("scoreboard"),
    ));
}
