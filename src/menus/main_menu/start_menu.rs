use bevy::prelude::*;

use crate::ball::resources::{BallScaler, GrowStats};
use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::my_colors;
use crate::AppState;

use crate::main_menu::MainMenuState;
use crate::navigation_button_plugin::{ButtonPushed, NavigationButton, SpawnNavigationButton};
use crate::slider_plugin::bundles::SliderWidgetComponent;
use crate::slider_plugin::{Slider, SpawnSlider};

#[derive(Component)]
pub struct SmallestBallSize;

#[derive(Component)]
pub struct GrowSpeed;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::StartMenu), setup_menu);
    }
}

pub fn setup_menu(
    mut commands: Commands,
    ball_scaler: Res<BallScaler>,
    grow_stats: Res<GrowStats>,
) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = ContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::StartMenu), parent_node))
        .insert(MenuComponent)
        .insert(Name::new("Start Menu"))
        .id();

    let button_container = commands.spawn(button_container_node).id();

    commands.entity(parent).add_children(&[button_container]);

    let start_button_entity = commands
        .spawn_empty()
        .queue(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Start Game"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: AppState::InGame,
        }))
        .observe(start_button_observer)
        .id();

    let ball_size_slider_entity = commands
        .spawn_empty()
        .queue(SpawnSlider::spawn(
            Slider {
                label: "Smallest Ball Size".into(),
                initial_value: ball_scaler.initial_size as u32,
                step_size: 3,
                min: 0,
                max: 120,
                selected_color: my_colors::BLUE,
                unselected_color: my_colors::PURPLE,
                track_color: my_colors::PINK,
            },
            SmallestBallSize,
        ))
        .id();

    let grow_speed_slider_entity = commands
        .spawn_empty()
        .queue(SpawnSlider::spawn(
            Slider {
                label: "Grow Speed".into(),
                initial_value: grow_stats.grow_speed as u32,
                step_size: 1,
                min: 0,
                max: 120,
                selected_color: my_colors::BLUE,
                unselected_color: my_colors::PURPLE,
                track_color: my_colors::PINK,
            },
            GrowSpeed,
        ))
        .id();

    let back_button_entity = commands
        .spawn_empty()
        .queue(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Back"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::InitialMenu,
        }))
        .id();

    commands.entity(button_container).add_children(&[
        start_button_entity,
        ball_size_slider_entity,
        grow_speed_slider_entity,
        back_button_entity,
    ]);
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
