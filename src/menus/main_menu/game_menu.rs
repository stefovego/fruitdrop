use crate::main_menu::menu_state::MainMenuState;
use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::my_colors;
use bevy::prelude::*;

use crate::navigation_button_plugin::{NavigationButton, SpawnNavigationButton};
use crate::selector_plugin::{Selector, SpawnSelector};
use crate::toggle_plugin::{SpawnToggle, Toggle};

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
    commands.entity(parent).push_children(&[button_container]);

    commands
        .entity(button_container)
        .add(SpawnToggle::spawn(Toggle {
            initial_value: true,
            label: "Cool?".into(),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }));

    commands
        .entity(button_container)
        .add(SpawnSelector::spawn(Selector {
            label: "Stuff".into(),
            selections: ["Stuff ONe".into(), "stuff 2".into(), "stuff3".into()].into(),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }));

    let back_button_entity = commands
        .spawn_empty()
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Back"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::OptionsMenu,
        }))
        .id();

    commands
        .entity(button_container)
        .add_child(back_button_entity);
}
