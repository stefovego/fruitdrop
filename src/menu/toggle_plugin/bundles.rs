use crate::menu::components::*;
use crate::my_colors;
use bevy::prelude::*;

#[derive(Component)]
pub struct SelectedColor(pub Color);

#[derive(Component)]
pub struct UnselectedColor(pub Color);

#[derive(Component)]
pub struct OnButtonEntity(pub Entity);

#[derive(Component)]
pub struct OffButtonEntity(pub Entity);

#[derive(Component)]
pub struct OnButtonComponent;

#[derive(Component)]
pub struct OffButtonComponent;

#[derive(Component)]
pub struct ToggleWidgetComponent {
    pub current_value: bool,
}

#[derive(Bundle)]
pub struct ToggleButtonBundle {
    pub button: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}
impl Default for ToggleButtonBundle {
    fn default() -> Self {
        Self {
            button: Node {
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
            background_color: BackgroundColor(my_colors::PURPLE),
            border_color: Color::NONE.into(),
        }
    }
}

#[derive(Bundle)]
pub struct OffToggleBundle {
    pub toggle_button: ToggleButtonBundle,
    pub off_component: OffButtonComponent,
    pub name: Name,
}

impl Default for OffToggleBundle {
    fn default() -> Self {
        Self {
            toggle_button: ToggleButtonBundle::default(),
            off_component: OffButtonComponent,
            name: Name::new("Off Toggle Button"),
        }
    }
}

#[derive(Bundle)]
pub struct OnToggleBundle {
    pub toggle_button: ToggleButtonBundle,
    pub on_component: OnButtonComponent,
    pub name: Name,
}

impl Default for OnToggleBundle {
    fn default() -> Self {
        Self {
            toggle_button: ToggleButtonBundle::default(),
            on_component: OnButtonComponent,
            name: Name::new("On Toggle Button"),
        }
    }
}

#[derive(Bundle)]
pub struct ToggleWidgetBundle {
    pub node: Node,
    pub selectables: Selectables,
    pub selected_color: SelectedColor,
    pub unselected_color: UnselectedColor,
    //pub background_color: BackgroundColor,
}

impl Default for ToggleWidgetBundle {
    fn default() -> Self {
        Self {
            selectables: Selectables,
            node: Node {
                height: Val::Percent(30.0),
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

#[derive(Bundle)]
pub struct ToggleContainerBundle {
    node: Node,
}

impl Default for ToggleContainerBundle {
    fn default() -> Self {
        Self {
            node: Node {
                position_type: PositionType::Relative,
                align_self: AlignSelf::Center,     // vertical
                justify_self: JustifySelf::Center, //horizontal
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                display: Display::Flex,
                ..default()
            },
        }
    }
}
