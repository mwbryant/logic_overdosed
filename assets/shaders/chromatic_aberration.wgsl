// FROM: https://github.com/bevyengine/bevy/blob/v0.10.1/assets/shaders/custom_material_chromatic_aberration.wgsl
#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = coords_to_viewport_uv(position.xy, view.viewport);
    let offset_strength = 0.02;

    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv + vec2<f32>(offset_strength* sin(globals.time * 0.11 - 2.54), -offset_strength* sin(globals.time * 0.33 - 34.54))).r,
        textureSample(texture, our_sampler, uv + vec2<f32>(-offset_strength * sin(globals.time * 0.33 - 324.54), 0.0)).g,
        textureSample(texture, our_sampler, uv + vec2<f32>(0.0, offset_strength * sin(globals.time * 0.75))).b,
        0.7
    );

    return output_color;
}