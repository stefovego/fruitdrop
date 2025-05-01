use crate::menu::components::*;
use crate::slider_plugin::bundles::*;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

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
    mut slider_query: Query<
        (
            &mut SliderWidgetComponent,
            &SliderEntity,
            &SliderReadOutEntity,
        ),
        With<SliderWidgetComponent>,
    >,
    mut knob_query: Query<&mut Node, With<SliderKnob>>,
    mut readout_query: Query<&mut Text, With<ReadOut>>,
) {
    for (slider_component, SliderEntity(slider_entity), SliderReadOutEntity(readout_entity)) in
        &mut slider_query
    {
        if let Ok(mut style) = knob_query.get_mut(*slider_entity) {
            if let Val::Percent(knob_width) = style.width {
                let multiplier: f32 =
                    (100.0 - knob_width) / (slider_component.max - slider_component.min) as f32;
                let p: f32 = slider_component.current_value as f32 * multiplier;
                style.left = Val::Percent(p);
            }
        }
        if let Ok(mut readout_text) = readout_query.get_mut(*readout_entity) {
            readout_text.0 = format!("{}", slider_component.current_value);
        }
    }
}

pub fn mouse_system(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<SliderWidgetComponent>),
    >,
    parent_query: Query<Entity, With<MenuComponent>>,
    mut commands: Commands,
) {
    let parent_entity = if let Ok(parent_entity) = parent_query.single() {
        parent_entity
    } else {
        return;
    };
    for (entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                commands.entity(parent_entity).insert(SelectedEnt(entity));
            }
            Interaction::None => {}
            Interaction::Pressed => {}
        }
    }
}

pub fn selected_background(
    mut slider_query: Query<
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
    if selected_query.is_empty() {
        return;
    }

    let SelectedEnt(selected_ent) = selected_query.single().unwrap();

    for (
        entity,
        mut background_color,
        SelectedColor(selected_color),
        UnselectedColor(unselected_color),
    ) in &mut slider_query
    {
        if selected_ent == &entity {
            *background_color = BackgroundColor(*selected_color);
        } else {
            *background_color = BackgroundColor(*unselected_color);
        }
    }
}

pub fn mousy_stuffy(
    parent_query: Query<&ChildOf>,
    relative_cursor_query: Query<(Entity, &RelativeCursorPosition), With<RelativeCursorPosition>>,
    mut slider_query: Query<&mut SliderWidgetComponent>,
    mouse_click_query: Res<ButtonInput<MouseButton>>,
) {
    for (entity, relative_cursor_position) in &relative_cursor_query {
        if mouse_click_query.just_pressed(MouseButton::Left)
            && relative_cursor_position.mouse_over()
        {
            if let Some(relative_cursor_position) = relative_cursor_position.normalized {
                if let Some(parent_entity) = parent_query.iter_ancestors(entity).nth(1) {
                    if let Ok(mut slider_comp) = slider_query.get_mut(parent_entity) {
                        let l: u32 = ((slider_comp.max - slider_comp.min) as f32
                            * relative_cursor_position.x
                            + slider_comp.min as f32) as u32;
                        slider_comp.current_value = l - (l % slider_comp.step_size);
                    }
                }
            }
        }
    }
}
