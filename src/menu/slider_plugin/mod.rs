use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

pub mod bundles;
pub mod systems;

use bevy::ui::RelativeCursorPosition;
use bundles::*;
use systems::*;

pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_slider.run_if(any_with_component::<SliderWidgetComponent>),
        )
        .add_systems(
            Update,
            place_knob.run_if(any_with_component::<SliderWidgetComponent>),
        )
        .add_systems(
            Update,
            mouse_system.run_if(any_with_component::<SliderWidgetComponent>),
        )
        .add_systems(
            Update,
            mousy_stuffy.run_if(any_with_component::<SliderWidgetComponent>),
        )
        .add_systems(
            Update,
            selected_background.run_if(any_with_component::<SliderWidgetComponent>),
        );
    }
}

pub struct Slider<T> {
    pub initial_value: T,
    pub step_size: u32,
    pub min: u32,
    pub max: u32,
    pub label: String,
    pub selected_color: Color,
    pub unselected_color: Color,
    pub track_color: Color,
}

pub struct SpawnSlider<T, U> {
    pub slider: Slider<U>,
    pub marker: T,
}

impl<T, U> SpawnSlider<T, U> {
    pub fn spawn(slider: Slider<U>, marker: T) -> Self {
        Self { slider, marker }
    }
}

impl<T: Component, U: std::marker::Send + 'static> EntityCommand for SpawnSlider<T, U> {
    fn apply(self, parent_id: Entity, world: &mut World) {
        let slider_widget = world
            .entity_mut(parent_id)
            .insert(SliderWidgetBundle {
                selected_color: SelectedColor(self.slider.selected_color),
                unselected_color: UnselectedColor(self.slider.unselected_color),
                ..default()
            })
            .insert(BackgroundColor(self.slider.unselected_color))
            .id();
        let slider_container_widget = world.spawn(SliderContainerBundle::default()).id();
        let slider_knob_widget = world.spawn(SliderKnobBundle::default()).id();
        let slider_track_widget = world
            .spawn(SliderTrackBundle::default())
            .insert(BackgroundColor(self.slider.track_color))
            .id();

        let slider_track_container = world
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(40.0),
                    ..default()
                },
                ..default()
            })
            .id();

        let slider_readout_container = world
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .id();
        let slider_read_out_widget = world
            .spawn(TextBundle::from_section(
                "0",
                TextStyle {
                    color: Color::BLACK,
                    font_size: 150.0,
                    ..default()
                },
            ))
            .id();

        let slider_label = world
            .spawn(TextBundle::from_section(
                self.slider.label,
                TextStyle {
                    color: Color::BLACK,
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();

        let slider_label_container = world
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(35.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .id();

        world
            .entity_mut(slider_label_container)
            .add_child(slider_label);

        world.entity_mut(slider_read_out_widget).insert(ReadOut);
        world
            .entity_mut(slider_track_container)
            .push_children(&[slider_track_widget, slider_knob_widget])
            .insert(RelativeCursorPosition::default());

        world
            .entity_mut(slider_readout_container)
            .add_child(slider_read_out_widget);

        world.entity_mut(slider_container_widget).push_children(&[
            slider_label_container,
            slider_track_container,
            slider_readout_container,
        ]);
        world
            .entity_mut(slider_widget)
            .push_children(&[slider_container_widget]);
        world
            .entity_mut(slider_widget)
            .insert(SliderEntity(slider_knob_widget))
            .insert(SliderReadOutEntity(slider_read_out_widget))
            .insert(SliderTrackEntity())
            //.insert(SliderTrackEntity(slider_track_container))
            .insert(SliderWidgetComponent {
                step_size: self.slider.step_size,
                min: self.slider.min,
                max: self.slider.max,
                current_value: self.slider.initial_value,
            })
            .insert(self.marker);
    }

    fn with_entity(self, id: Entity) -> bevy::ecs::system::WithEntity<(), Self>
    where
        Self: Sized,
    {
        bevy::ecs::system::WithEntity {
            cmd: self,
            id,
            marker: std::marker::PhantomData,
        }
    }
}
