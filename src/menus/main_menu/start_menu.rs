use bevy::prelude::*;

use crate::ball::resources::{BallScaler, GrowStats};
use crate::menu::navigation_button_plugin::{ButtonPushed, NavigationButtonWidgetComponent};
use crate::menu::slider_plugin::SliderWidgetComponent;
use crate::menu::{MenuComponent, WidgetContainerComponent};
use crate::my_colors;
use crate::AppState;

use crate::main_menu::MainMenuState;

#[derive(Component)]
pub struct SmallestBallSize;

#[derive(Component)]
pub struct GrowSpeed;

#[derive(Component)]
pub struct StartButtonComponent;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::StartMenu), setup_menu)
            .add_observer(start_button_component_observer);
    }
}
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

pub fn setup_menu(
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
                //.observe(start_button_observer)
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
