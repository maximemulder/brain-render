mod browser;
mod utils;

use std::{borrow::Cow, sync::Mutex};

use nifti::{InMemNiftiVolume, NiftiObject, ReaderStreamedOptions};
use wasm_bindgen::{convert::FromWasmAbi, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, HtmlCanvasElement, Worker};
use js_sys::Promise;

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

// NOTE: A web file cannot be sent between threads.
// In an ideal architecture, the file is kept in the web worker, and the main thread asynchronously
// calls the web worker whenever it needs to read the file.
// static NIFTI: Mutex<Option<GenericNiftiObject<StreamedNiftiVolume<Either<BufReader<WebSysFile>, GzDecoder<BufReader<WebSysFile>>>>>>> = Mutex::new(None);

static NIFTI_SLICE: Mutex<Option<InMemNiftiVolume>> = Mutex::new(None);

async fn await_worker_response(worker: &web_sys::Worker, message: JsValue) -> Result<JsValue, JsValue> {
    let promise = Promise::new(&mut |resolve, _reject| {
        let closure = Closure::once(move |event: web_sys::MessageEvent| {
            let _ = resolve.call1(&JsValue::NULL, &event.data());
        });
        worker.set_onmessage(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    });

    worker.post_message(&message)?;
    JsFuture::from(promise).await
}

#[wasm_bindgen]
pub async fn read_file(file: File) {
    utils::set_panic_hook();
    log!("Starting to read the NIfTI file.");
    let nifti = ReaderStreamedOptions::new().read_web_file(file).expect("Cannot read NIfTI");
    let mut volume = nifti.into_volume();
    match volume.read_slice() {
        Ok(slice) => {
            {
                let mut guard = NIFTI_SLICE.lock().unwrap();
                *guard = Some(slice);
                log!("Successfully read NIfTI slice, slices left: {}", volume.slices_left());
                log!("Guard is some: {}", guard.is_some());
            }
            log!("Mutex is some: {}", NIFTI_SLICE.lock().unwrap().is_some());
        },
        Err(error) => {
            log!("Error while reading NIfTI slice: {:?}", error);
        },
    }
}

#[wasm_bindgen]
pub fn send_file() -> JsValue {
    // NIFTI_SLICE.lock().unwrap().clone().unwrap()
    let slice: &Option<InMemNiftiVolume> = &NIFTI_SLICE.lock().unwrap();
    match slice {
        Some(slice) => serde_wasm_bindgen::to_value(slice).unwrap(),
        None => JsValue::NULL,
    }
}

fn create_send_file_message() -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"action".into(), &"send-file".into()).unwrap();
    obj.into()
}

// To extract from JsValue:
fn extract_slice(js_value: JsValue) -> Result<InMemNiftiVolume, JsValue> {
    // Use serde-like approach if available, or:
    let obj = js_value.dyn_into::<js_sys::Object>()?;

    // Get the slice property
    let slice_js = js_sys::Reflect::get(&obj, &"slice".into())?;

    Ok(serde_wasm_bindgen::from_value(slice_js).expect("damn"))
}

use nifti::NiftiVolume;

/// Initiate the graphics features.
#[wasm_bindgen]
pub async fn init_graphics(nifti_worker: Worker) {
    utils::set_panic_hook();
    log!("NIfTI slice is set: {}", NIFTI_SLICE.lock().unwrap().is_some());
    let result = await_worker_response(&nifti_worker, create_send_file_message()).await.expect("Could not read the worker response");
    let slice = extract_slice(result).expect("Could not read the NIfTI slice");
    log!("{:?}", slice);
    /* let total_voxels
        = slice.dim()[1] // x
        * slice.dim()[2] // y
        * slice.dim()[3]; */
    let x = slice.dim()[1];
    let y = slice.dim()[2];
    let float_data: Vec<f32> = slice.raw_data().chunks(4)
        .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();
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

fn create_texture_from_f32_data(device: &wgpu::Device, queue: &wgpu::Queue, data: &[f32], width: u32, height: u32) -> wgpu::Texture {
    // Normalize f32 data to [0, 1] and convert to RGBA
    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);

    for &value in data {
        // Normalize your f32 data to [0, 1] range
        let normalized = (value - f32::MIN) / (f32::MAX - f32::MIN); // Adjust min/max as needed

        // Convert to grayscale RGBA
        rgba_data.extend_from_slice(&[
            (normalized * 255.0) as u8,
            (normalized * 255.0) as u8,
            (normalized * 255.0) as u8,
            255, // Alpha
        ]);
    }

    let texture_size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("NIfTI Texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: Default::default(),
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba_data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        texture_size,
    );

    texture
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
