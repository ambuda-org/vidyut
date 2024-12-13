//! Core data structures and utility functions that support the rest of the system.
//!
//! The core modules here are:
//!
//! - `term`, which defines the `Term` struct
//! - `prakriya`, which defines the `Prakriya` struct

pub mod char_view;
pub mod errors;
pub mod iterators;
pub mod operators;
pub mod prakriya_stack;

pub(crate) mod prakriya;
pub(crate) mod tag;
pub(crate) mod term;
pub(crate) mod term_view;

pub use errors::Error;
pub use prakriya::*;
pub use tag::*;
pub use term::*;
pub use term_view::*;
