mod browser;
mod nifti_file_worker;
mod nifti_slice;
mod renderer;
mod utils;
mod web;

use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use web_sys::{File, OffscreenCanvas};

use crate::nifti_slice::Nifti2DSlice;
use crate::renderer::Renderer;

thread_local! {
  static RENDERER: RefCell<Option<Renderer>> = RefCell::new(None);
}

/// Initiate the renderer.
#[wasm_bindgen]
pub async fn init_renderer(canvas: OffscreenCanvas) {
    utils::set_panic_hook();
    let renderer = Renderer::new(canvas).await;
    RENDERER.replace(Some(renderer));
}

/// Initiate the graphics features.
#[wasm_bindgen]
pub async fn render_slice(js_slice: JsValue, js_window: JsValue) {
    utils::set_panic_hook();
    // Get the slice property
    let slice = Nifti2DSlice::from_js(&js_slice).expect("could not deserialize slice");
    let window = serde_wasm_bindgen::from_value(js_window).expect("could not deserialize window");

    RENDERER.with_borrow_mut(|renderer| {
        let Some(renderer) = renderer.as_mut() else {
            log!("renderer not initialized yet");
            return;
        };

        renderer.update_nifti_slice(slice, window);
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
pub fn send_file(js_axis: JsValue, js_coordinate: JsValue, ) -> JsValue {
    utils::set_panic_hook();
    let coordinate = serde_wasm_bindgen::from_value(js_coordinate).expect("could not deserialize focal point");
    let axis = serde_wasm_bindgen::from_value(js_axis).expect("could not deserialize axis");
    nifti_file_worker::send_file(axis, coordinate)
}
