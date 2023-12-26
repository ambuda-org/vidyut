//! WebAssembly bindings for vidyut-lipi.
use crate::{transliterate, Scheme};
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    /// Exposes `console.error` in case we need to log anything to the JS console.
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error(s: &str);
}

/// WebAssembly API for vidyut-prakriya.
///
/// Within reason, we have tried to mimic a native JavaScript API. At some point, we wish to
/// support optional arguments, perhaps by using `Reflect`.
#[wasm_bindgen]
pub struct VidyutLipi {}

#[wasm_bindgen]
impl VidyutLipi {
    /// Creates a new API manager.
    ///
    /// This constructor is not called `new` because `new` is a reserved word in JavaScript.
    pub fn init() -> Self {
        // Logs panics to the console. Without this, panics are logged as "RuntimeError:
        // Unreachable executed", which is not useful.
        console_error_panic_hook::set_once();
        Self {}
    }

    /// Wrapper for `transliterate`.
    pub fn transliterate(&self, input: &str, from: Scheme, to: Scheme) -> JsValue {
        let output = transliterate(input, from.into(), to.into());
        JsValue::from_str(&output)
    }
}
