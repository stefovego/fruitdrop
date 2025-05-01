use std::usize;

use super::bundles::*;
use crate::menu::components::*;
use bevy::prelude::*;

pub fn mouse_system(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<SelectorWidgetComponent>),
    >,
    parent_query: Query<Entity, With<MenuComponent>>,
    mut commands: Commands,
) {
    // let parent_entity = parent_query.get_single().unwrap();
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
    mut widget_query: Query<
        (
            Entity,
            &mut BackgroundColor,
            &SelectedColor,
            &UnselectedColor,
        ),
        With<SelectorWidgetComponent>,
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
    ) in &mut widget_query
    {
        if selected_ent == &entity {
            *background_color = BackgroundColor(*selected_color);
        } else {
            *background_color = BackgroundColor(*unselected_color);
        }
    }
}
pub fn show_selection(
    selection_entity_query: Query<
        (&CurrentSelectionEntity, &SelectorWidgetComponent),
        With<CurrentSelectionEntity>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for (CurrentSelectionEntity(current_selection), selector_component) in &selection_entity_query {
        let mut selection_text = text_query.get_mut(*current_selection).unwrap();
        selection_text.0 =
            selector_component.selections[selector_component.current_index as usize].clone();
    }
}
// pub fn show_selection(
//     mut toggle_button_query: Query<
//         (
//             &mut ToggleWidgetComponent,
//             &OnButtonEntity,
//             &OffButtonEntity,
//         ),
//         With<ToggleWidgetComponent>,
//     >,
//     mut button_query: Query<&mut BorderColor, With<Button>>,
// ) {
//     for (toggle_component, OnButtonEntity(on_button), OffButtonEntity(off_button)) in
//         &mut toggle_button_query
//     {
//         let [mut off_button_border, mut on_button_border] = button_query
//             .get_many_mut([*off_button, *on_button])
//             .unwrap();
//         if !toggle_component.current_value {
//             *off_button_border = BorderColor(Color::ORANGE);
//             *on_button_border = BorderColor(Color::NONE);
//         }
//         if toggle_component.current_value {
//             *off_button_border = BorderColor(Color::NONE);
//             *on_button_border = BorderColor(Color::ORANGE);
//         }
//     }
// }

pub fn previous_button_clicked(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<PreviousComponent>),
    >,
    mut selector_query: Query<&mut SelectorWidgetComponent, With<SelectorWidgetComponent>>,
    parent_query: Query<&ChildOf>,
    //parent_query: Query<&Parent>,
) {
    for (previous_button_entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {}
            Interaction::None => {}
            Interaction::Pressed => {
                if let Some(parent_entity) =
                    parent_query.iter_ancestors(previous_button_entity).nth(1)
                {
                    if let Ok(mut selector_component) = selector_query.get_mut(parent_entity) {
                        let length = selector_component.selections.len() as u32;
                        if selector_component.current_index == 0 {
                            selector_component.current_index = length - 1
                        } else {
                            selector_component.current_index =
                                (selector_component.current_index - 1) % length;
                        }
                    }
                }
            }
        }
    }
}

pub fn next_button_clicked(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<NextComponent>),
    >,
    mut selector_query: Query<&mut SelectorWidgetComponent, With<SelectorWidgetComponent>>,
    parent_query: Query<&ChildOf>,
    //parent_query: Query<&Parent>,
) {
    for (previous_button_entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {}
            Interaction::None => {}
            Interaction::Pressed => {
                if let Some(parent_entity) =
                    parent_query.iter_ancestors(previous_button_entity).nth(1)
                {
                    if let Ok(mut selector_component) = selector_query.get_mut(parent_entity) {
                        let length = selector_component.selections.len() as u32;
                        selector_component.current_index =
                            (selector_component.current_index + 1) % length;
                    }
                }
            }
        }
    }
}

pub fn key_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut selector_query: Query<&mut SelectorWidgetComponent, With<SelectorWidgetComponent>>,
    selected_query: Query<&SelectedEnt, With<MenuComponent>>,
) {
    if selected_query.is_empty() {
        return;
    }
    let SelectedEnt(selected_ent) = selected_query.single().unwrap();

    if let Ok(mut selector_component) = selector_query.get_mut(*selected_ent) {
        if keys.just_pressed(KeyCode::ArrowLeft) {
            let length = selector_component.selections.len() as u32;
            if selector_component.current_index == 0 {
                selector_component.current_index = length - 1
            } else {
                selector_component.current_index = (selector_component.current_index - 1) % length;
            }
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            let length = selector_component.selections.len() as u32;
            selector_component.current_index = (selector_component.current_index + 1) % length;
        }
    }
}
