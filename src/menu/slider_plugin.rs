use crate::menu::{MenuComponent, Selectables, SelectedEnt};
use bevy::prelude::*;

use bevy::ui::{FocusPolicy, RelativeCursorPosition};

use super::my_colors;

//Plugin
//
pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SliderWidgetComponent>()
            .add_systems(
                Update,
                move_slider.run_if(any_with_component::<SliderWidgetComponent>),
            )
            .add_systems(
                Update,
                place_knob.run_if(any_with_component::<SliderWidgetComponent>),
            )
            .add_systems(
                Update,
                selected_background.run_if(any_with_component::<SliderWidgetComponent>),
            )
            .add_observer(slider_widget_component_observer)
            .add_observer(slider_track_component_observer);
    }
}
// Components
//
#[derive(Component, Reflect)]
#[require(
    Name::new("Slider Widget"),
    Selectables,
    Button,
    FocusPolicy::Pass,
    Node {
        height: Val::Percent(30.0),
        width: Val::Percent(100.0),
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        margin: UiRect {
            bottom: Val::Px(10.0),
            ..default()
        },
        display: Display::Flex,
        ..Default::default()
    },
    SelectedColor(Color::NONE),
    UnselectedColor(Color::NONE),
)]
pub struct SliderWidgetComponent {
    pub current_value: u32,
    pub step_size: u32,
    pub min: u32,
    pub max: u32,
    pub label: String,
    pub selected_color: Color,
    pub unselected_color: Color,
    pub track_color: Color,
}

#[derive(Component)]
#[require(
    Name::new("Slider Knob"),
    Node {
        border: UiRect {
            left: Val::Px(1.0),
            right: Val::Px(1.0),
            top: Val::Px(1.0),
            bottom: Val::Px(1.0),
        },
        position_type: PositionType::Absolute,
        align_self: AlignSelf::Center,
        height: Val::Percent(40.0),
        width: Val::Percent(5.0),
        left: Val::Percent(00.0),
        display: Display::Flex,
        ..default()
    },
    BackgroundColor(my_colors::BLUE),
    BorderColor(my_colors::BLACK),
)]
pub struct SliderKnobComponent;

#[derive(Component)]
#[require(
    Name::new("Slider Track"),
    Button,
    Node {
        position_type: PositionType::Absolute,
        align_self: AlignSelf::Center,     // vertical
        justify_self: JustifySelf::Center, //horizontal
        height: Val::Percent(5.0),
        width: Val::Percent(100.0),
        display: Display::Flex,
        ..default()
    },
    FocusPolicy::Pass,
    BorderColor(Color::BLACK.into()),
    BackgroundColor(Color::NONE.into()),
)]
pub struct SliderTrackComponent;

#[derive(Component)]
#[require(
    Name::new("Slider Container"),
    Node {
        position_type: PositionType::Relative,
        align_self: AlignSelf::Center,     // vertical
        justify_self: JustifySelf::Center, //horizontal
        height: Val::Percent(100.0),
        width: Val::Percent(100.0),
        display: Display::Flex,
        ..default()
    },
)]
pub struct SliderContainerComponent;

#[derive(Component)]
pub struct SliderTrackEntity();

#[derive(Component)]
pub struct ReadOut;

#[derive(Component)]
pub struct SelectedColor(pub Color);

#[derive(Component)]
pub struct UnselectedColor(pub Color);

// Observers
//
fn slider_track_component_observer(
    trigger: Trigger<OnAdd, SliderTrackComponent>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    commands.entity(entity).observe(track_clicked_observer);
}

fn slider_widget_component_observer(
    trigger: Trigger<OnAdd, SliderWidgetComponent>,
    mut commands: Commands,
    toggle_widget_query: Query<&SliderWidgetComponent>,
) {
    // Get the entity
    let entity = trigger.target();
    let slider_widget_component = toggle_widget_query.get(entity).unwrap();

    commands.entity(entity).insert((
        SelectedColor(slider_widget_component.selected_color),
        UnselectedColor(slider_widget_component.unselected_color),
        BackgroundColor(slider_widget_component.unselected_color),
        children![(
            SliderContainerComponent,
            children![
                (
                    Name::new("Slider Label Container"),
                    Node {
                        width: Val::Percent(35.0),
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![(
                        Name::new("Slider Label"),
                        Text::new(slider_widget_component.label.clone()),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: 50.0,
                            ..Default::default()
                        },
                    )],
                ),
                (
                    Name::new("Slider Track Container"),
                    Node {
                        width: Val::Percent(40.0),
                        ..default()
                    },
                    children![
                        (
                            Name::new("Slider Track"),
                            SliderTrackComponent,
                            RelativeCursorPosition::default(),
                            BackgroundColor(slider_widget_component.track_color)
                        ),
                        (Name::new("Slider Knob"), SliderKnobComponent),
                    ],
                ),
                (
                    Name::new("Slider Readout Container"),
                    Node {
                        width: Val::Percent(25.0),
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![(
                        ReadOut,
                        Text::new("0"),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: 150.0,
                            ..Default::default()
                        },
                    )],
                )
            ],
        )],
    ));
}

