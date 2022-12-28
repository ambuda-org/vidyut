use crate::args::errors::*;
use crate::tag::Tag;
use compact_str::CompactString;
use std::str::FromStr;

/// Defines an antargana.
///
/// The dhatus in the Dhatupatha are organized in ten large *gaṇa*s or classes. Within these larger
/// *gaṇa*s, certain *antargaṇa*s or subclasses have extra properties that affect the derivations
/// they produce. For example, dhatus in the *kuṭādi antargaṇa* generally do not allow *guṇa* vowel
/// changes.
///
/// Since most dhatus appear exactly once per *gaṇa*, this crate can usually infer whether a dhatu
/// is in a specific *antargaṇa*. However, some *gaṇa*s have dhatus that repeat, and these
/// repeating dhatus cause ambiguities for our code. (Examples: `juqa~` appears twice in
/// *tudādigaṇa*, once in *kuṭādi* and once outside of it.)
///
/// To avoid this ambiguity, we require that certain *antargaṇa*s are declared up-front.
///
/// (Can't we disambiguate by checking the dhatu's index within its gana? Unfortunately, no. There
/// is no canonical version of the Dhatupatha, and we cannot expect that a dhatu's index is
/// consistent across all of these versions. So we thought it better to avoid hard-coding indices
/// or requiring callers to follow our specific conventions.)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Antargana {
    /// Antargana of *tud* gana. Pratyayas that follow dhatus in kut-Adi will generally be marked
    /// Nit per 1.2.1. Required because of duplicates like `juqa~`.
    Kutadi,
    /// Antargana of *cur* gana ending with `kusma~`. A dhatu in this antargana is always
    /// ātmanepadī. Required because of duplicates like `daSi~`.
    Akusmiya,
}

/// One of the three common *sanAdi* pratyayas.
///
/// The *sanAdi* pratyayas create new dhatus per 3.1.32. They are introduced in rules 3.1.7 -
/// 3.1.30, and since rule 3.1.7 contains the word "dhAtoH", they can be called Ardhadhatuka by
/// 3.4.114.
///
/// Of the sanAdi pratyayas, most are added after either subantas or a handful of dhatus. But
/// three of these pratyayas are added after dhatus more generally: `san`, `yaN`, and `Ric`.
///
/// For details on what these pratyayas mean and what kinds of words they produce, see the comments
/// below.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Sanadi {
    /// `san`, which creates desiderative roots per 3.1.7.
    ///
    /// Examples: buBUzati, ninIzati.
    San,
    /// `yaN`, which creates intensive roots per 3.1.22. For certain dhatus, the semantics are
    /// instead "crooked movement" (by 3.1.23) or "contemptible" action (by 3.1.24).
    ///
    /// Examples: boBUyate, nenIyate.
    ///
    /// Constraints: can be used only if the dhatu starts with a consonant and has exactly one
    /// vowel. If this constraint is violated, our APIs will return an `ArgumentError`.
    Yan,
    /// `yaN`, with elision per 2.4.74. This is often listed separately due to its rarity and its
    /// very different form.
    ///
    /// Examples: boBavIti, boBoti, nenayIti, neneti.
    YanLuk,
    /// `Nic`, which creates causal roots per 3.1.26.
    ///
    /// Examples: BAvayati, nAyayati.
    Nic,
}

impl Sanadi {
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::San => "san",
            Self::Yan => "yan",
            Self::YanLuk => "yan-luk",
            Self::Nic => "nic",
        }
    }
}

impl FromStr for Sanadi {
    type Err = ArgumentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "san" => Self::San,
            "yan" => Self::Yan,
            "yan-luk" => Self::YanLuk,
            "nic" => Self::Nic,
            &_ => return Err(ArgumentError::enum_parse_error("Sanadi", s)),
        };
        Ok(res)
    }
}

