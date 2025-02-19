use bevy::prelude::*;

use crate::game_state::AppState;

#[derive(Component)]
struct InGameUi;

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::InGame), tear_down);
    }
}

fn setup(mut commands: Commands) {
    println!("InGameUi.setup");
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        Name::new("InGameUi"),
        InGameUi,
    ));
}
fn tear_down(mut commands: Commands) {}
