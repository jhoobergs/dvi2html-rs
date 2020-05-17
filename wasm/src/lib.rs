mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn dvi2html(input: &[u8]) -> Result<String, JsValue> {
    match dvi2html::dvi2html(input) {
        Ok(s) => Ok(s),
        Err(_) => Err(JsValue::from_str("Something went wrong")), //TODO
    }
}
