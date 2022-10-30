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

use modular_bitfield::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Lemma for `None` semantics or any other case where the lemma is unknown.
pub const NONE_LEMMA: &str = "[none]";

/// Indicates a failure to parse a string representation of some `semantics` enum.
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The error message.
    msg: String,
}
impl ParseError {
    fn new(s: &str) -> Self {
        ParseError { msg: s.to_owned() }
    }
}
impl Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

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
    fn get(&self, s: &str) -> Result<&String, ParseError> {
        if let Some(val) = self.0.get(s) {
            Ok(val)
        } else {
            Err(ParseError::new(&format!("Could not parse `{}`", s)))
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
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s {
            "m" => Linga::Pum,
            "f" => Linga::Stri,
            "n" => Linga::Napumsaka,
            "_" => Linga::None,
            // Legacy format on `github.com/sanskrit/data`
            "none" => Linga::None,
            _ => return Err(ParseError::new(&format!("could not parse linga `{}`", s))),
        };
        Ok(val)
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
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s {
            "_" => Vacana::None,
            "s" => Vacana::Eka,
            "d" => Vacana::Dvi,
            "p" => Vacana::Bahu,
            _ => return Err(ParseError::new("could not parse vacana")),
        };
        Ok(val)
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
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
            _ => return Err(ParseError::new("could not parse vibhakti")),
        };
        Ok(val)
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
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s {
            "_" => Purusha::None,
            "3" => Purusha::Prathama,
            "2" => Purusha::Madhyama,
            "1" => Purusha::Uttama,
            _ => return Err(ParseError::new("could not parse purusha")),
        };
        Ok(val)
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

impl FromStr for Lakara {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s {
            "_" => Lakara::None,
            "lat" => Lakara::Lat,
            "lit" => Lakara::Lit,
            "lut" => Lakara::Lut,
            "lrt" => Lakara::Lrt,
            "lot" => Lakara::Lot,
            "lan" => Lakara::Lan,
            "lin-vidhi" => Lakara::LinVidhi,
            "lin-ashih" => Lakara::LinAshih,
            "lun" => Lakara::Lun,
            "lun-no-agama" => Lakara::LunNoAgama,
            "lrn" => Lakara::Lrn,
            _ => return Err(ParseError::new("could not parse lakara")),
        };
        Ok(val)
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
    // Unknown or missing *kṛt-pratyaya*.
    None,

    // The *-tum* suffix (infinitive).
    Tumun,
    // The *-tvā* suffix (unprefixed gerund).
    Ktva,
    // The *-ya* suffix (prefixed gerund).
    Lyap,

    // The *-vas* suffix (perfect participle).
    Kvasu,
    // The -*āna* suffix (perfect participle).
    Kanac,

    // The *-ta* suffix (past passive participle).
    Kta,
    // The *-tavat* suffix (past active participle).
    Ktavat,

    // The *-at* suffix (present active participle).
    Shatr,
    // The *-āna* suffix (present middle participle).
    Shanac,
    // The *-ya vikaraṇa* followed by the *-āna* suffix (present passive participle).
    YakShanac,

    // The *-sya vikaraṇa* followed by the *-at* suffix (future active participle).
    SyaShatr,
    // The *-sya vikaraṇa* followed by the *-āna* suffix (future middle participle).
    SyaShanac,
    // The *-tavya*, *-anīya*, and *-ya* suffixes, etc. (future past participle, gerundive).
    Krtya,
}
impl KrtPratyaya {
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
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
            _ => return Err(ParseError::new("Could not parse krtpratyaya")),
        };
        Ok(val)
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
    /// *ātmanepada* in *karmaṇi* or *bhāve prayoga*.
    AtmanepadaNotKartari,
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
    Krdanta { dhatu: Dhatu, pratyaya: KrtPratyaya },
}

impl Pratipadika {
    /// Returns the lemma that the *prātipadika* is based on.
    pub fn lemma(&self) -> String {
        match &self {
            Pratipadika::Basic { text, .. } => text.clone(),
            Pratipadika::Krdanta { dhatu, .. } => dhatu.0.clone(),
        }
    }
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
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
                    .collect::<Result<Vec<_>, _>>()?
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
            Err(ParseError::new(&format!(
                "Could not parse string as pratipadika: `{s}`"
            )))
        }
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
    pub pratipadika: Pratipadika,
    pub linga: Linga,
    pub vacana: Vacana,
    pub vibhakti: Vibhakti,
    // Whether this *subanta* is part of some compound but not the final member of it.
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
    pub dhatu: Dhatu,
    pub purusha: Purusha,
    pub vacana: Vacana,
    pub lakara: Lakara,
    pub pada: PadaPrayoga,
}

/// Models the semantics of an *avyaya*.
///
/// An *avyaya*s (indeclinable) is traditionally modeled as a subtype of the *subanta* that has had
/// its *sup* suffix elided. But we model the *avyaya* separately because we felt that doing so
/// would be easier to reason about in downstream code.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Avyaya {
    pub pratipadika: Pratipadika,
}

/// Models the semantics of a Sanskrit *pada* (word).
///
/// This enum can be packed into an unsigned integer via the `vidyut::packing` module.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Pada {
    /// Unknown or missing semantics.
    None,
    /// One or more prefixes.
    /// NOTE: we will likely remove this type in the future.
    PrefixGroup(String),
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
            Pada::PrefixGroup(s) => s.clone(),
            Pada::None => NONE_LEMMA.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pratipadika_serde_with_basic() -> Result<(), Box<dyn Error>> {
        let p = Pratipadika::Basic {
            text: "agni".to_string(),
            lingas: vec![Linga::Pum],
        };
        assert_eq!(p, p.as_str().parse()?);
        Ok(())
    }

    #[test]
    fn test_pratipadika_serde_with_krdanta() -> Result<(), Box<dyn Error>> {
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
