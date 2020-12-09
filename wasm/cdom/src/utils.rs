use wasm_bindgen::*;
use web_sys::console;
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub unsafe fn console_log(input: &str) {
    console::log_1(&JsValue::from_str(input))
}

pub unsafe fn rand_betwe(min: f64, max: f64) -> f64 {
    (js_sys::Math::random() * (max - min) + min) as f64
}

pub unsafe fn rand_int(n: i32) -> i32 {
    (js_sys::Math::random() * n as f64) as i32
}