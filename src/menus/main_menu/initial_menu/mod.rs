use crate::game_state::AppState;
use crate::main_menu::MainMenuState;
use bevy::prelude::*;
pub mod components;
pub mod systems;
use systems::*;

pub struct InitialMenuPlugin;

impl Plugin for InitialMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::InitialMenu), setup_menu)
            .enable_state_scoped_entities::<AppState>()
            .add_systems(
                Update,
                (keyboard_select, button_clicked_system)
                    .chain()
                    .run_if(in_state(MainMenuState::InitialMenu)),
            );
    }
}
