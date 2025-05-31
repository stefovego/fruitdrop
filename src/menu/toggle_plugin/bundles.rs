use crate::menu::components::*;
use crate::my_colors;
use bevy::prelude::*;

#[derive(Component)]
pub struct SelectedColor(pub Color);

#[derive(Component)]
pub struct UnselectedColor(pub Color);

#[derive(Component)]
pub struct OnButtonComponent;

#[derive(Component)]
pub struct OffButtonComponent;

#[derive(Component)]
#[require(
    Name::new("Toggle Widget"), 
    Selectables,
    Node {
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
)]
pub struct ToggleWidgetComponent {
    pub label: String,
    pub current_value: bool,
    pub selected_color: Color,
    pub unselected_color: Color,
}
#[derive(Component)]
#[require(

    Node {
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
    BackgroundColor(my_colors::PURPLE),
    BorderColor(Color::NONE.into()),
    Button,
)]

pub struct ToggleButtonComponent;
