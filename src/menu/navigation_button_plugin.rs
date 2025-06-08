use std::marker::PhantomData;

use crate::menu::components::*;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use bevy::state::state::FreelyMutableState;

use crate::my_colors;

#[derive(Event)]
pub struct ButtonPushed;

pub struct NavigationButtonPlugin<T> {
    pub phantom_t: PhantomData<T>,
}

impl<T: States + FreelyMutableState> Plugin for NavigationButtonPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            mouse_system::<T>.run_if(any_with_component::<NavigationButtonWidgetComponent<T>>),
        )
        .add_systems(
            Update,
            selected_background::<T>
                .run_if(any_with_component::<NavigationButtonWidgetComponent<T>>),
        )
        .add_systems(
            Update,
            keyboard_select.run_if(any_with_component::<NavigationButtonWidgetComponent<T>>),
        )
        .add_observer(add_navigation_button_widget_observer::<T>);
    }
}

impl<T> Default for NavigationButtonPlugin<T> {
    fn default() -> Self {
        Self {
            phantom_t: PhantomData,
        }
    }
}

// Components
//
#[derive(Component)]
#[require(
    Name::new("Navigation Node"),
    Node {
        width: Val::Percent(60.0),
        justify_content: JustifyContent::Center,
        margin: UiRect { bottom: Val::Px(10.0),
            ..default()
        },
        ..default()
    },
    Selectables,
    SelectedColor(Color::NONE),
    UnselectedColor(Color::NONE)
)]
pub struct NavigationButtonWidgetComponent<T: States> {
    pub text: String,
    pub selected_color: Color,
    pub unselected_color: Color,
    pub next_state: T,
}

#[derive(Component)]
pub struct SelectedColor(pub Color);

#[derive(Component)]
pub struct UnselectedColor(pub Color);

// Observers
//
fn add_navigation_button_widget_observer<T: States + FreelyMutableState>(
    trigger: Trigger<OnAdd, NavigationButtonWidgetComponent<T>>,
    mut commands: Commands,
    selector_widget_query: Query<&NavigationButtonWidgetComponent<T>>,
) {
    let entity = trigger.target();
    let naviagation_button_widget_component = selector_widget_query.get(entity).unwrap();

    commands.entity(entity).insert((
        SelectedColor(naviagation_button_widget_component.selected_color),
        UnselectedColor(naviagation_button_widget_component.unselected_color),
        BackgroundColor(naviagation_button_widget_component.unselected_color),
        children![(
            Text::new(naviagation_button_widget_component.text.clone()),
            TextColor(my_colors::PINK),
            TextFont {
                font_size: 100.0,
                ..Default::default()
            },
        )],
    ));
    commands.entity(entity).observe(button_pushed_observer::<T>);
}

fn button_pushed_observer<T: States + FreelyMutableState>(
    trigger: Trigger<ButtonPushed>,
    mut next_states: ResMut<NextState<T>>,

    selector_widget_query: Query<&NavigationButtonWidgetComponent<T>>,
) {
    let entity = trigger.target();
    let naviagation_button_widget_component = selector_widget_query.get(entity).unwrap();
    next_states.set(naviagation_button_widget_component.next_state.clone());
}

// Systems
//
pub fn selected_background<T: States>(
    mut item_query: Query<
        (
            &mut BackgroundColor,
            Entity,
            &SelectedColor,
            &UnselectedColor,
        ),
        With<NavigationButtonWidgetComponent<T>>,
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

pub fn mouse_system<T: States>(
    mut interaction_query: Query<
        (Entity, &Interaction),
        (
            Changed<Interaction>,
            With<NavigationButtonWidgetComponent<T>>,
        ),
    >,
    parent_query: Query<Entity, With<MenuComponent>>,
    mut commands: Commands,
) {
    let parent_entity = parent_query.single().unwrap();
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

    let SelectedEnt(currently_selected) = parent_query.single().unwrap();

    if keys.just_pressed(KeyCode::Enter) {
        commands.trigger_targets(ButtonPushed, *currently_selected);
    }
}
