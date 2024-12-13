/*!
Common arguments for the crate's main functions.

Before we begin a *prakriyā*, we must declare certain morphological information up-front, such as
our desired *puruṣa* and *vacana*, the *dhātu* we wish to use, and so on. To better document the
API and to help users avoid configuration mistakes, we model this information through the enums and
structs in this module.

For extra flexibility, all of the pratyaya enums here provides `as_str` and `from_str` methods. For
details on which strings are valid arguments in `from_str`, please read the source code directly.
*/

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod macros;

pub(crate) mod dhatu;
mod internal;
mod krt;
mod pada;
mod pratipadika;
mod samasa;
mod slp1_string;
mod sup;
mod taddhita;
mod tin;
mod unadi;

pub use dhatu::*;
pub(crate) use internal::*;
pub use krt::*;
pub use pada::*;
pub use pratipadika::*;
pub use samasa::*;
pub use slp1_string::Slp1String;
pub use sup::*;
pub use taddhita::*;
pub use tin::*;
pub use unadi::Unadi;

/// Models a semantic condition that applies to the *prakriyā* as a whole.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Artha {
    /// A semantic condition for a *kṛdanta* derivation.
    Krt(KrtArtha),
    /// A semantic condition for a *taddhitānta* derivation.
    Taddhita(TaddhitaArtha),
}
