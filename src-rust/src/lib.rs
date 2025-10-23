mod browser;
mod nifti_file_worker;
mod nifti_slice;
mod renderer;
mod utils;
mod web;

use wasm_bindgen::prelude::*;
use web_sys::File;

use crate::nifti_slice::Nifti2DSlice;
use crate::renderer::Renderer;
use crate::web::get_canvas;

/// Initiate the graphics features.
#[wasm_bindgen]
pub async fn init_graphics(slice_js: JsValue) {
    utils::set_panic_hook();
    // Get the slice property
    let slice = Nifti2DSlice::from_js(&slice_js).expect("Could not re-create slice");
    let canvas = get_canvas();
    let mut gfx_state = Renderer::new(canvas, slice).await;
    gfx_state.render();
}

#[wasm_bindgen]
pub async fn read_file(file: File) -> JsValue {
    utils::set_panic_hook();
    let properties = nifti_file_worker::read_file(file).await;
    serde_wasm_bindgen::to_value(&properties).expect("could not serialize nifti file properties")
}

#[wasm_bindgen]
pub fn send_file(js_focal_point: JsValue) -> JsValue {
    utils::set_panic_hook();
    nifti_file_worker::send_file(serde_wasm_bindgen::from_value(js_focal_point).expect("could not deserialize focal point"))
}
