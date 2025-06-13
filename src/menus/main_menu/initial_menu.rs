use bevy::prelude::*;

use crate::main_menu::menu_state::MainMenuState;
use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::menu::navigation_button_plugin::NavigationButtonWidgetComponent;
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
    let parent_node = ScreenParentBundle::default();

    let button_container_node = WidgetContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::InitialMenu), parent_node))
        .insert(MenuComponent)
        .insert(Name::new("Menu Screen"))
        .id();

    let button_container = commands
        .spawn(button_container_node)
        .insert(Name::new("Button Container"))
        .id();

    commands.entity(parent).add_children(&[button_container]);

    let new_game_buttom_entity = commands
        .spawn(NavigationButtonWidgetComponent {
            text: String::from("New Game"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::StartMenu,
        })
        .id();

    let options_button_entity = commands
        .spawn(NavigationButtonWidgetComponent {
            text: String::from("Options"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::OptionsMenu,
        })
        .id();

    let quit_button_entity = commands
        .spawn(NavigationButtonWidgetComponent {
            text: String::from("Quit"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::QuitGame,
        })
        .id();

    commands.entity(button_container).add_children(&[
        new_game_buttom_entity,
        options_button_entity,
        quit_button_entity,
    ]);
}
