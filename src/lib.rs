//! A lightning-fast Sanskrit toolkit.

pub mod io;

pub mod config;
pub mod generator;
pub mod lexicon;
pub mod old_lexicon;
pub mod packing;
pub mod sandhi;
pub mod scoring;
pub mod segmenting;
pub mod semantics;
pub mod sounds;

pub mod conllu;
pub mod dcs;
pub mod translit;

mod normalize_text;
mod strict_mode;
