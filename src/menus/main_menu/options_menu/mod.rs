use crate::main_menu::MainMenuState;
use bevy::prelude::*;

pub mod systems;
use systems::*;

pub struct OptionsMenuPlugin;

impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::OptionsMenu), setup_menu)
            .add_systems(
                Update,
                (keyboard_select, button_clicked_system)
                    .chain()
                    .run_if(in_state(MainMenuState::OptionsMenu)),
            );
    }
}
