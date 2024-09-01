use crate::main_menu::MainMenuState;
use bevy::prelude::*;

pub mod systems;
use systems::*;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoingBackEvent>()
            .add_systems(OnEnter(MainMenuState::GameMenu), setup_menu)
            .add_systems(
                Update,
                (keyboard_select, back_system)
                    .chain()
                    .run_if(in_state(MainMenuState::GameMenu)),
            )
            .add_systems(Update, exit_menu.run_if(in_state(MainMenuState::GameMenu)));
    }
}
