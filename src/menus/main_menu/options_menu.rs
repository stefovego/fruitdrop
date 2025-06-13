use bevy::prelude::*;

use crate::main_menu::MainMenuState;
use crate::menu::bundles::*;
use crate::menu::components::*;

use crate::menu::navigation_button_plugin::NavigationButtonWidgetComponent;
use crate::my_colors;

pub struct OptionsMenuPlugin;

impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::OptionsMenu), setup_menu);
    }
}

pub fn setup_menu(mut commands: Commands) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = WidgetContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::OptionsMenu), parent_node))
        .insert(MenuComponent)
        .id();

    let button_container = commands.spawn(button_container_node).id();

    commands.entity(parent).add_children(&[button_container]);

    let video_button_entity = commands
        .spawn(NavigationButtonWidgetComponent {
            text: String::from("Video"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::VideoMenu,
        })
        .id();

    let game_button_entity = commands
        .spawn(NavigationButtonWidgetComponent {
            text: String::from("Game"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::GameMenu,
        })
        .id();

    let back_button_entity = commands
        .spawn(NavigationButtonWidgetComponent {
            text: String::from("Back"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::InitialMenu,
        })
        .id();

    commands.entity(button_container).add_children(&[
        video_button_entity,
        game_button_entity,
        back_button_entity,
    ]);
}
