use crate::args::dhatu::Dhatu;
use crate::core::errors::Error;
use crate::core::{PrakriyaTag, Tag};
use crate::enum_boilerplate;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The *prayoga* of some *tiṅanta*.
///
/// *Prayoga* is roughly similar to the Western concept of verb *voice*.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wasm_bindgen]
pub enum Prayoga {
    /// Usage coreferent with the agent, e.g. "The horse *goes* to the village."
    Kartari,
    /// Usage coreferent with the object, e.g. "The village *is gone to* by the horse."
    Karmani,
    /// Usage without a referent, e.g. "*There is motion* by the horse to the village."
    /// *bhāve prayoga* generally produces the same forms as karmani prayoga.
    Bhave,
}

enum_boilerplate!(Prayoga, {
    Kartari => "kartari",
    Karmani => "karmani",
    Bhave => "bhave",
});

impl Prayoga {
    pub(crate) fn as_tag(&self) -> PrakriyaTag {
        match self {
            Self::Kartari => PrakriyaTag::Kartari,
            Self::Karmani => PrakriyaTag::Karmani,
            Self::Bhave => PrakriyaTag::Bhave,
        }
    }
}

/// The person of some *tiṅanta*.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wasm_bindgen]
pub enum Purusha {
    /// The third person.
    Prathama,
    /// The second person.
    Madhyama,
    /// The first person.
    Uttama,
}

enum_boilerplate!(Purusha, {
    Prathama => "prathama",
    Madhyama => "madhyama",
    Uttama => "uttama",
});

impl Purusha {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Prathama => Tag::Prathama,
            Self::Madhyama => Tag::Madhyama,
            Self::Uttama => Tag::Uttama,
        }
    }
}

/// The number of some *tiṅanta* or *subanta*.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wasm_bindgen]
pub enum Vacana {
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}

enum_boilerplate!(Vacana, {
    Eka => "eka",
    Dvi => "dvi",
    Bahu => "bahu",
});

impl Vacana {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Eka => Tag::Ekavacana,
            Self::Dvi => Tag::Dvivacana,
            Self::Bahu => Tag::Bahuvacana,
        }
    }
}

/// The tense/mood of some *tiṅanta*.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wasm_bindgen]
pub enum Lakara {
    /// Describes action in the present tense. Ssometimes called the *present indicative*.
    Lat,
    /// Describes unwitnessed past action. Sometimes called the *perfect*.
    Lit,
    /// Describes future action after the current day. Sometimes called the *periphrastic future*.
    Lut,
    /// Describes general future action. Sometimes called the *simple future*.
    Lrt,
    /// The Vedic subjunctive. `vidyut-prakriya` currently has poor support for this lakara.
    Let,
    /// Describes commands. Sometimes called the *imperative*.
    Lot,
    /// Describes past action before the current day. Sometimes called the *imperfect*.
    Lan,
    /// Describes potential or hypothetical actions. Sometimes called the *optative*.
    VidhiLin,
    /// Describes wishes and prayers. Sometimes called the *benedictive*.
    AshirLin,
    /// Describes general past action. Sometimes called the *aorist*.
    Lun,
    /// Describes past counterfactuals ("would not have ..."). Sometimes called the *conditional*.
    Lrn,
}

enum_boilerplate!(Lakara, {
    Lat => "lat",
    Lit => "lit",
    Lut => "lut",
    Lrt => "lrt",
    Let => "let",
    Lot => "lot",
    Lan => "lan",
    VidhiLin => "vidhi-lin",
    AshirLin => "ashir-lin",
    Lun => "lun",
    Lrn => "lrn",
});

