//! WebAssembly bindings for vidyut-lipi.
use crate::{detect as rust_detect, Lipika, Scheme};
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    /// Exposes `console.error` in case we need to log anything to the JS console.
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error(s: &str);
}

/// Wrapper for `transliterate`.
#[wasm_bindgen]
pub fn transliterate(input: &str, from: Scheme, to: Scheme) -> JsValue {
    console_error_panic_hook::set_once();
    let mut lipika = Lipika::new();
    let output = lipika.transliterate(input, from, to);
    JsValue::from_str(&output)
}

/// Wrapper for `detect`.
///
/// `wasm_bindgen` struggles when returning optional types, so our default option here is just
/// Harvard-Kyoto.
#[wasm_bindgen]
pub fn detect(input: &str) -> Scheme {
    console_error_panic_hook::set_once();
    rust_detect(input).unwrap_or(Scheme::HarvardKyoto)
}
