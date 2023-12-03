use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ball::Ball;
use crate::game_state::AppState;

pub const WIDTH: f32 = 500.;
pub const HEIGHT: f32 = 50.;

pub struct LoserBoxPlugin;

impl Plugin for LoserBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::GameOver), tear_down)
            .add_systems(Update, handle_collisions.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct LoserBox;

fn tear_down(mut commands: Commands, loser_box_query: Query<Entity, With<LoserBox>>) {
    for loser_box_entity in &loser_box_query {
        commands.entity(loser_box_entity).despawn_recursive();
    }
}

fn handle_collisions(
    rapier_context: Res<RapierContext>,
    ball_query: Query<(Entity, &Velocity), With<Ball>>,
    loser_box_query: Query<Entity, With<LoserBox>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for loser_box_entity in &loser_box_query {
        for (ball_entity, velocity) in &ball_query {
            if rapier_context.intersection_pair(loser_box_entity, ball_entity) == Some(true) {
                if velocity.linvel.y >= 0. {
                    next_state.set(AppState::GameOver);
                }
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Collider::cuboid(WIDTH, HEIGHT),
        TransformBundle::from(Transform::from_xyz(0., 350., 0.)),
        Sensor,
        LoserBox,
        Name::new("LoserBox"),
    ));
}
