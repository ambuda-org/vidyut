#![doc = include_str!("../README.md")]
#![warn(clippy::unwrap_used)]

pub use crate::config::Config;
pub use crate::segmenting::{Chedaka, Token};

pub mod scoring;

// TODO: move this to its own crate?
pub mod sounds;

// Evaluation code. TODO: move to its own crate?
pub mod conllu;
pub mod dcs;

mod config;
mod normalize_text;
mod segmenting;
mod strict_mode;
