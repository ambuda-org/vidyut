#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

pub use crate::ashtadhyayi::{Ashtadhyayi, AshtadhyayiBuilder};
pub use crate::dhatupatha::Dhatupatha;
pub use crate::errors::Error;
pub use crate::prakriya::{Prakriya, Rule, RuleChoice, Step};

// Public modules.
// - `args` defines the API contract.
// - `dhatupatha` defines convenience functions for reading our version of the Dhatupatha.
//   These functions are used only in our binaries (in `src/bin`).
// - `private` contains convenience functions for the code in `src/bin`.
pub mod args;
pub mod dhatupatha;

mod errors;

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
mod iterators;
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
mod unadi_pratyaya;
mod vikarana;

pub mod wasm;
