// TODO decide whether to return specific html element subtypes or HtmlElement
// TODO reduce redundancy in match checks for get function
use std::{
    rc::Rc,
    cell::RefCell,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::{
    window, Element, HtmlElement,
};
use super::win;

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

pub fn _p(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlParagraphElement> {
    _get::<web_sys::HtmlParagraphElement>("p", contents, id)
}

pub fn _li(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlLiElement> {
    _get::<web_sys::HtmlLiElement>("li", contents, id)
}

pub fn _div(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlDivElement> {
    _get::<web_sys::HtmlDivElement>("div", contents, id)
}

pub fn _canvas(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlCanvasElement> {
    _get::<web_sys::HtmlCanvasElement>("canvas", contents, id)
}

pub fn _input(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlInputElement> {
    _get::<web_sys::HtmlInputElement>("canvas", contents, id)
}

pub fn _a(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlLinkElement> {
    _get::<web_sys::HtmlLinkElement>("a", contents, id)
}

pub fn _img(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlImageElement> {
    _get::<web_sys::HtmlImageElement>("img", contents, id)
}

pub fn _span(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlSpanElement> {
    _get::<web_sys::HtmlSpanElement>("img", contents, id)
}

pub fn _ul(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlSpanElement> {
    _get::<web_sys::HtmlSpanElement>("img", contents, id)
}

pub fn _ol(contents: Option<&str>, id: Option<&str>) -> Option<web_sys::HtmlSpanElement> {
    _get::<web_sys::HtmlSpanElement>("img", contents, id)
}

pub fn req_animation_frame(f: &Closure<dyn FnMut()>) {
    win()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Should register 'requestAnimationFrame'");
}

pub fn anim() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i> 300 {
            body().set_text_content(Some("All done"));
            let _ = f.borrow_mut().take();
            return;
        }
        i += 1;
        let txt = format!("requestAnimationFrame has been called {} times", i);
        body().set_text_content(Some(&txt));
        req_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    req_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
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
