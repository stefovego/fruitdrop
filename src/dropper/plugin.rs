use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::Rng;

use crate::dropper::components::*;
use crate::dropper::resources::*;

use crate::ball::resources::{BallColors, BallScaler};
use crate::game_board::GameBoard;
use crate::game_state::{AppState, GameState};
use crate::handle_input::Action;
use crate::walls::LEVEL_WIDTH;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct InitDropperSet;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct UpdateDropperSet;

const MAX_DROPPABLE_LEVEL: usize = 4;

pub struct DropperPlugin;
impl Plugin for DropperPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();
        app.insert_resource(LoadedBall(rng.gen_range(0..MAX_DROPPABLE_LEVEL)))
            .insert_resource(DropperStats {
                speed: 500.,
                delay_time: 0.5,
            })
            .add_systems(
                OnEnter(AppState::InGame),
                spawn_dropper.in_set(InitDropperSet),
            )
            .add_systems(
                Update,
                (dropper_movement, mouse_system, restrict_dropper_movement)
                    .chain()
                    .run_if(in_state(GameState::Playing))
                    .in_set(UpdateDropperSet),
            )
            .add_systems(
                Update,
                loaded_ball_change.run_if(
                    resource_exists_and_changed::<LoadedBall>.and(in_state(GameState::Playing)),
                ),
            );
    }
}

#[allow(unused)]
fn tear_down(mut commands: Commands, ball_query: Query<Entity, With<Dropper>>) {
    for ball_entity in &ball_query {
        commands.entity(ball_entity).despawn();
    }
}

fn loaded_ball_change(
    mut commands: Commands,
    loaded_ball: Res<LoadedBall>,
    load_ball_query: Single<(&ChildOf, Entity), With<LoadedBallComponent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ball_scaler: Res<BallScaler>,
    ball_colors: Res<BallColors>,
) {
    let BallColors(ball_colors) = *ball_colors;
    let (child_of, child) = load_ball_query.into_inner();
    commands.entity(child_of.parent()).remove_children(&[child]);
    commands.entity(child).despawn();

    let LoadedBall(level) = *loaded_ball;
    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(level as f32);

    let loadball_entity = commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(ball_size))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(ball_colors[level]))),
            Transform::from_xyz(0.0, 0., 1.),
            LoadedBallComponent,
        ))
        .id();

    commands
        .entity(child_of.parent())
        .add_child(loadball_entity);
}
fn spawn_dropper(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut loaded_ball: ResMut<LoadedBall>,
    ball_scaler: Res<BallScaler>,
    ball_colors: Res<BallColors>,
    game_board: Single<Entity, With<GameBoard>>,
) {
    let BallColors(ball_colors) = *ball_colors;
    let mut rng = rand::thread_rng();

    let level = rng.gen_range(0..MAX_DROPPABLE_LEVEL);
    *loaded_ball = LoadedBall(level);
    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(level as f32);

    // Spawn The Dropper Entity itself
    commands.entity(*game_board).with_children(|parent| {
        parent.spawn((
            Transform::from_xyz(0.0, 400.0, 0.0),
            Name::new("Dropper"),
            Visibility::Visible,
            Dropper,
            children![(
                Name::new("Ball Preview"),
                Mesh2d(meshes.add(Circle::new(ball_size))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(ball_colors[level]))),
                Transform::from_xyz(0.0, 0., 1.),
                LoadedBallComponent,
            )],
        ));
    });
}

fn dropper_movement(
    dropper_query: Single<&mut Transform, With<Dropper>>,
    time: Res<Time>,
    dropper_stats: Res<DropperStats>,
    input: Res<ActionState<Action>>,
) {
    let mut transform = dropper_query.into_inner();
    let mut direction = Vec3::ZERO;

    if input.pressed(&Action::MoveLeft) {
        direction += Vec3::new(-1., 0., 0.);
    }

    if input.pressed(&Action::MoveRight) {
        direction += Vec3::new(1., 0., 0.);
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    transform.translation += direction * dropper_stats.speed * time.delta_secs();
}

fn mouse_system(
    mut cursor_evr: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut dropper_query: Query<&mut Transform, With<Dropper>>,
) {
    for ev in cursor_evr.read() {
        let (camera, camera_transform) = q_camera.single().unwrap();

        let blah = camera
            .viewport_to_world_2d(camera_transform, Vec2::new(ev.position.x, ev.position.y))
            .unwrap();

        if let Ok(mut transform) = dropper_query.single_mut() {
            transform.translation.x = blah.x;
        }
    }
}

fn restrict_dropper_movement(
    dropper_query: Single<&mut Transform, With<Dropper>>,
    loaded_ball: Res<LoadedBall>,
    ball_scaler: Res<BallScaler>,
) {
    let LoadedBall(level) = *loaded_ball;
    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(level as f32);
    let min_x: f32 = -LEVEL_WIDTH / 2. + ball_size + 5.;
    let max_x: f32 = LEVEL_WIDTH / 2. - ball_size - 5.;

    let mut transform = dropper_query.into_inner();

    if transform.translation.x < min_x {
        transform.translation.x = min_x;
    }
    if transform.translation.x > max_x {
        transform.translation.x = max_x;
    }
}
