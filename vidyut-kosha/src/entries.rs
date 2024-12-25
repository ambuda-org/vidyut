//! Models the morphology of Sanskrit words, including their bases and endings.
use crate::errors::*;
use modular_bitfield::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use vidyut_prakriya::args as vp;

use serde::{Deserialize, Serialize};

/// Implements various boilerplate for our enums:
///
/// - `as_str`
/// - `iter`
/// - `FromStr`
/// - `Display`
macro_rules! enum_boilerplate {
    ($Enum:ident, { $( $variant:ident => $str:literal ),* $(,)? }) => {
        impl $Enum {
            /// Returns a string representation of this enum.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $Enum::$variant => $str,
                    )*
                }
            }

            /// Iterates over the values of this enum in order.
            pub fn iter() -> impl Iterator<Item = &'static $Enum> {
                const ITEMS: &[$Enum] = &[
                    $(
                        $Enum::$variant,
                    )*
                ];
                ITEMS.iter()
            }
        }

        impl FromStr for $Enum {
            type Err = Error;
            fn from_str(s: &str) -> Result<Self> {
                let val = match s {
                    $(
                        $str => $Enum::$variant,
                    )*
                    _ => return Err(Error::ParseEnum(stringify!($Enum), s.to_string())),
                };
                Ok(val)
            }
        }

        impl Display for $Enum {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "{}", self.as_str())
            }
        }
    }
}

/// A short part-of-speech tag for some `Pada`.
///
/// We use this tag when calculating lemma counts. For example, *eva* is a common *avyaya* but
/// not a common *subanta*, and our statistics should reflect that distinction. Coarser
/// distinctions that include linga, vacana, etc. are interesting but less useful given our
/// limited training data.
#[derive(Clone, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[bits = 2]
pub enum POSTag {
    /// A token with missing, unknown, or undefined semantics.
    Unknown,
    /// A nominal.
    Subanta,
    /// A verb.
    Tinanta,
    /// An indeclinable.
    Avyaya,
}

enum_boilerplate!(POSTag, {
    Unknown => "_",
    Subanta => "s",
    Tinanta => "t",
    Avyaya => "a",
});

/// A dhatu with its metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct DhatuEntry {
    dhatu: vp::Dhatu,
    readable: String,
}

impl DhatuEntry {
    /// Creates a new `DhatuEntry`.
    ///
    /// `readable` should be the text obtained by calling `Vyakarana::derive_dhatus` on `dhatu`.
    pub fn new(dhatu: vp::Dhatu, readable: String) -> Self {
        Self { dhatu, readable }
    }

    /// Returns the dhatu that generates this entry.
    pub fn dhatu(&self) -> &vp::Dhatu {
        &self.dhatu
    }

    /// Returns the human-readable text representation of this dhatu.
    ///
    /// Examples:
    ///
    /// - `qukf\\Y` --> `kf`
    /// - `vidi~` --> `vind`
    pub fn text(&self) -> &str {
        &self.readable
    }
}

/// A pratipadika with its metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct PratipadikaEntry {
    pratipadika: vp::Pratipadika,
    lingas: Vec<vp::Linga>,
}

impl PratipadikaEntry {
    /// Creates a new `PratipadikaEntry`.
    pub fn new(pratipadika: vp::Pratipadika, lingas: Vec<vp::Linga>) -> Self {
        Self {
            pratipadika,
            lingas,
        }
    }

    /// Returns the pratipadika that generates this entry.
    pub fn pratipadika(&self) -> &vp::Pratipadika {
        &self.pratipadika
    }

    /// Returns the lingas that this pratipadika is allowed to use.
    ///
    /// If empty, the pratipadika has no specific linga.
    pub fn lingas(&self) -> &[vp::Linga] {
        &self.lingas
    }
}

/// Wraps a `Subanta` with metadata about the pratipadika.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SubantaEntry {
    subanta: vp::Subanta,
    entry: PratipadikaEntry,
}

impl SubantaEntry {
    /// Creates a new SubantaEntry.
    pub fn new(subanta: vp::Subanta, entry: PratipadikaEntry) -> Self {
        Self { subanta, entry }
    }

    /// The underlying subanta.
    pub fn subanta(&self) -> &vp::Subanta {
        &self.subanta
    }

    /// The pratipadika metadata.
    pub fn entry(&self) -> &PratipadikaEntry {
        &self.entry
    }
}

impl From<vp::Subanta> for SubantaEntry {
    fn from(val: vp::Subanta) -> Self {
        let p_entry = PratipadikaEntry::new(val.pratipadika().clone(), vec![]);
        Self::new(val, p_entry)
    }
}

/// Models the semantics of a Sanskrit *pada* (word).
///
/// This enum can be packed into an unsigned integer via the `packing` module.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum PadaEntry {
    /// Unknown data.
    Unknown,
    /// A *subanta* that is not an *avyaya*.
    Subanta(SubantaEntry),
    /// A *subanta* that is also an *avyaya*.
    Avyaya(SubantaEntry),
    /// A *tiṅanta* (verb).
    Tinanta(vp::Tinanta),
}

impl PadaEntry {
    /// Returns a placeholder lemma.
    pub fn lemma(&self) -> &str {
        match self {
            Self::Subanta(e) => {
                use vp::Pratipadika::*;
                match e.subanta().pratipadika() {
                    Basic(b) => b.text(),
                    _ => "",
                }
            }
            _ => "",
        }
    }

    /// Returns a part-of-speech tag.
    pub fn pos_tag(&self) -> POSTag {
        match self {
            PadaEntry::Unknown => POSTag::Unknown,
            PadaEntry::Subanta(_) => POSTag::Subanta,
            PadaEntry::Avyaya(_) => POSTag::Avyaya,
            PadaEntry::Tinanta(_) => POSTag::Tinanta,
        }
    }
}
