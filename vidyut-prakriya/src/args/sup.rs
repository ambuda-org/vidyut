use crate::args::errors::*;
use crate::args::tin::Vacana;
use crate::tag::Tag;
use std::str::FromStr;

/// The gender of some subanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Linga {
    /// The masculine.
    Pum,
    /// The feminine.
    Stri,
    /// The neuter.
    Napumsaka,
}
impl Linga {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Pum => Tag::Pum,
            Self::Stri => Tag::Stri,
            Self::Napumsaka => Tag::Napumsaka,
        }
    }
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pum => "pum",
            Self::Stri => "stri",
            Self::Napumsaka => "napumsaka",
        }
    }
}
impl FromStr for Linga {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "pum" => Self::Pum,
            "stri" => Self::Stri,
            "napumsaka" => Self::Napumsaka,
            &_ => return Err("Could not parse Linga"),
        };
        Ok(res)
    }
}

/// The case ending of some subanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// that conditions the first vibhakti. But we felt that users would find it more convenient to
    /// have this condition available on `Vibhakti` directly rather than have to define the
    /// *sambodhana* condition separately.
    Sambodhana,
}
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
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Prathama => "1",
            Self::Dvitiya => "2",
            Self::Trtiya => "3",
            Self::Caturthi => "4",
            Self::Panchami => "5",
            Self::Sasthi => "6",
            Self::Saptami => "7",
            Self::Sambodhana => "s",
        }
    }
}
impl FromStr for Vibhakti {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "1" => Self::Prathama,
            "2" => Self::Dvitiya,
            "3" => Self::Trtiya,
            "4" => Self::Caturthi,
            "5" => Self::Panchami,
            "6" => Self::Sasthi,
            "7" => Self::Saptami,
            "s" => Self::Sambodhana,
            &_ => return Err("Could not parse Vibhakti"),
        };
        Ok(res)
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

/// Convenience struct for building a `SubantaArgs` object.
#[derive(Default)]
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
    pub fn build(&self) -> Result<SubantaArgs, ArgumentError> {
        Ok(SubantaArgs {
            linga: match self.linga {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
            vacana: match self.vacana {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
            vibhakti: match self.vibhakti {
                Some(x) => x,
                _ => return Err(ArgumentError::new("foo")),
            },
        })
    }
}
