use bevy::app::AppExit;
use bevy::prelude::*;

mod initial_menu;

use crate::menu::components::MenuComponent;
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
            .add_plugins(InitialMenuPlugin)
            .add_systems(OnExit(PauseMenuState::InitialMenu), exit_initial_pause)
            .add_systems(OnExit(GameState::Paused), exit_paused_state)
            .add_systems(OnEnter(AppState::MainMenu), enter_main_menu_state);
    }
}

pub fn enter_main_menu_state() {
    info!("Entered Main Menu");
}

pub fn exit_paused_state() {
    info!("Pause State Exit");
}

pub fn exit_initial_pause() {
    info!("Pause Initial Menu Exit");
}

pub fn quit_system(mut events: EventWriter<AppExit>) {
    events.send(AppExit::Success);
}
