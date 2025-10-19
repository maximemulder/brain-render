mod browser;
mod nifti_slice;
mod renderer;
mod utils;
mod web;

use std::sync::Mutex;

use nifti::{InMemNiftiVolume, NiftiObject, ReaderStreamedOptions};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, Worker};
use js_sys::Promise;
use crate::nifti_slice::get_slice_from_volume;
use crate::renderer::Renderer;
use crate::web::get_canvas;

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

/// Initiate the graphics features.
#[wasm_bindgen]
pub async fn init_graphics(nifti_worker: Worker) {
    utils::set_panic_hook();
    log!("NIfTI slice is set: {}", NIFTI_SLICE.lock().unwrap().is_some());
    let result = await_worker_response(&nifti_worker, create_send_file_message()).await.expect("Could not read the worker response");
    let volume = extract_slice(result).expect("Could not read the NIfTI slice");
    log!("NIfTI volume: {:?}", volume);
    let slice = get_slice_from_volume(volume);
    let canvas = get_canvas();
    let mut gfx_state = Renderer::new(canvas, slice).await;
    gfx_state.render();
}
