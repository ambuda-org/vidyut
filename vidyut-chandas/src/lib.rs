#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

mod akshara;
mod chandas;
mod error;
mod sounds;
mod vrtta;
mod wasm;

pub use akshara::{Akshara, Weight};
pub use chandas::{Chandas, MatchResult};
pub use vrtta::{Jati, MatchType, Vrtta};
