use bevy::prelude::*;

use crate::main_menu::menu_state::MainMenuState;
use crate::menu::navigation_button_plugin::NavigationButtonWidgetComponent;
use crate::menu::MenuComponent;
use crate::menu::WidgetContainerComponent;
use crate::my_colors;
use crate::AppState;

pub struct InitialMenuPlugin;

impl Plugin for InitialMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::InitialMenu), setup_menu)
            .enable_state_scoped_entities::<AppState>();
    }
}

pub fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Menu Screen"),
        MenuComponent,
        StateScoped(MainMenuState::InitialMenu),
        children![(
            WidgetContainerComponent,
            children![
                NavigationButtonWidgetComponent {
                    text: String::from("New Game"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::StartMenu,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Options"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::OptionsMenu,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Quit"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::QuitGame,
                },
            ]
        )],
    ));
}
