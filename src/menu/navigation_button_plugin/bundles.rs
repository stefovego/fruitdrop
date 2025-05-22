use crate::menu::components::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(
    Name::new("Navigation Node"),
    Node {
        width: Val::Percent(60.0),
        justify_content: JustifyContent::Center,
        margin: UiRect { bottom: Val::Px(10.0),
            ..default()
        },
        ..default()
    },
    Selectables,
    SelectedColor(Color::NONE),
    UnselectedColor(Color::NONE)
)]
pub struct NavigationButtonComponent;

#[derive(Component)]
pub struct SelectedColor(pub Color);

#[derive(Component)]
pub struct UnselectedColor(pub Color);
