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

/// The *liṅga* (gender) of a *subanta*.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// The *vacana* (number) of a *subanta* or tiṅanta.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// The *vibhakti* (case) of a *subanta*.
///
/// The term *vibhakti* refers generally to any triad of inflectional endings for a *subanta*
/// (nominal) or tiṅanta (verb). Here, `Vibhakti` refers specifically to the nominal tridas.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// The *puruṣa* (person) of a tiṅanta.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// The *lakāra* (tense/mood) of a *tiṅanta*.
///
/// The *lakāras* are morphological categories that other grammatical rules then map to specific
/// semantics.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// The *pada* of a tiṅanta.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
