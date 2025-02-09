use avian2d::prelude::*;
use bevy::prelude::*;

use crate::ball::components::{Ball, FreshBall};
use crate::game_state::GameState;

pub const WIDTH: f32 = 700.;
pub const HEIGHT: f32 = 50.;
pub const BORDER_HEIGHT: f32 = 350.;
pub const DANGER_ZONE: f32 = 300.;

pub struct LoserBoxPlugin;

impl Plugin for LoserBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(OnExit(GameState::GameOver), tear_down)
            .add_systems(
                Update,
                handle_collisions.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, danger_warning.run_if(in_state(GameState::Playing)));
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
    warning_query: Query<
        (Entity, &MeshMaterial2d<ColorMaterial>, &DangerWarning),
        With<DangerWarning>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut danger_amount: f32 = 0.;
    let mut largest_y: f32 = -500.;

    for ball_transform in &ball_query {
        if ball_transform.translation.y > largest_y {
            largest_y = ball_transform.translation.y;
        }
        if largest_y > BORDER_HEIGHT - DANGER_ZONE {
            danger_amount = (largest_y - (BORDER_HEIGHT - DANGER_ZONE)) / DANGER_ZONE;
        }
    }

    for (warning_entity, _warning_color, danger_bar) in &warning_query {
        commands
            .entity(warning_entity)
            .insert(MeshMaterial2d(materials.add(ColorMaterial {
                color: danger_bar.color.with_alpha(danger_amount),
                ..default()
            })));
    }
}

fn handle_collisions(
    spatial_query: SpatialQuery,
    ball_query: Query<Entity, (With<Ball>, Without<FreshBall>)>,
    loser_box_query: Query<(Entity, &Collider, &Transform), With<LoserBox>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (_loser_box_entity, loser_box_colllider, loser_box_transform) in &loser_box_query {
        let aabb = loser_box_colllider.aabb(loser_box_transform.translation.xy(), 0.);
        let aabb_intersections = spatial_query.aabb_intersections_with_aabb(aabb);
        for entity in aabb_intersections.iter() {
            if ball_query.contains(*entity) {
                next_state.set(GameState::GameOver);
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
        Collider::rectangle(WIDTH, HEIGHT),
        Transform::from_xyz(0., BORDER_HEIGHT, 0.),
        Sensor,
        LoserBox,
        Name::new("LoserBox"),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(WIDTH, 10.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgb(1.0, 0.0, 0.0)))),
        Transform::from_translation(Vec3::new(0., BORDER_HEIGHT, 1.0)),
        //Transform::from_xyz(0.0, BORDER_HEIGHT, 0.0),
        DangerWarning {
            color: Color::linear_rgb(1.0, 0.0, 0.0),
        },
        Name::new("danger_warning"),
    ));
}
