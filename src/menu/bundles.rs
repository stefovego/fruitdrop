use crate::my_colors;
use bevy::prelude::*;

use super::components::WidgetContainerComponent;

#[derive(Bundle)]
pub struct ScreenParentBundle {
    node: Node,
    background_color: BackgroundColor,
}

impl Default for ScreenParentBundle {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                position_type: PositionType::Relative,
                width: Val::Percent(90.0),
                height: Val::Percent(90.0),
                bottom: Val::Percent(5.0),
                top: Val::Percent(5.0),
                left: Val::Percent(5.0),
                right: Val::Percent(5.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(my_colors::GRAY),
        }
    }
}

#[derive(Bundle)]
pub struct WidgetContainerBundle {
    name: Name,
    node: Node,
    container: WidgetContainerComponent,
}

impl Default for WidgetContainerBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Widget Container"),
            node: Node {
                position_type: PositionType::Relative,
                display: Display::Flex,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            container: WidgetContainerComponent,
        }
    }
}
