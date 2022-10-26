//! Utilities for reading linguistic data.
//!
//! Most of our data comes from the [sanskrit/data](https://github.com/sanskrit/data) project. In
//! the future, Vidyut might generate its own linguistic data instead.

use crate::lexicon::{EndingMap, PadaMap, StemMap};
use crate::semantics::*;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Defines all of the input data paths we use in Vidyut.
pub struct DataPaths {
    pub indeclinables: PathBuf,
    pub nominal_endings_compounded: PathBuf,
    pub nominal_endings_inflected: PathBuf,
    pub nominal_stems: PathBuf,
    pub participle_stems: PathBuf,
    pub prefix_groups: PathBuf,
    pub prefixed_roots: PathBuf,
    pub pronouns: PathBuf,
    pub sandhi_rules: PathBuf,
    pub unprefixed_roots: PathBuf,
    pub verb_endings: PathBuf,
    pub verb_prefixes: PathBuf,
    pub verbal_indeclinables: PathBuf,
    pub verbs: PathBuf,

    pub lemma_counts: PathBuf,
}

impl DataPaths {
    pub fn from_dir(base: &Path) -> Self {
        DataPaths {
            indeclinables: base.join("indeclinables.csv"),
            nominal_endings_compounded: base.join("nominal-endings-compounded.csv"),
            nominal_endings_inflected: base.join("nominal-endings-inflected.csv"),
            nominal_stems: base.join("nominal-stems.csv"),
            participle_stems: base.join("participle-stems.csv"),
            prefix_groups: base.join("prefix-groups.csv"),
            prefixed_roots: base.join("prefixed-roots.csv"),
            pronouns: base.join("pronouns.csv"),
            sandhi_rules: base.join("sandhi-rules.csv"),
            unprefixed_roots: base.join("unprefixed-roots.csv"),
            verb_endings: base.join("verb-endings.csv"),
            verb_prefixes: base.join("verb-prefixes.csv"),
            verbal_indeclinables: base.join("verbal-indeclinables.csv"),
            verbs: base.join("verbs.csv"),

            lemma_counts: base.join("model/lemma-counts.csv"),
        }
    }
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

fn add_indeclinables(path: &Path, padas: &mut PadaMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let pada = r[0].to_string();
        padas.insert(pada, Semantics::Avyaya);
    }
    Ok(())
}

fn add_nominal_endings_compounded(path: &Path, endings: &mut EndingMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let stem_lingas = parse_stem_linga(&r[1]);
        let ending = r[2].to_string();
        let ending_linga = parse_linga(&r[3]);

        let semantics = Semantics::Subanta(Subanta {
            stem: Stem::Basic {
                stem: stem.clone(),
                lingas: stem_lingas,
            },
            linga: ending_linga,
            vibhakti: Vibhakti::None,
            vacana: Vacana::None,
            is_purvapada: true,
        });
        endings.insert(ending, (stem, semantics));
    }
    Ok(())
}

fn add_nominal_endings_inflected(path: &Path, endings: &mut EndingMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;

        let stem = r[0].to_string();
        let ending = r[2].to_string();
        let linga = parse_linga(&r[3]);
        let semantics = Semantics::Subanta(Subanta {
            stem: Stem::Basic {
                stem: stem.clone(),
                lingas: vec![linga.clone()],
            },
            linga,
            vibhakti: parse_vibhakti(&r[4]),
            vacana: parse_vacana(&r[5]),
            is_purvapada: false,
        });
        endings.insert(ending, (stem, semantics));
    }
    Ok(())
}

fn add_nominal_stems(path: &Path, padas: &mut StemMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let lingas = parse_stem_linga(&r[1]);
        let semantics = Stem::Basic {
            stem: stem.clone(),
            lingas,
        };
        padas.insert(stem, semantics);
    }
    Ok(())
}

fn add_participle_stems(path: &Path, padas: &mut StemMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let root = r[1].to_string();
        padas.insert(
            stem,
            Stem::Krdanta {
                root,
                tense: parse_tense(&r[4]),
                prayoga: parse_prayoga(&r[5]),
            },
        );
    }
    Ok(())
}

fn add_prefix_groups(path: &Path, padas: &mut PadaMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        padas.insert(r[0].to_string(), Semantics::PrefixGroup);
    }
    Ok(())
}

fn add_pronouns(path: &Path, padas: &mut PadaMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;

        let stem = r[0].to_string();
        let text = r[2].to_string();
        let linga = parse_linga(&r[3]);
        let semantics = Semantics::Subanta(Subanta {
            stem: Stem::Basic {
                stem: stem.clone(),
                lingas: vec![linga.clone()],
            },
            linga: linga.clone(),
            vibhakti: parse_vibhakti(&r[4]),
            vacana: parse_vacana(&r[5]),
            is_purvapada: false,
        });
        padas.insert(text, semantics);
    }
    Ok(())
}

fn add_verbal_indeclinables(path: &Path, padas: &mut PadaMap) -> Result<()> {
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

fn add_verbs(path: &Path, padas: &mut PadaMap) -> Result<()> {
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

pub fn read_nominal_endings(paths: &DataPaths) -> Result<EndingMap> {
    let mut endings = EndingMap::new();
    add_nominal_endings_compounded(&paths.nominal_endings_compounded, &mut endings)?;
    add_nominal_endings_inflected(&paths.nominal_endings_inflected, &mut endings)?;
    Ok(endings)
}

pub fn read_stems(paths: &DataPaths) -> Result<StemMap> {
    let mut stems = StemMap::new();
    add_nominal_stems(&paths.nominal_stems, &mut stems)?;
    add_participle_stems(&paths.participle_stems, &mut stems)?;
    Ok(stems)
}

pub fn read_padas(paths: &DataPaths) -> Result<PadaMap> {
    let mut padas = PadaMap::new();
    add_indeclinables(&paths.indeclinables, &mut padas)?;
    add_prefix_groups(&paths.prefix_groups, &mut padas)?;
    add_pronouns(&paths.pronouns, &mut padas)?;
    add_verbal_indeclinables(&paths.verbal_indeclinables, &mut padas)?;
    add_verbs(&paths.verbs, &mut padas)?;
    Ok(padas)
}

pub fn read_lemma_probabilities(paths: &DataPaths) -> Result<HashMap<String, f32>> {
    let mut ret = HashMap::new();

    let mut rdr = csv::Reader::from_path(&paths.lemma_counts)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let lemma = &r[0];
        let prob = r[1].parse::<f32>()?;
        ret.insert(lemma.to_string(), prob.log10());
    }
    Ok(ret)
}
