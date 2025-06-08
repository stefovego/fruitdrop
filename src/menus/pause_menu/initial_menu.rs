use super::menu_state::PauseMenuState;
use bevy::prelude::*;

use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::menu::navigation_button_plugin::ButtonPushed;
use crate::menu::navigation_button_plugin::NavigationButtonWidgetComponent;
use crate::my_colors;
use crate::AppState;
use crate::GameState;
pub struct InitialMenuPlugin;

impl Plugin for InitialMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PauseMenuState::InitialMenu), setup_menu)
            .enable_state_scoped_entities::<AppState>();
    }
}

pub fn setup_menu(mut commands: Commands) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = ContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(GameState::Paused), parent_node))
        .insert(MenuComponent)
        .id();

    let button_container = commands.spawn(button_container_node).id();

    commands.entity(parent).add_children(&[button_container]);

    let unpause_button_entity = commands
        .spawn(NavigationButtonWidgetComponent {
            text: String::from("Unpause"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: GameState::Playing,
        })
        .id();

    let quit_button_entity = commands
        .spawn(NavigationButtonWidgetComponent {
            text: String::from("Quit"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: GameState::Playing,
        })
        .observe(quit_button_observer)
        .id();

    commands
        .entity(button_container)
        .add_children(&[unpause_button_entity, quit_button_entity]);
}

pub fn quit_button_observer(
    _trigger: Trigger<ButtonPushed>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::MainMenu);
}
