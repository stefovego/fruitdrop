use bevy::prelude::*;

#[derive(Component)]
pub struct MenuComponent;

#[derive(Component)]
pub struct WidgetContainerComponent;

#[derive(Component)]
pub struct SelectedComponent;

#[derive(Component, Default)]
pub struct Selectables;

#[derive(Component, Reflect)]
pub struct SelectedEnt(pub Entity);
