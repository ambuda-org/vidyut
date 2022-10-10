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
        tense: StemTense,
        prayoga: StemPrayoga,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Semantics {
    None,
    PrefixGroup,
    Avyaya,
    Ktva,
    Tumun,
    Subanta {
        linga: Linga,
        vacana: Vacana,
        vibhakti: Vibhakti,
        is_compounded: bool,
    },
    Tinanta {
        purusha: Purusha,
        vacana: Vacana,
        lakara: Lakara,
        pada: VerbPada,
    },
}
