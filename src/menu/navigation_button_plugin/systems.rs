use crate::menu::components::*;
use bevy::prelude::*;

use super::{ButtonPushed, NavigationButtonComponent, SelectedColor, UnselectedColor};

pub fn selected_background(
    mut item_query: Query<
        (
            &mut BackgroundColor,
            Entity,
            &SelectedColor,
            &UnselectedColor,
        ),
        With<NavigationButtonComponent>,
    >,
    parent_query: Query<&SelectedEnt, (With<MenuComponent>, Changed<SelectedEnt>)>,
) {
    for SelectedEnt(currently_selected) in &parent_query {
        for (
            mut background_color,
            menu_item,
            SelectedColor(selected_color),
            UnselectedColor(unselected_color),
        ) in &mut item_query
        {
            if currently_selected == &menu_item {
                *background_color = BackgroundColor(*selected_color);
            } else {
                *background_color = BackgroundColor(*unselected_color);
            }
        }
    }
}

pub fn mouse_system(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<NavigationButtonComponent>),
    >,
    parent_query: Query<Entity, With<MenuComponent>>,
    mut commands: Commands,
) {
    let parent_entity = parent_query.get_single().unwrap();
    for (entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                commands.entity(parent_entity).insert(SelectedEnt(entity));
            }
            Interaction::None => {}
            Interaction::Pressed => {
                commands.trigger_targets(ButtonPushed, entity);
            }
        }
    }
}

pub fn keyboard_select(
    keys: Res<ButtonInput<KeyCode>>,
    parent_query: Query<&SelectedEnt, With<MenuComponent>>,
    mut commands: Commands,
) {
    if parent_query.is_empty() {
        return;
    }

    let SelectedEnt(currently_selected) = parent_query.single();

    if keys.just_pressed(KeyCode::Enter) {
        commands.trigger_targets(ButtonPushed, *currently_selected);
    }
}
