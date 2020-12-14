pub mod utils;
pub mod event;
pub mod dom;
pub mod store;

use std::f64;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, HtmlCanvasElement, Element};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
macro_rules! console_warn {
    ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    unsafe {console_log!("Hello, blog!")};
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = dom::win();
    let document = dom::doc();
    let body = dom::body();

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    let h1 = document.create_element("h1")?;
    val.set_inner_html("Hello from Rust!");
    h1.set_inner_html("clp.is");
    //val.set_inner_html(std::thread::spawn(move ||get_github()).join().unwrap());

    body.append_child(&val)?;
    body.append_child(&h1)?;
    body.append_child(&val)?;
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 100.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
    let button: Element = document.create_element("button")
        .unwrap();
    button
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_inner_text("Trigger");
    body
        .dyn_into::<web_sys::Node>()
        .unwrap()
        .append_child(&button)
        .unwrap();

  // Create a Javascript closure that will trigger an alert
    let cb =
        Closure::wrap(Box::new(|| {
            web_sys::window()
                .unwrap()
                .alert_with_message("You hit the button!")
                .unwrap();
        }) as Box<dyn FnMut()>);
    button
        .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
        .unwrap();

    cb.forget();
    Ok(())
}

// Get response from weather api
async fn get_github() -> js_sys::JsString {
    let url = "http://api.github.com/users/clpi";
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    unsafe { js_sys::JSON::stringify(&wasm_bindgen::JsValue::from_str(&resp)).unwrap() }
}
