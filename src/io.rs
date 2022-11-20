//! Utilities for reading linguistic data.
//!
//! Most of our data comes from the [sanskrit/data](https://github.com/sanskrit/data) project. In
//! the future, Vidyut might generate its own linguistic data instead.

use crate::old_lexicon::{EndingMap, PadaMap, StemMap};
use crate::semantics::*;
use lazy_static::lazy_static;
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

fn parse_pada_prayoga(code: &str) -> PadaPrayoga {
    match code {
        "para" => PadaPrayoga::Parasmaipada,
        "atma" => PadaPrayoga::AtmanepadaKartari,
        "pass" => PadaPrayoga::AtmanepadaNotKartari,
        &_ => panic!("Unknown type {}", code),
    }
}

fn parse_krt_pratyaya(tense: &str, voice: &str) -> KrtPratyaya {
    match (tense, voice) {
        ("past", "active") => KrtPratyaya::Ktavat,
        ("past", "pass") => KrtPratyaya::Kta,

        ("pres", "para") => KrtPratyaya::Shatr,
        ("pres", "atma") => KrtPratyaya::Shanac,
        ("pres", "pass") => KrtPratyaya::YakShanac,

        ("fut", "para") => KrtPratyaya::SyaShatr,
        ("fut", "atma") => KrtPratyaya::SyaShanac,
        ("fut", "pass") => KrtPratyaya::Krtya,

        ("perf", "para") => KrtPratyaya::Kvasu,
        ("perf", "atma") => KrtPratyaya::Kanac,
        ("perf", "pass") => KrtPratyaya::Kanac,
        (&_, &_) => panic!("Unknown type (`{tense}`, `{voice}`)"),
    }
}

fn add_indeclinables(path: &Path, padas: &mut PadaMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let pada = r[0].to_string();
        padas.insert(
            pada.clone(),
            Pada::Avyaya(Avyaya {
                pratipadika: Pratipadika::Basic {
                    text: pada,
                    lingas: Vec::new(),
                },
            }),
        );
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

        let semantics = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: stem.clone(),
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
        let linga = r[3].parse()?;
        let semantics = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: stem.clone(),
                lingas: vec![linga],
            },
            linga,
            vibhakti: r[4].parse()?,
            vacana: r[5].parse()?,
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
        let semantics = Pratipadika::Basic {
            text: stem.clone(),
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
            Pratipadika::Krdanta {
                dhatu: Dhatu(root),
                pratyaya: parse_krt_pratyaya(&r[4], &r[5]),
            },
        );
    }
    Ok(())
}

fn add_prefix_groups(path: &Path, padas: &mut PadaMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let value = &r[0];
        // FIXME: consider deleting this logic.
        let semantics = Pada::Avyaya(Avyaya {
            pratipadika: Pratipadika::Basic {
                text: value.to_string(),
                lingas: Vec::new(),
            },
        });
        padas.insert(value.to_string(), semantics);
    }
    Ok(())
}

fn add_pronouns(path: &Path, padas: &mut PadaMap) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;

        let stem = r[0].to_string();
        let text = r[2].to_string();
        let linga = r[3].parse()?;
        let semantics = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: stem.clone(),
                lingas: vec![linga],
            },
            linga,
            vibhakti: r[4].parse()?,
            vacana: r[5].parse()?,
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
        let pratyaya = match &row[3] {
            "gerund" => {
                if pada.ends_with("ya") {
                    KrtPratyaya::Lyap
                } else {
                    KrtPratyaya::Ktva
                }
            }
            "infinitive" => KrtPratyaya::Tumun,
            &_ => panic!("Unknown indeclinable type `{}`", &row[3]),
        };
        let semantics = Pada::Avyaya(Avyaya {
            pratipadika: Pratipadika::Krdanta {
                dhatu: Dhatu(root),
                pratyaya,
            },
        });

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

        let vacana = r[5].parse()?;

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

        let pada = parse_pada_prayoga(&r[7]);

        padas.insert(
            text,
            Pada::Tinanta(Tinanta {
                dhatu: Dhatu(root),
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

/// Create spelling variants for the given stems.
fn get_variants(text: &str) -> Vec<String> {
    lazy_static! {
        static ref PREFIXES: Vec<(String, String)> = vec![
            ("saMp".to_string(), "samp".to_string()),
            ("saMb".to_string(), "samb".to_string()),
            ("saMB".to_string(), "samB".to_string()),
            ("samp".to_string(), "saMp".to_string()),
            ("samb".to_string(), "saMb".to_string()),
            ("samB".to_string(), "saMB".to_string()),
        ];
    }

    let mut variants = vec![];
    for (old, new) in PREFIXES.iter() {
        if text.starts_with(old) {
            variants.push(text.replace(old, new));
        }
    }

    if text.contains("ttr") {
        variants.push(text.replace("ttr", "tr"));
    }
    variants
}

pub fn read_stems(paths: &DataPaths) -> Result<StemMap> {
    let mut stems = StemMap::new();
    add_nominal_stems(&paths.nominal_stems, &mut stems)?;
    add_participle_stems(&paths.participle_stems, &mut stems)?;

    // Add simple support for variants.
    let mut variants = StemMap::new();
    for (k, v) in stems.iter() {
        for k_variant in get_variants(k) {
            variants.insert(k_variant, v.clone());
        }
    }
    stems.extend(variants);

    Ok(stems)
}

pub fn read_padas(paths: &DataPaths) -> Result<PadaMap> {
    let mut padas = PadaMap::new();
    add_indeclinables(&paths.indeclinables, &mut padas).expect("Could not find indeclinables");
    add_prefix_groups(&paths.prefix_groups, &mut padas).expect("Could not find prefix groups");
    add_pronouns(&paths.pronouns, &mut padas).expect("Could not find pronouns");
    add_verbal_indeclinables(&paths.verbal_indeclinables, &mut padas)
        .expect("Could not find verbal indeclinables");
    add_verbs(&paths.verbs, &mut padas).expect("Could not find verbs");

    let mut variants = PadaMap::new();
    for (k, v) in padas.iter() {
        for k_variant in get_variants(k) {
            variants.insert(k_variant, v.clone());
        }
    }
    padas.extend(variants);

    Ok(padas)
}
