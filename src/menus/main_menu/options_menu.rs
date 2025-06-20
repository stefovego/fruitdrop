use bevy::prelude::*;

use crate::main_menu::MainMenuState;

use crate::menu::navigation_button_plugin::NavigationButtonWidgetComponent;
use crate::menu::{MenuComponent, WidgetContainerComponent};
use crate::my_colors;

pub struct OptionsMenuPlugin;

impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::OptionsMenu), setup_menu);
    }
}

pub fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Options Menu"),
        StateScoped(MainMenuState::OptionsMenu),
        MenuComponent,
        children![(
            WidgetContainerComponent,
            children![
                NavigationButtonWidgetComponent {
                    text: String::from("Video"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::VideoMenu,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Game"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::GameMenu,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Back"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::InitialMenu,
                },
            ]
        )],
    ));
}
