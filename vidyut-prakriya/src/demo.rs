//! WASM-exported demo functions.
use crate::{
    args::{self, Gana},
    Ashtadhyayi,
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

/// Returns each possible *prakriyā* for `upadesha` in "laṭ".
/// See documentation of Dhatu::new etc.
#[wasm_bindgen]
pub fn demo(upadesha: &str, gana: Gana) -> JsValue {
    use args::*;

    let a = Ashtadhyayi::new();

    let dhatu = Dhatu::new(upadesha, gana);
    let args = TinantaArgs::builder()
        .lakara(Lakara::Lat)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .build()
        .unwrap();

    let prakriyas = a.derive_tinantas(&dhatu, &args);
    serde_wasm_bindgen::to_value(&prakriyas).unwrap()
}
