//! Models the semantics of Sanskrit words, including their stems and endings.
//!
//! For details on how we represent semantics, see the `Pada` enum and its comments.
//!
//! We designed this module with the following design principles in mind:
//!
//! 1. Aim for pragmatism. Our goal is to model Sanskrit words with enough detail to be useful but
//!    not with so much detail that we merely replicate the Ashtadhyayi.
//!
//! 2. Prefer traditional terms. The vocabulary and conceptual schema of traditional Sanskrit
//!    grammar was designed specifically for Sanskrit and fits Sanskrit like a glove.
//!
//! 3. Prefer morphological names. For example, we refer to the various senses of the `-tum` suffix
//!    with the simple label `KrtPratyaya::Tum`. For a counterexample, we explicitly model `Linga`,
//!    `Vacana`, `Vibhakti`, etc. because using a single `Sup` enum is more trouble than it's
//!    worth.

use crate::errors::*;
use modular_bitfield::prelude::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Lemma for `None` semantics or any other case where the lemma is unknown.
pub const NONE_LEMMA: &str = "[none]";

/// Utility struct for reading complex serialized enums.
struct FeatureMap(HashMap<String, String>);

impl FeatureMap {
    fn from_str(s: &str) -> Self {
        let map = s
            .split('|')
            .flat_map(|x| x.split_once('='))
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect::<HashMap<_, _>>();

        FeatureMap(map)
    }
    fn get(&self, s: &str) -> Result<&String> {
        if let Some(val) = self.0.get(s) {
            Ok(val)
        } else {
            Err(Error::Generic(format!(
                "Could not parse `{s}` as a feature map."
            )))
        }
    }
}

/// The *liṅga* (gender) of a *subanta*.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[bits = 2]
pub enum Linga {
    /// Unknown or missing gender.
    None,
    /// The masculine gender.
    Pum,
    /// The feminine gender.
    Stri,
    /// The neuter gender.
    Napumsaka,
}

impl Linga {
    /// Returns a string representation of this enum.
    pub fn as_str(&self) -> &'static str {
        match self {
            Linga::Pum => "m",
            Linga::Stri => "f",
            Linga::Napumsaka => "n",
            Linga::None => "_",
        }
    }
}

impl FromStr for Linga {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let val = match s {
            "m" => Linga::Pum,
            "f" => Linga::Stri,
            "n" => Linga::Napumsaka,
            "_" => Linga::None,
            // Legacy format on `github.com/sanskrit/data`
            "none" => Linga::None,
            _ => return Err(Error::EnumParse("Linga", s.to_string())),
        };
        Ok(val)
    }
}

impl Display for Linga {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/// The *vacana* (number) of a *subanta* or *tiṅanta*.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[bits = 2]
pub enum Vacana {
    /// Unknown or missing *vacana*.
    None,
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}

impl Vacana {
    /// Returns a string representation of this enum.
    pub fn as_str(&self) -> &'static str {
        match self {
            Vacana::None => "_",
            Vacana::Eka => "s",
            Vacana::Dvi => "d",
            Vacana::Bahu => "p",
        }
    }
}

impl FromStr for Vacana {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let val = match s {
            "_" => Vacana::None,
            "s" => Vacana::Eka,
            "d" => Vacana::Dvi,
            "p" => Vacana::Bahu,
            _ => return Err(Error::EnumParse("Vacana", s.to_string())),
        };
        Ok(val)
    }
}

