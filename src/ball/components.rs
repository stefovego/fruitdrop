use std::usize;

use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Seed;

#[derive(Component)]
pub struct FreshBall;

#[derive(Component, Reflect)]
pub struct DropTimer {
    pub timer: Timer,
}

#[derive(Component, Reflect)]
pub struct BallLevel(pub usize);
