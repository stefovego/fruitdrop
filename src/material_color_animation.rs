use std::any::TypeId;

use bevy::{
    animation::{animated_field, AnimationEvaluationError, AnimationTarget, AnimationTargetId},
    prelude::*,
};

// https://docs.rs/bevy/latest/bevy/prelude/trait.AnimatableProperty.html
// https://docs.rs/bevy/latest/src/animated_ui/animated_ui.rs.html#169

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct MaterialColorAnimationSet;

pub struct MaterialColorAnimation;

impl Plugin for MaterialColorAnimation {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_color.in_set(MaterialColorAnimationSet))
            .register_type::<AnimatedColor>()
            .register_type::<CurrentColor>()
            .add_observer(component_added)
            .add_observer(animation_end);
    }
}

#[derive(Component, Reflect, Debug)]
pub struct AnimatedColor {
    pub color: Color,
    pub time_seconds: f32,
}

#[derive(Component, Event, Clone)]
struct ComponentInserted;

#[derive(Event, Clone)]
struct AnimationEnd;

fn component_added(
    trigger: Trigger<OnAdd, AnimatedColor>,
    mut commands: Commands,
    name_query: Query<&Name>,
    animated_color_query: Query<&AnimatedColor>,
    color_material_query: Query<&MeshMaterial2d<ColorMaterial>>,
    color_material_asset: Res<Assets<ColorMaterial>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
) {
    let entity = trigger.target();

    // Get entities initial color
    let MeshMaterial2d(color_material_handle) = color_material_query.get(entity).unwrap();
    let ColorMaterial {
        color: initial_color,
        ..
    } = color_material_asset.get(color_material_handle).unwrap();

    commands.entity(entity).insert(CurrentColor {
        color: initial_color.to_srgba(),
    });
    // Get entities final color
    let AnimatedColor {
        color: final_color,
        time_seconds,
    } = *animated_color_query.get(entity).unwrap();

    // Get the entities animation_graph
    // Do the animation stuff

    let animation_target_name = name_query.get(entity).unwrap();
    let animation_target_id = AnimationTargetId::from_name(&animation_target_name);

    let mut animation_clip = AnimationClip::default();

    animation_clip.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            ColorProperty,
            AnimatableKeyframeCurve::new([
                (0.0, initial_color.to_srgba()),
                (time_seconds, final_color.to_srgba()),
            ])
            .expect("should be able to create curve"),
        ),
    );
    animation_clip.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::scale),
            AnimatableKeyframeCurve::new(
                [0.0, time_seconds]
                    .into_iter()
                    .zip([Vec3::splat(0.5), Vec3::splat(1.0)]),
            )
            .expect("Ball Animation Fail"),
        ),
    );
    animation_clip.add_event(time_seconds, AnimationEnd);

    // Save the animation clip as an asset.
    let animation_clip_handle = animation_clips.add(animation_clip);

    // Create an animation graph with that clip.

    let (animation_graph, animation_node_index) = AnimationGraph::from_clip(animation_clip_handle);
    let animation_graph_handle = animation_graphs.add(animation_graph);

    let mut animation_player = AnimationPlayer::default();

    animation_player.play(animation_node_index);
    commands
        .entity(entity)
        .insert(AnimationTarget {
            id: animation_target_id,
            player: entity,
        })
        .insert(AnimationGraphHandle(animation_graph_handle))
        .insert(animation_player);
}

fn animation_end(trigger: Trigger<AnimationEnd>, mut commands: Commands) {
    let entity = trigger.target();
    commands
        .entity(entity)
        .remove::<AnimatedColor>()
        .remove::<CurrentColor>()
        .remove::<AnimationTarget>()
        .remove::<AnimationPlayer>()
        .remove::<AnimationGraphHandle>();
}

fn update_color(
    mut commands: Commands,
    mut color_material_asset: ResMut<Assets<ColorMaterial>>,
    current_color_query: Query<(Entity, &CurrentColor), With<CurrentColor>>,
) {
    for (entity, current_color_component) in &current_color_query {
        commands
            .entity(entity)
            .insert(MeshMaterial2d(color_material_asset.add(ColorMaterial {
                color: current_color_component.color.into(),
                ..default()
            })));
    }
}

#[derive(Component, Reflect)]
struct CurrentColor {
    color: Srgba,
}

#[derive(Clone)]
struct ColorProperty;

impl AnimatableProperty for ColorProperty {
    type Property = Srgba;
    fn get_mut<'a>(
        &self,
        entity: &'a mut bevy::animation::AnimationEntityMut,
    ) -> std::result::Result<&'a mut Self::Property, bevy::animation::AnimationEvaluationError>
    {
        let component = entity
            .get_mut::<CurrentColor>()
            .ok_or(AnimationEvaluationError::ComponentNotPresent(TypeId::of::<
                CurrentColor,
            >(
            )))?
            .into_inner();
        Ok(&mut component.color)
    }

    fn evaluator_id(&self) -> EvaluatorId {
        EvaluatorId::Type(TypeId::of::<Self>())
    }
}
