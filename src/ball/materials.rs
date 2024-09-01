use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BallMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material2d for BallMaterial {
    //fn fragment_shader() -> ShaderRef {
    //"shaders/custom_material.wgsl".into()
}
