use crate::ball;
use crate::ball::components::*;
use crate::ball::resources::*;
use crate::ball::utils::*;
use crate::dropper::components::Dropper;
use crate::dropper::resources::DropperStats;
use crate::dropper::resources::LoadedBall;
use crate::game_state::AppState;
use crate::loserbox::LoserBox;
use crate::ondeck::OnDeckBall;
use crate::physics::Layer;
use crate::score::PlayerScore;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::prelude::*;

//pub const DROP_TIMER_LIMIT: f32 = 0.8;
pub const KING_BALL: BallType = BallType::XXLarge;

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
            .add_systems(Update, fresh_balls.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                (seed_systems, handle_collisions, apply_deferred, grow_balls).chain(),
            )
            .add_systems(OnExit(AppState::GameOver), tear_down);
    }
}

fn tear_down(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    for ball_entity in &ball_query {
        commands.entity(ball_entity).despawn_recursive();
    }
}

fn grow_balls(
    mut commands: Commands,
    mut grow_timer_query: Query<
        (
            Entity,
            &mut Transform,
            &mut GrowTimer,
            &Handle<ColorMaterial>,
        ),
        With<GrowTimer>,
    >,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, mut transform, mut grow_timer, mut handle_color) in &mut grow_timer_query {
        grow_timer.timer.tick(time.delta());
        if grow_timer.timer.just_finished() {
            transform.scale = Vec3::new(1., 1., 1.0);
            commands
                .entity(entity)
                .insert(materials.add(ColorMaterial::from(grow_timer.new_color)));
            commands.entity(entity).remove::<GrowTimer>();
        } else {
            let grow_percent = (1. - grow_timer.initial_multiplier) * grow_timer.timer.percent()
                + grow_timer.initial_multiplier;
            //TODO: This needs a clean up
            let mut current_color = &mut materials.get_mut(handle_color).unwrap().color;
            let o = grow_timer.old_color.as_linear_rgba_f32();
            let old = Vec4::new(o[0], o[1], o[2], o[3]);

            let n = grow_timer.new_color.as_linear_rgba_f32();
            let new = Vec4::new(n[0], n[1], n[2], n[3]);

            let m = old.lerp(new, grow_timer.timer.percent());

            commands
                .entity(entity)
                .insert(materials.add(ColorMaterial::from(Color::rgba_linear(m.x, m.y, m.z, m.w))));
            transform.scale = Vec3::new(grow_percent, grow_percent, 1.0);
        }
    }
}

fn fresh_balls(
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

fn seed_systems(
    mut commands: Commands,
    mut seed_query: Query<(Entity, &mut CollisionLayers), With<Seed>>,
) {
    for (entity, mut collision_layer) in &mut seed_query {
        // collision_layer.remove_mask(Layer::NotBall);
        collision_layer.add_mask(Layer::Ball);
        commands.entity(entity).remove::<CollisionLayers>();
        commands.entity(entity).remove::<Seed>();
        commands.entity(entity).insert(CollisionLayers::new(
            [Layer::Ball],
            [Layer::Wall, Layer::Ball],
        ));
    }
}
fn handle_collisions(
    mut commands: Commands,
    ball_query: Query<(Entity, &BallType, &Transform), With<Ball>>,
    mut collision_events: EventReader<CollisionStarted>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_score: ResMut<PlayerScore>,
    ball_scaler: Res<BallScaler>,
    grow_stats: Res<GrowStats>,
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
                    let og_ball_type = get_ball_stats(*ft);
                    player_score.value += og_ball_type.points;
                    let new_ball = get_ball_stats(og_ball_type.upgraded);
                    let new_ball_size =
                        ball_scaler.initial_size * ball_scaler.size_multiplier.powf(new_ball.level);
                    let mesh_material = MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(new_ball_size).into()).into(),
                        material: materials.add(ColorMaterial::from(new_ball.color)),
                        transform: Transform {
                            translation: third_ball_translation,
                            scale: Vec3::new(
                                grow_stats.initial_multiplier,
                                grow_stats.initial_multiplier,
                                1.,
                            ),
                            ..default()
                        },
                        ..default()
                    };
                    commands
                        .spawn(ball::bundles::new_seed(
                            mesh_material,
                            og_ball_type.upgraded,
                            ball_scaler.clone(),
                        ))
                        .insert(GrowTimer {
                            timer: Timer::from_seconds(grow_stats.grow_speed, TimerMode::Once),
                            initial_multiplier: grow_stats.initial_multiplier,
                            old_color: og_ball_type.color,
                            new_color: new_ball.color,
                        })
                        .insert(Seed);
                } else {
                    player_score.value += get_ball_stats(KING_BALL).points;
                }
            }
        }
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    dropper_query: Query<&Transform, With<Dropper>>,
    mut drop_timer_query: Query<(Entity, &mut DropTimer)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut loadedball: ResMut<LoadedBall>,
    mut on_deck_ball: ResMut<OnDeckBall>,
    ball_scaler: Res<BallScaler>,
    grow_stats: Res<GrowStats>,
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
        if keyboard_input.just_pressed(KeyCode::Space) {
            let balldata = get_ball_stats(loadedball.balltype);
            let ball_size =
                ball_scaler.initial_size * ball_scaler.size_multiplier.powf(balldata.level);
            let mesh_material = MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ball_size).into()).into(),
                material: materials.add(ColorMaterial::from(balldata.color)),
                transform: Transform {
                    translation: Vec3::new(
                        transform.translation.x,
                        transform.translation.y - 50.0,
                        1.0,
                    ),
                    ..default()
                },
                ..default()
            };
            commands
                .spawn(ball::bundles::new(
                    mesh_material,
                    loadedball.balltype,
                    ball_scaler.clone(),
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
