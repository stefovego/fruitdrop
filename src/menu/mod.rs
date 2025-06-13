use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

pub mod bundles;
pub mod components;
pub mod my_colors;
pub mod navigation_button_plugin;
pub mod selector_plugin;
pub mod slider_plugin;
pub mod toggle_plugin;

mod systems;
use self::components::MenuComponent;
use self::components::SelectedEnt;
use self::systems::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SelectedEnt>()
            //.add_plugins(navigation_button_plugin::NavigationButtonPlugin<T>)
            .add_plugins(selector_plugin::SelectorPlugin)
            .add_plugins(slider_plugin::SliderPlugin)
            .add_plugins(toggle_plugin::TogglePlugin)
            .add_systems(
                Update,
                key_input.run_if(any_with_component::<MenuComponent>),
            );
    }
}

pub struct InsertWidgetCommand(Entity);

impl EntityCommand for InsertWidgetCommand {
    fn apply(self, mut entity_world: EntityWorldMut) {
        let entity = entity_world.id();
        entity_world.world_scope(move |world: &mut World| {
            world.entity_mut(entity).add_children(&[self.0]);
        });
    }
}
