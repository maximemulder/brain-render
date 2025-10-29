use crate::{Nifti2DSlice, nifti_slice::DisplayWindow, renderer::Renderer};

// TODO: Move slicing to the GPU.
fn slice_to_bytes(nifti_slice: &Nifti2DSlice) -> Vec<u8> {
    // Try the fast path first (contiguous data)
    if let Some(slice) = nifti_slice.data.as_slice() {
        return bytemuck::cast_slice::<f32, u8>(slice).to_vec();
    }

    // Fallback: copy and convert
    nifti_slice.data.iter()
        .flat_map(|&value| bytemuck::bytes_of(&value).to_vec())
        .collect()
}

pub fn create_texture_from_nifti_slice(renderer: &Renderer, nifti_slice: Nifti2DSlice, window: DisplayWindow) -> wgpu::BindGroup {
    let texture_size = wgpu::Extent3d {
        width:  nifti_slice.width as u32,
        height: nifti_slice.height as u32,
        depth_or_array_layers: 1,
    };

    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("nifti_texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R32Float, // Single channel f32
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let bytes = slice_to_bytes(&nifti_slice);

    renderer.queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &bytes,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * nifti_slice.width as u32), // 4 bytes per f32
            rows_per_image: Some(nifti_slice.height as u32),
        },
        texture_size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let window_buffer = create_window_buffer(&renderer.device, &renderer.queue, window);

    renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("nifti_bind_group"),
        layout: &renderer.bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: window_buffer.as_entire_binding(),
            },
        ],
    })
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
                    view_dimension: wgpu::TextureViewDimension::D2,
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
