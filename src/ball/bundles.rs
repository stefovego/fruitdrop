use crate::ball::components::*;
use crate::ball::utils::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct BallBundle {
    material_mesh_2d: MaterialMesh2dBundle<ColorMaterial>,
    rigid_body: RigidBody,
    velocity: Velocity,
    gravity_scale: GravityScale,
    collider: Collider,
    restitution: Restitution,
    active_events: ActiveEvents,
    balltype: BallType,
    name: Name,
    ball: Ball,
}

pub fn new(materialmesh: MaterialMesh2dBundle<ColorMaterial>, ball_type: BallType) -> BallBundle {
    let balldata = get_ball_stats(ball_type);
    BallBundle {
        material_mesh_2d: materialmesh,
        rigid_body: RigidBody::Dynamic,
        velocity: Velocity::zero(),
        gravity_scale: GravityScale(20.0),
        collider: Collider::ball(balldata.size),
        restitution: Restitution::coefficient(0.4),
        active_events: ActiveEvents::COLLISION_EVENTS,
        balltype: ball_type,
        name: Name::new("Ball"),
        ball: Ball {},
    }
}
