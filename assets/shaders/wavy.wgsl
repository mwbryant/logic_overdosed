#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils
//#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

 fn rotate2d(angle: f32)-> mat2x2<f32>{
    return mat2x2<f32>(cos(angle),-sin(angle),
                sin(angle),cos(angle));
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    var uv = coords_to_viewport_uv(position.xy, view.viewport);

    var sample_uv = uv - vec2<f32>(0.5, 0.5);
    sample_uv = rotate2d(sin(globals.time * 0.06) * 2.3 * 3.1415) * sample_uv;
    //sample_uv = rotate2d(sin(globals.time * 1.153) * 0.2) * sample_uv;
    sample_uv = sample_uv + vec2<f32>(0.5, 0.5);

    uv.y = uv.y + sin(globals.time * 3.9 + uv.x * 103.253) * 0.01;

    uv = uv - vec2<f32>(0.5, 0.5);
    uv = rotate2d(sin(globals.time * 0.17 + 1.032) * 0.12 * 3.1415) * uv;
    uv = uv + vec2<f32>(0.5, 0.5);

    sample_uv.y = sample_uv.y + sin(globals.time * 1.0 + uv.x * 23.253) * 0.01;
    sample_uv.x = sample_uv.x + sin(globals.time * 0.1 + uv.x * 25.253) * 0.01;

    let mix = sin(globals.time * 0.5 + 424.0424) * .25 + 0.75;
    let color = textureSample(texture, our_sampler, uv).rgb * (1.0 - mix)+ 
        textureSample(texture, our_sampler, sample_uv).bgr * mix;

    var output_color = vec4<f32>(
        color,
        //(sin(globals.time*1.123 + 503.2523) * 0.15 + 0.9)
        1.0
    );

    return output_color;
}