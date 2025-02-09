use bevy::app::AppExit;
use bevy::prelude::*;

mod game_menu;
mod initial_menu;
mod menu_state;
mod options_menu;
mod start_menu;
mod video_menu;

use menu_state::MainMenuState;

use self::game_menu::GameMenuPlugin;
use self::initial_menu::InitialMenuPlugin;
use self::options_menu::OptionsMenuPlugin;
use self::start_menu::StartMenuPlugin;
use self::video_menu::VideoMenuPlugin;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MainMenuState>()
            .enable_state_scoped_entities::<MainMenuState>()
            .add_plugins((
                InitialMenuPlugin,
                OptionsMenuPlugin,
                VideoMenuPlugin,
                GameMenuPlugin,
                StartMenuPlugin,
            ))
            .add_systems(OnEnter(menu_state::MainMenuState::QuitGame), quit_system);
    }
}

pub fn quit_system(mut events: EventWriter<AppExit>) {
    events.send(AppExit::Success);
}
