use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

#[derive(PhysicsLayer)]
pub enum Layer {
    Ball,
    NotBall,
    Wall,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::NEG_Y * 80.0));
    }
}
