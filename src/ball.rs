use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use crate::dropper::Dropper;

pub const DROP_TIMER_LIMIT: f32 =  0.5;
pub struct BallPlugin;

#[derive(Component, Reflect)]
struct DropTimer{
    timer: Timer
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_ball);
    }
}

#[derive(Component)]
struct Ball;

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut dropper_query: Query<&mut Transform, With<Dropper>>,
    mut drop_timer_query: Query<(Entity, &mut DropTimer)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
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
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(4.0))
                .insert(Collider::ball(50.0))
                .insert(Restitution::coefficient(0.7))
                .insert(TransformBundle::from(Transform::from_xyz(transform.translation.x, transform.translation.y - 50.0, 1.0)))
                .insert(DropTimer{
                        timer: Timer::from_seconds(DROP_TIMER_LIMIT, TimerMode::Once)
                })
                .insert(Ball);
        }
    }
}

