mod browser;
mod nifti_file_worker;
mod nifti_slice;
mod renderer;
mod utils;
mod web;

use nifti::InMemNiftiVolume;
use wasm_bindgen::prelude::*;
use web_sys::{File, Worker};

use crate::nifti_slice::get_slice_from_volume;
use crate::renderer::Renderer;
use crate::web::get_canvas;

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
    let result = web::await_worker_response(&nifti_worker, create_send_file_message()).await.expect("Could not read the worker response");
    let volume = extract_slice(result).expect("Could not read the NIfTI slice");
    log!("NIfTI volume: {:?}", volume);
    let slice = get_slice_from_volume(volume);
    let canvas = get_canvas();
    let mut gfx_state = Renderer::new(canvas, slice).await;
    gfx_state.render();
}

#[wasm_bindgen]
pub async fn read_file(file: File) {
    utils::set_panic_hook();
    nifti_file_worker::read_file(file).await;
}

#[wasm_bindgen]
pub fn send_file() -> JsValue {
    utils::set_panic_hook();
    nifti_file_worker::send_file()
}
