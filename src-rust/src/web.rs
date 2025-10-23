use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlCanvasElement;

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

pub async fn await_worker_response(worker: &web_sys::Worker, message: JsValue) -> Result<JsValue, JsValue> {
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
