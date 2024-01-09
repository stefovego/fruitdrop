use crate::ball::resources::*;
use crate::ball::systems::*;
use crate::game_state::AppState;
use bevy::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallScaler>()
            .register_type::<BallScaler>()
            .insert_resource(BallScaler {
                initial_size: 10.,
                size_multiplier: f32::sqrt(2.),
            })
            .init_resource::<GrowStats>()
            .register_type::<GrowStats>()
            .insert_resource(GrowStats {
                initial_multiplier: 0.0,
                grow_speed: 0.8,
            })
            .add_systems(Update, spawn_ball.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                ball_scaler_changed.run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, fresh_balls.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                (seed_systems, handle_collisions, apply_deferred, grow_balls).chain(),
            )
            .add_systems(OnExit(AppState::GameOver), tear_down);
    }
}
