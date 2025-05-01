use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

use crate::ball::{materials::BallMaterial, resources::*, systems::*};
use crate::game_state::{AppState, GameState};

pub struct BallPlugin;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct InitBallSet;

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
            .insert_resource(BallColors([
                Color::srgb_u8(0xEF, 0x3A, 0x0C),
                Color::srgb_u8(0x27, 0x64, 0x68),
                Color::srgb_u8(0x9B, 0x1A, 0x0A),
                Color::srgb_u8(0xEF, 0x69, 0x1F),
                Color::srgb_u8(0xEF, 0xAC, 0x28),
                Color::srgb_u8(0xEF, 0xB7, 0x75),
                Color::srgb_u8(0xA5, 0x8C, 0x27),
                Color::srgb_u8(0xA5, 0x62, 0x43),
                Color::srgb_u8(0x18, 0x3F, 0x39),
                Color::srgb_u8(0x3C, 0x9F, 0x9C),
                Color::srgb_u8(0x39, 0x57, 0x1C),
            ]))
            .add_systems(
                Update,
                spawn_ball.run_if(in_state(AppState::InGame)), //.run_if(in_state(GameState::Playing))
            )
            .add_systems(
                Update,
                ball_scaler_changed.run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, fresh_balls.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                (
                    seed_systems,
                    handle_collisions,
                    ApplyDeferred,
                    // grow_balls
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
                //.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::GameOver), tear_down);
    }
}
