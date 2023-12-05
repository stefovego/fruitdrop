use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::ball::get_ball_stats;
use crate::ball::{random_ball, BallType};
use crate::game_state::AppState;
use crate::walls::{LEVEL_WIDTH, WALL_THICKNESS};

pub struct DropperPlugin;
pub const DROPPER_SPEED: f32 = 500.0;
pub const DROPPER_WIDTH: f32 = 100.;

impl Plugin for DropperPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedBall {
            balltype: random_ball(),
        })
        .add_systems(OnEnter(AppState::InGame), spawn_dropper)
        .add_systems(
            Update,
            (dropper_movement, restrict_dropper_movement)
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, loaded_ball_change)
        .add_systems(OnExit(AppState::GameOver), tear_down);
    }
}

#[derive(Resource)]
pub struct LoadedBall {
    pub balltype: BallType,
}
#[derive(Component)]
pub struct LoadedBallComponent;

#[derive(Component)]
pub struct Dropper;

fn tear_down(mut commands: Commands, ball_query: Query<Entity, With<Dropper>>) {
    for ball_entity in &ball_query {
        commands.entity(ball_entity).despawn_recursive();
    }
}
fn loaded_ball_change(
    mut commands: Commands,
    loaded_ball: Res<LoadedBall>,
    mut load_ball_query: Query<(&Parent, Entity), With<LoadedBallComponent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if loaded_ball.is_changed() {
        if let Ok((parent, child)) = load_ball_query.get_single_mut() {
            commands.entity(parent.get()).remove_children(&[child]);
            commands.entity(child).despawn();

            let ball = get_ball_stats(loaded_ball.balltype);
            let loadball_entity = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(ball.size).into()).into(),
                    material: materials.add(ColorMaterial::from(ball.color)),
                    ..default()
                })
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0., 1.)))
                .insert(LoadedBallComponent)
                .id();

            commands.entity(parent.get()).add_child(loadball_entity);
        }
    }
}
fn spawn_dropper(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut loaded_ball: ResMut<LoadedBall>,
) {
    loaded_ball.balltype = random_ball();

    // Spawn The Dropper Entity itself
    let droper_entity = commands
        .spawn(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(Name::new("Dropper"))
        .insert(VisibilityBundle {
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::VISIBLE,
            ..Default::default()
        })
        .insert(Dropper)
        .id();

    let ball = get_ball_stats(loaded_ball.balltype);
    let loadball_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(ball.size).into()).into(),
            material: materials.add(ColorMaterial::from(ball.color)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0., 1.)))
        .insert(LoadedBallComponent)
        .id();

    commands.entity(droper_entity).add_child(loadball_entity);
}

fn dropper_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut dropper_query: Query<&mut Transform, With<Dropper>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = dropper_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * DROPPER_SPEED * time.delta_seconds();
    }
}

fn restrict_dropper_movement(mut dropper_query: Query<&mut Transform, With<Dropper>>, loaded_ball: Res<LoadedBall>) {

    let min_x: f32 = -LEVEL_WIDTH / 2. + get_ball_stats(loaded_ball.balltype).size;
    //let min_x: f32 = -(LEVEL_WIDTH + WALL_THICKNESS ) / 2. + get_ball_stats(loaded_ball.balltype).size;
    let max_x: f32 = LEVEL_WIDTH / 2. - get_ball_stats(loaded_ball.balltype).size;
    if let Ok(mut transform) = dropper_query.get_single_mut() {
        if transform.translation.x < min_x {
            transform.translation.x = min_x;
        }
        if transform.translation.x > max_x {
            transform.translation.x = max_x;
        }
    }
}
