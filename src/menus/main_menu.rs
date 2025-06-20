use bevy::app::AppExit;
use bevy::prelude::*;

use crate::ball::resources::{BallScaler, GrowStats};
use crate::game_state::{AppState, GameState};
use crate::menu::navigation_button_plugin::{
    ButtonPushed, NavigationButtonPlugin, NavigationButtonWidgetComponent,
};
use crate::menu::selector_plugin::SelectorWidgetComponent;
use crate::menu::slider_plugin::SliderWidgetComponent;
use crate::menu::toggle_plugin::ToggleWidgetComponent;
use crate::menu::{my_colors, MenuComponent, WidgetContainerComponent};

// Components
//
#[derive(Component)]
pub struct SmallestBallSize;

#[derive(Component)]
pub struct GrowSpeed;

#[derive(Component)]
pub struct StartButtonComponent;

// States
//
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::MainMenu)]
pub enum MainMenuState {
    #[default]
    InitialMenu,
    StartMenu,
    OptionsMenu,
    VideoMenu,
    GameMenu,
    QuitGame,
}

// Plugin
//
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MainMenuState>()
            .enable_state_scoped_entities::<MainMenuState>()
            .enable_state_scoped_entities::<AppState>()
            .add_plugins((
                NavigationButtonPlugin::<GameState>::default(),
                NavigationButtonPlugin::<MainMenuState>::default(),
                NavigationButtonPlugin::<AppState>::default(),
            ))
            .add_systems(OnEnter(MainMenuState::InitialMenu), setup_initial_menu)
            .add_systems(OnEnter(MainMenuState::OptionsMenu), setup_options_menu)
            .add_systems(OnEnter(MainMenuState::VideoMenu), setup_video_menu)
            .add_systems(OnEnter(MainMenuState::GameMenu), setup_game_menu)
            .add_systems(OnEnter(MainMenuState::StartMenu), setup_start_menu)
            .add_systems(OnEnter(MainMenuState::QuitGame), quit_system)
            .add_observer(start_button_component_observer);
    }
}

// Observers
//
pub fn start_button_component_observer(
    trigger: Trigger<OnAdd, StartButtonComponent>,
    mut commands: Commands,
) {
    // Get the entity
    let entity = trigger.target();

    commands.entity(entity).observe(start_button_observer);
}

pub fn start_button_observer(
    _trigger: Trigger<ButtonPushed>,
    size_slider_query: Query<&SliderWidgetComponent, With<SmallestBallSize>>,
    grow_speed_slider_query: Query<&SliderWidgetComponent, With<GrowSpeed>>,
    mut ball_scaler_stats: ResMut<BallScaler>,
    mut grow_stats: ResMut<GrowStats>,
) {
    let size_slider_value = size_slider_query.single().unwrap();
    let grow_speed_slider_value = grow_speed_slider_query.single().unwrap();
    ball_scaler_stats.initial_size = size_slider_value.current_value as f32;
    grow_stats.grow_speed = grow_speed_slider_value.current_value as f32;
}

// Systems
//

pub fn setup_initial_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Menu Screen"),
        MenuComponent,
        StateScoped(MainMenuState::InitialMenu),
        children![(
            WidgetContainerComponent,
            children![
                NavigationButtonWidgetComponent {
                    text: String::from("New Game"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::StartMenu,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Options"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::OptionsMenu,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Quit"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::QuitGame,
                },
            ]
        )],
    ));
}

pub fn setup_options_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Options Menu"),
        StateScoped(MainMenuState::OptionsMenu),
        MenuComponent,
        children![(
            WidgetContainerComponent,
            children![
                NavigationButtonWidgetComponent {
                    text: String::from("Video"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::VideoMenu,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Game"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::GameMenu,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Back"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::InitialMenu,
                },
            ]
        )],
    ));
}

pub fn setup_video_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Video Menu"),
        MenuComponent,
        StateScoped(MainMenuState::VideoMenu),
        children![(
            WidgetContainerComponent,
            children![
                (
                    Text::new("Under Construction"),
                    TextFont {
                        font_size: 50.0,
                        ..Default::default()
                    },
                    TextColor(my_colors::PINK),
                ),
                NavigationButtonWidgetComponent {
                    text: String::from("Back"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::OptionsMenu,
                },
            ]
        )],
    ));
}

pub fn setup_game_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Game Menu"),
        MenuComponent,
        StateScoped(MainMenuState::GameMenu),
        children![(
            WidgetContainerComponent,
            children![
                ToggleWidgetComponent {
                    label: "Cool?".into(),
                    current_value: true,
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                },
                SelectorWidgetComponent {
                    current_index: 0,
                    label: "Stuff".into(),
                    selections: ["Stuff ONe".into(), "stuff 2".into(), "stuff3".into()].into(),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Back"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::OptionsMenu,
                }
            ]
        )],
    ));
}

pub fn setup_start_menu(
    mut commands: Commands,
    ball_scaler: Res<BallScaler>,
    grow_stats: Res<GrowStats>,
) {
    commands.spawn((
        Name::new("Start Menu"),
        StateScoped(MainMenuState::StartMenu),
        MenuComponent,
        children![(
            WidgetContainerComponent,
            children![
                (
                    StartButtonComponent,
                    NavigationButtonWidgetComponent {
                        text: String::from("Start Game"),
                        selected_color: my_colors::BLUE,
                        unselected_color: my_colors::PURPLE,
                        next_state: AppState::InGame,
                    }
                ),
                (
                    SliderWidgetComponent {
                        label: "Smallest Ball Size".into(),
                        current_value: ball_scaler.initial_size as u32,
                        step_size: 3,
                        min: 0,
                        max: 120,
                        selected_color: my_colors::BLUE,
                        unselected_color: my_colors::PURPLE,
                        track_color: my_colors::PINK,
                    },
                    SmallestBallSize,
                ),
                (
                    SliderWidgetComponent {
                        label: "Grow Speed".into(),
                        current_value: grow_stats.grow_speed as u32,
                        step_size: 1,
                        min: 0,
                        max: 120,
                        selected_color: my_colors::BLUE,
                        unselected_color: my_colors::PURPLE,
                        track_color: my_colors::PINK,
                    },
                    GrowSpeed,
                ),
                NavigationButtonWidgetComponent {
                    text: String::from("Back"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::InitialMenu,
                },
            ]
        )],
    ));
}

pub fn quit_system(mut events: EventWriter<AppExit>) {
    events.write(AppExit::Success);
}
