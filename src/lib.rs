//! A lightning-fast Sanskrit toolkit.

pub mod io;

pub mod fst_lexicon;
pub mod lexicon;
pub mod packing;
pub mod parsing;
pub mod sandhi;
pub mod scoring;
pub mod semantics;
pub mod sounds;

pub mod conllu;
pub mod dcs;
pub mod translit;

mod strict_mode;
