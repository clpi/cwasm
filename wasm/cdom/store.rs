use js_sys::JSON;
use wasm_bindgen::prelude::*;
pub struct Store {

    local_storage: web_sys::Storage,
}

pub struct Item {
}

pub struct ItemList {
}

impl Store {

    pub fn new(name: &str) -> Option<Store> {
        let window = web_sys::window()?;
        if let Ok(Some(local_storage)) = window.local_storage() {
            let mut store = Store {
                local_storage,
            };
            Some(store)
        } else {
            None
        }
    }
}

