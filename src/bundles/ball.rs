use crate::ball::{Ball, BallType};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

// pub const KING_BALL: BallType = BallType::XLarge;

#[derive(Bundle)]
pub struct BallBundle {
    material_mesh_2d: MaterialMesh2dBundle<ColorMaterial>,
    rigid_body: RigidBody,
    velocity: Velocity,
    gravity_scale: GravityScale,
    collider: Collider,
    restitution: Restitution,
    //transform: TransformBundle,
    active_events: ActiveEvents,
    balltype: BallType,
    name: Name,
    ball: Ball,
}

pub struct BallData {
    pub color: Color,
    pub points: u32,
    pub size: f32,
    pub upgraded: BallType,
}

pub const XXXSMALL: BallData = BallData {
    color: Color::TOMATO,
    points: 100,
    size: 20.,
    upgraded: BallType::XXSmall,
};

pub const XXSMALL: BallData = BallData {
    size: 28.3,
    points: 200,
    color: Color::BLUE,
    upgraded: BallType::XSmall,
};

pub const XSMALL: BallData = BallData {
    size: 40.,
    points: 300,
    color: Color::YELLOW,
    upgraded: BallType::Small,
};

pub const SMALL: BallData = BallData {
    size: 56.7,
    points: 300,
    color: Color::PINK,
    upgraded: BallType::Medium,
};

pub const MEDIUM: BallData = BallData {
    size: 80.,
    points: 400,
    color: Color::ORANGE,
    upgraded: BallType::Large,
};

pub const LARGE: BallData = BallData {
    size: 113.1,
    points: 500,
    color: Color::TEAL,
    upgraded: BallType::XLarge,
};

pub const XLARGE: BallData = BallData {
    size: 160.,
    points: 2000,
    color: Color::YELLOW_GREEN,
    upgraded: BallType::XXSmall,
};

pub fn get_ball_stats(ball_type: BallType) -> BallData {
    match ball_type {
        BallType::XXXSmall => XXXSMALL,
        BallType::XXSmall => XXSMALL,
        BallType::XSmall => XSMALL,
        BallType::Small => SMALL,
        BallType::Medium => MEDIUM,
        BallType::Large => LARGE,
        BallType::XLarge => XLARGE,
    }
}

pub fn new(
    materialmesh: MaterialMesh2dBundle<ColorMaterial>,
    ball_type: BallType,
) -> BallBundle {
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
