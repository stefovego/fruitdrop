use bevy::prelude::*;

pub mod my_colors;
pub mod navigation_button_plugin;
pub mod selector_plugin;
pub mod slider_plugin;
pub mod toggle_plugin;

//use crate::menu::components::{MenuComponent, Selectables, SelectedEnt, WidgetContainerComponent};

// Plugins
//
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SelectedEnt>()
            .add_plugins(selector_plugin::SelectorPlugin)
            .add_plugins(slider_plugin::SliderPlugin)
            .add_plugins(toggle_plugin::TogglePlugin)
            .add_systems(
                Update,
                key_input.run_if(any_with_component::<MenuComponent>),
            )
            .add_observer(widget_component_observer);
    }
}
// Systems
//

#[derive(Component)]
pub struct SelectedComponent;

#[derive(Component, Default)]
pub struct Selectables;

#[derive(Component, Reflect)]
pub struct SelectedEnt(pub Entity);

#[derive(Component)]
#[require(          Node {
                display: Display::Flex,
                position_type: PositionType::Relative,
                width: Val::Percent(90.0),
                height: Val::Percent(90.0),
                bottom: Val::Percent(5.0),
                top: Val::Percent(5.0),
                left: Val::Percent(5.0),
                right: Val::Percent(5.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(my_colors::GRAY))]

pub struct MenuComponent;

#[derive(Component)]
#[require(
            Name::new("Widget Container"),
            Node {
                position_type: PositionType::Relative,
                display: Display::Flex,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
    )]
pub struct WidgetContainerComponent;

//Observers
//
fn widget_component_observer(trigger: Trigger<OnAdd, Selectables>, mut commands: Commands) {
    // Get the entity
    let entity = trigger.target();
    commands
        .entity(entity)
        .observe(widget_mouse_over_observer)
        .observe(widget_mouse_out_observer);
}

pub fn widget_mouse_over_observer(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    menu_query: Query<Entity, With<MenuComponent>>,
) {
    let entity = trigger.target();
    let menu_entity = menu_query.single().unwrap();
    commands.entity(menu_entity).insert(SelectedEnt(entity));
}

pub fn widget_mouse_out_observer(
    _trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    menu_query: Query<Entity, With<MenuComponent>>,
) {
    let menu_entity = menu_query.single().unwrap();
    commands.entity(menu_entity).remove::<SelectedEnt>();
}

// Systems
//
pub fn key_input(
    keys: Res<ButtonInput<KeyCode>>,
    selected_items: Query<(Entity, &SelectedEnt), With<MenuComponent>>,
    menu_query: Query<Entity, With<MenuComponent>>,
    container_query: Query<&Children, With<WidgetContainerComponent>>,
    items_query: Query<Entity, With<Selectables>>,
    mut commands: Commands,
) {
    let c = items_query.iter().next().unwrap();
    if selected_items.is_empty() {
        if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowDown) {
            commands
                .entity(menu_query.single().unwrap())
                .insert(SelectedEnt(c));
        }
        return;
    }

    let (entity, SelectedEnt(currently_selected)) = &selected_items.single().unwrap();
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

#[allow(dead_code)]
pub fn teardown_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuComponent>>) {
    for entity in &menu_query {
        commands.entity(entity).despawn();
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
