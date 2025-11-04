mod browser;
mod display_window;
mod nifti;
mod nifti_reader;
mod renderer;
mod utils;

use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use web_sys::{File, OffscreenCanvas};

use crate::{nifti::Nifti, renderer::Renderer};

thread_local! {
    static RENDERER: RefCell<Option<Renderer>> = RefCell::new(None);

   static NIFTI: RefCell<Option<Nifti>> = RefCell::new(None);
}

/// Read a NIfTI file.
#[wasm_bindgen(js_name = readFile)]
pub async fn read_file(file: File) -> JsValue {
    utils::set_panic_hook();
    let nifti = nifti_reader::read_nifti_file(file).await;
    let properties = nifti.get_properties();
    NIFTI.replace(Some(nifti));
    serde_wasm_bindgen::to_value(&properties).expect("could not serialize nifti file properties")
}

/// Initiate the renderer.
#[wasm_bindgen(js_name = initRenderer)]
pub async fn init_renderer(canvas: OffscreenCanvas) -> JsValue {
    utils::set_panic_hook();
    match Renderer::new(canvas).await {
        Ok(renderer) => {
            RENDERER.replace(Some(renderer));
            JsValue::NULL
        }
        Err(error) => {
            crate::error!("[renderer] {}", error);
            error.into()
        }
    }
}

/// Render a slice.
#[wasm_bindgen(js_name = renderSlice)]
pub async fn render_slice(js_axis: JsValue, js_coordinate: JsValue, js_timepoint: JsValue, js_window: JsValue, js_rotation: JsValue) {
    utils::set_panic_hook();
    // Get the slice property
    let axis = serde_wasm_bindgen::from_value(js_axis).expect("could not deserialize axis");
    let coordinate = serde_wasm_bindgen::from_value(js_coordinate).expect("could not deserialize coordinate");
    let timepoint = serde_wasm_bindgen::from_value(js_timepoint).expect("could not deserialize timepoint");
    let rotation = serde_wasm_bindgen::from_value(js_rotation).expect("could not deserialize rotation");
    let window: display_window::DisplayWindow = serde_wasm_bindgen::from_value(js_window).expect("could not deserialize window");

    RENDERER.with_borrow_mut(|renderer| {
        let Some(renderer) = renderer.as_mut() else {
            log!("renderer not initialized yet");
            return;
        };

        NIFTI.with_borrow(|state| {
            let state = state.as_ref().expect("volume not initialized");
            renderer.update_nifti_slice(&state.volume, window, coordinate, timepoint, axis, rotation);
        });
        renderer.render();
    });
}