impl Display for Vacana {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/// The *vibhakti* (case) of a *subanta*.
///
/// The term *vibhakti* refers generally to any triad of inflectional endings for a *subanta*
/// or *tiṅanta*. Here, `Vibhakti` refers specifically to the *subanta* tridas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[bits = 4]
pub enum Vibhakti {
    /// Unknown or missing *vibhakti*.
    None,
    /// The first *vibhakti* (nominative case).
    V1,
    /// The second *vibhakti* (accusative case).
    V2,
    /// The third *vibhakti* (instrumental case).
    V3,
    /// The fourth *vibhakti* (dative case).
    V4,
    /// The fifth *vibhakti* (ablative case).
    V5,
    /// The sixth *vibhakti* (genitive case).
    V6,
    /// The seventh *vibhakti* (locative case).
    V7,
    /// The first *vibhakti* in the condition of *sambodhana* (vocative case).
    Sambodhana,
}

impl Vibhakti {
    /// Returns a string representation of this enum.
    pub fn as_str(&self) -> &'static str {
        match self {
            Vibhakti::V1 => "1",
            Vibhakti::V2 => "2",
            Vibhakti::V3 => "3",
            Vibhakti::V4 => "4",
            Vibhakti::V5 => "5",
            Vibhakti::V6 => "6",
            Vibhakti::V7 => "7",
            Vibhakti::Sambodhana => "8",
            Vibhakti::None => "_",
        }
    }
}

impl FromStr for Vibhakti {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let val = match s {
            "_" => Vibhakti::None,
            "1" => Vibhakti::V1,
            "2" => Vibhakti::V2,
            "3" => Vibhakti::V3,
            "4" => Vibhakti::V4,
            "5" => Vibhakti::V5,
            "6" => Vibhakti::V6,
            "7" => Vibhakti::V7,
            "8" => Vibhakti::Sambodhana,
            _ => return Err(Error::EnumParse("Vibhakti", s.to_string())),
        };
        Ok(val)
    }
}

impl Display for Vibhakti {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/// The *puruṣa* (person) of a *tiṅanta*.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[bits = 2]
pub enum Purusha {
    /// Unknown or missing *puruṣa*.
    None,
    /// The first *puruṣa* (third person).
    Prathama,
    /// The middle *puruṣa* (second person).
    Madhyama,
    /// The last *puruṣa* (first person).
    Uttama,
}

impl Purusha {
    /// Returns a string representation of this enum.
    pub fn as_str(&self) -> &'static str {
        match self {
            Purusha::None => "_",
            Purusha::Prathama => "3",
            Purusha::Madhyama => "2",
            Purusha::Uttama => "1",
        }
    }
}

impl FromStr for Purusha {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        use Purusha::*;
        let val = match s {
            "_" => None,
            "3" => Prathama,
            "2" => Madhyama,
            "1" => Uttama,
            _ => return Err(Error::EnumParse("Purusha", s.to_string())),
        };
        Ok(val)
    }
}

impl Display for Purusha {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/// The *lakāra* (tense/mood) of a *tiṅanta*.
///
/// The *lakāras* are morphological categories, but each typically expresses a specific meaning.
/// For example, *laṭ-lakāra* almost always expresses an action in the present tense.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[bits = 4]
pub enum Lakara {
    /// Unknown or missing *lakāra*.
    None,
    /// *laṭ-lakāra* (present indicative).
    Lat,
    /// *liṭ-lakāra* (perfect).
    Lit,
    /// *luṭ-lakāra* (periphrastic future).
    Lut,
    /// *lṛṭ-lakāra* (simple future).
    Lrt,
    /// *leṭ-lakāra* (Vedic subjunctive).
    Let,
    /// *loṭ-lakāra* (imperative).
    Lot,
    /// *laṅ-lakāra* (imperfect).
    Lan,
    /// *liṅ-lakāra* in the sense of benediction (benedictive).
    LinAshih,
    /// *liṅ-lakāra* in the sense of a rule or injunction (optative).
    LinVidhi,
    /// *luṅ-lakāra* (aorist).
    Lun,
    /// *luṅ-lakāra* without its *a-* prefix (injunctive).
    LunNoAgama,
    /// *lṛṅ-lakāra* (conditional).
    Lrn,
}

impl Lakara {
    /// Returns a string representation of this enum.
    pub fn as_str(&self) -> &'static str {
        match self {
            Lakara::None => "_",
            Lakara::Lat => "lat",
            Lakara::Lit => "lit",
            Lakara::Lut => "lut",
            Lakara::Lrt => "lrt",
            Lakara::Let => "let",
            Lakara::Lot => "lot",
            Lakara::Lan => "lan",
            Lakara::LinVidhi => "lin-vidhi",
            Lakara::LinAshih => "lin-ashih",
            Lakara::Lun => "lun",
            Lakara::LunNoAgama => "lun-no-agama",
            Lakara::Lrn => "lrn",
        }
    }
}

