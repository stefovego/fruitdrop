use crate::physics::Layer;
use avian2d::prelude::*;
use bevy::prelude::*;
use std::usize;

const BALL_DENSITY: f32 = 30.;

#[derive(Component)]
#[require(
    Name::new("Ball"),
    RigidBody::Dynamic, 
    ColliderDensity(BALL_DENSITY),
    CollisionEventsEnabled, 
    CollisionLayers::new([Layer::Ball], [Layer::Ball, Layer::Wall]), 
    GravityScale(20.0),
)]
pub struct Ball;

#[derive(Component)]
pub struct Seed;

#[derive(Component)]
pub struct FreshBall;

#[derive(Component, Reflect)]
pub struct DropTimer {
    pub timer: Timer,
}

#[derive(Component, Reflect, Debug)]
pub struct BallLevel(pub usize);
