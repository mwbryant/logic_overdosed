#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils
//#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var distortion : texture_2d<f32>;
@group(1) @binding(3)
var distortion_sampler: sampler;

 fn rotate2d(angle: f32)-> mat2x2<f32>{
    return mat2x2<f32>(cos(angle),-sin(angle),
                sin(angle),cos(angle));
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    //FIXME magic number here because wasm (release 0.2)
    /*
    Caused by:
        In Device::create_render_pipeline
        note: label = `transparent_mesh2d_pipeline`
        In the provided shader, the type given for group 1 binding 4 has a size of 4. As the device does not support `DownlevelFlags::BUFFER_BINDINGS_NOT_16_BYTE_ALIGNED`, the type must have a size that is a multiple of 16 bytes.
    */
    let offset_strength = 0.045;
    // Get screen position with coordinates from 0 to 1
    var uv = coords_to_viewport_uv(position.xy, view.viewport);

    var sample_uv = uv - vec2<f32>(0.5, 0.5);
    sample_uv = rotate2d(sin(globals.time * 1.153) * 0.3) * sample_uv;
    sample_uv = sample_uv + vec2<f32>(0.5, 0.5);

    let uv_offset = textureSample(distortion, distortion_sampler, sample_uv).rg;
    uv = uv + 
     sin(globals.time * 1.342 + 1.542) * offset_strength * (uv_offset * vec2<f32>(2.0, 2.0) - vec2<f32>(1.0, 1.0));

    uv = uv - vec2<f32>(0.5, 0.5);
    uv = rotate2d(sin(globals.time * 0.65367) * 0.05) * uv;
    uv = uv + vec2<f32>(0.5, 0.5);

    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv).rgb,
        (sin(globals.time*1.123) * 0.25 + 0.7)
        //1.0
    );

    return output_color;
}