use crate::my_colors;
use bevy::prelude::*;

use crate::menu::bundles::*;
use crate::menu::components::*;

use crate::main_menu::menu_state::MainMenuState;
use crate::navigation_button_plugin::{NavigationButton, SpawnNavigationButton};

#[derive(Component)]
pub struct BackButton;

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

    commands
        .entity(button_container)
        .push_children(&[under_construction_label]);

    commands
        .entity(button_container)
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Back"),
            marker_component: BackButton,
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }));
}

pub fn keyboard_select(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    parent_query: Query<&SelectedEnt, With<MenuComponent>>,
    back_query: Query<Entity, With<BackButton>>,
) {
    if parent_query.is_empty() {
        return;
    }
    let SelectedEnt(currently_selected) = parent_query.single();
    let back_entity = back_query.single();

    if keys.just_pressed(KeyCode::Enter) && back_entity == *currently_selected {
        next_state.set(MainMenuState::OptionsMenu);
    }
}

pub fn back_system(
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<BackButton>),
    >,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(MainMenuState::OptionsMenu);
        }
    }
}
