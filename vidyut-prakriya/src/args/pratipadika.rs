use crate::args::{Krdanta, Samasa, Taddhitanta};

/// A nominal stem.
///
/// Rules 1.2.45 and 1.2.46 define a pratipadika as either:
///
/// 1. A meaningful term that is neither a dhatu nor a pratyaya;
/// 2. A term whose last element is krt, taddhita, or a samasa.
///
/// A pratipadika is the base to which we add sup-pratyayas. Through this process, we create
/// subantas (nominals), which are complete words.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

/// Models a basic *prātipadika* that is not created with any other *pratyaya*s.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BasicPratipadika {
    pub(crate) text: String,
    pub(crate) is_avyaya: bool,
    pub(crate) is_nyap: bool,
}

impl Pratipadika {
    /// (unstable) A simple constructor for `Pratipadika::Basic`.
    pub fn basic(text: impl AsRef<str>) -> Self {
        Self::Basic(BasicPratipadika {
            text: text.as_ref().to_string(),
            is_nyap: false,
            is_avyaya: false,
        })
    }

    /// (unstable) A simple constructor for `Pratipadika::Basic` that marks the pratipadika as an
    /// avyaya.
    pub fn avyaya(text: impl AsRef<str>) -> Self {
        Self::Basic(BasicPratipadika {
            text: text.as_ref().to_string(),
            is_nyap: false,
            is_avyaya: true,
        })
    }

    /// (unstable) A simple constructor for `Pratipadika::Basic` that indicates that the
    /// pratipadika already ends in a nyAp-pratyaya.
    pub fn nyap(text: impl AsRef<str>) -> Self {
        Self::Basic(BasicPratipadika {
            text: text.as_ref().to_string(),
            is_nyap: true,
            is_avyaya: false,
        })
    }
}

impl From<&str> for Pratipadika {
    fn from(s: &str) -> Self {
        Self::basic(s)
    }
}

impl From<&Pratipadika> for Pratipadika {
    fn from(p: &Pratipadika) -> Self {
        p.clone()
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
