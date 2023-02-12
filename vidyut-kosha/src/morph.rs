//! Models the morphology of Sanskrit words, including their stems and endings.
//!
//! For details on how we represent morphological data, see the `Pada` enum and its comments.
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
//!    with the simple label `KrtPratyaya::Tum`. As a counterexample, we explicitly model `Linga`,
//!    `Vacana`, `Vibhakti`, etc. because using a single `Sup` enum is more trouble than it's
//!    worth.

use crate::errors::*;
use modular_bitfield::prelude::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Implements various boilerplate for our enums:
///
/// - `as_str`
/// - `iter`
/// - `FromStr`
/// - `Display`
macro_rules! enum_boilerplate {
    ($Enum:ident, { $( $variant:ident => $str:literal ),* $(,)? }) => {
        impl $Enum {
            /// Returns a string representation of this enum.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $Enum::$variant => $str,
                    )*
                }
            }

            /// Iterates over the values of this enum in order.
            pub fn iter() -> impl Iterator<Item = &'static $Enum> {
                const ITEMS: &[$Enum] = &[
                    $(
                        $Enum::$variant,
                    )*
                ];
                ITEMS.iter()
            }
        }

        impl FromStr for $Enum {
            type Err = Error;
            fn from_str(s: &str) -> Result<Self> {
                let val = match s {
                    $(
                        $str => $Enum::$variant,
                    )*
                    _ => return Err(Error::ParseEnum(stringify!($Enum), s.to_string())),
                };
                Ok(val)
            }
        }

        impl Display for $Enum {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "{}", self.as_str())
            }
        }
    }
}

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[bits = 2]
pub enum Linga {
    /// The masculine gender.
    Pum,
    /// The feminine gender.
    Stri,
    /// The neuter gender.
    Napumsaka,
}

enum_boilerplate!(Linga, {
    Pum => "m",
    Stri => "f",
    Napumsaka => "n",
});

/// The *vacana* (number) of a *subanta* or *tiṅanta*.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[bits = 2]
pub enum Vacana {
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}

enum_boilerplate!(Vacana, {
    Eka => "s",
    Dvi => "d",
    Bahu => "p",
});

/// The *vibhakti* (case) of a *subanta*.
///
/// The term *vibhakti* refers generally to any triad of inflectional endings for a *subanta*
/// or *tiṅanta*. Here, `Vibhakti` refers specifically to the *subanta* tridas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[bits = 4]
pub enum Vibhakti {
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

enum_boilerplate!(Vibhakti, {
    V1 => "1",
    V2 => "2",
    V3 => "3",
    V4 => "4",
    V5 => "5",
    V6 => "6",
    V7 => "7",
    Sambodhana => "8",
});

/// The *puruṣa* (person) of a *tiṅanta*.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[bits = 2]
pub enum Purusha {
    /// The first *puruṣa* (third person).
    Prathama,
    /// The middle *puruṣa* (second person).
    Madhyama,
    /// The last *puruṣa* (first person).
    Uttama,
}

enum_boilerplate!(Purusha, {
     Prathama => "3",
     Madhyama => "2",
     Uttama => "1",
});

/// The *lakāra* (tense/mood) of a *tiṅanta*.
///
/// The *lakāras* are morphological categories, but each typically expresses a specific meaning.
/// For example, *laṭ-lakāra* almost always expresses an action in the present tense.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[bits = 4]
pub enum Lakara {
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
    AshirLin,
    /// *liṅ-lakāra* in the sense of a rule or injunction (optative).
    VidhiLin,
    /// *luṅ-lakāra* (aorist).
    Lun,
    /// *luṅ-lakāra* without its *a-* prefix (injunctive).
    LunNoAgama,
    /// *lṛṅ-lakāra* (conditional).
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
    LunNoAgama => "lun-no-agama",
    Lrn => "lrn",
});

