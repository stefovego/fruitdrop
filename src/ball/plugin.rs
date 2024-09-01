use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

use crate::ball::{materials::BallMaterial, resources::*, systems::*};
use crate::game_state::GameState;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallScaler>()
            .add_plugins(Material2dPlugin::<BallMaterial>::default())
            .register_type::<BallScaler>()
            .insert_resource(BallScaler {
                initial_size: 10.,
                size_multiplier: f32::sqrt(2.),
            })
            .init_resource::<GrowStats>()
            .register_type::<GrowStats>()
            .insert_resource(GrowStats {
                initial_multiplier: 0.3,
                grow_speed: 0.7,
            })
            .add_systems(Update, spawn_ball.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                ball_scaler_changed.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, fresh_balls.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                (seed_systems, handle_collisions, apply_deferred, grow_balls).chain().run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::GameOver), tear_down);
    }
}
