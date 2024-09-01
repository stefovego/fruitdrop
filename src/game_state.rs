use bevy::prelude::*;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
    Paused,
}
