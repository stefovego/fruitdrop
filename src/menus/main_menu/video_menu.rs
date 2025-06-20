use bevy::prelude::*;

use crate::main_menu::MainMenuState;
use crate::menu::navigation_button_plugin::NavigationButtonWidgetComponent;
use crate::menu::{MenuComponent, WidgetContainerComponent};
use crate::my_colors;

pub struct VideoMenuPlugin;

impl Plugin for VideoMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::VideoMenu), setup_menu);
    }
}

pub fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Video Menu"),
        MenuComponent,
        StateScoped(MainMenuState::VideoMenu),
        children![(
            WidgetContainerComponent,
            children![
                (
                    Text::new("Under Construction"),
                    TextFont {
                        font_size: 50.0,
                        ..Default::default()
                    },
                    TextColor(my_colors::PINK),
                ),
                NavigationButtonWidgetComponent {
                    text: String::from("Back"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::OptionsMenu,
                },
            ]
        )],
    ));
}
