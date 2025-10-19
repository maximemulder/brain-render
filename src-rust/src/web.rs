use wasm_bindgen::JsCast;
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
