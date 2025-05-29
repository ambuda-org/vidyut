#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

/// Sanskrit syllable (akshara) analysis and syllabification.
pub mod akshara;
mod chandas;
mod error;
mod macros;
mod padya;
mod sounds;

mod wasm;

pub use akshara::{scan_line, Akshara, Weight};
pub use chandas::{Chandas, Match, Matches};
pub use padya::{Jati, MatchType, Vrtta, VrttaPada, VrttaWeight};
