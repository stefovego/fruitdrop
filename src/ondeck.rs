use bevy::prelude::*;
use rand::Rng;

use crate::ball::resources::{BallColors, BallScaler};
use crate::game_state::{AppState, GameState};

#[derive(Resource)]
pub struct OnDeckBall(pub usize);

#[derive(Component)]
pub struct OnDeckBallComponent;

#[derive(Component)]
pub struct OnDeck;

#[derive(Component)]
pub struct OnDeckBox;

pub const BOX_WIDTH: f32 = 150.;
pub const BOX_HEIGHT: f32 = 150.;
pub const BOX_X: f32 = 650.;
pub const BOX_Y: f32 = 250.;
pub const BOX_THICKNESS: f32 = 4.;
const MAX_DROPPABLE_LEVEL: usize = 4;

pub struct OnDeckPlugin;

impl Plugin for OnDeckPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();
        app.insert_resource(OnDeckBall(rng.gen_range(0..MAX_DROPPABLE_LEVEL)))
            .add_systems(OnEnter(AppState::InGame), spawn_deck)
            .add_systems(
                Update,
                on_deck_ball_change.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(AppState::InGame), tear_down_ball)
            .add_systems(OnExit(AppState::InGame), tear_down_box);
    }
}

fn tear_down_ball(mut commands: Commands, ball_query: Query<Entity, With<OnDeckBallComponent>>) {
    for ball_entity in &ball_query {
        commands.entity(ball_entity).despawn_recursive();
    }
}

fn tear_down_box(mut commands: Commands, box_query: Query<Entity, With<OnDeckBox>>) {
    for box_entity in &box_query {
        commands.entity(box_entity).despawn_recursive();
    }
}

fn spawn_deck(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<BallMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut on_deck_ball: ResMut<OnDeckBall>,
    ball_scaler: Res<BallScaler>,
    ball_colors: Res<BallColors>,
) {
    let BallColors(ball_colors) = *ball_colors;
    let mut rng = rand::thread_rng();
    let level = rng.gen_range(0..MAX_DROPPABLE_LEVEL);

    *on_deck_ball = OnDeckBall(level);
    // Spawn The Box Entity
    commands.spawn((
        Sprite::from_color(
            Color::linear_rgb(1.0, 0.38823529411764707, 0.2784313725490196), //orange
            Vec2::new(BOX_WIDTH, BOX_THICKNESS),
        ),
        Transform::from_translation(Vec3::new(BOX_X, BOX_Y + BOX_HEIGHT / 2., 0.0)),
        OnDeckBox,
    ));
    commands.spawn((
        Sprite::from_color(
            Color::linear_rgb(1.0, 0.38823529411764707, 0.2784313725490196), //orange
            Vec2::new(BOX_WIDTH, BOX_THICKNESS),
        ),
        Transform::from_translation(Vec3::new(BOX_X, BOX_Y - BOX_HEIGHT / 2., 0.)),
        OnDeckBox,
    ));

    commands.spawn((
        Sprite::from_color(
            Color::linear_rgb(1.0, 0.38823529411764707, 0.2784313725490196), //orange
            Vec2::new(BOX_THICKNESS, BOX_HEIGHT),
        ),
        Transform::from_translation(Vec3::new(BOX_X + BOX_WIDTH / 2., BOX_Y, 0.)),
        OnDeckBox,
    ));

    commands.spawn((
        Sprite::from_color(
            Color::linear_rgb(1.0, 0.38823529411764707, 0.2784313725490196), //orange
            Vec2::new(BOX_THICKNESS, BOX_HEIGHT),
        ),
        Transform::from_translation(Vec3::new(BOX_X - BOX_WIDTH / 2., BOX_Y, 0.0)),
        OnDeckBox,
    ));

    let on_deck_entity = commands
        .spawn((
            Transform::from_xyz(650.0, 250.0, 0.0),
            InheritedVisibility::VISIBLE,
            OnDeck,
        ))
        .id();

    let ball_size = ball_scaler.initial_size * ball_scaler.size_multiplier.powf(level as f32);
    let on_deck_ball_entity = commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(ball_size))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(ball_colors[level]))),
            Transform::from_xyz(0.0, 0., 1.),
            OnDeckBallComponent,
        ))
        .id();

    commands
        .entity(on_deck_entity)
        .add_child(on_deck_ball_entity);
}

fn on_deck_ball_change(
    mut commands: Commands,
    on_deck_ball: Res<OnDeckBall>,
    mut on_deck_ball_query: Query<(&Parent, Entity), With<OnDeckBallComponent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ball_scaler: Res<BallScaler>,
    ball_colors: Res<BallColors>,
) {
    let BallColors(ball_colors) = *ball_colors;
    if on_deck_ball.is_changed() {
        if let Ok((parent, child)) = on_deck_ball_query.get_single_mut() {
            commands.entity(parent.get()).remove_children(&[child]);
            commands.entity(child).despawn();

            let OnDeckBall(level) = *on_deck_ball;
            let ball_size =
                ball_scaler.initial_size * ball_scaler.size_multiplier.powf(level as f32);
            let loadball_entity = commands
                .spawn((
                    Mesh2d(meshes.add(Circle::new(ball_size))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(ball_colors[level]))),
                    Transform::from_xyz(0., 0., 1.),
                    OnDeckBallComponent,
                    Name::new("OnDeck"),
                ))
                .id();

            commands.entity(parent.get()).add_child(loadball_entity);
        }
    }
}
