use bevy::prelude::*;

use crate::main_menu::initial_menu::components::*;
use crate::main_menu::menu_state::MainMenuState;
use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::menu::navigation_button_plugin::{NavigationButton, SpawnNavigationButton};
use crate::my_colors;
use crate::AppState;

pub fn setup_menu(mut commands: Commands) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = ContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::InitialMenu), parent_node))
        .insert(MenuComponent)
        .id();

    let button_container = commands.spawn(button_container_node).id();

    commands.entity(parent).push_children(&[button_container]);

    commands
        .entity(button_container)
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("New Game"),
            marker_component: NewGameComponent,
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }))
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Options"),
            marker_component: OptionsComponent,
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }))
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Quit"),
            marker_component: QuitComponent,
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }));
}

pub fn keyboard_select(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    parent_query: Query<&SelectedEnt, With<MenuComponent>>,
    new_game_query: Query<Entity, With<NewGameComponent>>,
    options_query: Query<Entity, With<OptionsComponent>>,
    quit_query: Query<Entity, With<QuitComponent>>,
) {
    if parent_query.is_empty() {
        return;
    }
    let SelectedEnt(currently_selected) = parent_query.single();
    let new_game_entity = new_game_query.single();
    let options_entity = options_query.single();
    let quit_entity = quit_query.single();

    if keys.just_pressed(KeyCode::Enter) {
        if new_game_entity == *currently_selected {
            next_app_state.set(AppState::InGame);
        }
        if options_entity == *currently_selected {
            next_state.set(MainMenuState::OptionsMenu);
        }
        if quit_entity == *currently_selected {
            next_state.set(MainMenuState::QuitGame);
        }
    }
}

pub fn button_clicked_system(
    mut next_state: ResMut<NextState<MainMenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Button>)>,
    new_game_query: Query<Entity, With<NewGameComponent>>,
    options_query: Query<Entity, With<OptionsComponent>>,
    quit_query: Query<Entity, With<QuitComponent>>,
) {
    if interaction_query.is_empty() {
        return;
    }
    let new_game_entity = new_game_query.single();
    let options_entity = options_query.single();
    let quit_entity = quit_query.single();

    for (interaction, selected) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if new_game_entity == selected {
                next_app_state.set(AppState::InGame);
            }
            if options_entity == selected {
                next_state.set(MainMenuState::OptionsMenu);
            }
            if quit_entity == selected {
                next_state.set(MainMenuState::QuitGame);
            }
        }
    }
}
