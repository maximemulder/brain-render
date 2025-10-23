mod browser;
mod nifti_file_worker;
mod nifti_slice;
mod renderer;
mod utils;
mod web;

use wasm_bindgen::prelude::*;
use web_sys::{File, Worker};

use crate::nifti_slice::Nifti2DSlice;
use crate::renderer::Renderer;
use crate::web::get_canvas;

fn create_send_file_message() -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"action".into(), &"send-file".into()).unwrap();
    obj.into()
}

/// Initiate the graphics features.
#[wasm_bindgen]
pub async fn init_graphics(nifti_worker: Worker) {
    utils::set_panic_hook();
    let result = web::await_worker_response(&nifti_worker, create_send_file_message()).await.expect("Could not read the worker response");
    let obj = result.dyn_into::<js_sys::Object>().expect("a");

    // Get the slice property
    let slice_js = js_sys::Reflect::get(&obj, &"slice".into()).expect("b");
    let slice = Nifti2DSlice::from_js(&slice_js).expect("Could not re-create slice");
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