/// A *pratyaya* (suffix) that creates a new *dhātu* (verb root)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[bits = 2]
pub enum DhatuPratyaya {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KrtPratyaya {
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

enum_boilerplate!(KrtPratyaya, {
    Tumun => "tumun",
    Ktva => "ktvA",
    Lyap => "lyap",
    Kvasu => "kvasu",
    Kanac => "kAnac",
    Kta => "kta",
    Ktavat => "ktavat",
    Shatr => "Satf",
    Shanac => "SAnac",
    YakShanac => "yak-SAnac",
    SyaShatr => "sya-Satf",
    SyaShanac => "sya-SAnac",
    Krtya => "kftya",
});

/// The *pada* and *prayoga* of the *tiṅanta*. Roughly, these correspond respectively to the
/// concepts of "voice" and "thematic relation."
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, BitfieldSpecifier)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[bits = 2]
pub enum PadaPrayoga {
    /// *parasmaipada*, which is always in *kartari prayoga*.
    Parasmaipada,
    /// *ātmanepada* in *kartari prayoga*.
    AtmanepadaKartari,
    /// *ātmanepada* in *bhāve* or *karmaṇi prayoga*.
    AtmanepadaNotKartari,
}

enum_boilerplate!(PadaPrayoga, {
    Parasmaipada => "para",
    AtmanepadaKartari => "atma-kartari",
    AtmanepadaNotKartari => "atma-not-kartari",
});

/// Models the semantics of a *dhātu* (verb root).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Dhatu(pub String);

impl Dhatu {
    /// The text of this dhatu.
    pub fn text(&self) -> &String {
        &self.0
    }
}

/// Models the semantics of a *prātipadika*.
///
/// An *prātipadika* is generally synonymous with a nominal base.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub fn lemma(&self) -> &str {
        match &self {
            Pratipadika::Basic { text, .. } => text,
            Pratipadika::Krdanta { dhatu, .. } => &dhatu.0,
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
            Err(Error::ParseEnum("Pratipadika", s.to_string()))
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[bits = 2]
pub enum POSTag {
    /// A token with missing, unknown, or undefined semantics.
    Unknown,
    /// A nominal.
    Subanta,
    /// A verb.
    Tinanta,
    /// An indeclinable.
    Avyaya,
}

enum_boilerplate!(POSTag, {
    Unknown => "_",
    Subanta => "s",
    Tinanta => "t",
    Avyaya => "a",
});

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Subanta {
    /// The nominal's stem.
    pub pratipadika: Pratipadika,
    /// The nominal's gender.
    pub linga: Option<Linga>,
    /// The nominal's number.
    pub vacana: Option<Vacana>,
    /// The nominal's case.
    pub vibhakti: Option<Vibhakti>,
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Avyaya {
    /// The indeclinable's stem.
    pub pratipadika: Pratipadika,
}

/// Models the semantics of a Sanskrit *pada* (word).
///
/// This enum can be packed into an unsigned integer via the `packing` module.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pada {
    /// Unknown or missing semantics.
    Unknown,
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
    pub fn lemma(&self) -> &str {
        match &self {
            Pada::Tinanta(t) => &t.dhatu.0,
            Pada::Subanta(s) => s.pratipadika.lemma(),
            Pada::Avyaya(a) => a.pratipadika.lemma(),
            Pada::Unknown => NONE_LEMMA,
        }
    }

    /// Returns the part of speech tag for the given `Pada`.
    pub fn part_of_speech_tag(&self) -> POSTag {
        match self {
            Pada::Tinanta(_) => POSTag::Tinanta,
            Pada::Subanta(_) => POSTag::Subanta,
            Pada::Avyaya(_) => POSTag::Avyaya,
            Pada::Unknown => POSTag::Unknown,
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
        for val in [Pum, Stri, Napumsaka] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_vacana_serde() -> TestResult {
        use Vacana::*;
        for val in [Eka, Dvi, Bahu] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_vibhakti_serde() -> TestResult {
        use Vibhakti::*;
        for val in [V1, V2, V3, V4, V5, V6, V7, Sambodhana] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_purusha_serde() -> TestResult {
        use Purusha::*;
        for val in [Prathama, Madhyama, Uttama] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_lakara_serde() -> TestResult {
        use Lakara::*;
        for val in [
            Lat, Lit, Lut, Lrt, Let, Lot, Lan, VidhiLin, AshirLin, Lun, Lrn,
        ] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_krt_pratyaya_serde() -> TestResult {
        use KrtPratyaya::*;
        for val in [
            Tumun, Ktva, Lyap, Kvasu, Kanac, Kta, Ktavat, Shatr, Shanac, YakShanac, SyaShatr,
            SyaShanac, Krtya,
        ] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_pada_prayoga() -> TestResult {
        use PadaPrayoga::*;
        for val in [Parasmaipada, AtmanepadaKartari, AtmanepadaNotKartari] {
            assert_eq!(val, val.as_str().parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_dhatu() {
        let d = Dhatu("BU".to_string());
        assert_eq!(d.text(), "BU");
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
            linga: Some(Linga::Pum),
            vacana: Some(Vacana::Eka),
            vibhakti: Some(Vibhakti::V2),
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
            linga: Some(Linga::Pum),
            vacana: Some(Vacana::Eka),
            vibhakti: Some(Vibhakti::V2),
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
        let p = Pada::Unknown;
        assert_eq!(p.lemma(), NONE_LEMMA);
    }
}
