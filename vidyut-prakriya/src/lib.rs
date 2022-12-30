#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub use crate::ashtadhyayi::{Ashtadhyayi, AshtadhyayiBuilder};
pub use crate::prakriya::{Prakriya, Rule, RuleChoice, Step};

// Public modules.
// - `args` defines the API contract.
// - `dhatupatha` defines convenience functions for reading our version of the Dhatupatha.
//   These functions are used only in our binaries (in `src/bin`).
// - `private` contains convenience functions for the code in `src/bin`.
pub mod args;
pub mod dhatupatha;

mod binary_only;
#[doc(hidden)]
pub mod private {
    // Common code for our binaries. Don't use or depend on this!
    pub use crate::binary_only::*;
}

// Data structures
mod char_view;
mod prakriya;
mod prakriya_stack;
mod sounds;
mod tag;
mod term;

// Utility functions
mod filters;
mod operators;

// Rules
mod ac_sandhi;
mod angasya;
mod ardhadhatuka;
mod ashtadhyayi;
mod atidesha;
mod atmanepada;
mod dhatu_gana;
mod dhatu_karya;
mod dvitva;
mod it_agama;
mod it_samjna;
mod krt_pratyaya;
mod la_karya;
mod pratipadika_karya;
mod samjna;
mod samprasarana;
mod sanadi;
mod stem_gana;
mod sup_karya;
mod tin_pratyaya;
mod tripadi;
mod vikarana;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

/// Returns each possible *prakriyā* for "bhū" in "laṭ".
#[wasm_bindgen]
pub fn prakriyas() -> JsValue {
    use args::*;

    let a = Ashtadhyayi::new();

    let dhatu = Dhatu::new("BU", Gana::Bhvadi);
    let args = TinantaArgs::builder()
        .lakara(Lakara::Lat)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .build()
        .unwrap();

    let prakriyas = a.derive_tinantas(&dhatu, &args);
    // JsValue::from_serde(&prakriyas).unwrap()
    serde_wasm_bindgen::to_value(&prakriyas).unwrap()
}
