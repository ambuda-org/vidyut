#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

pub use crate::ashtadhyayi::{Ashtadhyayi, AshtadhyayiBuilder};
pub use crate::core::errors::Error;
pub use crate::core::{Prakriya, Rule, RuleChoice, Step};
pub use crate::dhatupatha::Dhatupatha;

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

// Data structures and utilities
mod core;
mod sounds;

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
mod ganapatha;
mod it_agama;
mod it_samjna;
mod krt;
mod la_karya;
mod linganushasanam;
mod misc;
mod pratipadika_karya;
mod samasa;
mod samjna;
mod samprasarana;
mod sanadi;
mod stem_gana;
mod stritva;
mod sup_karya;
mod sutrapatha;
mod taddhita;
mod tin_pratyaya;
mod tripadi;
mod uttarapade;
mod vikarana;

pub mod wasm;
