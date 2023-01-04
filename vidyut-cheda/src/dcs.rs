//! Utility functions for reading DCS data.
use crate::conllu::{Token as EvalToken, TokenFeatures};
use crate::segmenting::Token;
use std::error::Error;
use std::fmt;
use vidyut_kosha::semantics::*;
use vidyut_lipi::{transliterate, Scheme};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct ConversionError(String);
impl ConversionError {
    fn new(s: &str) -> Box<Self> {
        Box::new(ConversionError(s.to_string()))
    }
}
impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse value `{}`", self.0)
    }
}
impl Error for ConversionError {
    fn description(&self) -> &str {
        &self.0
    }
}

fn to_slp1(text: &str) -> String {
    transliterate(text, Scheme::Iast, Scheme::Slp1)
}

/// Convert DCS semantics to Vidyut semantics.
pub fn standardize(t: &EvalToken) -> Result<Token> {
    let slp1_lemma = standardize_lemma(&t.lemma);

    let semantics = match t.upos.as_str() {
        "NOUN" | "PRON" | "ADJ" | "NUM" => parse_subanta(t)?,
        "CONJ" | "CCONJ" | "SCONJ" | "ADV" | "PART" | "INTJ" | "ADP" => Pada::Avyaya(Avyaya {
            pratipadika: Pratipadika::Basic {
                text: slp1_lemma.clone(),
                lingas: Vec::new(),
            },
        }),
        "VERB" => {
            if t.features.contains_key("VerbForm") {
                parse_krdanta(t)?
            } else if t.features.contains_key("Gender") {
                parse_krdanta_subanta(t)?
            } else {
                parse_verb(t)?
            }
        }
        "MANTRA" => Pada::None,
        _ => panic!("Unknown upos `{}`", t.upos),
    };

    Ok(Token {
        // The original form is not consistently present in the DCS data, so just use the lemma.
        text: slp1_lemma,
        info: semantics,
    })
}

/// Standardizes the DCS lemma against Vidyut's conventions.
fn standardize_lemma(raw_lemma: &str) -> String {
    let lemma = to_slp1(raw_lemma);

    // Bagavant, hanumant, etc.
    if let Some(fragment) = lemma.strip_suffix("ant") {
        return String::from(fragment) + "at";
    }
    // kIrtay, etc.
    if let Some(fragment) = lemma.strip_suffix("ay") {
        return match fragment {
            "BAv" => "BU".to_string(),
            "niyoj" => "niyuj".to_string(),
            "moh" => "muh".to_string(),
            "vimoh" => "vimuh".to_string(),
            "vart" => "vft".to_string(),
            "udvart" => "udvft".to_string(),
            "pravart" => "pravft".to_string(),
            "anuvart" => "anuvft".to_string(),
            "kAr" => "kf".to_string(),
            "DAr" => "Df".to_string(),
            "upaDAr" => "upaDf".to_string(),
            "vidAr" => "vidF".to_string(),
            "boD" => "buD".to_string(),
            "kled" => "klid".to_string(),
            "samBAv" => "samBU".to_string(),
            _ => String::from(fragment),
        };
    }
    match lemma.as_str() {
        "paS" => "dfS".to_string(),
        "tf" => "tF".to_string(),
        "pf" => "pF".to_string(),
        "trA" => "trE".to_string(),
        "mad" => "asmad".to_string(),
        "enad" => "idam".to_string(),
        "tvad" => "yuzmad".to_string(),
        "uB" => "uBa".to_string(),
        // In DCS, usually should be "uBa"
        "uBaya" => "uBa".to_string(),
        "ka" => "kim".to_string(),
        _ => lemma,
    }
}

/// Reshapes a DCS nominal into a Vidyut subanta.
fn parse_subanta(t: &EvalToken) -> Result<Pada> {
    let stem = parse_stem(t);
    let linga = parse_linga(&t.features)?;
    let vibhakti = parse_vibhakti(&t.features)?;
    let vacana = parse_vacana(&t.features)?;
    let is_purvapada = parse_is_purvapada(&t.features);

    Ok(Pada::Subanta(Subanta {
        pratipadika: stem,
        linga,
        vacana,
        vibhakti,
        is_purvapada,
    }))
}

/// Reshapes a DCS verb into a Vidyut tinanta.
fn parse_verb(t: &EvalToken) -> Result<Pada> {
    let root = standardize_lemma(&t.lemma);
    let purusha = parse_purusha(&t.features)?;
    let vacana = parse_vacana(&t.features)?;
    let lakara = parse_lakara(&t.features)?;
    let pada = parse_verb_pada(&t.features);
    Ok(Pada::Tinanta(Tinanta {
        dhatu: Dhatu(root),
        purusha,
        vacana,
        lakara,
        pada,
    }))
}

/// Reshapes a DCS krdanta.
fn parse_krdanta(t: &EvalToken) -> Result<Pada> {
    match t
        .features
        .get("VerbForm")
        .expect("Should call with VerbForm")
        .as_str()
    {
        "Inf" | "Conv" => parse_krdanta_avyaya(t),
        _ => parse_krdanta_subanta(t),
    }
}

/// Reshapes a DCS krdanta subanta.
fn parse_krdanta_subanta(t: &EvalToken) -> Result<Pada> {
    let stem = Pratipadika::Krdanta {
        dhatu: Dhatu(standardize_lemma(&t.lemma)),
        pratyaya: parse_krt_pratyaya(&t.features)?,
    };
    let linga = parse_linga(&t.features)?;
    let vibhakti = parse_vibhakti(&t.features)?;
    let vacana = parse_vacana(&t.features)?;
    let is_purvapada = parse_is_purvapada(&t.features);

    Ok(Pada::Subanta(Subanta {
        pratipadika: stem,
        linga,
        vacana,
        vibhakti,
        is_purvapada,
    }))
}

