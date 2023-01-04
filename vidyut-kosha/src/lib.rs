#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod kosha;

pub use kosha::{Builder, Kosha};

pub mod packing;
pub mod semantics;
