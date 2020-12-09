use serde::{Serialize,Deserialize};
use super::{
    attribute::Attr,
    element::ElementKind,
};
pub struct VDom {

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VNode {
    pub el: ElementKind,
    pub class: Vec<String>,
    pub id: Option<String>,
    pub attrs: Vec<Attr>,
    pub children: Vec<VNode>,
    pub inner_html: String,
}
