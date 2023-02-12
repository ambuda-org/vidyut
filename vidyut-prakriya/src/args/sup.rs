use crate::args::tin::Vacana;
use crate::enum_boilerplate;
use crate::errors::Error;
use crate::tag::Tag;
use enumset::EnumSet;
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

/// The verb root to use for the derivation.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Pratipadika {
    text: String,
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
    pub fn text(&self) -> &String {
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
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PratipadikaBuilder {
    text: Option<String>,
    is_nyap: bool,
    is_dhatu: bool,
    is_pratyaya: bool,
}

impl PratipadikaBuilder {
    /// Sets the text of the pratipadika.
    pub fn text(&mut self, value: impl AsRef<str>) -> &mut Self {
        self.text = Some(String::from(value.as_ref()));
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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
