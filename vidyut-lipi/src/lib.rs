#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

mod autogen_schemes;
mod detect;
mod errors;
pub mod extensions;
mod lipika;
mod mapping;
mod numerals;
mod reshape;
mod scheme;
mod transliterate;
mod unicode_norm;
pub mod wasm;

pub use detect::detect;
pub use lipika::Lipika;
pub use mapping::{Mapping, SpanKind};
pub use scheme::Scheme;
pub use transliterate::transliterate;
