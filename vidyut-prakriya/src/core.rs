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

mod prakriya;
mod tag;
mod term;
mod term_view;

pub use prakriya::*;
pub use tag::*;
pub use term::*;
pub use term_view::*;
