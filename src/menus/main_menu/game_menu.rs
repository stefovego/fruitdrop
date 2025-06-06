use crate::main_menu::menu_state::MainMenuState;
use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::my_colors;
use bevy::prelude::*;

use crate::navigation_button_plugin::{NavigationButton, SpawnNavigationButton};
use crate::selector_plugin::*;
use crate::toggle_plugin::*;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::GameMenu), setup_menu);
    }
}

pub fn setup_menu(mut commands: Commands) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = ContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::GameMenu), parent_node))
        .insert(MenuComponent)
        .insert(Name::new("Game Menu"))
        .id();
    let button_container = commands.spawn(button_container_node).id();
    commands.entity(parent).add_children(&[button_container]);

    let cool_toggle_entity = commands
        .spawn(ToggleWidgetComponent {
            label: "Cool?".into(),
            current_value: true,
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        })
        .id();

    let stuff_selector_entity = commands
        .spawn(SelectorWidgetComponent {
            current_index: 0,
            label: "Stuff".into(),
            selections: ["Stuff ONe".into(), "stuff 2".into(), "stuff3".into()].into(),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        })
        .id();

    let back_button_entity = commands
        .spawn_empty()
        .queue(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Back"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::OptionsMenu,
        }))
        .id();

    commands.entity(button_container).add_children(&[
        cool_toggle_entity,
        stuff_selector_entity,
        back_button_entity,
    ]);
}
