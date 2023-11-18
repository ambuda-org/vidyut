use crate::args::tin::Vacana;
use crate::core::errors::Error;
use crate::core::Tag;
use crate::enum_boilerplate;
use wasm_bindgen::prelude::wasm_bindgen;

/// The gender of some subanta.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

/// The case ending of some subanta.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[wasm_bindgen]
pub enum Vibhakti {
    /// The first vibhakti . Sometimes called the *nominative case*.
    Prathama,
    /// The second vibhakti. Sometimes called the *accusative case*.
    Dvitiya,
    /// The third vibhakti. Sometimes called the *instrumental case*.
    Trtiya,
    /// The fourth vibhakti. Sometimes called the *dative case*.
    Caturthi,
    /// The fifth vibhakti. Sometimes called the *ablative case*.
    Panchami,
    /// The sixth vibhakti. Sometimes called the *genitive case*.
    Sasthi,
    /// The seventh vibhakti. Sometimes called the *locative case*.
    Saptami,
    /// The first vibhakti used in the sense of *sambodhana*. Sometimes called the *vocative case*.
    ///
    /// *Sambodhana* is technically not a *vibhakti but rather an additional semantic condition
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

/// The information required to derive a subanta in the grammar.
pub struct SubantaArgs {
    linga: Linga,
    vacana: Vacana,
    vibhakti: Vibhakti,
}

impl SubantaArgs {
    /// The linga to use in the derivation.
    pub fn linga(&self) -> Linga {
        self.linga
    }

    /// The vacana to use in the derivation.
    pub fn vacana(&self) -> Vacana {
        self.vacana
    }

    /// The vibhakti to use in the derivation.
    pub fn vibhakti(&self) -> Vibhakti {
        self.vibhakti
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> SubantaArgsBuilder {
        SubantaArgsBuilder::default()
    }
}

/// Convenience struct for building a `SubantaArgs` struct.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SubantaArgsBuilder {
    linga: Option<Linga>,
    vacana: Option<Vacana>,
    vibhakti: Option<Vibhakti>,
}

impl SubantaArgsBuilder {
    /// Sets the linga to use in the derivation.
    pub fn linga(&mut self, val: Linga) -> &mut Self {
        self.linga = Some(val);
        self
    }

    /// Sets the vacana to use in the derivation.
    pub fn vacana(&mut self, val: Vacana) -> &mut Self {
        self.vacana = Some(val);
        self
    }

    /// Sets the vibhakti to use in the derivation.
    pub fn vibhakti(&mut self, val: Vibhakti) -> &mut Self {
        self.vibhakti = Some(val);
        self
    }

    /// Converts the arguments in this builder into a `SubantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<SubantaArgs, Error> {
        Ok(SubantaArgs {
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
        })
    }
}
