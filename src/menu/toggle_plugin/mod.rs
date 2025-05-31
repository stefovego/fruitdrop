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
        )
        .add_observer(component_added);
    }
}

fn component_added(
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
                        ToggleButtonComponent,
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
                        ToggleButtonComponent,
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
