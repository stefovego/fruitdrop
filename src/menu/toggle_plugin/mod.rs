use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

pub mod bundles;
pub mod systems;

use bundles::*;
use systems::*;

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
            off_click_toggle.run_if(any_with_component::<ToggleWidgetComponent>),
        )
        .add_systems(
            Update,
            on_click_toggle.run_if(any_with_component::<ToggleWidgetComponent>),
        )
        .add_systems(
            Update,
            key_toggle.run_if(any_with_component::<ToggleWidgetComponent>),
        );
    }
}

pub struct Toggle {
    pub initial_value: bool,
    pub label: String,
    pub selected_color: Color,
    pub unselected_color: Color,
}

pub struct SpawnToggle {
    pub toggle: Toggle,
}

impl SpawnToggle {
    pub fn spawn(toggle: Toggle) -> Self {
        Self { toggle }
    }
}

impl EntityCommand for SpawnToggle {
    fn apply(self, parent_id: Entity, world: &mut World) {
        let toggle_widget = world
            .spawn(ToggleWidgetBundle {
                selected_color: SelectedColor(self.toggle.selected_color),
                unselected_color: UnselectedColor(self.toggle.unselected_color),
                ..default()
            })
            .insert(BackgroundColor(self.toggle.unselected_color))
            .insert(Name::new("Toggle Widget"))
            .id();

        let middle_spacer = world
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(40.0),
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("Middle Spacer"))
            .id();

        let buttons_container = world
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    column_gap: Val::Px(10.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("Buttons Container"))
            .id();

        let off_button = world
            .spawn(OffToggleBundle::default())
            .insert(Name::new("Off Button"))
            .id();

        let off_label = world
            .spawn(TextBundle::from_section(
                "Off",
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
            .insert(Name::new("Off Label"))
            .id();

        let on_button = world
            .spawn(OnToggleBundle::default())
            .insert(Name::new("On Button"))
            .id();

        let on_label = world
            .spawn(TextBundle::from_section(
                "On",
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
            .insert(Name::new("On Label"))
            .id();

        let toggle_label = world
            .spawn(TextBundle::from_section(
                self.toggle.label,
                TextStyle {
                    color: Color::BLACK,
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();

        let toggle_label_container = world
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

        world.entity_mut(off_button).add_child(off_label);

        world.entity_mut(on_button).add_child(on_label);

        world
            .entity_mut(toggle_label_container)
            .add_child(toggle_label);

        world
            .entity_mut(buttons_container)
            .add_child(off_button)
            .add_child(on_button);

        world
            .entity_mut(toggle_widget)
            .add_child(toggle_label_container)
            .add_child(middle_spacer)
            .add_child(buttons_container);

        world
            .entity_mut(toggle_widget)
            .insert(ToggleWidgetComponent {
                current_value: self.toggle.initial_value,
            })
            .insert(OnButtonEntity(on_button))
            .insert(OffButtonEntity(off_button));

        world.entity_mut(parent_id).push_children(&[toggle_widget]);
    }
}
