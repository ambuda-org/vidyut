use crate::args::tin::Vacana;
use crate::errors::Error;
use crate::tag::Tag;
use compact_str::CompactString;
use enumset::EnumSet;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;

/// The gender of some subanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[wasm_bindgen]
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
    /// that conditions the first vibhakti. But we felt that users would find it more convenient to
    /// have this condition available on `Vibhakti` directly rather than have to define the
    /// *sambodhana* condition separately.
    Sambodhana,
}

const VIBHAKTIS: &[Vibhakti] = &[
    Vibhakti::Prathama,
    Vibhakti::Dvitiya,
    Vibhakti::Trtiya,
    Vibhakti::Caturthi,
    Vibhakti::Panchami,
    Vibhakti::Sasthi,
    Vibhakti::Saptami,
    Vibhakti::Sambodhana,
];

impl Vibhakti {
    /// Iterates over the values of `Vibhakti` in order.
    pub fn iter() -> impl Iterator<Item = &'static Vibhakti> {
        VIBHAKTIS.iter()
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

impl FromStr for Vibhakti {
    type Err = Error;
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
            &_ => return Err(Error::enum_parse_error(s)),
        };
        Ok(res)
    }
}

/// The verb root to use for the derivation.
#[derive(Debug, Default, Clone, Hash)]
pub struct Pratipadika {
    text: CompactString,
    tags: EnumSet<Tag>,
}

impl Pratipadika {
    /// Creates a new pratipadika.
    pub fn new(text: impl AsRef<str>) -> Self {
        Pratipadika::builder()
            .text(text.as_ref())
            .build()
            .expect("should have text")
    }

    /// The text of this pratipadika.
    pub fn text(&self) -> &CompactString {
        &self.text
    }

    /// Returns whether this pratipadika ends in `NI` or `Ap.`
    pub fn is_nyap(&self) -> bool {
        self.tags.contains(Tag::StriNyap)
    }

    /// Returns whether this pratipadika ends in a dhatu.
    pub fn is_dhatu(&self) -> bool {
        self.tags.contains(Tag::Dhatu)
    }

    /// Returns whether this pratipadika ends in a pratyaya.
    pub fn is_pratyaya(&self) -> bool {
        self.tags.contains(Tag::Pratyaya)
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> PratipadikaBuilder {
        PratipadikaBuilder::default()
    }
}

/// Convenience struct for building a `Pratipadika` struct.
#[derive(Default)]
pub struct PratipadikaBuilder {
    text: Option<CompactString>,
    is_nyap: bool,
    is_dhatu: bool,
    is_pratyaya: bool,
}

impl PratipadikaBuilder {
    /// Sets the text of the pratipadika.
    pub fn text(&mut self, value: impl AsRef<str>) -> &mut Self {
        self.text = Some(CompactString::from(value.as_ref()));
        self
    }

    /// Sets whether this pratipadika should be treated as ending in `NI` or `Ap`.
    pub fn is_nyap(&mut self, yes: bool) -> &mut Self {
        self.is_nyap = yes;
        self
    }

    /// Sets whether this pratipadika should be treated as ending in a dhatu.
    pub fn is_dhatu(&mut self, yes: bool) -> &mut Self {
        self.is_dhatu = yes;
        self
    }

    /// Sets whether this pratipadika should be treated as ending in a dhatu.
    pub fn is_pratyaya(&mut self, yes: bool) -> &mut Self {
        self.is_pratyaya = yes;
        self
    }

    /// Converts the arguments in this builder into a `Pratipadika` struct.
    ///
    /// `build()` will fail if `text` is missing.
    pub fn build(&self) -> Result<Pratipadika, Error> {
        Ok(Pratipadika {
            text: match &self.text {
                Some(x) => x.clone(),
                None => return Err(Error::MissingRequiredField("text")),
            },
            tags: self.create_tags()?,
        })
    }

    fn create_tags(&self) -> Result<EnumSet<Tag>, Error> {
        let mut tags = EnumSet::default();
        if self.is_nyap {
            tags.insert(Tag::StriNyap);
        }
        if self.is_dhatu {
            tags.insert(Tag::Dhatu);
        }
        if self.is_dhatu {
            tags.insert(Tag::Pratyaya);
        }

        Ok(tags)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pratipadika() {
        let deva = Pratipadika::new("deva");
        assert_eq!(deva.text(), &"deva");
        assert!(!deva.is_nyap());
        assert!(!deva.is_dhatu());
    }

    #[test]
    fn create_pratipadika_with_nyap() {
        let mala = Pratipadika::builder()
            .text("mAlA")
            .is_nyap(true)
            .build()
            .unwrap();
        assert_eq!(mala.text(), &"mAlA");
        assert!(mala.is_nyap());
        assert!(!mala.is_dhatu());
    }

    #[test]
    fn create_pratipadika_with_dhatu() {
        let senani = Pratipadika::builder()
            .text("senAnI")
            .is_dhatu(true)
            .build()
            .unwrap();
        assert_eq!(senani.text(), &"senAnI");
        assert!(senani.is_dhatu());
        assert!(!senani.is_nyap());
    }
}