impl FromStr for Lakara {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let val = match s {
            "_" => Lakara::None,
            "lat" => Lakara::Lat,
            "lit" => Lakara::Lit,
            "lut" => Lakara::Lut,
            "lrt" => Lakara::Lrt,
            "let" => Lakara::Let,
            "lot" => Lakara::Lot,
            "lan" => Lakara::Lan,
            "lin-vidhi" => Lakara::LinVidhi,
            "lin-ashih" => Lakara::LinAshih,
            "lun" => Lakara::Lun,
            "lun-no-agama" => Lakara::LunNoAgama,
            "lrn" => Lakara::Lrn,
            _ => return Err(Error::EnumParse("Lakara", s.to_string())),
        };
        Ok(val)
    }
}

impl Display for Lakara {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/// A *pratyaya* (suffix) that creates a new *dhātu* (verb root)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[bits = 2]
pub enum DhatuPratyaya {
    /// No specific *dhātu-pratyaya*.
    None,
    /// *ṇic-pratyaya* (*i*), which expresses a causal action.
    Nic,
    /// *san-pratyaya* (*sa*), which expresses a desiderative action.
    San,
    /// *yaṅ-pratyaya* (*ya*), which expresses an intensive or frequentative action.
    Yan,
}

/// A *kṛt-pratyaya* (root or primary suffix).
///
/// This list is not exhaustive.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KrtPratyaya {
    /// Unknown or missing *kṛt-pratyaya*.
    None,

    /// The *-tum* suffix (infinitive).
    Tumun,
    /// The *-tvā* suffix (unprefixed gerund).
    Ktva,
    /// The *-ya* suffix (prefixed gerund).
    Lyap,

    /// The *-vas* suffix (perfect participle).
    Kvasu,
    /// The -*āna* suffix (perfect participle).
    Kanac,

    /// The *-ta* suffix (past passive participle).
    Kta,
    /// The *-tavat* suffix (past active participle).
    Ktavat,

    /// The *-at* suffix (present active participle).
    Shatr,
    /// The *-āna* suffix (present middle participle).
    Shanac,
    /// The *-ya vikaraṇa* followed by the *-āna* suffix (present passive participle).
    YakShanac,

    /// The *-sya vikaraṇa* followed by the *-at* suffix (future active participle).
    SyaShatr,
    /// The *-sya vikaraṇa* followed by the *-āna* suffix (future middle participle).
    SyaShanac,
    /// The *-tavya*, *-anīya*, and *-ya* suffixes, etc. (future past participle, gerundive).
    Krtya,
}

impl KrtPratyaya {
    /// Returns a string representation of this enum.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "_",
            Self::Tumun => "tumun",
            Self::Ktva => "ktvA",
            Self::Lyap => "lyap",
            Self::Kvasu => "kvasu",
            Self::Kanac => "kAnac",
            Self::Kta => "kta",
            Self::Ktavat => "ktavat",
            Self::Shatr => "Satf",
            Self::Shanac => "SAnac",
            Self::YakShanac => "yak-SAnac",
            Self::SyaShatr => "sya-Satf",
            Self::SyaShanac => "sya-SAnac",
            Self::Krtya => "kftya",
        }
    }
}

impl FromStr for KrtPratyaya {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let val = match s {
            "_" => Self::None,
            "tumun" => Self::Tumun,
            "ktvA" => Self::Ktva,
            "lyap" => Self::Lyap,
            "kvasu" => Self::Kvasu,
            "kAnac" => Self::Kanac,
            "kta" => Self::Kta,
            "ktavat" => Self::Ktavat,
            "Satf" => Self::Shatr,
            "SAnac" => Self::Shanac,
            "yak-SAnac" => Self::YakShanac,
            "sya-Satf" => Self::SyaShatr,
            "sya-SAnac" => Self::SyaShanac,
            "kftya" => Self::Krtya,
            _ => return Err(Error::EnumParse("KrtPratyaya", s.to_string())),
        };
        Ok(val)
    }
}

