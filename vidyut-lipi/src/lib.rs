//! Hacky transliteration functions that other crates might need.
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

mod autogen_schemes;
mod detect;
mod lipika;
mod mapping;
mod numerals;
mod scheme;
mod transliterate;
mod unicode_norm;
pub mod wasm;

pub use detect::detect;
pub use lipika::Lipika;
pub use mapping::Mapping;
pub use scheme::Scheme;
pub use transliterate::transliterate;
