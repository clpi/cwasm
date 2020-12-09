use serde::{Serialize, Deserialize};
use std::borrow::Cow;

pub type AttrKey = String;
pub type AttrVal = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Attr {
    key: AttrKey,
    val: AttrVal,
}
