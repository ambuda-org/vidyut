#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

mod akshara;
mod chandas;
mod error;
mod macros;
mod padya;
mod sounds;

mod wasm;

pub use akshara::{Akshara, Weight};
pub use chandas::{Chandas, Match, Matches};
pub use padya::{Jati, MatchType, Vrtta, VrttaPada, VrttaWeight};
