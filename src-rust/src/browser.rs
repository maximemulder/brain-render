use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub(crate) fn console_log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    pub(crate) fn console_debug(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub(crate) fn console_error(s: &str);
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        crate::browser::console_log(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        crate::browser::console_debug(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        crate::browser::console_error(&format!($($arg)*))
    };
}
