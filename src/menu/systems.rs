use crate::menu::components::*;
use bevy::prelude::*;

pub fn key_input(
    keys: Res<ButtonInput<KeyCode>>,
    selected_items: Query<(Entity, &SelectedEnt), With<MenuComponent>>,
    menu_query: Query<Entity, With<MenuComponent>>,
    container_query: Query<&Children, With<ContainerComponent>>,
    items_query: Query<Entity, With<Selectables>>,
    mut commands: Commands,
) {
    let c = items_query.iter().next().unwrap();
    if selected_items.is_empty() {
        if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowDown) {
            commands.entity(menu_query.single()).insert(SelectedEnt(c));
        }
        return;
    }

    let (entity, SelectedEnt(currently_selected)) = &selected_items.single();
    let item_list: Vec<Entity> = container_query
        .iter()
        .flat_map(|ent| items_query.iter_many(ent))
        .collect();

    if keys.just_pressed(KeyCode::ArrowUp) {
        commands
            .entity(*entity)
            .insert(SelectedEnt(prev_entity(*currently_selected, &item_list)));
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        commands
            .entity(*entity)
            .insert(SelectedEnt(next_entity(*currently_selected, &item_list)));
    }
}

pub fn teardown_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuComponent>>) {
    print!("Tear Down This Menu");
    for entity in &menu_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn prev_entity(needle: Entity, haystack: &[Entity]) -> Entity {
    let haystack_length = haystack.len();

    if haystack.len() == 1 {
        return haystack[0];
    }

    if haystack[0] == needle {
        return haystack[haystack_length - 1];
    }

    for (i, hay) in haystack.iter().enumerate() {
        if hay == &needle {
            return haystack[i - 1];
        }
    }
    haystack[0]
}

pub fn next_entity(needle: Entity, haystack: &[Entity]) -> Entity {
    let haystack_length = haystack.len();

    if haystack.len() == 1 {
        return haystack[0];
    }

    if haystack[haystack_length - 1] == needle {
        return haystack[0];
    }

    for (i, hay) in haystack.iter().enumerate() {
        if hay == &needle {
            return haystack[i + 1];
        }
    }
    haystack[0]
}
