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

pub struct InsertWidgetCommand(Entity);

impl InsertWidgetCommand {
    pub fn spawn(wiget_entity: Entity) -> Self
where {
        Self(wiget_entity)
    }
}

impl EntityCommand for InsertWidgetCommand {
    fn apply(self, parent_id: Entity, world: &mut World) {
        world.entity_mut(parent_id).add_children(&[self.0]);
    }
}

//fn init_menu_entity<T>(mut commands: Commands) {
//    let parent_node = ScreenParentBundle::default();
//
//    let button_container_node = ContainerBundle::default();
//
//    let parent = commands
//        .spawn((StateScoped(MainMenuState::InitialMenu), parent_node))
//        .insert(MenuComponent)
//        .id();
//
//    let button_container = commands.spawn(button_container_node).id();
//
//    commands.entity(parent).add_children(&[button_container]);
//}
