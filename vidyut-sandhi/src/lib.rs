#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

pub use errors::Error;
pub use generator::{generate_rules, Rule};
pub use splitter::{Kind, Location, Split, SplitsMap, Splitter};

mod errors;
mod generator;
mod sounds;
mod splitter;
