use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::ball::components::{Ball, FreshBall};
use crate::game_state::AppState;

pub const WIDTH: f32 = 500.;
pub const HEIGHT: f32 = 50.;
pub const BORDER_HEIGHT: f32 = 350.;
pub const DANGER_ZONE: f32 = 300.;

pub struct LoserBoxPlugin;

impl Plugin for LoserBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::GameOver), tear_down)
            .add_systems(Update, handle_collisions.run_if(in_state(AppState::InGame)))
            .add_systems(Update, danger_warning.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct LoserBox;

#[derive(Component)]
pub struct DangerWarning {
    pub color: Color,
}

fn tear_down(mut commands: Commands, loser_box_query: Query<Entity, With<LoserBox>>) {
    for loser_box_entity in &loser_box_query {
        commands.entity(loser_box_entity).despawn_recursive();
    }
}

fn danger_warning(
    mut commands: Commands,
    ball_query: Query<&Transform, (With<Ball>, Without<FreshBall>)>,
    // mut warning_query: Query<(Entity, &DangerWarning), With<DangerWarning>>,
    mut warning_query: Query<(Entity, &Handle<ColorMaterial>, &DangerWarning), With<DangerWarning>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut danger_amount: f32 = 0.;
    let mut largest_y: f32 = -500.;

    for (ball_transform) in &ball_query {
        // if ball_velocity.linvel.y >= 0. && ball_transform.translation.y > largest_y {
        if ball_transform.translation.y > largest_y {
            largest_y = ball_transform.translation.y;
        }
        if largest_y > BORDER_HEIGHT - DANGER_ZONE {
            danger_amount = (largest_y - (BORDER_HEIGHT - DANGER_ZONE)) / DANGER_ZONE;
        }
    }

    for (warning_entity, warning_color, danger_bar) in &warning_query {
        commands
            .entity(warning_entity)
            .insert(materials.add(ColorMaterial {
                color: danger_bar.color.with_a(danger_amount),
                ..default()
            }));
    }
}

fn handle_collisions(
    rapier_context: Res<RapierContext>,
    ball_query: Query<Entity, (With<Ball>, Without<FreshBall>)>,
    loser_box_query: Query<Entity, With<LoserBox>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for loser_box_entity in &loser_box_query {
        for ball_entity in &ball_query {
            if rapier_context.intersection_pair(loser_box_entity, ball_entity) == Some(true) {
                next_state.set(AppState::GameOver);
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Collider::cuboid(WIDTH, HEIGHT),
        TransformBundle::from(Transform::from_xyz(0., BORDER_HEIGHT, 0.)),
        Sensor,
        LoserBox,
        Name::new("LoserBox"),
    ));

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(WIDTH, 10.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform {
                translation: Vec3::new(0., BORDER_HEIGHT, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            BORDER_HEIGHT,
            0.0,
        )))
        .insert(DangerWarning { color: Color::RED })
    .insert(Name::new("danger_warning"));
}
