mod browser;
mod nifti_file_worker;
mod nifti_slice;
mod renderer;
mod utils;
mod web;

use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use web_sys::{File, HtmlCanvasElement};

use crate::nifti_slice::Nifti2DSlice;
use crate::renderer::Renderer;

thread_local! {
  static RENDERER: RefCell<Option<Renderer>> = RefCell::new(None);
}

/// Initiate the graphics features.
#[wasm_bindgen]
pub async fn init_graphics(slice_js: JsValue, canvas: HtmlCanvasElement) {
    utils::set_panic_hook();
    // Get the slice property
    let slice = Nifti2DSlice::from_js(&slice_js).expect("Could not re-create slice");

    let initialized = RENDERER.with_borrow(|renderer| renderer.is_some());
    if !initialized {
        let renderer = Renderer::new(canvas).await;
        RENDERER.replace(Some(renderer));
    }

    RENDERER.with_borrow_mut(|renderer| {
        let renderer = renderer.as_mut().expect("renderer not initialized");
        renderer.update_nifti_slice(slice);
        renderer.render();
    });
}

#[wasm_bindgen]
pub async fn read_file(file: File) -> JsValue {
    utils::set_panic_hook();
    let properties = nifti_file_worker::read_file(file).await;
    serde_wasm_bindgen::to_value(&properties).expect("could not serialize nifti file properties")
}

#[wasm_bindgen]
pub fn send_file(js_focal_point: JsValue, js_orientation: JsValue) -> JsValue {
    utils::set_panic_hook();
    let focal_point = serde_wasm_bindgen::from_value(js_focal_point).expect("could not deserialize focal point");
    let orientation = serde_wasm_bindgen::from_value(js_orientation).expect("could not deserialize orientation");
    nifti_file_worker::send_file(focal_point, orientation)
}
