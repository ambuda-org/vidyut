#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

pub use crate::core::{Decision, Error, Prakriya, Rule, RuleChoice, Step};
pub use crate::dhatupatha::Dhatupatha;
pub use crate::vyakarana::{Vyakarana, VyakaranaBuilder};

// Public modules.
// - `args` defines the API contract.
// - `dhatupatha` defines convenience functions for reading our version of the Dhatupatha.
//   These functions are used only in our binaries (in `src/bin`).
// - `ganapatha` defines various useful ganas.
pub mod args;
pub mod dhatupatha;
pub mod ganapatha;

// Data structures and utilities
mod caching;
mod core;
mod sounds;

// Other texts.
mod ashtadhyayi;
mod linganushasanam;
mod phit_sutraani;

// Components of the sutrapatha.
mod ac_sandhi;
mod angasya;
mod ardhadhatuka;
mod atidesha;
mod atmanepada;
mod dhatu_gana;
mod dhatu_karya;
mod dvitva;
mod it_agama;
mod it_samjna;
mod krt;
mod la_karya;
mod misc;
mod pratipadika_karya;
mod samasa;
mod samjna;
mod samprasarana;
mod sanadi;
mod stem_gana;
mod stritva;
mod sup_karya;
mod svara;
mod taddhita;
mod tin_pratyaya;
mod tripadi;
mod uttarapade;
mod vikarana;
mod vyakarana;

pub mod wasm;
