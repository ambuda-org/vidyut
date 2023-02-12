use crate::enum_boilerplate;
use crate::errors::Error;
use crate::tag::Tag;
use wasm_bindgen::prelude::wasm_bindgen;

/// The prayoga of some tinanta.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[wasm_bindgen]
pub enum Prayoga {
    /// Usage coreferent with the agent, e.g. "The horse *goes* to the village."
    Kartari,
    /// Usage coreferent with the object, e.g. "The village *is gone to* by the horse."
    Karmani,
    /// Usage without a referent, e.g. "*There is motion* by the horse to the village."
    /// bhAve prayoga generally produces the same forms as karmani prayoga.
    Bhave,
}

enum_boilerplate!(Prayoga, {
    Kartari => "kartari",
    Karmani => "karmani",
    Bhave => "bhave",
});

impl Prayoga {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Kartari => Tag::Kartari,
            Self::Karmani => Tag::Karmani,
            Self::Bhave => Tag::Bhave,
        }
    }
}

/// The person of some tinanta.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

/// The number of some tinanta or subanta.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

/// The tense/mood of some tinanta.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

/// The pada of some tinanta.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[wasm_bindgen]
pub enum Pada {
    /// Parasmaipada.
    Parasmai,
    /// Atmanepada.
    Atmane,
}

enum_boilerplate!(Pada, {
    Parasmai => "parasmai",
    Atmane => "atmane",
});

impl Pada {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Parasmai => Tag::Parasmaipada,
            Self::Atmane => Tag::Atmanepada,
        }
    }
}

/// The information required to derive a tinanta in the grammar.
///
/// If a tinanta were just a matter of prayoga/purusha/lakara/vacana, a struct like this would not
/// be necessary. However, a tinanta's derivation can have many other constraints, including:
///
/// - specific upasargas or other prefixes
/// - specific sanAdi pratyayas
/// - other constraints on the overall derivation
///
/// Since we want to keep these args manageable and don't want to repeatedly break our main API, we
/// decided to wrap args in this struct and expose its values through accessors.
pub struct TinantaArgs {
    prayoga: Prayoga,
    purusha: Purusha,
    lakara: Lakara,
    vacana: Vacana,
    pada: Option<Pada>,
}

impl TinantaArgs {
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

    /// The vacana to use in the derivation.
    pub fn vacana(&self) -> Vacana {
        self.vacana
    }

    /// (optional) The pada to use in the derivation.
    ///
    /// If unset, the program will use whatever padas are allowed by the Atmanepada section of the
    /// Ashtadhyayi. See the `atmanepada` module for details.
    pub fn pada(&self) -> Option<Pada> {
        self.pada
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> TinantaArgsBuilder {
        TinantaArgsBuilder::default()
    }
}

/// Convenience struct for building a `TinantaArgs` object.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct TinantaArgsBuilder {
    prayoga: Option<Prayoga>,
    purusha: Option<Purusha>,
    lakara: Option<Lakara>,
    vacana: Option<Vacana>,
    pada: Option<Pada>,
}

impl TinantaArgsBuilder {
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

    /// Sets the pada to use in the derivation.
    ///
    /// If unset, the program will use whatever padas are allowed by the Atmanepada section of the
    /// Ashtadhyayi. See the `atmanepada` module for details.
    pub fn pada(mut self, val: Pada) -> Self {
        self.pada = Some(val);
        self
    }

    /// Converts the arguments in this builder into a `TinantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(self) -> Result<TinantaArgs, Error> {
        Ok(TinantaArgs {
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
        })
    }
}