/// Reshapes a DCS krdanta avyaya.
fn parse_krdanta_avyaya(t: &EvalToken) -> Result<Pada> {
    let stem = Pratipadika::Krdanta {
        dhatu: Dhatu(standardize_lemma(&t.lemma)),
        pratyaya: parse_krt_pratyaya(&t.features)?,
    };

    Ok(Pada::Avyaya(Avyaya { pratipadika: stem }))
}

/// Reshapes a DCS stem into a Vidyut stem.
fn parse_stem(t: &EvalToken) -> Pratipadika {
    Pratipadika::Basic {
        text: standardize_lemma(&t.lemma),
        lingas: Vec::new(),
    }
}

/// Reshapes a DCS tense into a Vidyut tense.
fn parse_krt_pratyaya(f: &TokenFeatures) -> Result<KrtPratyaya> {
    let val = match f.get("Tense") {
        Some(s) => match s.as_str() {
            // FIXME: not enough information to reconstruct.
            "Pres" => KrtPratyaya::Shatr,
            "Past" => KrtPratyaya::Kta,
            "Fut" => KrtPratyaya::SyaShatr,
            &_ => return Err(ConversionError::new(s)),
        },
        None => KrtPratyaya::None,
    };
    Ok(val)
}

/// Reshapes a DCS gender into a Vidyut linga.
fn parse_linga(f: &TokenFeatures) -> Result<Linga> {
    let val = match f.get("Gender") {
        Some(s) => match s.as_str() {
            "Masc" => Linga::Pum,
            "Fem" => Linga::Stri,
            "Neut" => Linga::Napumsaka,
            &_ => return Err(ConversionError::new(s)),
        },
        None => Linga::None,
    };
    Ok(val)
}

/// Reshapes a DCS case into a Vidyut vibhakti.
fn parse_vibhakti(f: &TokenFeatures) -> Result<Vibhakti> {
    let val = match f.get("Case") {
        Some(s) => match s.as_str() {
            "Nom" => Vibhakti::V1,
            "Acc" => Vibhakti::V2,
            "Ins" => Vibhakti::V3,
            "Dat" => Vibhakti::V4,
            "Abl" => Vibhakti::V5,
            "Gen" => Vibhakti::V6,
            "Loc" => Vibhakti::V7,
            "Voc" => Vibhakti::Sambodhana,
            "Cpd" => Vibhakti::None,
            &_ => return Err(ConversionError::new(s)),
        },
        None => Vibhakti::None,
    };
    Ok(val)
}

/// Reshapes a DCS compound flag.
fn parse_is_purvapada(f: &TokenFeatures) -> bool {
    match f.get("Case") {
        Some(s) => match s.as_str() {
            "Cpd" => true,
            &_ => false,
        },
        None => false,
    }
}

/// Reshapes a DCS person into a Vidyut purusha.
fn parse_purusha(f: &TokenFeatures) -> Result<Purusha> {
    let val = match f.get("Person") {
        Some(s) => match s.as_str() {
            "3" => Purusha::Prathama,
            "2" => Purusha::Madhyama,
            "1" => Purusha::Uttama,
            &_ => return Err(ConversionError::new(s)),
        },
        None => Purusha::None,
    };
    Ok(val)
}

/// Reshapes a DCS number into a Vidyut vacana.
fn parse_vacana(f: &TokenFeatures) -> Result<Vacana> {
    let val = match f.get("Number") {
        Some(s) => match s.as_str() {
            "Sing" => Vacana::Eka,
            "Dual" => Vacana::Dvi,
            "Plur" => Vacana::Bahu,
            &_ => return Err(ConversionError::new("Could not parse number")),
        },
        None => Vacana::None,
    };
    Ok(val)
}

/// Reshapes a DCS tense/mood into a Vidyut lakara.
fn parse_lakara(f: &TokenFeatures) -> Result<Lakara> {
    let tense = match f.get("Tense") {
        Some(s) => s,
        None => return Err(ConversionError::new("`Tense` not found")),
    };
    let mood = match f.get("Mood") {
        Some(s) => s,
        None => return Err(ConversionError::new("`Mood` not found")),
    };

    let val = match (tense.as_str(), mood.as_str()) {
        ("Aor", "Ind") => Lakara::Lun,
        ("Aor", "Imp") => Lakara::None,
        ("Aor", "Jus") => Lakara::LunNoAgama,
        ("Aor", "Prec") => Lakara::LinAshih,
        ("Fut", "Cond") => Lakara::Lrn,
        ("Fut", "Ind") => Lakara::Lrt,
        ("Impf", "Ind") => Lakara::Lan,
        ("Perf", "Ind") => Lakara::Lit,
        ("Perf", "Sub") => Lakara::None,
        ("Pres", "Imp") => Lakara::Lot,
        ("Pres", "Ind") => Lakara::Lat,
        ("Pres", "Jus") => Lakara::None,
        ("Pres", "Opt") => Lakara::LinVidhi,
        ("Pres", "Sub") => Lakara::Lot,
        (&_, &_) => Lakara::None,
    };
    Ok(val)
}

fn parse_verb_pada(_f: &TokenFeatures) -> PadaPrayoga {
    // FIXME: unsupported in DCS?
    PadaPrayoga::None
}
