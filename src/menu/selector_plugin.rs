use bevy::{prelude::*, ui::FocusPolicy};

use crate::menu::{MenuComponent, Selectables, SelectedEnt};

// Plugin
//
pub struct SelectorPlugin;
impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            show_selection.run_if(any_with_component::<SelectorWidgetComponent>),
        )
        .add_systems(
            Update,
            selected_background.run_if(any_with_component::<SelectorWidgetComponent>),
        )
        .add_systems(
            Update,
            previous_button_clicked.run_if(any_with_component::<SelectorWidgetComponent>),
        )
        .add_systems(
            Update,
            next_button_clicked.run_if(any_with_component::<SelectorWidgetComponent>),
        )
        .add_systems(
            Update,
            key_toggle.run_if(any_with_component::<SelectorWidgetComponent>),
        )
        .add_observer(add_selector_widget_observer);
    }
}

// Components
//
#[derive(Component)]
pub struct SelectedColor(pub Color);

#[derive(Component)]
pub struct UnselectedColor(pub Color);

#[allow(dead_code)]
#[derive(Component)]
pub struct NextButtonEntity(pub Entity);

#[allow(dead_code)]
#[derive(Component)]
pub struct PreviousButtonEntity(pub Entity);

#[derive(Component, Default)]
#[require(
    Name::new("Change Button"), 
    FocusPolicy::Pass,
    Button,
    Node {
        border: UiRect {
            left: Val::Px(2.0),
            right: Val::Px(2.0),
            top: Val::Px(2.0),
            bottom: Val::Px(2.0),
        },
        width: Val::Px(90.0),
        height: Val::Px(90.0),
        align_self: AlignSelf::Center,
        justify_content: JustifyContent::Center,
        ..default()
    },
)]
pub struct ChangeButtonComponent;

#[derive(Component)]
pub struct NextComponent;

#[derive(Component)]
#[require(Name::new("Previous Button"), ChangeButtonComponent)]
pub struct PreviousButtonComponent;

#[derive(Component)]
#[require(Name::new("Next Button"), ChangeButtonComponent)]
pub struct NextButtonComponent;

#[derive(Component)]
#[require(
    Name::new("Selector Widget"),
    Button,
    Selectables,
    Node {
        height: Val::Auto,
        width: Val::Percent(100.0),
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        margin: UiRect {
            bottom: Val::Px(10.0),
            ..default()
        },
        display: Display::Flex,
        ..default()
    },
    SelectedColor(Color::NONE),
    UnselectedColor(Color::NONE),
)]
pub struct SelectorWidgetComponent {
    pub current_index: u32,
    pub selections: Vec<String>,
    pub label: String,
    pub selected_color: Color,
    pub unselected_color: Color,
}

#[derive(Component)]
#[require(
    Name::new("Current Selection"),
    Node {
        position_type: PositionType::Relative,
        align_self: AlignSelf::Center,           // vertical
        justify_self: JustifySelf::Center,       //horizontal
        justify_content: JustifyContent::Center, //horizontal
        height: Val::Percent(100.0),
        width: Val::Percent(100.0),
        display: Display::Flex,
        ..default()
    },
)]
pub struct CurrentSelectionComponent;

// Observers
//
fn add_selector_widget_observer(
    trigger: Trigger<OnAdd, SelectorWidgetComponent>,
    mut commands: Commands,
    selector_widget_query: Query<&SelectorWidgetComponent>,
) {
    let entity = trigger.target();
    let selector_widget_component = selector_widget_query.get(entity).unwrap();
    commands.entity(entity).insert((
        SelectedColor(selector_widget_component.selected_color),
        UnselectedColor(selector_widget_component.unselected_color),
        BackgroundColor(selector_widget_component.unselected_color),
        children![
            (
                Name::new("Label Container"),
                Node {
                    width: Val::Percent(35.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(
                    Name::new("Label"),
                    Text::new(&selector_widget_component.label),
                    TextColor(Color::BLACK),
                    TextFont {
                        font_size: 50.0,
                        ..Default::default()
                    },
                )],
            ),
            (
                Name::new("Middle Spacer"),
                Node {
                    width: Val::Percent(20.0),
                    ..default()
                }
            ),
            (
                Name::new("Selection Container"),
                Node {
                    width: Val::Percent(100.0),
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                children![
                    (
                        PreviousButtonComponent,
                        children![(
                            Name::new("Previous Label"),
                            Text::new("<"),
                            TextColor(Color::BLACK),
                            TextFont {
                                font_size: 50.0,
                                ..Default::default()
                            },
                        )]
                    ),
                    (
                        Name::new("Current Selection Label"),
                        CurrentSelectionComponent,
                        Text::new(selector_widget_component.selections.first().unwrap()),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: 50.0,
                            ..Default::default()
                        },
                    ),
                    (
                        NextButtonComponent,
                        children![(
                            Name::new("Next Label"),
                            Text::new(">"),
                            TextColor(Color::BLACK),
                            TextFont {
                                font_size: 50.0,
                                ..Default::default()
                            },
                        )]
                    ),
                ],
            ),
        ],
    ));
}

// Systems
//
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
