use bevy::sprite::Material2d;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::prelude::*;

use crate::ball::components::*;
use crate::ball::materials::BallMaterial;
use crate::ball::resources::BallScaler;
use crate::ball::utils::*;

use crate::physics::Layer;
const BALL_DENSITY: f32 = 30.;

#[derive(Bundle)]
pub struct BallBundle {
    material_mesh_2d: MaterialMesh2dBundle<BallMaterial>,
    rigid_body: RigidBody,
    gravity_scale: GravityScale,
    collider: Collider,
    collision_layer: CollisionLayers,
    collider_density: ColliderDensity,
    balltype: BallType,
    name: Name,
    ball: Ball,
}

pub fn new(
    materialmesh: MaterialMesh2dBundle<BallMaterial>,
    ball_type: BallType,
    ball_scaler: BallScaler,
) -> BallBundle {
    let balldata = get_ball_stats(ball_type);
    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(balldata.level);
    BallBundle {
        material_mesh_2d: materialmesh,
        rigid_body: RigidBody::Dynamic,
        collider: Collider::ball(ball_size),
        collision_layer: CollisionLayers::new([Layer::Ball], [Layer::Ball, Layer::Wall]),
        gravity_scale: GravityScale(20.0),
        collider_density: ColliderDensity(BALL_DENSITY),
        balltype: ball_type,
        ball: Ball {},
        name: Name::new("Ball"),
    }
}

pub fn new_seed(
    materialmesh: MaterialMesh2dBundle<BallMaterial>,
    ball_type: BallType,
    ball_scaler: BallScaler,
) -> BallBundle {
    let balldata = get_ball_stats(ball_type);
    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(balldata.level);
    BallBundle {
        material_mesh_2d: materialmesh,
        rigid_body: RigidBody::Dynamic,
        collider: Collider::ball(ball_size),
        collision_layer: CollisionLayers::new([Layer::NotBall], [Layer::Ball, Layer::Wall]),
        gravity_scale: GravityScale(20.0),
        collider_density: ColliderDensity(BALL_DENSITY),
        balltype: ball_type,
        ball: Ball {},
        name: Name::new("Ball"),
    }
}