impl Lakara {
    /// Returns the *aupadeśika* form of this *pratyaya*.
    pub fn aupadeshika(&self) -> &'static str {
        use Lakara::*;
        match self {
            Lat => "la~w",
            Lit => "li~w",
            Lut => "lu~w",
            Lrt => "lf~w",
            Let => "le~w",
            Lot => "lo~w",
            Lan => "laN",
            VidhiLin => "li~N",
            AshirLin => "li~N",
            Lun => "lu~N",
            Lrn => "lf~N",
        }
    }

    /// Returns whether or not this lakara is Nit.
    pub(crate) fn is_nit(&self) -> bool {
        matches![
            self,
            Lakara::Lan | Lakara::AshirLin | Lakara::VidhiLin | Lakara::Lun | Lakara::Lrn
        ]
    }

    /// Returns whether or not this lakara will be termed sArvadhAtuka.
    pub(crate) fn is_sarvadhatuka(&self) -> bool {
        matches!(
            self,
            Lakara::Lat | Lakara::Lot | Lakara::Lan | Lakara::VidhiLin
        )
    }

    /// Returns whether or not this lakara will be termed ArdhadhAtuka.
    pub(crate) fn is_ardhadhatuka(&self) -> bool {
        !self.is_sarvadhatuka()
    }
}

/// The pada of some *tiṅanta* or *kṛdanta*.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wasm_bindgen]
pub enum DhatuPada {
    /// *Parasmaipada*.
    Parasmai,
    /// *Ātmanepada*.
    Atmane,
}

enum_boilerplate!(DhatuPada, {
    Parasmai => "parasmai",
    Atmane => "atmane",
});

impl DhatuPada {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Parasmai => Tag::Parasmaipada,
            Self::Atmane => Tag::Atmanepada,
        }
    }
}

/// The information required to derive a *tiṅanta*.
///
/// A *tiṅanta* (verb) is any word that ends with one of the eighteen suffixes in the *tiṅ* list:
///
/// | Singular    | Dual        | Plural      |
/// |-------------|-------------|-------------|
/// | *tip*       | *tas*       | *jhi (nti)* |
/// | *sip*       | *tas*       | *tha*       |
/// | *mip*       | *vas*       | *mas*       |
///
/// | Singular    | Dual        | Plural      |
/// |-------------|-------------|-------------|
/// | *ta*        | *ātām*      | *jha (nta)* |
/// | *thās*      | *āthām*     | *dhvam*     |
/// | *iṭ*        | *vahi*      | *mahiṅ*     |
///
/// If a *tiṅanta* were just a matter of prayoga/purusha/lakara/vacana, a struct like this would
/// not be necessary. However, a *tiṅanta*'s derivation can have many other constraints, including:
///
/// - specific *upasarga*s or other prefixes
/// - specific *sanādi pratyaya*s
/// - whether or not we should skip adding the *aṭ*/*āṭ* *āgama* for certain past forms.
/// - other constraints on the overall derivation
///
/// Since we want to keep these args manageable and don't want to repeatedly break our main API, we
/// decided to wrap args in this struct and expose its values through accessors.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tinanta {
    dhatu: Dhatu,
    prayoga: Prayoga,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    pada: Option<DhatuPada>,
    skip_at_agama: bool,
}

impl Tinanta {
    /// Creates a new `Tinanta`.
    ///
    /// For more options, see `Tinanta::build()`.
    pub fn new(
        dhatu: Dhatu,
        prayoga: Prayoga,
        lakara: Lakara,
        purusha: Purusha,
        vacana: Vacana,
    ) -> Self {
        Self {
            dhatu,
            prayoga,
            lakara,
            purusha,
            vacana,
            pada: None,
            skip_at_agama: false,
        }
    }

    /// The dhatu to use in the derivation.
    pub fn dhatu(&self) -> &Dhatu {
        &self.dhatu
    }

    /// The linga to use in the derivation.
    pub fn prayoga(&self) -> Prayoga {
        self.prayoga
    }

    /// The purusha to use in the derivation.
    pub fn purusha(&self) -> Purusha {
        self.purusha
    }

