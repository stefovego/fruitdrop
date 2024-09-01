use bevy::prelude::*;

#[derive(Component)]
pub struct MenuComponent;

#[derive(Component)]
pub struct ContainerComponent;

#[derive(Component)]
pub struct SelectedComponent;

#[derive(Component)]
pub struct Selectables;

#[derive(Component)]
pub struct SelectedEnt(pub Entity);
