mod browser;
mod utils;

use std::borrow::Cow;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, File};

/// Get the HTML canvas element on which to render from the document.
pub fn get_canvas() -> HtmlCanvasElement {
    web_sys::window()
        .expect("Window not found.")
        .document()
        .expect("Document not found.")
        .get_element_by_id("canvas")
        .expect("Canvas not found.")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Element is not a canvas.")
}

/* use wasm_bindgen_file_reader::WebSysFile;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom; */
use nifti::{NiftiObject, ReaderStreamedOptions};
use tempfile::tempdir;

#[wasm_bindgen]
pub async fn read_file(file: File) {
    browser::log("LOG 1");
    std::fs::write("test.txt", [1,2,3,4]).expect("Could not write file.");
    // let temp_dir = tempdir().expect("Cannot create temporary directory");
    browser::log("LOG 2");
    //let dest_path = temp_dir.path().join(file.name());
    browser::log("LOG 3");
    //let a = ReaderStreamedOptions::new().read_file(dest_path).expect("Cannot read NIfTI");
    browser::log("LOG 4");
    //browser::log(&format!("{}", String::from_utf8(a.header().descrip.clone()).expect("Cannot read description as a string.")));
    browser::log("LOG 5");
// Read one byte from the file at a given offset.
// #[wasm_bindgen]
// pub fn read_at_offset_sync(file: web_sys::File, offset: u64) -> u8 {
//     let mut wf = WebSysFile::new(file);
//
//     // Now we can seek as if this was a real file
//     wf.seek(SeekFrom::Start(offset))
//         .expect("failed to seek to offset");
//
//     // Use 1-byte buffer because we only want to read one byte
//     let mut buf = [0];
//
//     // The Read API works as with real files
//     wf.read_exact(&mut buf).expect("failed to read bytes");
//
//     buf[0]
// }
}

/// Initiate the graphics features.
#[wasm_bindgen]
pub async fn init_graphics() {
    utils::set_panic_hook();
    let canvas = get_canvas();
    let mut gfx_state = GfxState::new(canvas).await;
    gfx_state.render();
}

struct GfxState {
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  render_pipeline: wgpu::RenderPipeline,
}

impl GfxState {
  async fn new(canvas: HtmlCanvasElement) -> Self {
    let x = canvas.width();
    let y = canvas.height();
    let instance = wgpu::Instance::default();
    let surface = wgpu::SurfaceTarget::Canvas(canvas);
    let surface = instance.create_surface(surface).expect("Failed to create surface");

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::Performance,
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            trace: wgpu::Trace::Off,
        })
        .await
        .expect("Failed to create device");

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                r#"
                @vertex
                fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                    let x = f32(i32(in_vertex_index) - 1);
                    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
                    return vec4<f32>(x, y, 0.0, 1.0);
                }

                @fragment
                fn fs_main() -> @location(0) vec4<f32> {
                    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
                }
                "#,
            )),
        });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(swapchain_format.into())],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: x,
        height: y,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 2,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    Self {
        surface,
        device,
        queue,
        config,
        render_pipeline,
    }
  }

  fn render(&mut self) {
    let frame = self
        .surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1);
    }

    self.queue.submit(Some(encoder.finish()));
    frame.present();
  }
}