impl Display for KrtPratyaya {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/// The *pada* and *prayoga* of the *tiṅanta*. Roughly, these correspond respectively to the
/// concepts of "voice" and "thematic relation."
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[bits = 2]
pub enum PadaPrayoga {
    /// Unknown or missing *prayoga*.
    None,
    /// *parasmaipada*, which is always in *kartari prayoga*.
    Parasmaipada,
    /// *ātmanepada* in *kartari prayoga*.
    AtmanepadaKartari,
    /// *ātmanepada* in *bhāve* or *karmaṇi prayoga*.
    AtmanepadaNotKartari,
}

impl PadaPrayoga {
    /// Returns a string representation of this enum.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "_",
            Self::Parasmaipada => "para",
            Self::AtmanepadaKartari => "atma-kartari",
            Self::AtmanepadaNotKartari => "atma-not-kartari",
        }
    }
}

impl FromStr for PadaPrayoga {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let val = match s {
            "_" => Self::None,
            "para" => Self::Parasmaipada,
            "atma-kartari" => Self::AtmanepadaKartari,
            "atma-not-kartari" => Self::AtmanepadaNotKartari,
            _ => return Err(Error::EnumParse("PadaPrayoga", s.to_string())),
        };
        Ok(val)
    }
}

impl Display for PadaPrayoga {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/// Models the semantics of a *dhātu* (verb root).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dhatu(pub String);

/// Models the semantics of a *prātipadika*.
///
/// An *prātipadika* is generally synonymous with a nominal base.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Pratipadika {
    /// A basic *prātipadika* that cannot be analyzed further.
    Basic {
        /// The text of the *prātipadika*.
        text: String,
        /// The lingas this *prātipadika* uses in most contexts.
        lingas: Vec<Linga>,
    },
    /// A *prātipadika* formed by combining a *dhātu* with one or more suffixes.
    Krdanta {
        /// The dhatu on which this krdanta is based.
        dhatu: Dhatu,
        /// The pratyaya that created this krdanta.
        pratyaya: KrtPratyaya,
    },
}

impl Pratipadika {
    /// Returns the lemma that the *prātipadika* is based on.
    pub fn lemma(&self) -> String {
        match &self {
            Pratipadika::Basic { text, .. } => text.clone(),
            Pratipadika::Krdanta { dhatu, .. } => dhatu.0.clone(),
        }
    }

    /// Returns a string representation of this enum.
    pub fn as_str(&self) -> String {
        match self {
            Pratipadika::Basic { text, lingas } => {
                let lingas = lingas
                    .iter()
                    .map(Linga::as_str)
                    .collect::<Vec<_>>()
                    .join(",");
                format!("basic:text={text}|lingas={lingas}")
            }
            Pratipadika::Krdanta { dhatu, pratyaya } => {
                format!("krdanta:dhatu={}|pratyaya={}", dhatu.0, pratyaya.as_str())
            }
        }
    }
}

impl FromStr for Pratipadika {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Some(s) = s.strip_prefix("basic:") {
            let kv = FeatureMap::from_str(s);
            let text = kv.get("text")?.clone();

            let linga_str = kv.get("lingas")?;
            let lingas = if linga_str.is_empty() {
                Vec::new()
            } else {
                linga_str
                    .split(',')
                    .map(Linga::from_str)
                    .collect::<Result<Vec<_>>>()?
            };

            Ok(Pratipadika::Basic { text, lingas })
        } else if let Some(s) = s.strip_prefix("krdanta:") {
            let kv = FeatureMap::from_str(s);

            let dhatu = kv.get("dhatu")?.clone();
            let pratyaya = (kv.get("pratyaya")?).parse()?;

            Ok(Pratipadika::Krdanta {
                dhatu: Dhatu(dhatu),
                pratyaya,
            })
        } else {
            Err(Error::EnumParse("Pratipadika", s.to_string()))
        }
    }
}

