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

struct FragmentParams {
    volume_dims: vec3<f32>,
    polarity: u32,
    window: vec2<f32>,
    axis: u32,
    slice_index: f32,
    rotation: u32,
}

@group(0) @binding(0)
var volume_texture: texture_3d<f32>;
@group(0) @binding(1)
var volume_sampler: sampler;
@group(0) @binding(2)
var<uniform> params: FragmentParams;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Get the voxel coordinates with rotation applied
    let voxel_coords = get_voxel_coords(input.tex_coords, params);

    // Get the raw intensity from the volume.
    let raw_value = textureSample(volume_texture, volume_sampler, voxel_coords).r;

    // Normalize the value based on window parameters.
    let normalized_value = (raw_value - params.window.x) / (params.window.y - params.window.x);

    // Clamp the normalized value into a grayscale value.
    let grayscale_value = clamp(normalized_value, 0.0, 1.0);

    // Invert the color if specified in the parameters.
    let final_value = select(grayscale_value, 1.0 - grayscale_value, params.polarity == 1);

    // Return the pixel as an RGBA value.
    return vec4<f32>(final_value, final_value, final_value, 1.0);
}

fn get_voxel_coords(tex_coords: vec2<f32>, params: FragmentParams) -> vec3<f32> {
    switch params.axis {
        case 0: { // Axial (XY plane at Z)
            let rotated_uv = rotate_slice_coords(tex_coords, params.rotation, params.volume_dims.xy);
            return vec3<f32>(rotated_uv.x, rotated_uv.y, params.slice_index);
        }
        case 1: { // Coronal (XZ plane at Y)
            let rotated_uv = rotate_slice_coords(tex_coords, params.rotation, params.volume_dims.xz);
            return vec3<f32>(rotated_uv.x, params.slice_index, rotated_uv.y);
        }
        case 2: { // Sagittal (YZ plane at X)
            let rotated_uv = rotate_slice_coords(tex_coords, params.rotation, params.volume_dims.yz);
            return vec3<f32>(params.slice_index, rotated_uv.x, rotated_uv.y);
        }
        default: { // Default to axial
            let rotated_uv = rotate_slice_coords(tex_coords, params.rotation, params.volume_dims.xy);
            return vec3<f32>(rotated_uv.x, rotated_uv.y, params.slice_index);
        }
    }
}

fn rotate_slice_coords(tex_coords: vec2<f32>, rotation: u32, plane_dims: vec2<f32>) -> vec2<f32> {
    let voxel_pos = tex_coords * plane_dims;

    switch rotation {
        case 0: { // No rotation
            return tex_coords;
        }
        case 1: { // 90 degrees clockwise
            let rotated_voxel = vec2<f32>(voxel_pos.y, plane_dims.x - voxel_pos.x);
            return rotated_voxel / vec2<f32>(plane_dims.y, plane_dims.x);
        }
        case 2: { // 180 degrees
            let rotated_voxel = vec2<f32>(plane_dims.x - voxel_pos.x, plane_dims.y - voxel_pos.y);
            return rotated_voxel / plane_dims;
        }
        case 3: { // 270 degrees clockwise
            let rotated_voxel = vec2<f32>(plane_dims.y - voxel_pos.y, voxel_pos.x);
            return rotated_voxel / vec2<f32>(plane_dims.y, plane_dims.x);
        }
        default: { // Default to no rotation
            return tex_coords;
        }
    }
}
