//! WASM-exported demo functions.
#[cfg(feature = "wasm_bindings")]
use crate::{args, Ashtadhyayi};
#[cfg(feature = "wasm_bindings")]
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

/// Returns each possible *prakriyā* for `upadesha` in "laṭ".
/// See documentation of Dhatu::new etc.
#[cfg(feature = "wasm_bindings")]
#[cfg_attr(feature = "wasm_bindings", wasm_bindgen)]
pub fn demo(
    upadesha: &str,
    gana: args::Gana,
    lakara: args::Lakara,
    prayoga: args::Prayoga,
    purusha: args::Purusha,
    vacana: args::Vacana,
) -> JsValue {
    use args::*;

    let a = Ashtadhyayi::new();

    let dhatu = Dhatu::new(upadesha, gana);
    let args = TinantaArgs::builder()
        .lakara(lakara)
        .prayoga(prayoga)
        .purusha(purusha)
        .vacana(vacana)
        .build()
        .unwrap();

    let prakriyas = a.derive_tinantas(&dhatu, &args);
    serde_wasm_bindgen::to_value(&prakriyas).unwrap()
}
