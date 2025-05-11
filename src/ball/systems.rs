use avian2d::prelude::*;
use bevy::animation::{animated_field, AnimationTarget, AnimationTargetId};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::Rng;

use crate::ball::{components::*, resources::*};

use crate::dropper::components::Dropper;
use crate::dropper::resources::{DropperStats, LoadedBall};
use crate::game_board::GameBoard;
use crate::handle_input::Action;
use crate::loserbox::LoserBox;
use crate::material_color_animation::{AnimatedColor, MaterialColorAnimation};
use crate::ondeck::OnDeckBall;
use crate::physics::Layer;
use crate::score::PlayerScore;

const MAX_DROPPABLE_LEVEL: usize = 4;

pub fn tear_down(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    for ball_entity in &ball_query {
        commands.entity(ball_entity).despawn();
    }
}

// Prevents the ball from triggering the gameover event when first dropped
pub fn fresh_balls(
    mut commands: Commands,
    ball_query: Query<Entity, With<FreshBall>>,
    loser_box_query: Query<Entity, With<LoserBox>>,
    mut collision_events: EventReader<CollisionStarted>,
) {
    for collision_event in collision_events.read() {
        for entity in [collision_event.0, collision_event.1] {
            if let Ok(entity) = ball_query.get(entity) {
                let other_entity = if entity == collision_event.0 {
                    collision_event.1
                } else {
                    collision_event.0
                };
                if loser_box_query.get(other_entity) == Ok(other_entity) {
                } else {
                    commands.entity(entity).remove::<FreshBall>();
                }
            }
        }
    }
}

pub fn seed_systems(
    mut commands: Commands,
    mut seed_query: Query<(Entity, &mut CollisionLayers), With<Seed>>,
) {
    for (entity, mut collision_layer) in &mut seed_query {
        collision_layer.memberships.add(Layer::Ball);
        commands.entity(entity).remove::<CollisionLayers>();
        commands.entity(entity).remove::<Seed>();
        commands.entity(entity).insert(CollisionLayers::new(
            [Layer::Ball],
            [Layer::Wall, Layer::Ball],
        ));
    }
}

pub fn handle_collisions(
    mut commands: Commands,
    ball_query: Query<(Entity, &Transform, &BallLevel), With<Ball>>,
    mut collision_events: EventReader<CollisionStarted>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_score: ResMut<PlayerScore>,
    ball_scaler: Res<BallScaler>,
    grow_stats: Res<GrowStats>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    ball_colors: Res<BallColors>,
    game_board: Single<Entity, With<GameBoard>>,
) {
    for collision_event in collision_events.read() {
        let BallColors(ball_colors) = *ball_colors;
        if let (
            Ok((f, first_transform, BallLevel(first_level))),
            Ok((s, second_transform, BallLevel(second_level))),
        ) = (
            ball_query.get(collision_event.0),
            ball_query.get(collision_event.1),
        ) {
            if *first_level == *second_level {
                let third_ball_translation =
                    (first_transform.translation + second_transform.translation) / 2.0;
                commands.entity(f).despawn();
                commands.entity(s).despawn();
                if *first_level != ball_colors.len() {
                    let next_level = *first_level + 1;
                    let new_ball_size = ball_scaler.initial_size
                        * ball_scaler.size_multiplier.powf(next_level as f32);
                    let ball_color = ball_colors[next_level - 1 as usize];
                    let next_ball_color = ball_colors[next_level as usize];
                    spawn_new_ball(
                        &mut commands,
                        &game_board,
                        &mut meshes,
                        &mut materials,
                        &mut animations,
                        &mut graphs,
                        &grow_stats,
                        new_ball_size,
                        ball_color,
                        next_ball_color,
                        third_ball_translation,
                        next_level,
                    );

                    player_score.value += 500_u32 * 2_u32.pow(*first_level as u32 + 1);
                } else {
                    player_score.value += 500_u32 * 2_u32.pow(*first_level as u32 + 1);
                }
            }
        }
    }
}

// Spawns a ball after two matching balls collide
pub fn spawn_new_ball(
    commands: &mut Commands,
    game_board: &Single<Entity, With<GameBoard>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    animations: &mut ResMut<Assets<AnimationClip>>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    grow_stats: &Res<GrowStats>,
    ball_size: f32,
    ball_color: Color,
    next_ball_color: Color,
    translation: Vec3,
    next_level: usize,
) {
    let name_component = Name::new("Ball");

    let ball_entity = commands
        .spawn((
            Ball,
            Collider::circle(ball_size),
            BallLevel(next_level),
            Mesh2d(meshes.add(Circle::new(ball_size))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(ball_color))),
            Transform {
                translation: translation,
                scale: Vec3::new(
                    grow_stats.initial_multiplier,
                    grow_stats.initial_multiplier,
                    1.,
                ),
                ..default()
            },
            AnimatedColor {
                color: next_ball_color,
                time_seconds: 2.0,
            },
        ))
        .id();
    commands.entity(**game_board).add_child(ball_entity);
}

// Spawns a freshly dropped ball
pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    dropper_query: Query<&Transform, With<Dropper>>,
    mut drop_timer_query: Query<(Entity, &mut DropTimer)>,
    input: Res<ActionState<Action>>,
    time: Res<Time>,
    mut loadedball: ResMut<LoadedBall>,
    mut on_deck_ball: ResMut<OnDeckBall>,
    ball_scaler: Res<BallScaler>,
    _grow_stats: Res<GrowStats>,
    dropper_stats: Res<DropperStats>,
    ball_colors: Res<BallColors>,
    game_board: Single<Entity, With<GameBoard>>,
) {
    let BallColors(ball_colors) = *ball_colors;
    for (entity, mut droptimer) in &mut drop_timer_query {
        droptimer.timer.tick(time.delta());
        if droptimer.timer.just_finished() {
            commands.entity(entity).remove::<DropTimer>();
        } else {
            return;
        }
    }

    if let Ok(transform) = dropper_query.single() {
        if input.just_pressed(&Action::DropBall) {
            let LoadedBall(level) = *loadedball;
            let ball_size =
                ball_scaler.initial_size * ball_scaler.size_multiplier.powf(level as f32);
            commands
                .entity(*game_board)
                .with_child((
                    Ball,
                    Collider::circle(ball_size),
                    BallLevel(level),
                    Mesh2d(meshes.add(Circle::new(ball_size))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(ball_colors[level]))),
                    Transform {
                        translation: Vec3::new(
                            transform.translation.x,
                            transform.translation.y - 50.0,
                            1.0,
                        ),
                        ..default()
                    },
                ))
                .insert(DropTimer {
                    timer: Timer::from_seconds(dropper_stats.delay_time, TimerMode::Once),
                })
                .insert(FreshBall);
            let OnDeckBall(loaded_ball_level) = *on_deck_ball;
            *loadedball = LoadedBall(loaded_ball_level);

            let mut rng = rand::thread_rng();
            *on_deck_ball = OnDeckBall(rng.gen_range(0..MAX_DROPPABLE_LEVEL));
        }
    }
}

pub fn ball_scaler_changed(
    mut commands: Commands,
    ball_scaler: Res<BallScaler>,
    mut meshes: ResMut<Assets<Mesh>>,
    ball_query: Query<(Entity, &BallLevel), With<Ball>>,
) {
    if ball_scaler.is_changed() {
        for (entity, &BallLevel(level)) in &ball_query {
            let new_ball_size =
                ball_scaler.initial_size * ball_scaler.size_multiplier.powf(level as f32);
            commands
                .entity(entity)
                .insert(Mesh2d(meshes.add(Circle::new(new_ball_size))));
        }
    }
}
