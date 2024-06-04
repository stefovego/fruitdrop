#import bevy_pbr::{
  forward_io::VertexOutput,
   // mesh_view_bindings::globals,
  }
#import noisy_bevy::simplex_noise_3d
#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals;

struct CustomMaterial {
    color: vec4<f32>,
}

@group(2) @binding(0) var<uniform> material: CustomMaterial;
@group(2) @binding(1) var color_texture: texture_2d<f32>;
// @group(1) @binding(2) var color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let u = mesh.uv * 3.0;
    let p = vec3(u.x, u.y, globals.time);
    //let p = vec3(mesh.uv.x, mesh.uv.y, globals.time);
    // let p = vec3(mesh.uv.x, mesh.uv.y, 5.0);
    let n = simplex_noise_3d(p) * 0.25 + 0.75;

    return vec4<f32>(material.color * n);
    //return vec4<f32>(mesh.uv.x, mesh.uv.y, 0.0, 1.0);
}
