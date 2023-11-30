use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use crate::dropper::{Dropper, LoadedBall};
use rand::Rng;

pub const DROP_TIMER_LIMIT: f32 =  0.5;
pub struct BallPlugin;

#[derive(Component, Reflect)]
struct DropTimer{
    timer: Timer
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BallType {
    Small,
    Medium,
    Large
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, spawn_ball)
        .add_systems(Update, debug_collisions);
    }
}

#[derive(Component)]
struct Ball;

pub fn random_ball() -> BallType {
    let mut rng = rand::thread_rng();
            
    match rng.gen_range(0..3) {
        0 => BallType::Small,
        1 => BallType::Medium,
        2 => BallType::Large,
        _ => unreachable!(),
    }
}

fn debug_collisions(
    mut commands: Commands,
    ball_query: Query<(Entity, &BallType), With<Ball>>,
    mut collision_events: EventReader<CollisionEvent>
) {
    for collision_event in collision_events.read(){
        info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                //let world = world::World::new();
                if let (Ok((f, ft)), Ok((s, st))) = (ball_query.get(*first_entity), ball_query.get(*second_entity)) 
                {
                    if *ft == *st {
                        commands.entity(f).despawn();
                        commands.entity(s).despawn();
                    }
                }

            },
            CollisionEvent::Stopped(first_entity, second_entity, event) => {
            }
        }

    }
}


fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut dropper_query: Query<&mut Transform, With<Dropper>>,
    mut drop_timer_query: Query<(Entity, &mut DropTimer)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut loadedball: ResMut<LoadedBall>
    ) {
    for (entity, mut droptimer) in &mut drop_timer_query {
        droptimer.timer.tick(time.delta());
        if droptimer.timer.just_finished(){
            commands.entity(entity).remove::<DropTimer>();
        }
        else {
            return;
        }
    }
    
    if let Ok(mut transform) = dropper_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            let ball_type = loadedball.balltype;
            loadedball.balltype = random_ball();
            let mut ball_size = 1.;
            match ball_type {
                BallType::Small => { 
                    ball_size = 30.;
                },
                BallType::Medium=> {
                    ball_size = 40.;
                },
                BallType::Large=> {
                    ball_size = 50.;
                },
            }
             commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(ball_size).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(4.0))
                .insert(Collider::ball(ball_size))
                .insert(Restitution::coefficient(0.7))
                .insert(TransformBundle::from(Transform::from_xyz(transform.translation.x, transform.translation.y - 50.0, 1.0)))
                .insert(DropTimer{
                        timer: Timer::from_seconds(DROP_TIMER_LIMIT, TimerMode::Once)
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ball_type)
                .insert(Ball); 
        }
    }
}

