#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils
//#import bevy_pbr::mesh_view_bindings

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
    var uv = coords_to_viewport_uv(position.xy, view.viewport);

    let main = textureSample(texture, our_sampler, uv).rgb;
    let up = textureSample(texture, our_sampler, uv + vec2<f32>(0.0, 0.006)).rgb;
    let down = textureSample(texture, our_sampler, uv - vec2<f32>(0.0, 0.007)).rgb;
    let right = textureSample(texture, our_sampler, uv + vec2<f32>(0.006, 0.0)).rgb;
    let left = textureSample(texture, our_sampler, uv - vec2<f32>(0.006, 0.0)).rgb;

    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        main * 0.2 + up * 0.2 + down * 0.2 + left * 0.2 + right * 0.2,
        (sin(globals.time*1.523) * 0.5 + 0.7)
    );

    return output_color;
}