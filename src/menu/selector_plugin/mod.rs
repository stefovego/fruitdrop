use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

pub mod bundles;
pub mod systems;

use bundles::*;
use systems::*;

pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            selected_background.run_if(any_with_component::<SelectorWidgetComponent>),
        )
        .add_systems(
            Update,
            mouse_system.run_if(any_with_component::<SelectorWidgetComponent>),
        )
        .add_systems(
            Update,
            show_selection.run_if(any_with_component::<SelectorWidgetComponent>),
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
        );
    }
}

pub struct Selector {
    pub label: String,
    pub selections: Vec<String>,
    pub selected_color: Color,
    pub unselected_color: Color,
}

pub struct SpawnSelector {
    pub selector: Selector,
}

impl SpawnSelector {
    pub fn spawn(selector: Selector) -> Self {
        Self { selector }
    }
}

impl EntityCommand for SpawnSelector {
    fn apply(self, parent_id: Entity, world: &mut World) {
        let selection_widget = world
            .entity_mut(parent_id)
            .insert(SelectionWidgetBundle {
                selected_color: SelectedColor(self.selector.selected_color),
                unselected_color: UnselectedColor(self.selector.unselected_color),
                ..default()
            })
            .insert(BackgroundColor(self.selector.unselected_color))
            .insert(Name::new("Selector Widget"))
            .id();

        let middle_spacer = world
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(20.0),
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("Middle Spacer"))
            .id();

        let selection_container = world
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("Selection Container"))
            .id();

        let previous_button = world
            .spawn(PreviousButtonBundle::default())
            .insert(Name::new("Previous Button"))
            .id();

        let previous_label = world
            .spawn(TextBundle::from_section(
                "<",
                TextStyle {
                    color: Color::BLACK,
                    font_size: 50.0,
                    ..default()
                },
            ))
            .insert(Style {
                align_self: AlignSelf::Center,
                ..default()
            })
            .insert(Name::new("Previous Label"))
            .id();

        let next_button = world
            .spawn(NextButtonBundle::default())
            .insert(Name::new("Next Button"))
            .id();

        let next_label = world
            .spawn(TextBundle::from_section(
                ">",
                TextStyle {
                    color: Color::BLACK,
                    font_size: 50.0,
                    ..default()
                },
            ))
            .insert(Style {
                align_self: AlignSelf::Center,
                ..default()
            })
            .insert(Name::new("Next Button Label"))
            .id();

        let current_selection = world.spawn(CurrentSelectionBundle { ..default() }).id();

        let current_selection_label = world
            .spawn(TextBundle::from_section(
                self.selector.selections.first().unwrap(),
                TextStyle {
                    color: Color::BLACK,
                    font_size: 50.0,
                    ..default()
                },
            ))
            .insert(Style {
                align_self: AlignSelf::Center,
                ..default()
            })
            .insert(Name::new("Current Selection Label"))
            .id();

        let label = world
            .spawn(TextBundle::from_section(
                self.selector.label,
                TextStyle {
                    color: Color::BLACK,
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();

        let label_container = world
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(35.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("Label Container"))
            .id();

        world.entity_mut(previous_button).add_child(previous_label);

        world.entity_mut(next_button).add_child(next_label);

        world.entity_mut(label_container).add_child(label);

        world
            .entity_mut(current_selection)
            .add_child(current_selection_label);

        world
            .entity_mut(selection_container)
            .add_child(previous_button)
            .add_child(current_selection)
            .add_child(next_button);

        world
            .entity_mut(selection_widget)
            .add_child(label_container)
            .add_child(middle_spacer)
            .add_child(selection_container);

        world
            .entity_mut(selection_widget)
            .insert(SelectorWidgetComponent {
                current_index: 0,
                selections: self.selector.selections,
            })
            .insert(PreviousButtonEntity(previous_button))
            .insert(NextButtonEntity(next_button))
            .insert(CurrentSelectionEntity(current_selection_label));
    }
}
