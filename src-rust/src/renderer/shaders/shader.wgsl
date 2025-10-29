struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let pos = array<vec2<f32>, 4>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0,  1.0)
    );

    let tex = array<vec2<f32>, 4>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0)
    );

    var output: VertexOutput;
    output.clip_position = vec4<f32>(pos[vertex_index], 0.0, 1.0);
    output.tex_coords = tex[vertex_index];
    return output;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
@group(0) @binding(2)
var<uniform> window_params: vec2<f32>;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Get the raw pixel intensity from the NIfTI slice.
    let raw_value = textureSample(texture, texture_sampler, input.tex_coords).r;

    // Normalize the value based on window parameters
    let normalized_value = (raw_value - window_params.x) / (window_params.y - window_params.x);

    // Clamp the normalized value into a grayscale value.
    let grayscale_value = clamp(normalized_value, 0.0, 1.0);

    // Return the pixel as an RGBA value.
    return vec4<f32>(grayscale_value, grayscale_value, grayscale_value, 1.0);
}
