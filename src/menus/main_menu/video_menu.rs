use bevy::prelude::*;

use crate::main_menu::MainMenuState;
use crate::menu::bundles::*;
use crate::menu::components::*;
use crate::menu::navigation_button_plugin::NavigationButtonWidgetComponent;
use crate::my_colors;

pub struct VideoMenuPlugin;

impl Plugin for VideoMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::VideoMenu), setup_menu);
    }
}

pub fn setup_menu(mut commands: Commands) {
    let parent_node = ScreenParentBundle::default();

    let button_container_node = WidgetContainerBundle::default();

    let parent = commands
        .spawn((StateScoped(MainMenuState::VideoMenu), parent_node))
        .insert(MenuComponent)
        .id();

    let button_container = commands.spawn(button_container_node).id();

    commands.entity(parent).add_children(&[button_container]);

    let under_construction_text = Text::new("Under Construction");
    let under_construction_text_font = TextFont {
        font_size: 50.0,
        ..Default::default()
    };
    let under_construction_text_color = TextColor(my_colors::PINK);

    let under_construction_label = commands
        .spawn((
            under_construction_text,
            under_construction_text_font,
            under_construction_text_color,
        ))
        .id();

    let back_button_entity = commands
        .spawn(NavigationButtonWidgetComponent {
            text: String::from("Back"),
            selected_color: my_colors::BLUE,
            unselected_color: my_colors::PURPLE,
            next_state: MainMenuState::OptionsMenu,
        })
        .id();

    commands
        .entity(button_container)
        .add_children(&[under_construction_label, back_button_entity]);
}
