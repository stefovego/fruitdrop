use crate::main_menu::menu_state::MainMenuState;
use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::my_colors;
use crate::slider_plugin::bundles::SliderWidgetComponent;
use bevy::prelude::*;

use crate::navigation_button_plugin::{NavigationButton, SpawnNavigationButton};
use crate::selector_plugin::{Selector, SpawnSelector};
use crate::slider_plugin::{Slider, SpawnSlider};
use crate::toggle_plugin::{SpawnToggle, Toggle};

#[derive(Event)]
pub struct GoingBackEvent;

#[derive(Component)]
pub struct BackButton;

pub fn setup_menu(mut commands: Commands) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = ContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::GameMenu), parent_node))
        .insert(MenuComponent)
        .insert(Name::new("Game Menu"))
        .id();
    let button_container = commands.spawn(button_container_node).id();
    commands.entity(parent).push_children(&[button_container]);

    commands
        .entity(button_container)
        .add(SpawnSlider::spawn(Slider {
            label: "Awesomeness".into(),
            initial_value: 30,
            step_size: 3,
            min: 0,
            max: 120,
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            track_color: my_colors::PINK,
        }));

    commands
        .entity(button_container)
        .add(SpawnSlider::spawn(Slider {
            label: "Kickassness".into(),
            initial_value: 6,
            step_size: 3,
            min: 0,
            max: 60,
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            track_color: my_colors::PINK,
        }));

    commands
        .entity(button_container)
        .add(SpawnToggle::spawn(Toggle {
            initial_value: true,
            label: "Cool?".into(),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }));

    commands
        .entity(button_container)
        .add(SpawnSelector::spawn(Selector {
            label: "Stuff".into(),
            selections: ["Stuff ONe".into(), "stuff 2".into(), "stuff3".into()].into(),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
        }));

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
    parent_query: Query<&SelectedEnt, With<MenuComponent>>,
    back_button_query: Query<Entity, With<BackButton>>,
    mut going_back_event: EventWriter<GoingBackEvent>,
) {
    if parent_query.is_empty() {
        return;
    }

    let SelectedEnt(currently_selected) = parent_query.single();
    let back_button_entity = back_button_query.single();

    if keys.just_pressed(KeyCode::Enter) && back_button_entity == *currently_selected {
        going_back_event.send(GoingBackEvent);
    }
}

pub fn back_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<BackButton>),
    >,
    mut going_back_event: EventWriter<GoingBackEvent>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            going_back_event.send(GoingBackEvent);
        }
    }
}

pub fn exit_menu(
    mut ev_going_back: EventReader<GoingBackEvent>,
    mut next_state: ResMut<NextState<MainMenuState>>,
    slider_query: Query<&SliderWidgetComponent, With<SliderWidgetComponent>>,
) {
    for _ in ev_going_back.read() {
        for (i, slider_component) in slider_query.iter().enumerate() {
            println!("Slider {} Value: {}", i, slider_component.current_value);
        }
        next_state.set(MainMenuState::OptionsMenu);
    }
}
