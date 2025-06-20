use bevy::prelude::*;

use crate::{
    game_state::{AppState, GameState},
    menu::{
        my_colors,
        navigation_button_plugin::{ButtonPushed, NavigationButtonWidgetComponent},
        MenuComponent, WidgetContainerComponent,
    },
};

// Plugin
//
pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PauseMenuState>()
            .enable_state_scoped_entities::<GameState>()
            .enable_state_scoped_entities::<AppState>()
            .add_systems(OnEnter(PauseMenuState::InitialMenu), setup_menu)
            .add_observer(quit_button_component_observer);
    }
}

// Components
#[derive(Component)]
pub struct QuitButtonComponent;

// States
//
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Paused)]
pub enum PauseMenuState {
    #[default]
    InitialMenu,
    //Unpaused,
}
// Observers
//
pub fn quit_button_component_observer(
    trigger: Trigger<OnAdd, QuitButtonComponent>,
    mut commands: Commands,
) {
    // Get the entity
    let entity = trigger.target();

    commands.entity(entity).observe(quit_button_observer);
}

pub fn quit_button_observer(
    _trigger: Trigger<ButtonPushed>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::MainMenu);
}

// Systems
//
pub fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Initial Pause Menu"),
        StateScoped(GameState::Paused),
        MenuComponent,
        children![(
            WidgetContainerComponent,
            children![
                NavigationButtonWidgetComponent {
                    text: String::from("Unpause"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: GameState::Playing,
                },
                (
                    QuitButtonComponent,
                    NavigationButtonWidgetComponent {
                        text: String::from("Quit"),
                        selected_color: my_colors::BLUE,
                        unselected_color: my_colors::PURPLE,
                        next_state: GameState::Playing,
                    }
                )
            ]
        )],
    ));
}
