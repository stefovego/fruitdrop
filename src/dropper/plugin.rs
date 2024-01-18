use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use leafwing_input_manager::prelude::*;

use crate::dropper::components::*;
use crate::dropper::resources::*;

use crate::ball::resources::BallScaler;
use crate::ball::utils::{get_ball_stats, random_ball};
use crate::game_state::AppState;
use crate::handle_input::Action;
use crate::walls::{LEVEL_WIDTH, WALL_THICKNESS};

pub struct DropperPlugin;
impl Plugin for DropperPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedBall {
            balltype: random_ball(),
        })
        .insert_resource(DropperStats {
            speed: 500.,
            delay_time: 0.5,
        })
        .add_systems(OnEnter(AppState::InGame), spawn_dropper)
        .add_systems(
            Update,
            (dropper_movement, mouse_system, restrict_dropper_movement)
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, loaded_ball_change)
        .add_systems(OnExit(AppState::GameOver), tear_down);
    }
}

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
    ball_scaler: Res<BallScaler>,
) {
    if loaded_ball.is_changed() {
        if let Ok((parent, child)) = load_ball_query.get_single_mut() {
            commands.entity(parent.get()).remove_children(&[child]);
            commands.entity(child).despawn();

            let ball = get_ball_stats(loaded_ball.balltype);
            let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(ball.level);
            let loadball_entity = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(ball_size).into()).into(),
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
    ball_scaler: Res<BallScaler>,
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
    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(ball.level);
    let loadball_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(ball_size).into()).into(),
            material: materials.add(ColorMaterial::from(ball.color)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0., 1.)))
        .insert(LoadedBallComponent)
        .id();

    commands.entity(droper_entity).add_child(loadball_entity);
}

fn dropper_movement(
    mut dropper_query: Query<&mut Transform, With<Dropper>>,
    time: Res<Time>,
    dropper_stats: Res<DropperStats>,
    input: Res<ActionState<Action>>,
) {
    if let Ok(mut transform) = dropper_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if input.pressed(Action::MoveLeft) {
            direction += Vec3::new(-1., 0., 0.);
        }

        if input.pressed(Action::MoveRight) {
            direction += Vec3::new(1., 0., 0.);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * dropper_stats.speed * time.delta_seconds();
    }
}

fn mouse_system(
    mut cursor_evr: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut dropper_query: Query<&mut Transform, With<Dropper>>,
) {
    for ev in cursor_evr.read() {
        let (camera, camera_transform) = q_camera.single();

        let blah = camera
            .viewport_to_world_2d(camera_transform, Vec2::new(ev.position.x, ev.position.y))
            .unwrap();

        if let Ok(mut transform) = dropper_query.get_single_mut() {
            transform.translation.x = blah.x;
        }
    }
}

fn restrict_dropper_movement(
    mut dropper_query: Query<&mut Transform, With<Dropper>>,
    loaded_ball: Res<LoadedBall>,
    ball_scaler: Res<BallScaler>,
) {
    let ball = get_ball_stats(loaded_ball.balltype);
    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(ball.level);
    let min_x: f32 = -LEVEL_WIDTH / 2. + ball_size + 5.;

    let max_x: f32 = LEVEL_WIDTH / 2. - ball_size - 5.;
    if let Ok(mut transform) = dropper_query.get_single_mut() {
        if transform.translation.x < min_x {
            transform.translation.x = min_x;
        }
        if transform.translation.x > max_x {
            transform.translation.x = max_x;
        }
    }
}
