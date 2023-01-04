#![doc = include_str!("../README.md")]

pub use crate::config::Config;
pub use crate::segmenting::{Chedaka, Token};

pub mod scoring;

// TODO: move this to its own crate?
pub mod sandhi;

// Evaluation code. TODO: move to its own crate?
pub mod conllu;
pub mod dcs;
pub mod translit;

// TODO: see if we can make this private.
pub mod io;

// TODO: move into create_kosha
pub mod generator;

mod config;
mod normalize_text;
mod segmenting;
mod strict_mode;

// TODO: move this to its own crate?
mod sounds;

// TODO: refactor and delete.
mod old_lexicon;
