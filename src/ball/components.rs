use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct FreshBall;

#[derive(Component, Reflect)]
pub struct DropTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct GrowTimer {
    pub timer: Timer,
    pub old_color: Color,
    pub new_color: Color,
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BallType {
    XXXSmall,
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
}
