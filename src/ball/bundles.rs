use avian2d::prelude::*;
use bevy::prelude::*;

use crate::ball::components::*;
use crate::ball::resources::BallScaler;

use crate::physics::Layer;
const BALL_DENSITY: f32 = 30.;

#[derive(Bundle)]
pub struct BallBundle {
    rigid_body: RigidBody,
    gravity_scale: GravityScale,
    collider: Collider,
    collision_layer: CollisionLayers,
    collider_density: ColliderDensity,
    level: BallLevel,
    ball: Ball,
    name: Name,
    collision_event_enabled: CollisionEventsEnabled,
}

pub fn new(level: usize, ball_scaler: BallScaler) -> BallBundle {
    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(level as f32);
    BallBundle {
        rigid_body: RigidBody::Dynamic,
        collider: Collider::circle(ball_size),
        collision_layer: CollisionLayers::new([Layer::Ball], [Layer::Ball, Layer::Wall]),
        gravity_scale: GravityScale(20.0),
        collider_density: ColliderDensity(BALL_DENSITY),
        level: BallLevel(level),
        ball: Ball {},
        name: Name::new("Ball"),
        collision_event_enabled: CollisionEventsEnabled,
    }
}

pub fn new_seed(level: usize, ball_scaler: BallScaler) -> BallBundle {
    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(level as f32);
    BallBundle {
        rigid_body: RigidBody::Dynamic,
        collider: Collider::circle(ball_size),
        collision_layer: CollisionLayers::new([Layer::Ball], [Layer::Ball, Layer::Wall]),
        gravity_scale: GravityScale(20.0),
        collider_density: ColliderDensity(BALL_DENSITY),
        level: BallLevel(level),
        ball: Ball {},
        name: Name::new("Ball"),
        collision_event_enabled: CollisionEventsEnabled,
    }
}
