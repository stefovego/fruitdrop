use bevy::prelude::*;

use crate::main_menu::MainMenuState;
use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::my_colors;
use crate::navigation_button_plugin::{NavigationButton, SpawnNavigationButton};

pub struct VideoMenuPlugin;

impl Plugin for VideoMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::VideoMenu), setup_menu);
    }
}

pub fn setup_menu(mut commands: Commands) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = ContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::VideoMenu), parent_node))
        .insert(MenuComponent)
        .id();

    let button_container = commands.spawn(button_container_node).id();

    commands.entity(parent).push_children(&[button_container]);

    let under_construction_text = TextBundle::from_section(
        "Under Construction",
        TextStyle {
            color: my_colors::PINK,
            font_size: 50.0,
            ..default()
        },
    );

    let under_construction_label = commands.spawn(under_construction_text).id();

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
        .push_children(&[under_construction_label, back_button_entity]);
}
