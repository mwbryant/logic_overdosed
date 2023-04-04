#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var distortion : texture_2d<f32>;
@group(1) @binding(3)
var distortion_sampler: sampler;
@group(1) @binding(4)
var<uniform> offset_strength: f32;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    var uv = coords_to_viewport_uv(position.xy, view.viewport);

    let uv_offset = textureSample(distortion, distortion_sampler, uv).rg;
    uv = uv + offset_strength * (uv_offset * vec2<f32>(2.0, 2.0) - vec2<f32>(1.0, 1.0));

    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv).r,
        textureSample(texture, our_sampler, uv).g,
        textureSample(texture, our_sampler, uv).b,
        //0.7
        1.0
    );

    return output_color;
}