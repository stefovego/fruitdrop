use bevy::input::gamepad::GamepadEvent;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game_state::*;
use crate::handle_input::Action;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pause_game.run_if(in_state(GameState::Playing)))
            .add_systems(Update, unpause_game.run_if(in_state(GameState::Paused)));
    }
}

fn pause_game(input: Res<ActionState<Action>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(&Action::Pause) {
        info!("Game Paused");
        next_state.set(GameState::Paused);
    }
}

fn unpause_game(input: Res<ActionState<Action>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(&Action::Pause) {
        info!("Game Playing");
        next_state.set(GameState::Playing);
    }
}
