use serde::{Serialize, Deserialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::EventTarget;

use super::doc::{body, doc};

pub struct Element(Option<web_sys::Element>);

impl From<web_sys::Element> for Element {
    fn from(e: web_sys::Element) -> Element {
        Element(Some(e))
    }
}

impl From<web_sys::EventTarget> for Element {
    fn from(e: web_sys::EventTarget) -> Element {
        let e = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(e);
        Element(e.ok())
    }
}

impl From<Element> for Option<web_sys::Node> {
    fn from(obj: Element) -> Option<web_sys::Node> {
        match obj.0 {
            Some(e) => Some(e.into()),
            None => None,
        }
    }
}

impl Element {
    pub fn create(tag: &str) -> Option<Self> {
        match doc().create_element(tag) {
            Ok(e) => Some(e.into()),
            Err(_) => None,
        }
    }

    pub fn select(select: &str) -> Option<Self> {
        let e: web_sys::Element = body().into();
        let e = e.query_selector(select).ok()?;
        Some(Self(e))
    }

    pub fn add_listener<T>(&mut self, ev: &str, handle: T)
        where
            T: FnMut(web_sys::Event) + 'static,
    {
        let cb = Closure::wrap(Box::new(handle) as Box<dyn FnMut(_)>);
        if let Some(e) = self.0.take() {
            let ev_target: EventTarget = e.into();
            ev_target
                .add_event_listener_with_callback(ev, cb.as_ref().unchecked_ref())
                .expect("Could not add event listener");
            cb.forget();
            if let Ok(e) = ev_target.dyn_into::<web_sys::Element>() {
                self.0 = Some(e);
            }
        }

    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ElementKind {
    P,
    Br,
    Hr,
    Ul,
    Ol,
    Li,
    Span,
    Input,
    Form,
    Button,
    Header(HeaderLvl)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HeaderLvl {

}
