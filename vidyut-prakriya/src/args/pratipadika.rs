use crate::args::{Krdanta, Samasa, Slp1String, Taddhitanta};
use crate::core::errors::{Error, Result};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Models a basic *prātipadika* that is not created with any other *pratyaya*s.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BasicPratipadika {
    pub(crate) text: Slp1String,
    pub(crate) is_avyaya: bool,
    pub(crate) is_nyap: bool,
}

impl BasicPratipadika {
    /// Returns the text that constitutes this pratipadika.
    pub fn text(&self) -> &str {
        &self.text.0
    }
}

/// A nominal stem.
///
/// Rules 1.2.45 and 1.2.46 define a pratipadika as either:
///
/// 1. A meaningful term that is neither a dhatu nor a pratyaya;
/// 2. A term whose last element is krt, taddhita, or a samasa.
///
/// A pratipadika is the base to which we add sup-pratyayas. Through this process, we create
/// subantas (nominals), which are complete words.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pratipadika {
    /// A simple string that receives the pratipadika-samjna by rule 1.2.45.
    Basic(BasicPratipadika),
    /// A krdanta.
    Krdanta(Box<Krdanta>),
    /// A *taddhitānta*.
    Taddhitanta(Box<Taddhitanta>),
    /// A *samāsa*.
    Samasa(Box<Samasa>),
}

impl Pratipadika {
    /// (unstable) A simple constructor for `Pratipadika::Basic`.
    pub fn basic(text: Slp1String) -> Self {
        Self::Basic(BasicPratipadika {
            text,
            is_nyap: false,
            is_avyaya: false,
        })
    }

    /// (unstable) A simple constructor for `Pratipadika::Basic` that marks the pratipadika as an
    /// avyaya.
    pub fn avyaya(text: Slp1String) -> Self {
        Self::Basic(BasicPratipadika {
            text,
            is_nyap: false,
            is_avyaya: true,
        })
    }

    /// (unstable) A simple constructor for `Pratipadika::Basic` that indicates that the
    /// pratipadika already ends in a nyAp-pratyaya.
    pub fn nyap(text: Slp1String) -> Self {
        Self::Basic(BasicPratipadika {
            text,
            is_nyap: true,
            is_avyaya: false,
        })
    }

    /// Returns whether the pratipadika describes an avyaya.
    pub fn is_avyaya(&self) -> bool {
        match self {
            Self::Basic(b) => b.is_avyaya,
            Self::Krdanta(k) => k.krt().is_avyaya(),
            _ => false,
        }
    }
}

impl TryFrom<&str> for Pratipadika {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        Ok(Self::basic(Slp1String::from(s)?))
    }
}

impl From<&Pratipadika> for Pratipadika {
    fn from(p: &Pratipadika) -> Self {
        p.clone()
    }
}

impl From<BasicPratipadika> for Pratipadika {
    fn from(b: BasicPratipadika) -> Self {
        Self::Basic(b)
    }
}

impl From<&BasicPratipadika> for Pratipadika {
    fn from(b: &BasicPratipadika) -> Self {
        Self::Basic(b.clone())
    }
}

impl From<Krdanta> for Pratipadika {
    fn from(k: Krdanta) -> Self {
        Self::Krdanta(Box::new(k))
    }
}

impl From<&Krdanta> for Pratipadika {
    fn from(k: &Krdanta) -> Self {
        Self::Krdanta(Box::new(k.clone()))
    }
}

impl From<Taddhitanta> for Pratipadika {
    fn from(t: Taddhitanta) -> Self {
        Self::Taddhitanta(Box::new(t))
    }
}

impl From<&Taddhitanta> for Pratipadika {
    fn from(t: &Taddhitanta) -> Self {
        Self::Taddhitanta(Box::new(t.clone()))
    }
}

impl From<Samasa> for Pratipadika {
    fn from(s: Samasa) -> Self {
        Self::Samasa(Box::new(s))
    }
}

impl From<&Samasa> for Pratipadika {
    fn from(s: &Samasa) -> Self {
        Self::Samasa(Box::new(s.clone()))
    }
}
