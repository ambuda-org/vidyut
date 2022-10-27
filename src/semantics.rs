//! Models the semantics of Sanskrit words, including their stems and their inflectional endings.
//!
//! Traditional grammar has a rich and terse vocabulary for describing the morphology of Sanskrit
//! words, so that's generally the vocabulary we use in this module. But for readability, we also
//! use some English terms for less technical concepts like like "root" and "stem."
//!
//! In traditional Sanskirt grammar, all valid Sanskrit words are of two types:
//!
//! 1. *tiṅanta*s, or verbs, are words ending with one of the nine suffixes in the *tiṅ* list.
//!
//! 2. *subanta*s, or nominals, are words ending with one of the twenty-one suffixes in the *sup*
//!    list.
//!
//! All *avyaya*s (indeclinables) are modeled as a subtype of the *subanta* that has had its *sup*
//! suffix elided.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseError {
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

/// The *liṅga* (gender) of a *subanta*.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    pub fn to_str(&self) -> &'static str {
        match self {
            Linga::Pum => "m",
            Linga::Stri => "f",
            Linga::Napumsaka => "n",
            Linga::None => "none",
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
            "none" => Linga::None,
            _ => return Err(ParseError::new("could not parse linga")),
        };
        Ok(val)
    }
}

/// The *vacana* (number) of a *subanta* or tiṅanta.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Vacana {
    /// Unknown or missing vacana.
    None,
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}

impl Vacana {
    pub fn to_str(&self) -> &'static str {
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
/// (nominal) or tiṅanta (verb). Here, `Vibhakti` refers specifically to the nominal tridas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Vibhakti {
    /// Unknown or missing vibhakti.
    None,
    /// The first vibhakti (nominative case).
    V1,
    /// The second vibhakti (accusative case).
    V2,
    /// The third vibhakti (instrumental case).
    V3,
    /// The fourth vibhakti (dative case).
    V4,
    /// The fifth vibhakti (ablative case).
    V5,
    /// The sixth vibhakti (genitive case).
    V6,
    /// The seventh vibhakti (locative case).
    V7,
    /// The first vibhakti in the condition of *sambodhana* (vocative case).
    Sambodhana,
}

impl Vibhakti {
    pub fn to_str(&self) -> &'static str {
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

/// The *puruṣa* (person) of a tiṅanta.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Purusha {
    /// Unknown or missing *puruṣa*.
    None,
    /// The first *puruṣa* (third person).
    Prathama,
    /// The middle *puruṣa* (second person).
    Madhyama,
    /// The last *puruṣa* (third person).
    Uttama,
}

impl Purusha {
    pub fn to_str(&self) -> &'static str {
        match self {
            Purusha::Prathama => "3",
            Purusha::Madhyama => "2",
            Purusha::Uttama => "1",
            Purusha::None => "_",
        }
    }
}

impl FromStr for Purusha {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s {
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
/// The *lakāras* are morphological categories that other grammatical rules then map to specific
/// semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerbPada {
    None,
    Parasmaipada,
    Atmanepada,
    AtmanepadaKarmani,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StemTense {
    None,
    Past,
    Present,
    Future,
}

/// The *prayoga* of a tiṅanta.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StemPrayoga {
    None,
    Kartari,
    Bhave,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stem {
    Basic {
        stem: String,
        lingas: Vec<Linga>,
    },
    Krdanta {
        root: String,
        tense: StemTense,
        prayoga: StemPrayoga,
    },
}

/// Struct for `Semantics::Subanta`
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Subanta {
    pub stem: Stem,
    pub linga: Linga,
    pub vacana: Vacana,
    pub vibhakti: Vibhakti,
    // Whether this *subanta* is part of some compound but not the final member of it.
    pub is_purvapada: bool,
}

/// Struct for `Semantics::Tinanta`
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tinanta {
    pub root: String,
    pub purusha: Purusha,
    pub vacana: Vacana,
    pub lakara: Lakara,
    pub pada: VerbPada,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KrtAvyaya {
    pub root: String,
}

/// The semantics for a Sanskrit word.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Semantics {
    /// Unknown or missing semantics.
    None,
    /// One or more prefixes.
    /// NOTE: we will likely remove this type in the future.
    PrefixGroup,
    /// A basic *avyaya* (indeclinable).
    Avyaya,
    /// An *avyaya* formed with the *ktvā* suffix (indeclinable).
    Ktva(KrtAvyaya),
    /// An *avyaya* formed with the *tumun* suffix (infinitive).
    Tumun(KrtAvyaya),
    /// A *subanta* (nominal, excluding *avyaya*s)
    Subanta(Subanta),
    /// A *tiṅanta* (verb).
    Tinanta(Tinanta),
}
