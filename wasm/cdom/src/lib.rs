pub mod browser;
pub mod error;
pub mod utils;
pub mod dom;
// pub mod worker;
// pub mod render;
// pub mod ws;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Reflect;
use dom::{doc, win};
use web_sys::{MessageEvent, Event};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[macro_export]
macro_rules! console_warn {
    ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(js="testEcho")]
pub fn echo(input: String) -> JsValue {
    let echo = format!("Hello, {}", input);
    JsValue::from_str(&echo)
}

#[wasm_bindgen(start)]
pub fn init() {
    let onmessage_callback =
    Closure::wrap(
        Box::new(move |ev: MessageEvent| match ev.data().as_string() {
            Some(message) => {
                unsafe{console_warn!("{:?}", message)};
            }
            None => {}
        }) as Box<dyn FnMut(MessageEvent)>,
    );

}
