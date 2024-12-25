use crate::args::{Subanta, Tinanta};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The information required to derive a word.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pada {
    /// A nominal word or an indeclinable.
    Subanta(Subanta),
    /// A verb.
    Tinanta(Tinanta),
    /// A "chunk of text" without any specific morphology. This is a temporary variant that we hope
    /// to clean up later.
    Unknown(String),
    /// A dummy variant that we hope to clean up later.
    Nipata(String),
}

impl Pada {
    /// Creates a dummy pada from the given text.
    pub fn from_text(text: impl AsRef<str>) -> Self {
        Self::Unknown(text.as_ref().to_string())
    }

    /// Creates a dummy pada from the given text.
    pub fn nipata(text: impl AsRef<str>) -> Self {
        Self::Nipata(text.as_ref().to_string())
    }
}

impl From<Subanta> for Pada {
    fn from(s: Subanta) -> Self {
        Self::Subanta(s)
    }
}

impl From<&Subanta> for Pada {
    fn from(s: &Subanta) -> Self {
        Self::Subanta(s.clone())
    }
}

impl From<Tinanta> for Pada {
    fn from(t: Tinanta) -> Self {
        Self::Tinanta(t)
    }
}

impl From<&Tinanta> for Pada {
    fn from(t: &Tinanta) -> Self {
        Self::Tinanta(t.clone())
    }
}
