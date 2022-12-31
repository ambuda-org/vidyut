//! WASM-exported demo functions.
use crate::{args, Ashtadhyayi};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

/// Returns each possible *prakriyā* for `upadesha` in "laṭ".
/// See documentation of Dhatu::new etc.
#[wasm_bindgen]
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
