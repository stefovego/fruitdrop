use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::ball::{get_ball_stats, random_ball, BallType};
use crate::game_state::AppState;

pub struct OnDeckPlugin;

#[derive(Resource)]
pub struct OnDeckBall {
    pub balltype: BallType,
}

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

impl Plugin for OnDeckPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OnDeckBall {
            balltype: random_ball(),
        })
        .add_systems(OnEnter(AppState::InGame), spawn_deck)
        .add_systems(Update, on_deck_ball_change)
        .add_systems(OnExit(AppState::GameOver), tear_down_ball)
        .add_systems(OnExit(AppState::GameOver), tear_down_box);
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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut on_deck_ball: ResMut<OnDeckBall>,
) {
    on_deck_ball.balltype = random_ball();
    // Spawn The Box Entity
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2 {
                    x: BOX_WIDTH,
                    y: BOX_THICKNESS,
                }),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: BOX_X,
                    y: BOX_Y + BOX_HEIGHT / 2.,
                    z: 0.,
                },
                ..default()
            },
            ..default()
        },
        OnDeckBox,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2 {
                    x: BOX_WIDTH,
                    y: BOX_THICKNESS,
                }),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: BOX_X,
                    y: BOX_Y - BOX_HEIGHT / 2.,
                    z: 0.,
                },
                ..default()
            },
            ..default()
        },
        OnDeckBox,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2 {
                    x: BOX_THICKNESS,
                    y: BOX_HEIGHT,
                }),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: BOX_X + BOX_WIDTH / 2.,
                    y: BOX_Y,
                    z: 0.,
                },
                ..default()
            },
            ..default()
        },
        OnDeckBox,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2 {
                    x: BOX_THICKNESS,
                    y: BOX_HEIGHT,
                }),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: BOX_X - BOX_WIDTH / 2.,
                    y: BOX_Y,
                    z: 0.,
                },
                ..default()
            },
            ..default()
        },
        OnDeckBox,
    ));

    let on_deck_entity = commands
        .spawn(TransformBundle::from(Transform::from_xyz(
            650.0, 250.0, 0.0,
        )))
        .insert(VisibilityBundle {
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::VISIBLE,
            ..Default::default()
        })
        .insert(OnDeck)
        .id();

    let ball = get_ball_stats(on_deck_ball.balltype);
    let on_deck_ball_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(ball.size).into()).into(),
            material: materials.add(ColorMaterial::from(ball.color)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0., 1.)))
        .insert(OnDeckBallComponent)
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
) {
    if on_deck_ball.is_changed() {
        if let Ok((parent, child)) = on_deck_ball_query.get_single_mut() {
            commands.entity(parent.get()).remove_children(&[child]);
            commands.entity(child).despawn();

            let ball = get_ball_stats(on_deck_ball.balltype);
            let loadball_entity = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(ball.size).into()).into(),
                    material: materials.add(ColorMaterial::from(ball.color)),
                    ..default()
                })
                .insert(TransformBundle::from(Transform::from_xyz(0., 0., 1.)))
                .insert(OnDeckBallComponent)
                .insert(Name::new("OnDeck"))
                .id();

            commands.entity(parent.get()).add_child(loadball_entity);
        }
    }
}
