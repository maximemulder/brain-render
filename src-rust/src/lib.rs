mod browser;
mod nifti_file_worker;
mod nifti_slice;
mod renderer;
mod utils;
mod web;

use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use web_sys::{File, OffscreenCanvas};

use crate::nifti_file_worker::STATE;
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
pub async fn render_slice(js_axis: JsValue, js_coordinate: JsValue, js_window: JsValue) {
    utils::set_panic_hook();
    // Get the slice property
    let axis = serde_wasm_bindgen::from_value(js_axis).expect("could not deserialize axis");
    let coordinate = serde_wasm_bindgen::from_value(js_coordinate).expect("could not deserialize coordinate");
    let window: nifti_slice::DisplayWindow = serde_wasm_bindgen::from_value(js_window).expect("could not deserialize window");

    let slice = STATE.with_borrow(|state| {
        state.as_ref().expect("nifti file missing").get_slice(coordinate, axis)
    });

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
