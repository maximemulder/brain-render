use wgpu::util::DeviceExt;

use crate::{display_window::DisplayWindow, nifti::AnatomicalAxis, renderer::{Renderer, params::FragmentParams, texture}};

pub fn create_texture_from_nifti_slice(
    renderer: &mut Renderer,
    volume: &ndarray::Array4<f32>,
    window: DisplayWindow,
    axis: AnatomicalAxis,
    index: u32,
    timepoint: usize,
) -> wgpu::BindGroup {
    if renderer.texture_views.is_none() {
        let textures = create_textures(&renderer.device, &renderer.queue, volume);
        let texture_views = create_texture_views(textures);
        renderer.texture_views = Some(texture_views);
    };

    let dims: [usize; 4] = volume.dim().into();

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
    let slice_params = FragmentParams::new(dims, axis, index as usize, window);

    let slice_buffer = renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("slice_params_buffer"),
        contents: bytemuck::cast_slice(&[slice_params]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("nifti_bind_group"),
        layout: &renderer.bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&renderer.texture_views.as_ref().expect("texture view not initialized")[timepoint]),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: slice_buffer.as_entire_binding(),
            },
        ],
    })
}

pub fn create_textures(device: &wgpu::Device, queue: &wgpu::Queue, volume: &ndarray::Array4<f32>) -> Vec<wgpu::Texture> {
    let dims: [usize; 4] = volume.dim().into();
    let [x_size, y_size, z_size, t_size] = dims.map(|x| x as u32);

    let size =  wgpu::Extent3d {
        width: x_size,
        height: y_size,
        depth_or_array_layers: z_size,
    };

    // Create one texture per timepoint
    let textures: Vec<wgpu::Texture> = (0..t_size).map(|_| {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("nifti_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        })
    }).collect();

    for (i, texture) in textures.iter().enumerate() {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(volume.slice(ndarray::s![.., .., .., i]).as_slice_memory_order().expect("Could not slice data")),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dims[0] as u32), // 4 bytes per f32
                rows_per_image: Some(dims[1] as u32),
            },
            size,
        );
    }

    textures
}

pub fn create_texture_views(textures: Vec<wgpu::Texture>) -> Vec<wgpu::TextureView> {
    textures.into_iter()
        .map(|texture| texture.create_view(&wgpu::TextureViewDescriptor::default()))
        .collect()
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
        ],
    })
}
