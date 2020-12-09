mod utils;

use wasm_bindgen::prelude::*;
use comrak::{markdown_to_html, ComrakOptions};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn parse_md(input: &str) -> String {
    markdown_to_html(input, &ComrakOptions::default())
}

#[cfg(test)]
mod tests {

    #[test]
    fn h1_parse_from_md_ok() {
        let string = "# Header 1\n";
        let htm = super::parse_md(string);
        assert!(htm.contains("<h1>Header 1</h1>"))
    }

    #[test]
    fn bold_parse_from_md_ok() {
        let string = "This is **bold text** here";
        let htm = super::parse_md(string);
        debug_assert!(htm.contains("This is <b>bold text</b> here"))
    }
}
