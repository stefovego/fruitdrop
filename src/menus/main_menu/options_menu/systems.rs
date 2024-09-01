use bevy::prelude::*;

use crate::main_menu::menu_state::MainMenuState;
use crate::menu::bundles::*;
use crate::my_colors;
use crate::navigation_button_plugin::{NavigationButton, SpawnNavigationButton};

use crate::menu::components::*;

#[derive(Component)]
pub struct VideoButton;

#[derive(Component)]
pub struct GameButton;

#[derive(Component)]
pub struct BackButton;

pub fn setup_menu(mut commands: Commands) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = ContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::OptionsMenu), parent_node))
        .insert(MenuComponent)
        .id();

    let button_container = commands.spawn(button_container_node).id();

    commands.entity(parent).push_children(&[button_container]);

    commands
        .entity(button_container)
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Video"),
            marker_component: VideoButton,
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }))
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Game"),
            marker_component: GameButton,
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }))
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
    video_query: Query<Entity, With<VideoButton>>,
    game_query: Query<Entity, With<GameButton>>,
    back_query: Query<Entity, With<BackButton>>,
) {
    if parent_query.is_empty() {
        return;
    }
    let SelectedEnt(currently_selected) = parent_query.single();
    let video_entity = video_query.single();
    let game_entity = game_query.single();
    let back_entity = back_query.single();

    if keys.just_pressed(KeyCode::Enter) {
        if video_entity == *currently_selected {
            next_state.set(MainMenuState::VideoMenu);
        }
        if game_entity == *currently_selected {
            next_state.set(MainMenuState::GameMenu);
        }
        if back_entity == *currently_selected {
            next_state.set(MainMenuState::InitialMenu);
        }
    }
}

pub fn button_clicked_system(
    mut next_state: ResMut<NextState<MainMenuState>>,
    interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Button>)>,
    video_query: Query<Entity, With<VideoButton>>,
    game_query: Query<Entity, With<GameButton>>,
    back_query: Query<Entity, With<BackButton>>,
) {
    if interaction_query.is_empty() {
        return;
    }
    let video_entity = video_query.single();
    let game_entity = game_query.single();
    let back_entity = back_query.single();

    for (interaction, selected) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if video_entity == selected {
                next_state.set(MainMenuState::VideoMenu);
            }
            if game_entity == selected {
                next_state.set(MainMenuState::GameMenu);
            }
            if back_entity == selected {
                next_state.set(MainMenuState::InitialMenu);
            }
        }
    }
}
