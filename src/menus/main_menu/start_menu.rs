use bevy::prelude::*;

use crate::ball::resources::BallScaler;
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

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::StartMenu), setup_menu);
    }
}

pub fn setup_menu(mut commands: Commands, ball_scaler: Res<BallScaler>) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = ContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::StartMenu), parent_node))
        .insert(MenuComponent)
        .insert(Name::new("Start Menu"))
        .id();

    let button_container = commands.spawn(button_container_node).id();

    commands.entity(parent).push_children(&[button_container]);

    let start_button_entity = commands
        .spawn_empty()
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Start Game"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: AppState::InGame,
        }))
        .observe(start_button_observer)
        .id();
    commands
        .entity(button_container)
        .add_child(start_button_entity);

    commands.entity(button_container).add(SpawnSlider::spawn(
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
    ));

    let back_button_entity = commands
        .spawn_empty()
        .add(SpawnNavigationButton::spawn(NavigationButton {
            text: String::from("Back"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::InitialMenu,
        }))
        .id();

    commands
        .entity(button_container)
        .add_child(back_button_entity);
}

pub fn start_button_observer(
    _trigger: Trigger<ButtonPushed>,
    slider_query: Query<&SliderWidgetComponent, With<SmallestBallSize>>,
    mut ball_scaler_stats: ResMut<BallScaler>,
) {
    let slider_value = slider_query.get_single().unwrap();
    ball_scaler_stats.initial_size = slider_value.current_value as f32;
}
