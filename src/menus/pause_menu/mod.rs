use bevy::prelude::*;

mod initial_menu;

mod menu_state;
use crate::game_state::AppState;
use crate::game_state::GameState;
use menu_state::PauseMenuState;

use self::initial_menu::InitialMenuPlugin;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PauseMenuState>()
            .enable_state_scoped_entities::<GameState>()
            .add_plugins(InitialMenuPlugin);
    }
}
