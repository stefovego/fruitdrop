use crate::game_state::AppState;
use bevy::prelude::*;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, SubStates)]
//#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
#[source(AppState = AppState::MainMenu)]
pub enum MainMenuState {
    #[default]
    InitialMenu,
    OptionsMenu,
    VideoMenu,
    GameMenu,
    QuitGame,
}
