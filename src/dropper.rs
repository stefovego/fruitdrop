use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::walls::{LEVEL_WIDTH, WALL_THICKNESS};

pub struct DropperPlugin;
pub const DROPPER_SPEED: f32 = 500.0;
pub const DROPPER_WIDTH: f32 = 100.;
pub const DROPPER_HEIGHT: f32 = 60.;

impl Plugin for DropperPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dropper);
        app.add_systems(Update, (dropper_movement, restrict_dropper_movement).chain());
        //app.add_systems(Update, restrict_dropper_movement);
    }
}

#[derive(Component)]
pub struct Dropper;

fn spawn_dropper(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ){
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(DROPPER_WIDTH, DROPPER_HEIGHT)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GOLD)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 1.0)))
        .insert(Dropper);
    }

fn dropper_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut dropper_query: Query<&mut Transform, With<Dropper>>,
    time: Res<Time>
){
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

fn restrict_dropper_movement(mut dropper_query: Query<&mut Transform, With<Dropper>>) {
    let min_x: f32 = -(LEVEL_WIDTH + WALL_THICKNESS - DROPPER_WIDTH) / 2.; 
    let max_x: f32 = (LEVEL_WIDTH + WALL_THICKNESS - DROPPER_WIDTH)/ 2.; 
    if let Ok(mut transform) = dropper_query.get_single_mut() {
        if transform.translation.x < min_x {
            transform.translation.x = min_x;
        }
        if transform.translation.x > max_x {
            transform.translation.x = max_x;
        }
    }

}
