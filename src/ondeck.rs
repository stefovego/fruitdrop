use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use crate::ball::{BallType, random_ball, get_ball_stats};

pub struct OnDeckPlugin;

#[derive(Resource)]
pub struct OnDeckBall {
    pub balltype: BallType
}

#[derive(Component)]
pub struct OnDeckBallComponent;

#[derive(Component)]
pub struct OnDeck;

pub const BOX_WIDTH: f32 = 100.;
pub const BOX_HEIGHT: f32 = 100.;
pub const BOX_X: f32 = 650.;
pub const BOX_Y: f32 = 250.;
pub const BOX_THICKNESS: f32 = 4.;

impl Plugin for OnDeckPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(OnDeckBall { balltype: random_ball() })
        .add_systems(Startup, spawn_deck)
        .add_systems(Update, on_deck_ball_change);
    }
}

fn spawn_deck(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    on_deck_ball: Res<OnDeckBall>
    ){
    // Spawn The Box Entity
    let on_deck_entity = commands
        .spawn(TransformBundle::from(Transform::from_xyz(650.0, 250.0, 0.0)))
        /*
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(BOX_WIDTH, BOX_THICKNESS * 2.0)),
                ..default()
            },
            ..default()
        })
        */
        .insert(VisibilityBundle{
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

    commands.entity(on_deck_entity).add_child(on_deck_ball_entity);
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


