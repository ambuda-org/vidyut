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
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Pratipadika {
    Basic(String, bool),
    Krdanta(Box<Krdanta>),
    Taddhitanta(Box<Taddhitanta>),
    Samasa(Box<Samasa>),
}

#[allow(missing_docs)]
impl Pratipadika {
    pub fn basic(text: impl AsRef<str>) -> Self {
        Self::Basic(text.as_ref().to_string(), false)
    }
    pub fn nyap(text: impl AsRef<str>) -> Self {
        Self::Basic(text.as_ref().to_string(), true)
    }
}

impl From<&str> for Pratipadika {
    fn from(s: &str) -> Self {
        Self::Basic(s.to_string(), false)
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
