use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);
}