    /// The lakara to use in the derivation.
    pub fn lakara(&self) -> Lakara {
        self.lakara
    }

    /// The *vacana* to use in the derivation.
    pub fn vacana(&self) -> Vacana {
        self.vacana
    }

    /// Whether or not the *aṭ* and *āṭ* *āgama*s should be skipped in the derivation.
    pub fn skip_at_agama(&self) -> bool {
        self.skip_at_agama
    }

    /// (optional) The pada to use in the derivation.
    ///
    /// If unset, the program will use whatever padas are allowed by the Atmanepada section of the
    /// Ashtadhyayi. See the `atmanepada` module for details.
    pub fn pada(&self) -> Option<DhatuPada> {
        self.pada
    }

    /// Returns an updated version of `self` with the given `dhatu`.
    pub fn with_dhatu(mut self, dhatu: Dhatu) -> Self {
        self.dhatu = dhatu;
        self
    }

    /// Returns a new builder for this struct.
    ///
    /// For details, see `TinantaArgsBuilder`.
    pub fn builder() -> TinantaArgsBuilder {
        TinantaArgsBuilder::default()
    }
}

/// Convenience struct for building a `TinantaArgs` object.
///
///
/// ### Example
///
/// ````
/// # use vidyut_prakriya::*;
/// use vidyut_prakriya::args::*;
///
/// let dhatu = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi);
/// let args = Tinanta::builder()
///     .dhatu(dhatu)
///     .lakara(Lakara::Lat)
///     .prayoga(Prayoga::Kartari)
///     .purusha(Purusha::Prathama)
///     .vacana(Vacana::Eka)
///     .build()?;
/// # Ok::<(), Error>(())
/// ````
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct TinantaArgsBuilder {
    dhatu: Option<Dhatu>,
    prayoga: Option<Prayoga>,
    purusha: Option<Purusha>,
    lakara: Option<Lakara>,
    vacana: Option<Vacana>,
    pada: Option<DhatuPada>,
    skip_at_agama: bool,
}

impl TinantaArgsBuilder {
    /// Sets the dhatu to use in the derivation.
    pub fn dhatu(mut self, val: Dhatu) -> Self {
        self.dhatu = Some(val);
        self
    }

    /// Sets the prayoga to use in the derivation.
    pub fn prayoga(mut self, val: Prayoga) -> Self {
        self.prayoga = Some(val);
        self
    }

    /// Sets the purusha to use in the derivation.
    pub fn purusha(mut self, val: Purusha) -> Self {
        self.purusha = Some(val);
        self
    }

    /// Sets the lakara to use in the derivation.
    pub fn lakara(mut self, val: Lakara) -> Self {
        self.lakara = Some(val);
        self
    }

    /// Sets the vacana to use in the derivation.
    pub fn vacana(mut self, val: Vacana) -> Self {
        self.vacana = Some(val);
        self
    }

    /// Sets whether or not to skip the `a` Agama in the derivation.
    pub fn skip_at_agama(mut self, val: bool) -> Self {
        self.skip_at_agama = val;
        self
    }

    /// Sets the pada to use in the derivation.
    ///
    /// If unset, the program will use whatever padas are allowed by the Atmanepada section of the
    /// Ashtadhyayi. See the `atmanepada` module for details.
    pub fn pada(mut self, val: DhatuPada) -> Self {
        self.pada = Some(val);
        self
    }

    /// Converts the arguments in this builder into a `TinantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(self) -> Result<Tinanta, Error> {
        Ok(Tinanta {
            dhatu: match self.dhatu {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("dhatu")),
            },
            prayoga: match self.prayoga {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("prayoga")),
            },
            purusha: match self.purusha {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("purusha")),
            },
            lakara: match self.lakara {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("lakara")),
            },
            vacana: match self.vacana {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("vacana")),
            },
            pada: self.pada,
            skip_at_agama: self.skip_at_agama,
        })
    }
}
