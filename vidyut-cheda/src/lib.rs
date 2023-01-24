#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

pub use crate::config::Config;
pub use crate::errors::{Error, Result};
pub use crate::segmenting::{Chedaka, Token};

mod errors;
mod scoring;

/// Model structs.
///
/// These are exposed for training purposes only.
pub mod model {
    pub use crate::scoring::State;
}

// TODO: move this to its own crate?
pub mod sounds;

// Evaluation code. TODO: move to its own crate?
pub mod conllu;
pub mod dcs;

mod config;
mod normalize_text;
mod segmenting;
mod strict_mode;
