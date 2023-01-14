#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

pub use errors::Error;
pub use kosha::{Builder, Kosha};

pub mod packing;
pub mod semantics;

mod errors;
mod kosha;
