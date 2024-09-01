use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

pub mod bundles;
pub mod systems;

use bundles::*;
use systems::*;

use crate::my_colors;

pub struct NavigationButtonPlugin;

impl Plugin for NavigationButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            mouse_system.run_if(any_with_component::<NavigationButtonComponent>),
        )
        .add_systems(
            Update,
            selected_background.run_if(any_with_component::<NavigationButtonComponent>),
        );
    }
}
pub struct NavigationButton<T> {
    pub text: String,
    pub marker_component: T,
    pub selected_color: Color,
    pub unselected_color: Color,
}
pub struct SpawnNavigationButton<T> {
    pub navigation_button: NavigationButton<T>,
}

impl<T> SpawnNavigationButton<T> {
    pub fn spawn(navigation_button: NavigationButton<T>) -> Self
    where
        T: Component,
    {
        Self { navigation_button }
    }
}

impl<T> EntityCommand for SpawnNavigationButton<T>
where
    T: Component,
{
    fn apply(self, parent_id: Entity, world: &mut World) {
        let text_button_widget = world
            .spawn(NavigationButtonBundle {
                selected_color: SelectedColor(self.navigation_button.selected_color),
                unselected_color: UnselectedColor(self.navigation_button.unselected_color),
                ..default()
            })
            .insert(BackgroundColor(self.navigation_button.unselected_color))
            .insert(self.navigation_button.marker_component)
            .id();
        let text_button_label_widget = world
            .spawn(TextBundle::from_section(
                self.navigation_button.text,
                TextStyle {
                    color: my_colors::PINK,
                    font_size: 100.0,
                    ..default()
                },
            ))
            .id();
        world
            .entity_mut(text_button_widget)
            .push_children(&[text_button_label_widget]);
        world
            .entity_mut(parent_id)
            .push_children(&[text_button_widget]);
    }
}
