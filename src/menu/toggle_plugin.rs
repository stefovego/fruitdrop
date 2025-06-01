use crate::menu::components::*;
use crate::my_colors;
use bevy::prelude::*;

// Plugin
//
pub struct TogglePlugin;
impl Plugin for TogglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            mouse_system.run_if(any_with_component::<ToggleWidgetComponent>),
        )
        .add_systems(
            Update,
            show_selection.run_if(any_with_component::<ToggleWidgetComponent>),
        )
        .add_systems(
            Update,
            selected_background.run_if(any_with_component::<ToggleWidgetComponent>),
        )
        .add_systems(
            Update,
            handle_button_click.run_if(any_with_component::<ToggleWidgetComponent>),
        )
        .add_systems(
            Update,
            key_toggle.run_if(any_with_component::<ToggleWidgetComponent>),
        )
        .add_observer(toggle_widget_component_toggle);
    }
}

// Components
//
#[derive(Component)]
pub struct SelectedColor(pub Color);

#[derive(Component)]
pub struct UnselectedColor(pub Color);

#[derive(Component)]
pub struct OnButtonComponent;

#[derive(Component)]
pub struct OffButtonComponent;

#[derive(Component)]
#[require(
    Name::new("Toggle Widget"), 
    Selectables,
    Button,
    Node {
        height: Val::Percent(30.0),
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
)]
pub struct ToggleWidgetComponent {
    pub label: String,
    pub current_value: bool,
    pub selected_color: Color,
    pub unselected_color: Color,
}
#[derive(Component)]
#[require(

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
    BackgroundColor(my_colors::PURPLE),
    BorderColor(Color::NONE.into()),
    Button,
)]
pub struct ToggleButtonComponent(pub bool);

// Observers
//
fn toggle_widget_component_toggle(
    trigger: Trigger<OnAdd, ToggleWidgetComponent>,
    mut commands: Commands,
    toggle_widget_query: Query<&ToggleWidgetComponent>,
) {
    // Get the entity
    let entity = trigger.target();
    let toggle_widget_component = toggle_widget_query.get(entity).unwrap();

    commands.entity(entity).insert((
        SelectedColor(toggle_widget_component.selected_color),
        UnselectedColor(toggle_widget_component.unselected_color),
        BackgroundColor(toggle_widget_component.unselected_color),
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
                    Text::new(&toggle_widget_component.label),
                    TextColor(Color::BLACK),
                    TextFont {
                        font_size: 50.0,
                        ..Default::default()
                    },
                ),],
            ),
            (
                Name::new("Middle Spacer"),
                Node {
                    width: Val::Percent(40.0),
                    ..default()
                },
            ),
            (
                Name::new("Button Container"),
                Node {
                    width: Val::Percent(25.0),
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                children![
                    (
                        Name::new("Off Button"),
                        OffButtonComponent,
                        ToggleButtonComponent(false),
                        children![(
                            Name::new("Off Button Label"),
                            Text::new("Off"),
                            TextFont {
                                font_size: 50.0,
                                ..Default::default()
                            },
                            TextColor(Color::BLACK),
                        )]
                    ),
                    (
                        Name::new("On Button"),
                        OnButtonComponent,
                        ToggleButtonComponent(true),
                        children![(
                            Name::new("On Button Label"),
                            Text::new("On"),
                            TextFont {
                                font_size: 50.0,
                                ..Default::default()
                            },
                            TextColor(Color::BLACK),
                        )],
                    )
                ],
            )
        ],
    ));
}

// Systems
//
pub fn mouse_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<ToggleWidgetComponent>),
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
        With<ToggleWidgetComponent>,
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

    // Change the selected widgets background
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

pub fn handle_button_click(
    mut interaction_query: Query<
        (Entity, &ToggleButtonComponent, &Interaction),
        (Changed<Interaction>, With<ToggleButtonComponent>),
    >,
    mut toggle_button_query: Query<&mut ToggleWidgetComponent, With<ToggleWidgetComponent>>,
    parent_query: Query<&ChildOf>,
) {
    for (button_entity, &ToggleButtonComponent(button_value), interaction) in &mut interaction_query
    {
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
                toggle_component.current_value = button_value;
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
