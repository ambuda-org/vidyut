//! Utilities for reading linguistic data.
//!
//! Most of our data comes from the [sanskrit/data](https://github.com/sanskrit/data) project. In
//! the future, Vidyut might generate its own linguistic data instead.

use crate::lexicon::{EndingMap, PadaMap, StemMap};
use crate::semantics::*;
use std::error::Error;

/// Defines all of the input data paths we use in Vidyut.
pub struct DataPaths {
    pub indeclinables: String,
    pub nominal_endings_compounded: String,
    pub nominal_endings_inflected: String,
    pub nominal_stems: String,
    pub participle_stems: String,
    pub prefix_groups: String,
    pub prefixed_roots: String,
    pub pronouns: String,
    pub sandhi_rules: String,
    pub unprefixed_roots: String,
    pub verb_endings: String,
    pub verb_prefixes: String,
    pub verbal_indeclinables: String,
    pub verbs: String,
}

fn parse_linga(code: &str) -> Linga {
    match code {
        "m" => Linga::Pum,
        "f" => Linga::Stri,
        "n" => Linga::Napumsaka,
        "none" => Linga::None,
        &_ => panic!("Unknown type {}", code),
    }
}

fn parse_stem_linga(code: &str) -> Vec<Linga> {
    match code {
        "m" => vec![Linga::Pum],
        "f" => vec![Linga::Stri],
        "n" => vec![Linga::Napumsaka],
        "mf" => vec![Linga::Pum, Linga::Stri],
        "fn" => vec![Linga::Stri, Linga::Napumsaka],
        "mn" => vec![Linga::Pum, Linga::Napumsaka],
        "mfn" => vec![Linga::Pum, Linga::Stri, Linga::Napumsaka],
        &_ => panic!("Unknown type {}", code),
    }
}

fn parse_vibhakti(code: &str) -> Vibhakti {
    match code {
        "1" => Vibhakti::V1,
        "2" => Vibhakti::V2,
        "3" => Vibhakti::V3,
        "4" => Vibhakti::V4,
        "5" => Vibhakti::V5,
        "6" => Vibhakti::V6,
        "7" => Vibhakti::V7,
        "8" => Vibhakti::Sambodhana,
        &_ => panic!("Unknown type {}", code),
    }
}

fn parse_vacana(code: &str) -> Vacana {
    match code {
        "s" => Vacana::Eka,
        "d" => Vacana::Dvi,
        "p" => Vacana::Bahu,
        &_ => panic!("Unknown type {}", code),
    }
}

fn parse_verb_pada(code: &str) -> VerbPada {
    match code {
        "para" => VerbPada::Parasmaipada,
        "atma" => VerbPada::Atmanepada,
        "pass" => VerbPada::AtmanepadaKarmani,
        &_ => panic!("Unknown type {}", code),
    }
}

fn parse_prayoga(code: &str) -> StemPrayoga {
    match code {
        "para" => StemPrayoga::Kartari,
        "atma" => StemPrayoga::Kartari,
        "pass" => StemPrayoga::Bhave,
        "active" => StemPrayoga::Kartari,
        &_ => panic!("Unknown type {}", code),
    }
}

fn parse_tense(code: &str) -> StemTense {
    match code {
        "past" => StemTense::Past,
        "pres" => StemTense::Present,
        "fut" => StemTense::Future,
        "perf" => StemTense::Past,
        &_ => panic!("Unknown type {}", code),
    }
}

fn add_indeclinables(path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let pada = r[0].to_string();
        padas.insert(pada, Semantics::Avyaya);
    }
    Ok(())
}

fn add_nominal_endings_compounded(
    path: &str,
    endings: &mut EndingMap,
) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let ending = r[2].to_string();

        let semantics = Semantics::Subanta(Subanta {
            stem: stem.clone(),
            linga: Linga::None,
            vibhakti: Vibhakti::None,
            vacana: Vacana::None,
            is_compounded: true,
        });
        endings.insert(ending, (stem, semantics));
    }
    Ok(())
}

