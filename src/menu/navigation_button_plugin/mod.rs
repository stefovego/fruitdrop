use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

pub mod bundles;
pub mod systems;

use bevy::state::state::FreelyMutableState;
use bundles::*;
use systems::*;

use crate::my_colors;

pub struct NavigationButtonPlugin;

#[derive(Event)]
pub struct ButtonPushed;

impl Plugin for NavigationButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            mouse_system.run_if(any_with_component::<NavigationButtonComponent>),
        )
        .add_systems(
            Update,
            selected_background.run_if(any_with_component::<NavigationButtonComponent>),
        )
        .add_systems(
            Update,
            keyboard_select.run_if(any_with_component::<NavigationButtonComponent>),
        );
    }
}
pub struct NavigationButton<T: States> {
    pub text: String,
    pub selected_color: Color,
    pub unselected_color: Color,
    pub next_state: T,
}

pub struct SpawnNavigationButton<T: States> {
    pub navigation_button: NavigationButton<T>,
}

impl<T: States> SpawnNavigationButton<T> {
    pub fn spawn(navigation_button: NavigationButton<T>) -> Self
where {
        Self { navigation_button }
    }
}

impl<T: FreelyMutableState> EntityCommand for SpawnNavigationButton<T> {
    fn apply(self, parent_id: Entity, world: &mut World) {
        let text_button_widget = world
            .entity_mut(parent_id)
            .insert(NavigationButtonBundle {
                selected_color: SelectedColor(self.navigation_button.selected_color),
                unselected_color: UnselectedColor(self.navigation_button.unselected_color),
                ..default()
            })
            .insert(BackgroundColor(self.navigation_button.unselected_color))
            .id();

        let text_button_label_widget = world
            .spawn((
                Text::new(self.navigation_button.text),
                TextColor(my_colors::PINK),
                TextFont {
                    font_size: 100.0,
                    ..Default::default()
                },
            ))
            .id();
        world.entity_mut(text_button_widget).observe(
            move |_tigger: Trigger<ButtonPushed>, mut next_states: ResMut<NextState<T>>| {
                next_states.set(self.navigation_button.next_state.clone());
            },
        );
        world
            .entity_mut(text_button_widget)
            .add_children(&[text_button_label_widget]);
    }
}
