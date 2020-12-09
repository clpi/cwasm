use serde::{Serialize, Deserialize};

pub struct Element {}

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