fn add_nominal_endings_inflected(
    path: &str,
    endings: &mut EndingMap,
) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;

        let stem = r[0].to_string();
        let ending = r[2].to_string();
        let semantics = Semantics::Subanta(Subanta {
            stem: stem.clone(),
            linga: parse_linga(&r[3]),
            vibhakti: parse_vibhakti(&r[4]),
            vacana: parse_vacana(&r[5]),
            is_compounded: false,
        });
        endings.insert(ending, (stem, semantics));
    }
    Ok(())
}

fn add_nominal_stems(path: &str, padas: &mut StemMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let lingas = parse_stem_linga(&r[1]);
        let semantics = StemSemantics::Basic { lingas };
        padas.insert(stem, semantics);
    }
    Ok(())
}

fn add_participle_stems(path: &str, padas: &mut StemMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let root = r[1].to_string();
        padas.insert(
            stem,
            StemSemantics::Krdanta {
                root,
                tense: parse_tense(&r[4]),
                prayoga: parse_prayoga(&r[5]),
            },
        );
    }
    Ok(())
}

fn add_prefix_groups(path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        padas.insert(r[0].to_string(), Semantics::PrefixGroup);
    }
    Ok(())
}

fn add_pronouns(path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;

        let stem = r[0].to_string();
        let text = r[2].to_string();
        let semantics = Semantics::Subanta(Subanta {
            stem,
            linga: parse_linga(&r[3]),
            vibhakti: parse_vibhakti(&r[4]),
            vacana: parse_vacana(&r[5]),
            is_compounded: false,
        });
        padas.insert(text, semantics);
    }
    Ok(())
}

fn add_verbal_indeclinables(path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let pada = row[0].to_string();
        let root = row[1].to_string();
        let semantics = match &row[3] {
            "gerund" => Semantics::Ktva(KrtAvyaya { root }),
            "infinitive" => Semantics::Tumun(KrtAvyaya { root }),
            &_ => panic!("Unknown indeclinable type `{}`", &row[3]),
        };
        padas.insert(pada, semantics);
    }
    Ok(())
}

fn add_verbs(path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let text = r[0].to_string();
        let root = r[1].to_string();

        let purusha = match &r[4] {
            "3" => Purusha::Prathama,
            "2" => Purusha::Madhyama,
            "1" => Purusha::Uttama,
            &_ => panic!("Unknown type `{}`", &r[4]),
        };

        let vacana = parse_vacana(&r[5]);

        let lakara = match &r[6] {
            "pres" => Lakara::Lat,
            "ipft" => Lakara::Lan,
            "sfut" => Lakara::Lrt,
            "opt" => Lakara::LinVidhi,
            "ben" => Lakara::LinAshih,
            "inj" => Lakara::LunNoAgama,
            "pfut" => Lakara::Lut,
            "impv" => Lakara::Lot,
            "perf" => Lakara::Lit,
            "aor" => Lakara::Lun,
            "cond" => Lakara::Lrn,
            &_ => panic!("Unknown type {}", &r[6]),
        };

        let pada = parse_verb_pada(&r[7]);

        padas.insert(
            text,
            Semantics::Tinanta(Tinanta {
                root,
                purusha,
                vacana,
                lakara,
                pada,
            }),
        );
    }
    Ok(())
}

pub fn read_nominal_endings(paths: &DataPaths) -> Result<EndingMap, Box<dyn Error>> {
    let mut endings = EndingMap::new();
    add_nominal_endings_compounded(&paths.nominal_endings_compounded, &mut endings)?;
    add_nominal_endings_inflected(&paths.nominal_endings_inflected, &mut endings)?;
    Ok(endings)
}

pub fn read_stems(paths: &DataPaths) -> Result<StemMap, Box<dyn Error>> {
    let mut stems = StemMap::new();
    add_nominal_stems(&paths.nominal_stems, &mut stems)?;
    add_participle_stems(&paths.participle_stems, &mut stems)?;
    Ok(stems)
}

pub fn read_padas(paths: &DataPaths) -> Result<PadaMap, Box<dyn Error>> {
    let mut padas = PadaMap::new();
    add_indeclinables(&paths.indeclinables, &mut padas)?;
    add_prefix_groups(&paths.prefix_groups, &mut padas)?;
    add_pronouns(&paths.pronouns, &mut padas)?;
    add_verbal_indeclinables(&paths.verbal_indeclinables, &mut padas)?;
    add_verbs(&paths.verbs, &mut padas)?;
    Ok(padas)
}
