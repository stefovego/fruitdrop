use bevy::prelude::*;

use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::menu::navigation_button_plugin::{NavigationButton, SpawnNavigationButton};
use crate::my_colors;
use crate::pause_menu::initial_menu::components::*;
use crate::AppState;
use crate::GameState;

pub fn setup_menu(mut commands: Commands) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = ContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(GameState::Paused), parent_node))
        .insert(MenuComponent)
        .id();

    let button_container = commands.spawn(button_container_node).id();

    commands.entity(parent).push_children(&[button_container]);

    commands
        .entity(button_container)
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Unpause"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: GameState::Playing,
        }))
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Quit"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: AppState::MainMenu,
        }));
}

// pub fn keyboard_select(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut next_game_state: ResMut<NextState<GameState>>,
//     mut next_app_state: ResMut<NextState<AppState>>,
//     parent_query: Query<&SelectedEnt, With<MenuComponent>>,
//     unpause_query: Query<Entity, With<UnpauseComponent>>,
//     quit_query: Query<Entity, With<QuitComponent>>,
// ) {
//     if parent_query.is_empty() {
//         return;
//     }
//     let SelectedEnt(currently_selected) = parent_query.single();
//     let unpause_entity = unpause_query.single();
//     let quit_entity = quit_query.single();
//
//     if keys.just_pressed(KeyCode::Enter) {
//         if unpause_entity == *currently_selected {
//             next_game_state.set(GameState::Playing);
//         }
//         if quit_entity == *currently_selected {
//             next_game_state.set(GameState::Playing);
//             next_app_state.set(AppState::MainMenu);
//         }
//     }
// }
//
// pub fn button_clicked_system(
//     mut next_game_state: ResMut<NextState<GameState>>,
//     mut next_app_state: ResMut<NextState<AppState>>,
//     interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Button>)>,
//     unpause_query: Query<Entity, With<UnpauseComponent>>,
//     quit_query: Query<Entity, With<QuitComponent>>,
// ) {
//     if interaction_query.is_empty() {
//         return;
//     }
//     let unpause_entity = unpause_query.single();
//     let quit_entity = quit_query.single();
//
//     for (interaction, selected) in interaction_query.iter() {
//         if *interaction == Interaction::Pressed {
//             if unpause_entity == selected {
//                 next_game_state.set(GameState::Playing);
//             }
//             if quit_entity == selected {
//                 next_game_state.set(GameState::Playing);
//                 next_app_state.set(AppState::MainMenu);
//             }
//         }
//     }
// }
