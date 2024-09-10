use crate::game_state::GameState;
use bevy::prelude::*;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, SubStates)]
//#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
#[source(GameState = GameState::Paused)]
pub enum PauseMenuState {
    #[default]
    InitialMenu,
    Unpaused,
}
