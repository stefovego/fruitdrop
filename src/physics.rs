use avian2d::{dynamics::solver::SolverConfig, prelude::*};
use bevy::prelude::*;

use crate::game_state::GameState;

#[derive(PhysicsLayer, Default)]
pub enum Layer {
    Ball,
    #[default]
    NotBall,
    Wall,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::NEG_Y * 80.0))
            .insert_resource(SolverConfig {
                contact_damping_ratio: 20.,
                contact_frequency_factor: 3.,
                max_overlap_solve_speed: 16.,
                ..Default::default()
            })
            .insert_resource(SubstepCount(18))
            .add_systems(OnExit(GameState::Playing), pause_physics)
            .add_systems(OnEnter(GameState::Playing), unpause_physics);
    }
}

fn pause_physics(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn unpause_physics(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}