/// The prayoga of some tinanta.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Prayoga {
    /// Usage coreferent with the agent, e.g. "The horse *goes* to the village."
    Kartari,
    /// Usage coreferent with the object, e.g. "The village *is gone to* by the horse."
    Karmani,
    /// Usage without a referent, e.g. "*There is motion* by the horse to the village."
    /// bhAve prayoga generally produces the same forms as karmani prayoga.
    Bhave,
}

impl Prayoga {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Kartari => Tag::Kartari,
            Self::Karmani => Tag::Karmani,
            Self::Bhave => Tag::Bhave,
        }
    }
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Kartari => "kartari",
            Self::Karmani => "karmani",
            Self::Bhave => "bhave",
        }
    }
}

impl FromStr for Prayoga {
    type Err = ArgumentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "kartari" => Self::Kartari,
            "karmani" => Self::Karmani,
            "bhave" => Self::Bhave,
            &_ => return Err(ArgumentError::enum_parse_error("Prayoga", s)),
        };
        Ok(res)
    }
}

/// The person of some tinanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Purusha {
    /// The third person.
    Prathama,
    /// The second person.
    Madhyama,
    /// The first person.
    Uttama,
}

impl Purusha {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Prathama => Tag::Prathama,
            Self::Madhyama => Tag::Madhyama,
            Self::Uttama => Tag::Uttama,
        }
    }
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Prathama => "prathama",
            Self::Madhyama => "madhyama",
            Self::Uttama => "uttama",
        }
    }
}

impl FromStr for Purusha {
    type Err = ArgumentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "prathama" => Self::Prathama,
            "madhyama" => Self::Madhyama,
            "uttama" => Self::Uttama,
            &_ => return Err(ArgumentError::enum_parse_error("Purusha", s)),
        };
        Ok(res)
    }
}

/// The number of some tinanta or subanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vacana {
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}
impl Vacana {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Eka => Tag::Ekavacana,
            Self::Dvi => Tag::Dvivacana,
            Self::Bahu => Tag::Bahuvacana,
        }
    }
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eka => "eka",
            Self::Dvi => "dvi",
            Self::Bahu => "bahu",
        }
    }
}
impl FromStr for Vacana {
    type Err = ArgumentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "eka" => Self::Eka,
            "dvi" => Self::Dvi,
            "bahu" => Self::Bahu,
            &_ => return Err(ArgumentError::enum_parse_error("Vacana", s)),
        };
        Ok(res)
    }
}

/// The tense/mood of some tinanta.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl Lakara {
    /// Returns a simple human-readable string that represents this enum's value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Lakara::Lat => "lat",
            Lakara::Lit => "lit",
            Lakara::Lut => "lut",
            Lakara::Lrt => "lrt",
            Lakara::Let => "let",
            Lakara::Lot => "lot",
            Lakara::Lan => "lan",
            Lakara::VidhiLin => "vidhi-lin",
            Lakara::AshirLin => "ashir-lin",
            Lakara::Lun => "lun",
            Lakara::Lrn => "lrn",
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

impl FromStr for Lakara {
    type Err = ArgumentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "lat" => Self::Lat,
            "lit" => Self::Lit,
            "lut" => Self::Lut,
            "lrt" => Self::Lrt,
            "let" => Self::Let,
            "lot" => Self::Lot,
            "lan" => Self::Lan,
            "vidhi-lin" => Self::VidhiLin,
            "ashir-lin" => Self::AshirLin,
            "lun" => Self::Lun,
            "lrn" => Self::Lrn,
            &_ => return Err(ArgumentError::enum_parse_error("Lakara", s)),
        };
        Ok(res)
    }
}

/// The verb root to use for the derivation.
#[derive(Debug)]
pub struct Dhatu {
    /// The dhatu as stated in its aupadeshka form. `upadesha` should be an SLP1 string that
    /// includes any necessary svaras. For examples, see the `dhatu` column in the
    /// `data/dhatupatha.tsv` file included in this crate.
    pub upadesha: CompactString,
    /// The dhatu's gana. This should be a number between 1 and 10, inclusive.
    pub gana: u8,
    /// The antargana this Dhatu belongs to.
    pub antargana: Option<Antargana>,
    /// The sanAdi pratyayas to add after the dhatu.
    pub sanadi: Vec<Sanadi>,
}

