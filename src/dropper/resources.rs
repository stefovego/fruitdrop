use bevy::prelude::*;

#[derive(Reflect, Resource, Clone, Default)]
#[reflect(Resource)]
pub struct DropperStats {
    pub delay_time: f32,
    pub speed: f32,
}

#[derive(Resource)]
pub struct LoadedBall(pub usize);
