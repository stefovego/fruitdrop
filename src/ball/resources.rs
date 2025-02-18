use bevy::prelude::*;

#[derive(Reflect, Resource, Clone, Default)]
#[reflect(Resource)]
pub struct GrowStats {
    pub initial_multiplier: f32,
    pub grow_speed: f32,
}

#[derive(Reflect, Resource, Clone, Default)]
#[reflect(Resource)]
pub struct BallScaler {
    pub initial_size: f32,
    pub size_multiplier: f32,
}

#[derive(Resource)]
pub struct BallColors(pub [Color; 11]);
