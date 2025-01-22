#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]

pub use errors::Error;
pub use kosha::{Builder, Kosha};

pub mod entries;
pub mod packing;

mod errors;
mod kosha;
