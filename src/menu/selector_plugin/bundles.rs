use crate::menu::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ChangeButtonBundle {
    pub node: Node,
    //    pub background_color: BackgroundColor,
}
impl Default for ChangeButtonBundle {
    fn default() -> Self {
        Self {
            node: Node {
                border: UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(2.0),
                    top: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                },
                width: Val::Px(90.0),
                height: Val::Px(90.0),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct NextButtonBundle {
    pub change_button: ChangeButtonBundle,
    pub next_component: NextComponent,
    pub name: Name,
}

impl Default for NextButtonBundle {
    fn default() -> Self {
        Self {
            change_button: ChangeButtonBundle::default(),
            next_component: NextComponent,
            name: Name::new("Next Button"),
        }
    }
}

#[derive(Bundle)]
pub struct SelectionWidgetBundle {
    pub node: Node,
    pub selectables: Selectables,
    pub selected_color: SelectedColor,
    pub unselected_color: UnselectedColor,
    //pub background_color: BackgroundColor,
}

impl Default for SelectionWidgetBundle {
    fn default() -> Self {
        Self {
            selectables: Selectables,
            node: Node {
                height: Val::Auto,
                width: Val::Percent(100.0),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    bottom: Val::Px(10.0),
                    ..default()
                },
                display: Display::Flex,
                ..default()
            },
            selected_color: SelectedColor(Color::NONE),
            unselected_color: UnselectedColor(Color::NONE),
            //background_color: BackgroundColor(my_colors::YELLOW),
        }
    }
}
