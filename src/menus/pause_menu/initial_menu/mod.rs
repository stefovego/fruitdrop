use super::menu_state::PauseMenuState;
use crate::game_state::AppState;
use bevy::prelude::*;
pub mod components;
pub mod systems;
use systems::*;

pub struct InitialMenuPlugin;

impl Plugin for InitialMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PauseMenuState::InitialMenu), setup_menu)
            .enable_state_scoped_entities::<AppState>();
    }
}
