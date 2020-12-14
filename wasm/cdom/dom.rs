use web_sys::{
    HtmlElement, Element
};
use wasm_bindgen::{JsCast, prelude::*};

pub fn win() -> web_sys::Window {

    web_sys::window()
        .expect("Could not initialize window")
}

pub fn hist() -> web_sys::History {
    win().history()
        .expect("Could not initialize history")
}

pub fn doc() -> web_sys::Document {
    win().document()
        .expect("Cannot initialize document")
}

pub fn html_doc() -> web_sys::HtmlDocument {
    doc().dyn_into::<web_sys::HtmlDocument>()
        .expect("Could not get HTML doc")
}

pub fn body() -> web_sys::HtmlBodyElement {
    doc().body()
        .expect("Cannot initialize document body")
        .dyn_into::<web_sys::HtmlBodyElement>()
        .expect("Could not convert to body element")
}

pub fn add_chg_listener<E, F>(ev: E, mut f: F,id: &str) -> Result<(), JsValue>
    where
        E: Into<web_sys::Event>,
        F: FnMut(web_sys::Event) + 'static,
{
    let _cb = Closure::wrap(
        Box::new(move |e: web_sys::Event| {
            f(e) }) as Box<dyn FnMut(web_sys::Event)>);
    doc()
        .get_element_by_id(id)
        .expect("No element on DOM with that ID")
        .dyn_ref::<web_sys::HtmlInputElement>()
        .expect("COUDFJL")
        .set_onchange(Some(_cb.as_ref().unchecked_ref()));
    _cb.forget();
    Ok(())
}

pub fn append<N, T>(node: N) -> T
    where
        N: Into<web_sys::Node>,
        T: AsRef<HtmlElement>
            + Into<HtmlElement>
            + JsCast
{
    body().append_child(&node.into())
        .expect("Could not append element")
        .dyn_into::<T>()
        .expect("Could not convert to element")
}

pub mod get {
    use super::_get;
    pub fn p(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlParagraphElement> {
        _get::<web_sys::HtmlParagraphElement>("p", contents, id)
    }

    pub fn li(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlLiElement> {
        _get::<web_sys::HtmlLiElement>("li", contents, id)
    }

    pub fn div(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlDivElement> {
        _get::<web_sys::HtmlDivElement>("div", contents, id)
    }

    pub fn canvas(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlCanvasElement> {
        _get::<web_sys::HtmlCanvasElement>("canvas", contents, id)
    }

    pub fn input(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlInputElement> {
        _get::<web_sys::HtmlInputElement>("canvas", contents, id)
    }

    pub fn a(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlLinkElement> {
        _get::<web_sys::HtmlLinkElement>("a", contents, id)
    }

    pub fn img(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlImageElement> {
        _get::<web_sys::HtmlImageElement>("img", contents, id)
    }

    pub fn span(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlSpanElement> {
        _get::<web_sys::HtmlSpanElement>("span", contents, id)
    }

    pub fn ul(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlSpanElement> {
        _get::<web_sys::HtmlSpanElement>("ul", contents, id)
    }

    pub fn ol(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlSpanElement> {
        _get::<web_sys::HtmlSpanElement>("ol", contents, id)
    }

    pub fn textarea(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlTextAreaElement> {
        _get::<web_sys::HtmlTextAreaElement>("textarea", contents, id)
    }
}


pub fn req_animation_frame(f: &Closure<dyn FnMut()>) {
    win()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Should register 'requestAnimationFrame'");
}

/// Convenience base function to get element by id (if exists) or create new
/// element of the provided element type (if no match for id)
pub fn _get<T>
    (kind: &str,
     contents: Option<&str>,
     id: Option<&str>) -> Option<T>
    where
        T: wasm_bindgen::JsCast
            + AsRef<HtmlElement>
            + Into<HtmlElement>
{
    if let Some(id) = id {
        if let Some(el) = doc().get_element_by_id(id) {
            if let Some(contents) = contents {
                el.set_inner_html(contents);
                let _h = el.dyn_ref::<HtmlElement>().unwrap();
            }
            el.dyn_into::<T>().ok()
        } else {
            match doc().create_element(kind) {
                Ok(el) => {
                    if let Some(contents) = contents {
                        el.set_inner_html(contents);
                    }
                    el.set_id(id);
                    el.dyn_into::<T>().ok()
                },
                Err(_) => None,
            }
        }
    } else {
        match doc().create_element(kind) {
            Ok(el) => {
                if let Some(contents) = contents {
                    el.set_inner_html(contents);
                }
                el.dyn_into::<T>().ok()
            },
            Err(_) => None,
        }
    }
}

pub fn style<T>
    (el: T,
     key: &str,
     val: &str) -> ()
    where
        T: wasm_bindgen::JsCast
            + AsRef<HtmlElement>
            + Into<HtmlElement>
{
    el.dyn_into::<HtmlElement>().ok()
        .expect("dsf");
}

/// [unimplemented] Will eventually be convenience function to add node w/ contents
pub fn _put<T>
    (kind: &str,
     contents: Option<&str>,
     id: Option<&str>) -> Option<T>
    where
        T: wasm_bindgen::JsCast
            + AsRef<HtmlElement>
            + Into<HtmlElement>
{
    if let Some(id) = id {
        if let Some(el) = doc().get_element_by_id(id) {
            if let Some(contents) = contents {
                el.set_inner_html(contents);
            }
            el.dyn_into::<T>().ok()
        } else {
            match doc().create_element(kind) {
                Ok(el) => {
                    if let Some(contents) = contents {
                        el.set_inner_html(contents);
                    }
                    el.set_id(id);
                    el.dyn_into::<T>().ok()
                },
                Err(_) => None,
            }
        }
    } else {
        match doc().create_element(kind) {
            Ok(el) => {
                if let Some(contents) = contents {
                    el.set_inner_html(contents);
                }
                el.dyn_into::<T>().ok()
            },
            Err(_) => None,
        }
    }
}
