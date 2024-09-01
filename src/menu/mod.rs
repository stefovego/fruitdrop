use bevy::prelude::*;

pub mod components;
pub mod bundles;
pub mod my_colors;
pub mod navigation_button_plugin;
pub mod selector_plugin;
pub mod slider_plugin;
pub mod toggle_plugin;

mod systems;
use self::components::MenuComponent;
use self::systems::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(navigation_button_plugin::NavigationButtonPlugin)
            .add_plugins(selector_plugin::SelectorPlugin)
            .add_plugins(slider_plugin::SliderPlugin)
            .add_plugins(toggle_plugin::TogglePlugin)
            .add_systems(
                Update,
                key_input.run_if(any_with_component::<MenuComponent>),
            );
    }
}
