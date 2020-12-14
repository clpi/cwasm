use web_sys::{Element, Document, HtmlElement};
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
pub fn create_div(document: &Document, id: &str, class: &str) -> Element {
    let div = document.create_element("div").unwrap();
    div.set_id(id);
    div.set_class_name(class);
    div
}


pub fn create_submit_box(document: &Document) -> Element {
    let submit_box: Element = document.create_element("input").unwrap();
    submit_box.set_attribute("type", "button");
    submit_box.set_attribute("value", "Search");
    submit_box.set_attribute("name", "submit");
    submit_box.set_id("submit");
    submit_box.set_class_name(" ReportStyles-bootstrapButton btn btn-info");
    submit_box
}

pub fn create_input_box(document: &Document) -> Element {
    let input_box = document.create_element("input").unwrap();
    input_box.set_attribute("name", "name");
    input_box.set_attribute("value", "Delhi");
    input_box.set_attribute("type", "text");
    input_box.set_attribute("placeholder", "Type city name here");
    input_box.set_id("name");
    input_box.set_class_name("ReportStyles-search");
    input_box
}
