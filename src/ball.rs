use crate::bundles;
use crate::bundles::ball::new;
use crate::dropper::{Dropper, LoadedBall};
use crate::game_state::AppState;
use crate::ondeck::OnDeckBall;
use crate::score::PlayerScore;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub const DROP_TIMER_LIMIT: f32 = 0.5;
pub const KING_BALL: BallType = BallType::XLarge;

pub struct BallData {
    pub color: Color,
    pub points: u32,
    pub size: f32,
    pub upgraded: BallType,
}

pub const XXXSMALL: BallData = BallData {
    color: Color::TOMATO,
    points: 100,
    size: 20.,
    upgraded: BallType::XXSmall,
};

pub const XXSMALL: BallData = BallData {
    size: 28.3,
    points: 200,
    color: Color::BLUE,
    upgraded: BallType::XSmall,
};

pub const XSMALL: BallData = BallData {
    size: 40.,
    points: 300,
    color: Color::YELLOW,
    upgraded: BallType::Small,
};

pub const SMALL: BallData = BallData {
    size: 56.7,
    points: 300,
    color: Color::PINK,
    upgraded: BallType::Medium,
};

pub const MEDIUM: BallData = BallData {
    size: 80.,
    points: 400,
    color: Color::ORANGE,
    upgraded: BallType::Large,
};

pub const LARGE: BallData = BallData {
    size: 113.1,
    points: 500,
    color: Color::TEAL,
    upgraded: BallType::XLarge,
};

pub const XLARGE: BallData = BallData {
    size: 160.,
    points: 2000,
    color: Color::YELLOW_GREEN,
    upgraded: BallType::XXSmall,
};

pub struct BallPlugin;

#[derive(Component, Reflect)]
struct DropTimer {
    timer: Timer,
}

#[derive(Component)]
struct GrowTimer {
    timer: Timer,
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BallType {
    XXXSmall,
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_ball.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                (handle_collisions, apply_deferred, grow_balls).chain(),
            )
            .add_systems(OnExit(AppState::GameOver), tear_down);
    }
}

#[derive(Component)]
pub struct Ball;

pub fn random_ball() -> BallType {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..3) {
        0 => BallType::XXXSmall,
        1 => BallType::XXSmall,
        2 => BallType::XSmall,
        3 => BallType::Small,
        4 => BallType::Medium,
        5 => BallType::Large,
        _ => unreachable!(),
    }
}

pub fn get_ball_stats(ball_type: BallType) -> BallData {
    match ball_type {
        BallType::XXXSmall => XXXSMALL,
        BallType::XXSmall => XXSMALL,
        BallType::XSmall => XSMALL,
        BallType::Small => SMALL,
        BallType::Medium => MEDIUM,
        BallType::Large => LARGE,
        BallType::XLarge => XLARGE,
    }
}

fn tear_down(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    for ball_entity in &ball_query {
        commands.entity(ball_entity).despawn_recursive();
    }
}

fn grow_balls(
    mut commands: Commands,
    mut grow_timer_query: Query<(Entity, &mut Transform, &mut GrowTimer), With<GrowTimer>>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut grow_timer) in &mut grow_timer_query {
        grow_timer.timer.tick(time.delta());
        if grow_timer.timer.just_finished() {
            transform.scale = Vec3::new(1., 1., 1.0);
            commands.entity(entity).remove::<GrowTimer>();
        } else {
            let grow_percent = grow_timer.timer.percent() / 2. + 0.5;
            info!("Growing {:?}", grow_percent);
            transform.scale = Vec3::new(grow_percent, grow_percent, 1.0);
        }
    }
}

fn handle_collisions(
    mut commands: Commands,
    ball_query: Query<(Entity, &BallType, &Transform), With<Ball>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_score: ResMut<PlayerScore>,
) {
    for collision_event in collision_events.read() {
        info!("Count: {:?}", ball_query.iter().count());
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                //let world = world::World::new();
                if let (Ok((f, ft, first_transform)), Ok((s, st, second_transform))) = (
                    ball_query.get(*first_entity),
                    ball_query.get(*second_entity),
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
                            let mesh_material = MaterialMesh2dBundle {
                                mesh: meshes.add(shape::Circle::new(new_ball.size).into()).into(),
                                material: materials.add(ColorMaterial::from(new_ball.color)),
                                transform: Transform {
                                    translation: third_ball_translation,
                                    ..default()
                                },
                                ..default()
                            };
                            commands
                                .spawn(bundles::ball::new(mesh_material, og_ball_type.upgraded))
                                .insert(GrowTimer {
                                    timer: Timer::from_seconds(0.25, TimerMode::Once),
                                });
                        } else {
                            player_score.value += get_ball_stats(KING_BALL).points;
                        }
                    }
                }
            }
            _ => {}
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
            let mesh_material = MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(balldata.size).into()).into(),
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
                .spawn(bundles::ball::new(mesh_material, loadedball.balltype))
                .insert(DropTimer {
                    timer: Timer::from_seconds(DROP_TIMER_LIMIT, TimerMode::Once),
                });
            loadedball.balltype = on_deck_ball.balltype;
            on_deck_ball.balltype = random_ball();
        }
    }
}
