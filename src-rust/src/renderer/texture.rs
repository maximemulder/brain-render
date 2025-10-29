use wgpu::util::DeviceExt;

use crate::{Nifti2DSlice, nifti_file_worker::AnatomicalAxis, nifti_slice::DisplayWindow, renderer::Renderer};

pub fn create_texture_from_nifti_slice(
    renderer: &mut Renderer,
    volume: &ndarray::Array3<f32>,
    window: DisplayWindow,
    axis: AnatomicalAxis,
    index: u32,
) -> wgpu::BindGroup {
    if renderer.texture_view.is_none() {
        let texture_view = create_texture_view(&renderer.device, &renderer.queue, volume);
        renderer.texture_view = Some(texture_view);
    };

    let dims: [usize; 3] = volume.dim().into();

    let sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    // Create slice parameters buffer
    let slice_params = SliceParams::new(index as usize, axis, dims);

    let slice_buffer = renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("slice_params_buffer"),
        contents: bytemuck::cast_slice(&[slice_params]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let window_buffer = create_window_buffer(&renderer.device, &renderer.queue, window);

    renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("nifti_bind_group"),
        layout: &renderer.bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(renderer.texture_view.as_ref().expect("texture view not initialized")),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: window_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: slice_buffer.as_entire_binding(),
            },
        ],
    })
}

pub fn create_texture_view(device: &wgpu::Device, queue: &wgpu::Queue, volume: &ndarray::Array3<f32>) -> wgpu::TextureView {
    let volume_data = volume.as_slice_memory_order().expect("could not slice volume");
    let dims: [usize; 3] = volume.dim().into();

    let texture_size = wgpu::Extent3d {
        width: dims[0] as u32,
        height: dims[1] as u32,
        depth_or_array_layers: dims[2] as u32,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("nifti_texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D3,
        format: wgpu::TextureFormat::R32Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    // Upload entire volume
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        bytemuck::cast_slice(volume_data),
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * dims[0] as u32), // 4 bytes per f32
            rows_per_image: Some(dims[1] as u32),
        },
        texture_size,
    );

    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("texture_bind_group_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D3,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    })
}

pub fn create_window_buffer(device: &wgpu::Device, queue: &wgpu::Queue, window: DisplayWindow) -> wgpu::Buffer {
    // Create the window uniform buffer.
    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("window_uniform_buffer"),
        size: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress, // min and max values
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let window_params = [window.min(), window.max()];
    queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&window_params));
    uniform_buffer
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SliceParams {
    pub slice_index: f32,
    pub axis: u32,
    pub volume_dims: [f32; 3],
    pub padding: [u32; 3],
}

impl SliceParams {
    pub fn new(slice_index: usize, axis: AnatomicalAxis, volume_dims: [usize; 3]) -> Self {
        let slice_axis_dim = match axis {
            AnatomicalAxis::Axial => volume_dims[2],
            AnatomicalAxis::Coronal => volume_dims[1],
            AnatomicalAxis::Sagittal => volume_dims[0],
        };

        let normalized_slice = if slice_axis_dim > 1 {
            slice_index as f32 / (slice_axis_dim - 1) as f32
        } else {
            0.0
        };

        Self {
            slice_index: normalized_slice,
            axis: axis as u32,
            volume_dims: [
                volume_dims[0] as f32,
                volume_dims[1] as f32,
                volume_dims[2] as f32,
            ],
            padding: [0; 3], // padding to make it vec4 in shader
        }
    }
}
