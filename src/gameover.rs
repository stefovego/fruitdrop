use crate::game_state::*;
use crate::score::PlayerScore;
use bevy::prelude::*;

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

fn setup(mut commands: Commands, player_score: Res<PlayerScore>, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                display: Display::Grid,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            Name::new("Game Over Screen"),
            GameOver,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        display: Display::Grid,
                        width: Val::Px(WIDTH),
                        height: Val::Px(HEIGHT),
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.388, 0.278)), // Tomato
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Game Over!!!"),
                        TextFont {
                            font: asset_server.load("fonts/Roboto-Black.ttf"),
                            font_size: 80.0,
                            ..Default::default()
                        },
                    ));
                    parent.spawn((
                        Text::new(player_score.value.to_string()),
                        TextFont {
                            font: asset_server.load("fonts/Roboto-Black.ttf"),
                            font_size: 60.0,
                            ..Default::default()
                        },
                    ));
                });
        });
}
