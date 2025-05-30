use crate::menu::components::*;
use crate::my_colors;
use crate::toggle_plugin::bundles::*;
use bevy::prelude::*;

pub fn mouse_system(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<ToggleWidgetComponent>),
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
        With<ToggleWidgetComponent>,
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
    mut toggle_button_query: Query<
        (Entity, &mut ToggleWidgetComponent),
        With<ToggleWidgetComponent>,
    >,
    children_query: Query<&Children>,
    on_button_query: Query<Entity, With<OnButtonComponent>>,
    off_button_query: Query<Entity, With<OffButtonComponent>>,
    mut border_query: Query<&mut BorderColor>,
) {
    for (widget_entity, toggle_component) in &mut toggle_button_query {
        let on_button_entity = children_query
            .iter_descendants(widget_entity)
            .find(|child| on_button_query.get(*child).is_ok())
            .unwrap();

        let off_button_entity = children_query
            .iter_descendants(widget_entity)
            .find(|child| off_button_query.get(*child).is_ok())
            .unwrap();

        let [mut off_button_border, mut on_button_border] = border_query
            .get_many_mut([off_button_entity, on_button_entity])
            .unwrap();

        if !toggle_component.current_value {
            *off_button_border = BorderColor(my_colors::ORANGE); //Orange
            *on_button_border = BorderColor(Color::NONE);
        }
        if toggle_component.current_value {
            *off_button_border = BorderColor(Color::NONE);
            *on_button_border = BorderColor(my_colors::ORANGE);
        }
    }
}

pub fn on_click_toggle(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<OnButtonComponent>),
    >,
    mut toggle_button_query: Query<&mut ToggleWidgetComponent, With<ToggleWidgetComponent>>,
    parent_query: Query<&ChildOf>,
) {
    for (button_entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {}
            Interaction::None => {}
            Interaction::Pressed => {
                let toggle_widget_entity = parent_query
                    .iter_ancestors(button_entity)
                    .find(|parent| toggle_button_query.get(*parent).is_ok())
                    .unwrap();

                let mut toggle_component =
                    toggle_button_query.get_mut(toggle_widget_entity).unwrap();
                toggle_component.current_value = true;
            }
        }
    }
}

pub fn off_click_toggle(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<OffButtonComponent>),
    >,
    mut toggle_button_query: Query<&mut ToggleWidgetComponent, With<ToggleWidgetComponent>>,
    parent_query: Query<&ChildOf>,
) {
    for (button_entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {}
            Interaction::None => {}
            Interaction::Pressed => {
                let toggle_widget_entity = parent_query
                    .iter_ancestors(button_entity)
                    .find(|parent| toggle_button_query.get(*parent).is_ok())
                    .unwrap();

                let mut toggle_component =
                    toggle_button_query.get_mut(toggle_widget_entity).unwrap();
                toggle_component.current_value = false;
            }
        }
    }
}

pub fn key_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut toggle_button_query: Query<&mut ToggleWidgetComponent, With<ToggleWidgetComponent>>,
    selected_query: Query<&SelectedEnt, With<MenuComponent>>,
) {
    if selected_query.is_empty() {
        return;
    }
    let SelectedEnt(selected_ent) = selected_query.single().unwrap();

    if let Ok(mut toggle_query) = toggle_button_query.get_mut(*selected_ent) {
        if keys.just_pressed(KeyCode::ArrowLeft) {
            toggle_query.current_value = false;
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            toggle_query.current_value = true;
        }
        if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
            toggle_query.current_value = !toggle_query.current_value;
        }
    }
}
