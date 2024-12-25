//! Utility functions for reading DCS data.
use crate::chedaka::Token;
use crate::conllu::{Token as EvalToken, TokenFeatures};
use crate::errors::{Error, Result};
use compact_str::CompactString;
use vidyut_kosha::entries::*;
use vidyut_lipi::{transliterate, Mapping, Scheme};
use vidyut_prakriya::args as vp;
use vidyut_prakriya::args::{Linga, Pratipadika, Purusha, Slp1String, Subanta, Vacana, Vibhakti};

fn to_slp1(text: &str) -> String {
    let mapping = Mapping::new(Scheme::Iast, Scheme::Slp1);
    transliterate(text, &mapping)
}

/// Convert DCS semantics to Vidyut semantics.
pub fn standardize(t: &EvalToken) -> Result<Token> {
    let lemma = standardize_lemma(&t.lemma);
    let lemma = Slp1String::from(lemma).expect("ok");

    let semantics = match t.upos.as_str() {
        "NOUN" | "PRON" | "ADJ" | "NUM" => parse_subanta(t)?,
        "CONJ" | "CCONJ" | "SCONJ" | "ADV" | "PART" | "INTJ" | "ADP" => {
            let prati = Pratipadika::basic(lemma.clone());

            let subanta = Subanta::avyaya(prati.clone());
            let p_entry = PratipadikaEntry::new(prati, vec![]);
            PadaEntry::Avyaya(SubantaEntry::new(subanta, p_entry))
        }
        "VERB" => {
            if t.features.contains_key("VerbForm") {
                parse_krdanta(t)?
            } else if t.features.contains_key("Gender") {
                parse_krdanta_subanta(t)?
            } else {
                parse_verb(t)?
            }
        }
        "MANTRA" => PadaEntry::Unknown,
        _ => panic!("Unknown upos `{}`", t.upos),
    };

    Ok(Token {
        // The original form is not consistently present in the DCS data, so just use the lemma.
        text: CompactString::from(lemma.as_str()),
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
fn parse_subanta(t: &EvalToken) -> Result<PadaEntry> {
    let pratipadika = parse_pratipadika(t);
    let linga = parse_linga(&t.features)?.unwrap_or(Linga::Pum);
    let vibhakti = parse_vibhakti(&t.features)?.unwrap_or(Vibhakti::Prathama);
    let vacana = parse_vacana(&t.features)?.unwrap_or(Vacana::Eka);
    let _is_purvapada = parse_is_purvapada(&t.features);

    let subanta = vp::Subanta::new(pratipadika.clone(), linga, vibhakti, vacana);
    let p_entry = PratipadikaEntry::new(pratipadika, vec![]);
    Ok(PadaEntry::Subanta(SubantaEntry::new(subanta, p_entry)))
}

/// Reshapes a DCS verb into a Vidyut tinanta.
fn parse_verb(t: &EvalToken) -> Result<PadaEntry> {
    let root = standardize_lemma(&t.lemma);
    let purusha = parse_purusha(&t.features)?;
    let vacana = parse_vacana(&t.features)?.unwrap_or(Vacana::Eka);
    let (lakara, _skip_at_agama) = parse_lakara(&t.features)?.unwrap_or((vp::Lakara::Lat, false));
    Ok(PadaEntry::Tinanta(vp::Tinanta::new(
        vp::Dhatu::mula(Slp1String::from(root).expect("ok"), vp::Gana::Bhvadi),
        vp::Prayoga::Kartari,
        lakara,
        purusha,
        vacana,
    )))
}

/// Reshapes a DCS krdanta.
fn parse_krdanta(t: &EvalToken) -> Result<PadaEntry> {
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
fn parse_krdanta_subanta(t: &EvalToken) -> Result<PadaEntry> {
    let lemma = standardize_lemma(&t.lemma);
    let pratipadika = vp::Krdanta::new(
        vp::Dhatu::mula(Slp1String::from(lemma).expect("ok"), vp::Gana::Bhvadi),
        parse_krt_pratyaya(&t.features)?.unwrap_or(vp::BaseKrt::kta.into()),
    );
    let linga = parse_linga(&t.features)?.unwrap_or(Linga::Pum);
    let vibhakti = parse_vibhakti(&t.features)?.unwrap_or(Vibhakti::Prathama);
    let vacana = parse_vacana(&t.features)?.unwrap_or(Vacana::Eka);
    let _is_purvapada = parse_is_purvapada(&t.features);

    let pratipadika: vp::Pratipadika = pratipadika.into();
    let subanta = vp::Subanta::new(pratipadika.clone(), linga, vibhakti, vacana);
    let dummy_entry = PratipadikaEntry::new(pratipadika, vec![]);
    Ok(PadaEntry::Subanta(SubantaEntry::new(subanta, dummy_entry)))
}

/// Reshapes a DCS krdanta avyaya.
fn parse_krdanta_avyaya(t: &EvalToken) -> Result<PadaEntry> {
    let lemma = standardize_lemma(&t.lemma);
    let krdanta = vp::Krdanta::new(
        vp::Dhatu::mula(Slp1String::from(lemma).expect("ok"), vp::Gana::Bhvadi),
        // Use an arbitrary default.
        parse_krt_pratyaya(&t.features)?.unwrap_or(vp::BaseKrt::kta.into()),
    );

    let prati: Pratipadika = krdanta.into();
    let p_entry = PratipadikaEntry::new(prati.clone(), vec![]);
    let subanta = vp::Subanta::avyaya(prati);
    Ok(PadaEntry::Avyaya(SubantaEntry::new(subanta, p_entry)))
}

/// Reshapes a DCS stem into a Vidyut stem.
fn parse_pratipadika(t: &EvalToken) -> Pratipadika {
    Pratipadika::basic(Slp1String::from(&t.lemma).expect("ok"))
}

/// Reshapes a DCS tense into a Vidyut tense.
fn parse_krt_pratyaya(f: &TokenFeatures) -> Result<Option<vp::Krt>> {
    let val = match f.get("Tense") {
        Some(s) => match s.as_str() {
            // FIXME: not enough information to reconstruct.
            "Pres" => Some(vp::BaseKrt::Satf.into()),
            "Past" => Some(vp::BaseKrt::kta.into()),
            "Fut" => Some(vp::BaseKrt::Satf.into()),
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
            "Nom" => Some(Prathama),
            "Acc" => Some(Dvitiya),
            "Ins" => Some(Trtiya),
            "Dat" => Some(Caturthi),
            "Abl" => Some(Panchami),
            "Gen" => Some(Sasthi),
            "Loc" => Some(Saptami),
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
    use Purusha::*;
    let val = match f.get("Person") {
        Some(s) => match s.as_str() {
            "3" => Prathama,
            "2" => Madhyama,
            "1" => Uttama,
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
fn parse_lakara(f: &TokenFeatures) -> Result<Option<(vp::Lakara, bool)>> {
    use vp::Lakara::*;
    let tense = match f.get("Tense") {
        Some(s) => s,
        None => return Err(Error::dcs_undefined("Tense")),
    };
    let mood = match f.get("Mood") {
        Some(s) => s,
        None => return Err(Error::dcs_undefined("Mood")),
    };

    let val = match (tense.as_str(), mood.as_str()) {
        ("Aor", "Ind") => (Lun, false),
        ("Aor", "Jus") => (Lun, true),
        ("Aor", "Prec") => (AshirLin, false),
        ("Fut", "Cond") => (Lrn, false),
        ("Fut", "Pot") => (Lrn, false),
        ("Fut", "Ind") => (Lrt, false),
        ("Impf", "Ind") => (Lan, false),
        ("Perf", "Ind") => (Lit, false),
        ("Pres", "Imp") => (Lot, false),
        ("Pres", "Ind") => (Lat, false),
        ("Pres", "Opt") => (VidhiLin, false),
        ("Pres", "Sub") => (Lot, false),
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