pub fn track_clicked_observer(
    mut trigger: Trigger<Pointer<Click>>,
    mut widget_component_query: Query<&mut SliderWidgetComponent>,
    child_of: Query<&ChildOf>,
    relative_cursor_query: Query<&RelativeCursorPosition>,
) {
    let entity = trigger.target();
    trigger.propagate(false);

    let widget_entity = child_of
        .iter_ancestors(entity)
        .find(|parent| widget_component_query.get(*parent).is_ok())
        .unwrap();

    let mut widget_component = widget_component_query.get_mut(widget_entity).unwrap();

    let relativce_cursor_position = *relative_cursor_query.get(entity).unwrap();
    let relative_cursor_position = relativce_cursor_position.normalized.unwrap();
    let l: u32 = ((widget_component.max - widget_component.min) as f32 * relative_cursor_position.x
        + widget_component.min as f32) as u32;
    widget_component.current_value = l - (l % widget_component.step_size);
}

// Systems
//
pub fn move_slider(
    keys: Res<ButtonInput<KeyCode>>,
    mut slider_query: Query<(Entity, &mut SliderWidgetComponent), With<SliderWidgetComponent>>,
    selected_query: Query<&SelectedEnt, With<MenuComponent>>,
) {
    if selected_query.is_empty() {
        return;
    }

    let SelectedEnt(selected_ent) = selected_query.single().unwrap();

    for (entity, mut slider_component) in &mut slider_query {
        if keys.just_pressed(KeyCode::ArrowLeft) && selected_ent == &entity {
            if slider_component.current_value > slider_component.step_size + slider_component.min {
                slider_component.current_value -= slider_component.step_size;
            } else {
                slider_component.current_value = slider_component.min;
            }
        }
        if keys.just_pressed(KeyCode::ArrowRight) && selected_ent == &entity {
            slider_component.current_value = u32::clamp(
                slider_component.current_value + slider_component.step_size,
                0,
                slider_component.max,
            )
        }
    }
}

pub fn place_knob(
    mut widget_query: Query<(Entity, &mut SliderWidgetComponent), With<SliderWidgetComponent>>,
    children_query: Query<&Children>,
    knob_query: Query<Entity, With<SliderKnobComponent>>,
    readout_query: Query<Entity, With<ReadOut>>,
    mut node_query: Query<&mut Node>,
    mut text_query: Query<&mut Text>,
) {
    for (widget_entity, widget_component) in &mut widget_query {
        let knob_entity = children_query
            .iter_descendants(widget_entity)
            .find(|child| knob_query.get(*child).is_ok())
            .unwrap();
        let mut knob_node = node_query.get_mut(knob_entity).unwrap();

        let readout_entity = children_query
            .iter_descendants(widget_entity)
            .find(|child| readout_query.get(*child).is_ok())
            .unwrap();
        let mut readout_text = text_query.get_mut(readout_entity).unwrap();

        if let Val::Percent(knob_width) = knob_node.width {
            let multiplier: f32 =
                (100.0 - knob_width) / (widget_component.max - widget_component.min) as f32;
            let p: f32 = widget_component.current_value as f32 * multiplier;
            knob_node.left = Val::Percent(p);
        }
        readout_text.0 = format!("{}", widget_component.current_value);
    }
}

pub fn selected_background(
    mut widget_query: Query<
        (
            Entity,
            &mut BackgroundColor,
            &SelectedColor,
            &UnselectedColor,
        ),
        With<SliderWidgetComponent>,
    >,
    selected_query: Query<&SelectedEnt, With<MenuComponent>>,
) {
    // Set the widgets background to the default color when no widgets are selected
    if selected_query.is_empty() {
        for (
            _entity,
            mut background_color,
            SelectedColor(_selected_color),
            UnselectedColor(unselected_color),
        ) in &mut widget_query
        {
            *background_color = BackgroundColor(*unselected_color);
        }
        return;
    }

    // Change the selected widgets background
    let SelectedEnt(selected_ent) = selected_query.single().unwrap();
    for (
        entity,
        mut background_color,
        SelectedColor(selected_color),
        UnselectedColor(unselected_color),
    ) in &mut widget_query
    {
        if selected_ent == &entity {
            *background_color = BackgroundColor(*selected_color);
        } else {
            *background_color = BackgroundColor(*unselected_color);
        }
    }
}