/// A short part-of-speech tag for some `Pada`.
///
/// We use this tag when calculating lemma counts. For example, *eva* is a common *avyaya* but
/// not a common *subanta*, and our statistics should reflect that distinction. Coarser
/// distinctions that include linga, vacana, etc. are interesting but less useful given our
/// limited training data.
#[derive(Clone, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[bits = 2]
pub enum POSTag {
    /// A token with missing, unknown, or undefined semantics.
    None,
    /// A nominal.
    Subanta,
    /// A verb.
    Tinanta,
    /// An indeclinable.
    Avyaya,
}

impl FromStr for POSTag {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let val = match s {
            "_" => Self::None,
            "s" => Self::Subanta,
            "t" => Self::Tinanta,
            "a" => Self::Avyaya,
            _ => return Err(Error::EnumParse("POSTag", s.to_string())),
        };
        Ok(val)
    }
}

impl POSTag {
    /// Returns a string representation of this enum.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "_",
            Self::Subanta => "s",
            Self::Tinanta => "t",
            Self::Avyaya => "a",
        }
    }
}

impl Display for POSTag {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/// Models the semantics of a *subanta* if it is not an *avyaya*.
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
///
/// For *avyaya*s (indeclinables), see `Avyaya`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Subanta {
    /// The nominal's stem.
    pub pratipadika: Pratipadika,
    /// The nominal's gender.
    pub linga: Linga,
    /// The nominal's number.
    pub vacana: Vacana,
    /// The nominal's case.
    pub vibhakti: Vibhakti,
    /// Whether this *subanta* is part of some compound but not the final member of it.
    pub is_purvapada: bool,
}

/// Models the semantics of a *tiṅanta*.
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
/// A *tiṅanta* expresses person, number, tense/mood, and voice in addition to whatever semantics
/// are conveyed by the *dhātu* and its prefixes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tinanta {
    /// The verb's root.
    pub dhatu: Dhatu,
    /// The verb's person.
    pub purusha: Purusha,
    /// The verb's number.
    pub vacana: Vacana,
    /// The verb's tense/mood.
    pub lakara: Lakara,
    /// The verb's voice, roughly speaking.
    pub pada: PadaPrayoga,
}

/// Models the semantics of an *avyaya*.
///
/// An *avyaya*s (indeclinable) is traditionally modeled as a subtype of the *subanta* that has had
/// its *sup* suffix elided. But we model the *avyaya* separately because we felt that doing so
/// would be easier to reason about in downstream code.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Avyaya {
    /// The indeclinable's stem.
    pub pratipadika: Pratipadika,
}

/// Models the semantics of a Sanskrit *pada* (word).
///
/// This enum can be packed into an unsigned integer via the `packing` module.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Pada {
    /// Unknown or missing semantics.
    None,
    /// A *subanta* (nominal, excluding *avyaya*s)
    Subanta(Subanta),
    /// A *tiṅanta* (verb).
    Tinanta(Tinanta),
    /// A basic *avyaya* (indeclinable).
    Avyaya(Avyaya),
}

impl Pada {
    /// Returns the lemma of the given *pada*.
    ///
    /// The *lemma* of a word is a canonical form that represents a set of inflectional variants.
    /// For example, the word *gacchati*, *gantum*, *gamanam*, and *jagāma* are all inflectional
    /// variants of the lemma *gam*.
    ///
    /// In Vidyut, we use lemma frequencies to score different padaccheda solutions.
    ///
    /// In Sanskrit, a lemma is either a *dhātu* or a *prātipadika*.
    pub fn lemma(&self) -> String {
        match &self {
            Pada::Tinanta(t) => t.dhatu.0.clone(),
            Pada::Subanta(s) => s.pratipadika.lemma(),
            Pada::Avyaya(a) => a.pratipadika.lemma(),
            Pada::None => NONE_LEMMA.to_string(),
        }
    }

