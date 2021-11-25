#[macro_export]
macro_rules! log {
    ($x:expr) => {
        web_sys::console::log(&js_sys::Array::from(&wasm_bindgen::JsValue::from_str($x)));
    };
}
