use bevy::prelude::*;
use rand::Rng;

use crate::ball::components::BallType;

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
    color: Color::INDIGO,
    upgraded: BallType::XXLarge,
};

pub const XXLARGE: BallData = BallData {
    size: 226.,
    points: 4000,
    color: Color::YELLOW_GREEN,
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
