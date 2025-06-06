use crate::menu::components::*;
use crate::menu::selector_plugin::*;
use bevy::prelude::*;

pub fn mouse_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<SelectorWidgetComponent>),
    >,
    parent_query: Query<Entity, With<MenuComponent>>,
) {
    let parent_entity = parent_query.single().unwrap();
    for (entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                commands.entity(parent_entity).insert(SelectedEnt(entity));
            }
            Interaction::None => {
                commands.entity(parent_entity).remove::<SelectedEnt>();
            }
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
        (Entity, &SelectorWidgetComponent),
        Changed<SelectorWidgetComponent>,
    >,
    current_selection_query: Query<&CurrentSelectionComponent>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text>,
) {
    for (entity, selector_component) in &selection_entity_query {
        let current_selection_entity = children_query
            .iter_descendants(entity)
            .find(|child| current_selection_query.get(*child).is_ok())
            .unwrap();
        let mut selection_text = text_query.get_mut(current_selection_entity).unwrap();
        selection_text.0 =
            selector_component.selections[selector_component.current_index as usize].clone();
    }
}

pub fn previous_button_clicked(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<PreviousButtonComponent>),
    >,
    mut selector_query: Query<&mut SelectorWidgetComponent, With<SelectorWidgetComponent>>,
    parent_query: Query<&ChildOf>,
) {
    for (previous_button_entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {}
            Interaction::None => {}
            Interaction::Pressed => {
                let parent_entity = parent_query
                    .iter_ancestors(previous_button_entity)
                    .find(|parent| selector_query.get(*parent).is_ok())
                    .unwrap();

                let mut selector_component = selector_query.get_mut(parent_entity).unwrap();

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

pub fn next_button_clicked(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<NextButtonComponent>),
    >,
    mut selector_query: Query<&mut SelectorWidgetComponent, With<SelectorWidgetComponent>>,
    parent_query: Query<&ChildOf>,
    //parent_query: Query<&Parent>,
) {
    for (next_button_entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {}
            Interaction::None => {}
            Interaction::Pressed => {
                let parent_entity = parent_query
                    .iter_ancestors(next_button_entity)
                    .find(|parent| selector_query.get(*parent).is_ok())
                    .unwrap();

                let mut selector_component = selector_query.get_mut(parent_entity).unwrap();

                let length = selector_component.selections.len() as u32;
                selector_component.current_index = (selector_component.current_index + 1) % length;
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
