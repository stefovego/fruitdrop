use avian2d::prelude::*;
use bevy::animation::{animated_field, AnimationTarget, AnimationTargetId};
use leafwing_input_manager::prelude::*;

use crate::ball;
use crate::ball::{components::*, resources::*, utils::*};

use crate::dropper::components::Dropper;
use crate::dropper::resources::{DropperStats, LoadedBall};
use crate::handle_input::Action;
use crate::loserbox::LoserBox;
use crate::ondeck::OnDeckBall;
use crate::physics::Layer;
use crate::score::PlayerScore;
use bevy::prelude::*;

pub const KING_BALL: BallType = BallType::XXLarge;

pub fn tear_down(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    for ball_entity in &ball_query {
        commands.entity(ball_entity).despawn_recursive();
    }
}

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
        // collision_layer.remove_mask(Layer::NotBall);
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
    ball_query: Query<(Entity, &BallType, &Transform), With<Ball>>,
    mut collision_events: EventReader<CollisionStarted>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_score: ResMut<PlayerScore>,
    ball_scaler: Res<BallScaler>,
    grow_stats: Res<GrowStats>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    for collision_event in collision_events.read() {
        if let (Ok((f, ft, first_transform)), Ok((s, st, second_transform))) = (
            ball_query.get(collision_event.0),
            ball_query.get(collision_event.1),
        ) {
            if *ft == *st {
                let third_ball_translation =
                    (first_transform.translation + second_transform.translation) / 2.0;
                commands.entity(f).despawn();
                commands.entity(s).despawn();
                if *ft != KING_BALL {
                    let name_component = Name::new("Ball");
                    // Prep Animations
                    // Create animation
                    let mut animation = AnimationClip::default();
                    // Curve that modifies a transform
                    let ball_animation_target_id = AnimationTargetId::from_name(&name_component);
                    animation.add_curve_to_target(
                        ball_animation_target_id,
                        AnimatableCurve::new(
                            animated_field!(Transform::scale),
                            AnimatableKeyframeCurve::new(
                                [0.0, 0.5]
                                    .into_iter()
                                    .zip([Vec3::splat(0.5), Vec3::splat(1.0)]),
                            )
                            .expect("Ball Animation Fail"),
                        ),
                    );
                    let (graph, animation_index) =
                        AnimationGraph::from_clip(animations.add(animation));
                    // Create the animation player, and set it to repeat
                    let mut player = AnimationPlayer::default();
                    player.play(animation_index);

                    let og_ball_type = get_ball_stats(*ft);
                    player_score.value += og_ball_type.points;
                    let new_ball = get_ball_stats(og_ball_type.upgraded);
                    let new_ball_size =
                        ball_scaler.initial_size * ball_scaler.size_multiplier.powf(new_ball.level);
                    let ball_entity = commands
                        .spawn((
                            ball::bundles::new_seed(og_ball_type.upgraded, ball_scaler.clone()),
                            Mesh2d(meshes.add(Circle::new(new_ball_size))),
                            MeshMaterial2d(
                                materials.add(ColorMaterial::from_color(new_ball.color)),
                            ),
                            Transform {
                                translation: third_ball_translation,
                                scale: Vec3::new(
                                    grow_stats.initial_multiplier,
                                    grow_stats.initial_multiplier,
                                    1.,
                                ),
                                ..default()
                            },
                            //name_component,
                            AnimationGraphHandle(graphs.add(graph)),
                            player,
                        ))
                        .id();

                    commands.entity(ball_entity).insert(CollisionLayers::new(
                        [Layer::Ball],
                        [Layer::Wall, Layer::Ball],
                    ));
                    commands.entity(ball_entity).insert(AnimationTarget {
                        id: ball_animation_target_id,
                        player: ball_entity,
                    });
                } else {
                    player_score.value += get_ball_stats(KING_BALL).points;
                }
            }
        }
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<BallMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    dropper_query: Query<&Transform, With<Dropper>>,
    mut drop_timer_query: Query<(Entity, &mut DropTimer)>,
    // keyboard_input: Res<Input<KeyCode>>,
    input: Res<ActionState<Action>>,
    time: Res<Time>,
    mut loadedball: ResMut<LoadedBall>,
    mut on_deck_ball: ResMut<OnDeckBall>,
    ball_scaler: Res<BallScaler>,
    _grow_stats: Res<GrowStats>,
    dropper_stats: Res<DropperStats>,
) {
    for (entity, mut droptimer) in &mut drop_timer_query {
        droptimer.timer.tick(time.delta());
        if droptimer.timer.just_finished() {
            commands.entity(entity).remove::<DropTimer>();
        } else {
            return;
        }
    }

    if let Ok(transform) = dropper_query.get_single() {
        if input.just_pressed(&Action::DropBall) {
            let balldata = get_ball_stats(loadedball.balltype);
            let ball_size =
                ball_scaler.initial_size * ball_scaler.size_multiplier.powf(balldata.level);
            commands
                .spawn((
                    ball::bundles::new(loadedball.balltype, ball_scaler.clone()),
                    Mesh2d(meshes.add(Circle::new(ball_size))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(balldata.color))),
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
            loadedball.balltype = on_deck_ball.balltype;
            on_deck_ball.balltype = random_ball();
        }
    }
}

pub fn ball_scaler_changed(
    mut commands: Commands,
    ball_scaler: Res<BallScaler>,
    mut meshes: ResMut<Assets<Mesh>>,
    ball_query: Query<(Entity, &BallType), With<Ball>>,
) {
    if ball_scaler.is_changed() {
        for (entity, ball_type) in &ball_query {
            let ball_stats = get_ball_stats(*ball_type);
            let new_ball_size =
                ball_scaler.initial_size * ball_scaler.size_multiplier.powf(ball_stats.level);
            commands
                .entity(entity)
                .insert(Mesh2d(meshes.add(Circle::new(new_ball_size))));
        }
    }
}
