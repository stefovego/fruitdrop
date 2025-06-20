use std::marker::PhantomData;

// use crate::menu::components::*;
use bevy::prelude::*;

use bevy::state::state::FreelyMutableState;

use crate::menu::{MenuComponent, Selectables, SelectedEnt};
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
pub fn button_clicked_observer<T: States + FreelyMutableState>(
    trigger: Trigger<Pointer<Click>>,
    mut next_states: ResMut<NextState<T>>,
    selector_widget_query: Query<&NavigationButtonWidgetComponent<T>>,
) {
    let entity = trigger.target();
    let naviagation_button_widget_component = selector_widget_query.get(entity).unwrap();
    next_states.set(naviagation_button_widget_component.next_state.clone());
}

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
    commands
        .entity(entity)
        .observe(button_pushed_observer::<T>)
        .observe(button_clicked_observer::<T>);
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

pub fn keyboard_select(
    keys: Res<ButtonInput<KeyCode>>,
    selected_widget: Single<&SelectedEnt, With<MenuComponent>>,
    mut commands: Commands,
) {
    let SelectedEnt(currently_selected) = selected_widget.into_inner();

    if keys.just_pressed(KeyCode::Enter) {
        commands.trigger_targets(ButtonPushed, *currently_selected);
    }
}
