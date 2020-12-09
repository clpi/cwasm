pub mod doc;
pub mod element;
pub mod css;
pub mod vdom;
pub mod attribute;

pub fn win() -> web_sys::Window {
    web_sys::window()
        .expect("Could not initialize window")
}

pub fn hist() -> web_sys::History {
    win().history()
        .expect("Could not initialize history")
}
