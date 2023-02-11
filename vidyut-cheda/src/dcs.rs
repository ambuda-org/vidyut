//! Utility functions for reading DCS data.
use crate::conllu::{Token as EvalToken, TokenFeatures};
use crate::errors::{Error, Result};
use crate::segmenting::Token;
use compact_str::CompactString;
use vidyut_kosha::morph::*;
use vidyut_lipi::{transliterate, Scheme};

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
        "MANTRA" => Pada::Unknown,
        _ => panic!("Unknown upos `{}`", t.upos),
    };

    Ok(Token {
        // The original form is not consistently present in the DCS data, so just use the lemma.
        text: CompactString::from(slp1_lemma),
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
    let vacana = parse_vacana(&t.features)?.unwrap_or(Vacana::Eka);
    let lakara = parse_lakara(&t.features)?.unwrap_or(Lakara::Lat);
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
        pratyaya: parse_krt_pratyaya(&t.features)?.unwrap_or(KrtPratyaya::Kta),
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
        // Use an arbitrary default.
        pratyaya: parse_krt_pratyaya(&t.features)?.unwrap_or(KrtPratyaya::Kta),
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
fn parse_krt_pratyaya(f: &TokenFeatures) -> Result<Option<KrtPratyaya>> {
    let val = match f.get("Tense") {
        Some(s) => match s.as_str() {
            // FIXME: not enough information to reconstruct.
            "Pres" => Some(KrtPratyaya::Shatr),
            "Past" => Some(KrtPratyaya::Kta),
            "Fut" => Some(KrtPratyaya::SyaShatr),
            &_ => return Err(Error::parse_dcs("Tense", s)),
        },
        None => None,
    };
    Ok(val)
}

/// Reshapes a DCS gender into a Vidyut linga.
fn parse_linga(f: &TokenFeatures) -> Result<Option<Linga>> {
    let val = match f.get("Gender") {
        Some(s) => match s.as_str() {
            "Masc" => Some(Linga::Pum),
            "Fem" => Some(Linga::Stri),
            "Neut" => Some(Linga::Napumsaka),
            &_ => return Err(Error::parse_dcs("Gender", s)),
        },
        None => None,
    };
    Ok(val)
}

/// Reshapes a DCS case into a Vidyut vibhakti.
fn parse_vibhakti(f: &TokenFeatures) -> Result<Option<Vibhakti>> {
    use Vibhakti::*;
    let val = match f.get("Case") {
        Some(s) => match s.as_str() {
            "Nom" => Some(V1),
            "Acc" => Some(V2),
            "Ins" => Some(V3),
            "Dat" => Some(V4),
            "Abl" => Some(V5),
            "Gen" => Some(V6),
            "Loc" => Some(V7),
            "Voc" => Some(Sambodhana),
            "Cpd" => None,
            &_ => return Err(Error::parse_dcs("Case", s)),
        },
        None => None,
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
            &_ => return Err(Error::parse_dcs("Person", s)),
        },
        None => return Err(Error::dcs_undefined("Person")),
    };
    Ok(val)
}

/// Reshapes a DCS number into a Vidyut vacana.
fn parse_vacana(f: &TokenFeatures) -> Result<Option<Vacana>> {
    let val = match f.get("Number") {
        Some(s) => match s.as_str() {
            "Sing" => Vacana::Eka,
            "Dual" => Vacana::Dvi,
            "Plur" => Vacana::Bahu,
            &_ => return Err(Error::parse_dcs("Number", s)),
        },
        None => return Ok(None),
    };
    Ok(Some(val))
}

/// Reshapes a DCS tense/mood into a Vidyut lakara.
fn parse_lakara(f: &TokenFeatures) -> Result<Option<Lakara>> {
    let tense = match f.get("Tense") {
        Some(s) => s,
        None => return Err(Error::dcs_undefined("Tense")),
    };
    let mood = match f.get("Mood") {
        Some(s) => s,
        None => return Err(Error::dcs_undefined("Mood")),
    };

    let val = match (tense.as_str(), mood.as_str()) {
        ("Aor", "Ind") => Lakara::Lun,
        ("Aor", "Jus") => Lakara::LunNoAgama,
        ("Aor", "Prec") => Lakara::AshirLin,
        ("Fut", "Cond") => Lakara::Lrn,
        ("Fut", "Ind") => Lakara::Lrt,
        ("Impf", "Ind") => Lakara::Lan,
        ("Perf", "Ind") => Lakara::Lit,
        ("Pres", "Imp") => Lakara::Lot,
        ("Pres", "Ind") => Lakara::Lat,
        ("Pres", "Opt") => Lakara::VidhiLin,
        ("Pres", "Sub") => Lakara::Lot,
        ("Aor", "Imp") => return Ok(None),
        ("Past", "Ind") => return Ok(None),
        ("Past", "Imp") => return Ok(None),
        ("Past", "Sub") => return Ok(None),
        ("Plp", "Ind") => return Ok(None),
        ("Past", "Prec") => return Ok(None),
        ("Past", "Opt") => return Ok(None),
        ("Pqp", "Ind") => return Ok(None),
        ("Past", "Jus") => return Ok(None),
        ("Fut", "Con") => return Ok(None),
        ("Perf", "Sub") => return Ok(None),
        ("Pres", "Jus") => return Ok(None),
        (x, y) => panic!("Unknown lakara {x} {y}"),
    };
    Ok(Some(val))
}

fn parse_verb_pada(_f: &TokenFeatures) -> PadaPrayoga {
    // FIXME: unsupported in DCS?
    PadaPrayoga::Parasmaipada
}
