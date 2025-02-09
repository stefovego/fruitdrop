use crate::menu::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct NavigationButtonComponent;

#[derive(Component)]
pub struct SelectedColor(pub Color);

#[derive(Component)]
pub struct UnselectedColor(pub Color);

#[derive(Bundle)]
pub struct NavigationButtonBundle {
    pub node: Node,
    pub selectables: Selectables,
    pub navigation_button: NavigationButtonComponent,
    pub selected_color: SelectedColor,
    pub unselected_color: UnselectedColor,
}

impl Default for NavigationButtonBundle {
    fn default() -> Self {
        Self {
            node: Node {
                width: Val::Percent(60.0),
                justify_content: JustifyContent::Center,
                margin: UiRect {
                    bottom: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            selectables: Selectables,
            navigation_button: NavigationButtonComponent,
            selected_color: SelectedColor(Color::NONE),
            unselected_color: UnselectedColor(Color::NONE),
        }
    }
}
