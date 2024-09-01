use crate::main_menu::MainMenuState;
use bevy::prelude::*;

pub mod systems;
use systems::*;

pub struct VideoMenuPlugin;

impl Plugin for VideoMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::VideoMenu), setup_menu)
            .add_systems(
                Update,
                (keyboard_select, back_system)
                    .chain()
                    .run_if(in_state(MainMenuState::VideoMenu)),
            );
    }
}
