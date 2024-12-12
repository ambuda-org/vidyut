use crate::args::tin::Vacana;
use crate::args::Pratipadika;
use crate::core::errors::Error;
use crate::core::Tag;
use crate::enum_boilerplate;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The gender of some *subanta*.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wasm_bindgen]
pub enum Linga {
    /// The masculine.
    Pum,
    /// The feminine.
    Stri,
    /// The neuter.
    Napumsaka,
}

enum_boilerplate!(Linga, {
    Pum => "pum",
    Stri => "stri",
    Napumsaka => "napumsaka"
});

impl Linga {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Pum => Tag::Pum,
            Self::Stri => Tag::Stri,
            Self::Napumsaka => Tag::Napumsaka,
        }
    }
}

/// The case ending of some *subanta*.
///
/// A *vibhakti* is a set of 3 endings that share all of the same properties except for their
/// number (singular, dual, plural). While *tiṅanta*s also have *vibhakti*s, in practice the term
/// *vibhakti* refers more specifically to the endings used with *subanta*s.
///
/// *Vibhakti* is broadly similar to the Western notion of grammatical case.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wasm_bindgen]
pub enum Vibhakti {
    /// The first *vibhakti*. Sometimes called the *nominative case*.
    Prathama,
    /// The second *vibhakti*. Sometimes called the *accusative case*.
    Dvitiya,
    /// The third *vibhakti*. Sometimes called the *instrumental case*.
    Trtiya,
    /// The fourth *vibhakti*. Sometimes called the *dative case*.
    Caturthi,
    /// The fifth *vibhakti*. Sometimes called the *ablative case*.
    Panchami,
    /// The sixth *vibhakti*. Sometimes called the *genitive case*.
    Sasthi,
    /// The seventh *vibhakti*. Sometimes called the *locative case*.
    Saptami,
    /// The first *vibhakti* used in the sense of *sambodhana*. Sometimes called the *vocative case*.
    ///
    /// *Sambodhana* is technically not a *vibhakti* but rather an additional semantic condition
    /// on the first vibhakti. But we felt that users would find it more convenient to have this
    /// condition available on `Vibhakti` directly rather than have to define the *sambodhana*
    /// condition separately.
    Sambodhana,
}

enum_boilerplate!(Vibhakti, {
    Prathama => "1",
    Dvitiya => "2",
    Trtiya => "3",
    Caturthi => "4",
    Panchami => "5",
    Sasthi => "6",
    Saptami => "7",
    Sambodhana => "s",
});

impl Vibhakti {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Prathama => Tag::V1,
            Self::Dvitiya => Tag::V2,
            Self::Trtiya => Tag::V3,
            Self::Caturthi => Tag::V4,
            Self::Panchami => Tag::V5,
            Self::Sasthi => Tag::V6,
            Self::Saptami => Tag::V7,
            Self::Sambodhana => Tag::V1,
        }
    }
}

/// The information required to derive a *subanta*.
///
/// A *subanta* is any word that ends with one of the twenty-one suffixes in the *sup* list:
///
/// | Singular  | Dual      | Plural    |
/// |-----------|-----------|-----------|
/// | su        । au        । jas       |
/// | am        । auṭ       । śas       |
/// | ṭā        । bhyām     । bhis      |
/// | ṅe        । bhyām     । bhyas     |
/// | ṅasi      । bhyām     । bhyas     |
/// | ṅas       । os        । ām        |
/// | ṅi        । os        । sup       |
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Subanta {
    pratipadika: Pratipadika,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
    is_avyaya: bool,
}

impl Subanta {
    /// Creates a subanta.
    pub fn new(
        pratipadika: impl Into<Pratipadika>,
        linga: Linga,
        vibhakti: Vibhakti,
        vacana: Vacana,
    ) -> Self {
        let pratipadika = pratipadika.into();
        Self {
            pratipadika,
            linga,
            vibhakti,
            vacana,
            is_avyaya: false,
        }
    }

    /// Defines a *subanta* that is also an *avyaya*.
    pub fn avyaya(pratipadika: impl Into<Pratipadika>) -> Self {
        let pratipadika = pratipadika.into();
        Self {
            pratipadika,
            linga: Linga::Pum,
            vibhakti: Vibhakti::Prathama,
            vacana: Vacana::Eka,
            is_avyaya: true,
        }
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> SubantaBuilder {
        SubantaBuilder::default()
    }

    /// The pratipadika to use in the derivation.
    pub fn pratipadika(&self) -> &Pratipadika {
        &self.pratipadika
    }

    /// The linga to use in the derivation.
    pub fn linga(&self) -> Linga {
        self.linga
    }

    /// The *vacana* to use in the derivation.
    pub fn vacana(&self) -> Vacana {
        self.vacana
    }

    /// The *vibhakti* to use in the derivation.
    pub fn vibhakti(&self) -> Vibhakti {
        self.vibhakti
    }

    /// Returns whether or not this *subanta* is an *avyaya*.
    pub fn is_avyaya(&self) -> bool {
        self.is_avyaya
    }
}

/// Convenience struct for building a `SubantaArgs` struct.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SubantaBuilder {
    pratipadika: Option<Pratipadika>,
    linga: Option<Linga>,
    vacana: Option<Vacana>,
    vibhakti: Option<Vibhakti>,
}

impl SubantaBuilder {
    /// Sets the pratipadika to use in the derivation.
    pub fn pratipadika(&mut self, val: impl Into<Pratipadika>) -> &mut Self {
        self.pratipadika = Some(val.into());
        self
    }

    /// Sets the linga to use in the derivation.
    pub fn linga(&mut self, val: Linga) -> &mut Self {
        self.linga = Some(val);
        self
    }

    /// Sets the *vacana* to use in the derivation.
    pub fn vacana(&mut self, val: Vacana) -> &mut Self {
        self.vacana = Some(val);
        self
    }

    /// Sets the *vibhakti* to use in the derivation.
    pub fn vibhakti(&mut self, val: Vibhakti) -> &mut Self {
        self.vibhakti = Some(val);
        self
    }

    /// Converts the arguments in this builder into a `SubantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<Subanta, Error> {
        Ok(Subanta {
            pratipadika: match &self.pratipadika {
                Some(x) => x.clone(),
                _ => return Err(Error::missing_required_field("pratipadika")),
            },
            linga: match self.linga {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("linga")),
            },
            vacana: match self.vacana {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("vacana")),
            },
            vibhakti: match self.vibhakti {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("vibhakti")),
            },
            is_avyaya: false,
        })
    }
}
