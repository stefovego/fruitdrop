use bevy::prelude::*;
use rand::Rng;

use crate::ball::components::BallType;

pub struct BallData {
    pub color: LinearRgba,
    pub points: u32,
    pub level: f32,
    pub upgraded: BallType,
}

pub const XXXSMALL: BallData = BallData {
    level: 1.,
    color: LinearRgba::rgb(1.0, 0.388, 0.278), // TOMATO
    //color: ::from_u8_array_no_alpha([0xffu8, 0x63u8, 0x47u8]),
    points: 100,
    upgraded: BallType::XXSmall,
};

pub const XXSMALL: BallData = BallData {
    level: 2.,
    points: 200,
    color: LinearRgba::BLUE,
    upgraded: BallType::XSmall,
};

pub const XSMALL: BallData = BallData {
    level: 3.,
    points: 300,
    color: LinearRgba::rgb(1.0, 1.0, 0.0),
    upgraded: BallType::Small,
};

pub const SMALL: BallData = BallData {
    level: 4.,
    points: 300,
    color: LinearRgba::rgb(1., 0.753, 0.796),
    upgraded: BallType::Medium,
};

pub const MEDIUM: BallData = BallData {
    level: 5.,
    points: 400,
    color: LinearRgba::rgb(1.0, 0.647, 0.0), //Color::ORANGE,
    upgraded: BallType::Large,
};

pub const LARGE: BallData = BallData {
    level: 6.,
    points: 500,
    color: LinearRgba::rgb(0.0, 0.502, 0.502), //Color::TEAL,
    upgraded: BallType::XLarge,
};

pub const XLARGE: BallData = BallData {
    level: 7.,
    points: 2000,
    color: LinearRgba::rgb(0.294, 0.0, 0.51), //Color::INDIGO,
    upgraded: BallType::XXLarge,
};

pub const XXLARGE: BallData = BallData {
    level: 8.,
    points: 4000,
    color: LinearRgba::rgb(0.604, 0.804, 0.196), //Color::YELLOW_GREEN,
    upgraded: BallType::XXSmall,
};

pub fn random_ball() -> BallType {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..3) {
        0 => BallType::XXXSmall,
        1 => BallType::XXSmall,
        2 => BallType::XSmall,
        3 => BallType::Small,
        4 => BallType::Medium,
        5 => BallType::Large,
        _ => unreachable!(),
    }
}

pub fn get_ball_stats(ball_type: BallType) -> BallData {
    match ball_type {
        BallType::XXXSmall => XXXSMALL,
        BallType::XXSmall => XXSMALL,
        BallType::XSmall => XSMALL,
        BallType::Small => SMALL,
        BallType::Medium => MEDIUM,
        BallType::Large => LARGE,
        BallType::XLarge => XLARGE,
        BallType::XXLarge => XXLARGE,
    }
}