impl Dhatu {
    /// Creates a new dhatu with its gana.
    ///
    /// This is a convenience function for simple, straightforward dhatus. For more customization,
    /// use the `builder()` API instead.
    pub fn new(upadesha: &str, gana: u8) -> Self {
        Self {
            upadesha: CompactString::from(upadesha),
            gana,
            antargana: None,
            sanadi: Vec::new(),
        }
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> DhatuBuilder {
        DhatuBuilder::default()
    }
}

/// Convenience struct for building a `Dhatu` object.
#[derive(Default)]
pub struct DhatuBuilder {
    upadesha: Option<CompactString>,
    gana: Option<u8>,
    antargana: Option<Antargana>,
    sanadi: Vec<Sanadi>,
}

impl DhatuBuilder {
    /// Sets the upadesha of the dhatu.
    pub fn upadesha(mut self, text: &str) -> Self {
        self.upadesha = Some(CompactString::from(text));
        self
    }

    /// Sets the gana of the dhatu.
    pub fn gana(mut self, value: u8) -> Self {
        self.gana = Some(value);
        self
    }

    /// Sets the antargana of the dhatu, if one is necessary.
    pub fn antargana(mut self, value: Antargana) -> Self {
        self.antargana = Some(value);
        self
    }

    /// Sets the `sanAdi` pratyaya to add to the dhatu.
    pub fn sanadi(mut self, values: &[Sanadi]) -> Self {
        self.sanadi.clear();
        self.sanadi.extend(values);
        self
    }

    /// Helper function for creating error messages.
    fn field_missing(name: &str) -> ArgumentError {
        ArgumentError::new(&format!("Please define the `{name}` field."))
    }

    /// Converts the arguments in this builder into a `Dhatu` struct.
    pub fn build(self) -> Result<Dhatu, ArgumentError> {
        Ok(Dhatu {
            upadesha: match self.upadesha {
                Some(x) => x,
                _ => return Err(Self::field_missing("upadesha")),
            },
            gana: match self.gana {
                Some(x) => {
                    if (1..=10).contains(&x) {
                        x
                    } else {
                        return Err(ArgumentError::new(
                            "Received invalid value for `gana` (must be between 1 and 10",
                        ));
                    }
                }
                _ => return Err(Self::field_missing("gana")),
            },
            antargana: self.antargana,
            sanadi: self.sanadi,
        })
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

    /// Returns a new builder for this struct.
    pub fn builder() -> TinantaArgsBuilder {
        TinantaArgsBuilder::default()
    }
}

/// Convenience struct for building a `TinantaArgs` object.
#[derive(Default)]
pub struct TinantaArgsBuilder {
    prayoga: Option<Prayoga>,
    purusha: Option<Purusha>,
    lakara: Option<Lakara>,
    vacana: Option<Vacana>,
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

    /// Helper function for creating error messages.
    fn field_missing(name: &str) -> ArgumentError {
        ArgumentError::new(&format!("Please define the `{name}` field."))
    }

    /// Converts the arguments in this builder into a `TinantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(self) -> Result<TinantaArgs, ArgumentError> {
        Ok(TinantaArgs {
            prayoga: match self.prayoga {
                Some(x) => x,
                _ => return Err(Self::field_missing("prayoga")),
            },
            purusha: match self.purusha {
                Some(x) => x,
                _ => return Err(Self::field_missing("purusha")),
            },
            lakara: match self.lakara {
                Some(x) => x,
                _ => return Err(Self::field_missing("lakara")),
            },
            vacana: match self.vacana {
                Some(x) => x,
                _ => return Err(Self::field_missing("vacana")),
            },
        })
    }
}
