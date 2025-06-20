use crate::main_menu::menu_state::MainMenuState;
use crate::menu::MenuComponent;
use crate::menu::WidgetContainerComponent;
use crate::my_colors;
use bevy::prelude::*;

use crate::navigation_button_plugin::NavigationButtonWidgetComponent;
use crate::selector_plugin::*;
use crate::toggle_plugin::*;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::GameMenu), setup_menu);
    }
}

pub fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Game Menu"),
        MenuComponent,
        StateScoped(MainMenuState::GameMenu),
        children![(
            WidgetContainerComponent,
            children![
                ToggleWidgetComponent {
                    label: "Cool?".into(),
                    current_value: true,
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                },
                SelectorWidgetComponent {
                    current_index: 0,
                    label: "Stuff".into(),
                    selections: ["Stuff ONe".into(), "stuff 2".into(), "stuff3".into()].into(),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                },
                NavigationButtonWidgetComponent {
                    text: String::from("Back"),
                    selected_color: my_colors::BLUE,
                    unselected_color: my_colors::PURPLE,
                    next_state: MainMenuState::OptionsMenu,
                }
            ]
        )],
    ));
}
