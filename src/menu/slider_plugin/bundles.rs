use crate::{menu::components::*, my_colors};
use bevy::{math::u32, prelude::*};

#[derive(Component)]
pub struct SliderKnob;

#[derive(Component)]
pub struct SelectedColor(pub Color);

#[derive(Component)]
pub struct UnselectedColor(pub Color);

#[derive(Bundle)]
pub struct SliderKnobBundle {
    node: NodeBundle,
    slider_knob: SliderKnob,
    name: Name,
}

impl Default for SliderKnobBundle {
    fn default() -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    border: UiRect {
                        left: Val::Px(1.0),
                        right: Val::Px(1.0),
                        top: Val::Px(1.0),
                        bottom: Val::Px(1.0),
                    },
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    height: Val::Percent(40.0),
                    width: Val::Percent(5.0),
                    left: Val::Percent(00.0),
                    display: Display::Flex,
                    ..default()
                },
                background_color: BackgroundColor(my_colors::BLUE),
                border_color: BorderColor(my_colors::BLACK),
                ..default()
            },
            slider_knob: SliderKnob,
            name: Name::new("Slider Knob"),
        }
    }
}

#[derive(Component)]
pub struct SliderWidgetComponent {
    pub step_size: u32,
    pub current_value: u32,
    pub min: u32,
    pub max: u32,
}

#[derive(Bundle)]
pub struct SliderWidgetBundle {
    pub node: ButtonBundle,
    pub selectables: Selectables,
    pub selected_color: SelectedColor,
    pub unselected_color: UnselectedColor,
}

impl Default for SliderWidgetBundle {
    fn default() -> Self {
        Self {
            selectables: Selectables,
            node: ButtonBundle {
                style: Style {
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
                border_color: Color::BLACK.into(),
                ..default()
            },
            selected_color: SelectedColor(Color::NONE),
            unselected_color: UnselectedColor(Color::NONE),
        }
    }
}

#[derive(Bundle)]
pub struct SliderTrackBundle {
    node: NodeBundle,
    name: Name,
}

impl Default for SliderTrackBundle {
    fn default() -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,     // vertical
                    justify_self: JustifySelf::Center, //horizontal
                    height: Val::Percent(5.0),
                    width: Val::Percent(100.0),
                    display: Display::Flex,
                    ..default()
                },
                border_color: Color::BLACK.into(),
                background_color: Color::NONE.into(),
                ..default()
            },
            name: Name::new("Slider Track"),
        }
    }
}

#[derive(Bundle)]
pub struct SliderContainerBundle {
    node: NodeBundle,
    name: Name,
}

impl Default for SliderContainerBundle {
    fn default() -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    align_self: AlignSelf::Center,     // vertical
                    justify_self: JustifySelf::Center, //horizontal
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            },
            name: Name::new("Slider Container"),
        }
    }
}

#[derive(Component)]
pub struct SliderEntity(pub Entity);

#[derive(Component)]
pub struct SliderTrackEntity();

#[derive(Component)]
pub struct SliderReadOutEntity(pub Entity);

#[derive(Component)]
pub struct ReadOut;
