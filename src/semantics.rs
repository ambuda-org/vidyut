/// Models the semantics of Sanskrit words.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Linga {
    None,
    Pum,
    Stri,
    Napumsaka,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Vacana {
    None,
    Eka,
    Dvi,
    Bahu,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Vibhakti {
    None,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    Sambodhana,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Purusha {
    None,
    Prathama,
    Madhyama,
    Uttama,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lakara {
    None,
    Lat,
    Lit,
    Lut,
    Lrt,
    Let,
    Lot,
    Lan,
    LinAshih,
    LinVidhi,
    Lun,
    LunNoAgama,
    Lrn,
}

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StemPrayoga {
    None,
    Kartari,
    Bhave,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StemSemantics {
    Basic {
        lingas: Vec<Linga>,
    },
    Krdanta {
        root: String,
        tense: StemTense,
        prayoga: StemPrayoga,
    },
}

/// Struct for Subanta semantics, as part of the Semantics enum.
///
/// We use this clumsy syntax because Rust doesn't have great syntax
/// for enum variant types. For more context, see:
///
/// https://github.com/rust-lang/lang-team/issues/122
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Subanta {
    pub stem: String,
    pub linga: Linga,
    pub vacana: Vacana,
    pub vibhakti: Vibhakti,
    pub is_compounded: bool,
}

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KrtSubanta {
    pub root: String,
    pub linga: Linga,
    pub vacana: Vacana,
    pub vibhakti: Vibhakti,
    pub is_compounded: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Semantics {
    None,
    PrefixGroup,
    Avyaya,
    Ktva(KrtAvyaya),
    Tumun(KrtAvyaya),
    Subanta(Subanta),
    KrtSubanta(KrtSubanta),
    Tinanta(Tinanta),
}
