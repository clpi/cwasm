use wasm_bindgen::prelude::*;
use web_sys::{Event, EventListener, EventTarget};

pub fn new_listener() -> web_sys::EventListener {
    web_sys::EventListener::new()
}

pub fn new_target() -> EventTarget {
    EventTarget::new().unwrap()
}

pub fn new() {
}
