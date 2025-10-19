use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub(crate) fn console_log(s: &str);
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        crate::browser::console_log(&format!($($arg)*))
    };
}