    /// Returns the part of speech tag for the given `Pada`.
    pub fn part_of_speech_tag(&self) -> POSTag {
        match self {
            Pada::Tinanta(_) => POSTag::Tinanta,
            Pada::Subanta(_) => POSTag::Subanta,
            Pada::Avyaya(_) => POSTag::Avyaya,
            Pada::None => POSTag::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type TestResult = Result<()>;

    #[test]
    fn test_linga_serde() -> TestResult {
        use Linga::*;
        for val in [None, Pum, Stri, Napumsaka] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_vacana_serde() -> TestResult {
        use Vacana::*;
        for val in [None, Eka, Dvi, Bahu] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_vibhakti_serde() -> TestResult {
        use Vibhakti::*;
        for val in [None, V1, V2, V3, V4, V5, V6, V7, Sambodhana] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_purusha_serde() -> TestResult {
        use Purusha::*;
        for val in [None, Prathama, Madhyama, Uttama] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_lakara_serde() -> TestResult {
        use Lakara::*;
        for val in [
            Lat, Lit, Lut, Lrt, Let, Lot, Lan, LinVidhi, LinAshih, Lun, Lrn,
        ] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_krt_pratyaya_serde() -> TestResult {
        use KrtPratyaya::*;
        for val in [
            None, Tumun, Ktva, Lyap, Kvasu, Kanac, Kta, Ktavat, Shatr, Shanac, YakShanac, SyaShatr,
            SyaShanac, Krtya,
        ] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_pada_prayoga() -> TestResult {
        use PadaPrayoga::*;
        for val in [None, Parasmaipada, AtmanepadaKartari, AtmanepadaNotKartari] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_pratipadika_serde_with_basic() -> TestResult {
        let p = Pratipadika::Basic {
            text: "agni".to_string(),
            lingas: vec![Linga::Pum],
        };
        assert_eq!(p, p.as_str().parse()?);
        Ok(())
    }

    #[test]
    fn test_pratipadika_serde_with_krdanta() -> TestResult {
        let p = Pratipadika::Krdanta {
            dhatu: Dhatu("gam".to_string()),
            pratyaya: KrtPratyaya::Shatr,
        };
        assert_eq!(p, p.as_str().parse()?);
        Ok(())
    }

    #[test]
    fn test_subanta_lemma_with_basic_stem() {
        let p = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: "agni".to_string(),
                lingas: vec![Linga::Pum],
            },
            linga: Linga::Pum,
            vacana: Vacana::Eka,
            vibhakti: Vibhakti::V2,
            is_purvapada: false,
        });
        assert_eq!(p.lemma(), "agni");
    }

    #[test]
    fn test_subanta_lemma_with_krdanta_stem() {
        let p = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Krdanta {
                dhatu: Dhatu("gam".to_string()),
                pratyaya: KrtPratyaya::Shatr,
            },
            linga: Linga::Pum,
            vacana: Vacana::Eka,
            vibhakti: Vibhakti::V2,
            is_purvapada: false,
        });
        assert_eq!(p.lemma(), "gam");
    }

    #[test]
    fn test_tinanta_lemma() {
        let p = Pada::Tinanta(Tinanta {
            dhatu: Dhatu("gam".to_string()),
            purusha: Purusha::Prathama,
            vacana: Vacana::Eka,
            lakara: Lakara::Lat,
            pada: PadaPrayoga::Parasmaipada,
        });
        assert_eq!(p.lemma(), "gam");
    }

    #[test]
    fn test_avyaya_lemma_with_basic_stem() {
        let p = Pada::Avyaya(Avyaya {
            pratipadika: Pratipadika::Basic {
                text: "svar".to_string(),
                lingas: vec![],
            },
        });
        assert_eq!(p.lemma(), "svar");
    }

    #[test]
    fn test_avyaya_lemma_with_krdanta_stem() {
        let p = Pada::Avyaya(Avyaya {
            pratipadika: Pratipadika::Krdanta {
                dhatu: Dhatu("gam".to_string()),
                pratyaya: KrtPratyaya::Tumun,
            },
        });
        assert_eq!(p.lemma(), "gam");
    }

    #[test]
    fn test_none_lemma() {
        let p = Pada::None;
        assert_eq!(p.lemma(), NONE_LEMMA);
    }
}
