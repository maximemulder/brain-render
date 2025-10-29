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
var volume_texture: texture_3d<f32>;
@group(0) @binding(1)
var volume_sampler: sampler;
@group(0) @binding(2)
var<uniform> window_params: vec2<f32>;
@group(0) @binding(3)
var<uniform> slice_params: SliceParams;

struct SliceParams {
    slice_index: f32,
    axis: u32,
    volume_dims: vec3<f32>,
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let volume_coords = calculate_slice_coords(input.tex_coords, slice_params);

    // Sample from 3D texture
    let raw_value = textureSample(volume_texture, volume_sampler, volume_coords).r;

    // Apply windowing (same as before)
    let normalized_value = (raw_value - window_params.x) / (window_params.y - window_params.x);
    let grayscale_value = clamp(normalized_value, 0.0, 1.0);

    return vec4<f32>(grayscale_value, grayscale_value, grayscale_value, 1.0);
}

fn calculate_slice_coords(tex_coords: vec2<f32>, params: SliceParams) -> vec3<f32> {
    var coords = vec3<f32>(tex_coords, 0.0);

    switch params.axis {
        case 0: { // Axial (XY plane at Z)
            coords.x = tex_coords.x;
            coords.y = tex_coords.y;
            coords.z = params.slice_index;
        }
        case 1: { // Coronal (XZ plane at Y)
            coords.x = tex_coords.x;
            coords.y = params.slice_index;
            coords.z = tex_coords.y;
        }
        case 2: { // Sagittal (YZ plane at X)
            coords.x = params.slice_index;
            coords.y = tex_coords.x;
            coords.z = tex_coords.y;
        }
        default: {
            // Fallback to axial
            coords.x = tex_coords.x;
            coords.y = tex_coords.y;
            coords.z = params.slice_index;
        }
    }

    // Normalize coordinates to [0, 1] range
    return coords;
}
