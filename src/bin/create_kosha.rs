//! Creates an FST kosha using our raw linguistic data.
//!
//! This binary is computationally intensive and may take several minutes.
//!
//! TODO:
//! - prefixes
//! - sya-Satf, sya-SAnac, ya-SAnac
//! - upasarga + tvA (upAsitvA, etc.)
//! - pada variants
//! - dedupe krdantas with existing nominals
//! - update `morph` encoding for krdantas
use clap::Parser;
use lazy_static::lazy_static;
use log::info;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process;
use vidyut_cheda::sounds::{is_ac, is_ghosha, is_hal};
use vidyut_cheda::Config;
use vidyut_kosha::morph::*;
use vidyut_kosha::Builder;
use vidyut_prakriya::args as vp;
use vidyut_prakriya::dhatupatha::Entry as DhatuEntry;
use vidyut_prakriya::{Dhatupatha, Vyakarana};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type UpasargaDhatuMap = HashMap<String, Vec<Vec<String>>>;

/// A list of pratipadikas.
type StemVec = Vec<(String, Pratipadika)>;

/// A list of complete padas.
type PadaVec = Vec<(String, Pada)>;

/// A list of sup pratyayas.
type SupVec = Vec<(String, String, Pada)>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the underlying raw data.
    #[arg(short, long)]
    input_dir: PathBuf,

    /// Path to a dhatupatha file (e.g. the one used by vidyut-prakriya)
    #[arg(short, long)]
    dhatupatha: PathBuf,

    /// Path to the Vidyut output directory.
    #[arg(short, long)]
    output_dir: PathBuf,
}

/// Defines all of the input data paths we need to construct the FST.
pub struct DataPaths {
    pub indeclinables: PathBuf,
    pub nominal_endings_compounded: PathBuf,
    pub nominal_endings_inflected: PathBuf,
    pub basic_pratipadikas: PathBuf,
    pub irregular_subantas: PathBuf,
    pub prefix_groups: PathBuf,
    pub prefixed_roots: PathBuf,
    pub upasarga_dhatus: PathBuf,
}

impl DataPaths {
    pub fn new(base: impl AsRef<Path>) -> Self {
        let base = base.as_ref();
        DataPaths {
            indeclinables: base.join("indeclinables.csv"),
            nominal_endings_compounded: base.join("nominal-endings-compounded.csv"),
            nominal_endings_inflected: base.join("nominal-endings-inflected.csv"),
            basic_pratipadikas: base.join("nominal-stems.csv"),
            irregular_subantas: base.join("nominals-irregular.csv"),
            prefix_groups: base.join("prefix-groups.csv"),
            prefixed_roots: base.join("prefixed-roots.csv"),
            upasarga_dhatus: base.join("upasarga-dhatus.csv"),
        }
    }
}

/// Creates a collection of (linga, vibhakti, vacana) combinations.
fn linga_vibhakti_vacana_options() -> Vec<(vp::Linga, vp::Vibhakti, vp::Vacana)> {
    let mut ret = Vec::new();
    for linga in vp::Linga::iter() {
        for vibhakti in vp::Vibhakti::iter() {
            for vacana in vp::Vacana::iter() {
                ret.push((*linga, *vibhakti, *vacana))
            }
        }
    }
    ret
}

/// Creates a collection of common sanAdi combinations.
fn sanadi_options() -> Vec<Vec<vp::Sanadi>> {
    use vp::Sanadi::*;
    vec![
        vec![],
        vec![Ric],
        vec![san],
        vec![yaN],
        vec![yaNluk],
        vec![Ric, san],
        vec![san, Ric],
    ]
}

fn tinanta_options() -> Vec<(
    vp::Prayoga,
    vp::DhatuPada,
    vp::Lakara,
    vp::Purusha,
    vp::Vacana,
)> {
    let mut ret = Vec::new();
    for prayoga in vp::Prayoga::iter() {
        for pada in vp::DhatuPada::iter() {
            if *prayoga == vp::Prayoga::Bhave {
                // Duplicates karmani -- skip
                continue;
            }
            for lakara in vp::Lakara::iter() {
                if *lakara == vp::Lakara::Let {
                    // Experimental -- skip
                    continue;
                }
                for purusha in vp::Purusha::iter() {
                    for vacana in vp::Vacana::iter() {
                        ret.push((*prayoga, *pada, *lakara, *purusha, *vacana));
                    }
                }
            }
        }
    }
    ret
}

fn parse_stem_linga(code: &str) -> &[Linga] {
    use Linga::*;
    match code {
        "m" => &[Pum],
        "f" => &[Stri],
        "n" => &[Napumsaka],
        "mf" => &[Pum, Stri],
        "fn" => &[Stri, Napumsaka],
        "mn" => &[Pum, Napumsaka],
        "mfn" => &[Pum, Stri, Napumsaka],
        "none" => &[],
        &_ => panic!("Unknown type {}", code),
    }
}

/// Adds avyayas scraped from the MW dictionary.
fn add_avyayas(path: &Path, padas: &mut PadaVec) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let pada = r[0].to_string();
        padas.push((
            pada.clone(),
            Pada::Avyaya(Avyaya {
                pratipadika: Pratipadika::Basic {
                    text: pada,
                    lingas: Vec::new(),
                },
            }),
        ));
    }
    Ok(())
}

// Adds irregular subantas specified manually.
//
// TODO: can we deprecate this given vidyut-prakriya?
fn add_irregular_subantas(path: &Path, padas: &mut PadaVec) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let pratipadika = r[0].to_string();
        let stem_lingas = parse_stem_linga(&r[1]);
        let pada = r[2].to_string();
        let linga = r[3].parse().ok();
        let vibhakti = r[4].parse().ok();
        let vacana = r[5].parse().ok();

        let semantics = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: pratipadika.clone(),
                lingas: stem_lingas.to_vec(),
            },
            linga,
            vibhakti,
            vacana,
            is_purvapada: false,
        });

        padas.push((pada.clone(), semantics));
    }

    // `mahA` is common but missing upstream, so add it specially.
    let semantics = Pada::Subanta(Subanta {
        pratipadika: Pratipadika::Basic {
            text: "mahat".to_string(),
            lingas: vec![Linga::Pum, Linga::Stri, Linga::Napumsaka],
        },
        linga: None,
        vibhakti: None,
        vacana: None,
        is_purvapada: true,
    });
    padas.push(("mahA".to_string(), semantics));

    Ok(())
}

/// Add simple pratipadikas scraped from the MW dictionary.
///
/// TODO: deduplicate with our krdantas, etc.
fn add_basic_pratipadikas(path: &Path, stems: &mut StemVec) -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let lingas = parse_stem_linga(&r[1]);
        let semantics = Pratipadika::Basic {
            text: stem.clone(),
            lingas: lingas.to_vec(),
        };
        stems.push((stem, semantics));
    }
    Ok(())
}

/// Adds various common prefix groups.
///
/// TODO: this doesn't make sense. We aren't storing the split prefixes anywhere ...
fn add_prefix_groups(path: &Path, padas: &mut PadaVec) -> Result<()> {
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
        padas.push((value.to_string(), semantics));
    }
    Ok(())
}

/// TODO: delete this after migrating to vidyut-prakriya for everything.
fn read_sup_endings(paths: &DataPaths) -> Result<SupVec> {
    let mut endings = SupVec::new();

    let mut rdr = csv::Reader::from_path(&paths.nominal_endings_compounded)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let stem_lingas = parse_stem_linga(&r[1]);
        let ending = r[2].to_string();
        let ending_linga = r[3].parse()?;

        let semantics = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: stem.clone(),
                lingas: stem_lingas.to_vec(),
            },
            linga: Some(ending_linga),
            vibhakti: None,
            vacana: None,
            is_purvapada: true,
        });
        endings.push((ending, stem, semantics));
    }

    let mut rdr = csv::Reader::from_path(&paths.nominal_endings_inflected)?;
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
            linga: Some(linga),
            vibhakti: r[4].parse().ok(),
            vacana: r[5].parse().ok(),
            is_purvapada: false,
        });
        endings.push((ending, stem, semantics));
    }

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

fn read_stems(paths: &DataPaths) -> Result<StemVec> {
    let mut stems = StemVec::new();
    add_basic_pratipadikas(&paths.basic_pratipadikas, &mut stems)?;

    // Add simple support for variants.
    let mut variants = StemVec::new();
    for (k, v) in &stems {
        for k_variant in get_variants(&k) {
            variants.push((k_variant.clone(), v.clone()));
        }
    }
    stems.extend(variants);

    Ok(stems)
}

fn read_padas(paths: &DataPaths) -> Result<PadaVec> {
    let mut padas = PadaVec::with_capacity(20_000_000);
    add_avyayas(&paths.indeclinables, &mut padas).expect("Could not find indeclinables");
    add_prefix_groups(&paths.prefix_groups, &mut padas).expect("Could not find prefix groups");
    add_irregular_subantas(&paths.irregular_subantas, &mut padas)
        .expect("Could not find irregular subantas");

    let mut variants = PadaVec::new();
    for (k, v) in &padas {
        for k_variant in get_variants(&k) {
            variants.push((k_variant.clone(), v.clone()));
        }
    }
    padas.extend(variants);

    Ok(padas)
}

fn inflect_halanta_stem(stem: &str, sup: &str) -> String {
    if sup.starts_with(is_ac) {
        String::from(stem) + sup
    } else {
        let n = stem.len();
        let prefix = &stem[..n - 1];
        let stem_ending = &stem[n - 1..n];

        let stem_ending = match stem_ending {
            "k" | "K" | "g" | "G" => "k",
            "c" | "C" | "j" | "J" => "k",
            "w" | "W" | "q" | "Q" => "w",
            "t" | "T" | "d" | "D" => "t",
            "p" | "P" | "b" | "B" => "p",
            _ => stem_ending,
        };
        let stem_ending = if sup.starts_with(is_ghosha) {
            match stem_ending {
                "k" => "g",
                "w" => "q",
                "t" => "d",
                "p" => "b",
                _ => stem_ending,
            }
        } else {
            stem_ending
        };

        String::from(prefix) + stem_ending + sup
    }
}

// Generates all nominal padas and adds them to the pada map.
fn add_nominals(stems: &StemVec, endings: &SupVec, padas: &mut PadaVec) {
    let mut stem_to_endings = HashMap::new();
    for (ending, stem, semantics) in endings {
        if !stem_to_endings.contains_key(stem) {
            let stem = stem.clone();
            stem_to_endings.insert(stem, vec![]);
        }
        stem_to_endings
            .get_mut(stem)
            .unwrap()
            .push((ending.clone(), semantics.clone()));
    }

    // For all stems, ...
    for (stem_text, stem_semantics) in stems {
        let mut was_inserted = false;

        // And all stem endings ...
        for (stem_ending, sup_pratyayas) in stem_to_endings.iter() {
            // If the stem ends in this ending ...
            if let Some(prefix) = stem_text.strip_suffix(stem_ending) {
                // Then for all pratyayas that the ending allows, ...
                for (sup_text, sup_semantics) in sup_pratyayas {
                    let pada_text = prefix.to_string() + sup_text;

                    if let Pada::Subanta(sup_semantics) = sup_semantics {
                        // Create and insert the corresponding pada.
                        let pada_semantics = Pada::Subanta(Subanta {
                            pratipadika: stem_semantics.clone(),
                            ..sup_semantics.clone()
                        });
                        padas.push((pada_text.clone(), pada_semantics));
                    }
                }
                was_inserted = true;
            }
        }

        if !was_inserted {
            // If the stem is a special consonant ending ...
            if is_hal(stem_text.chars().last().unwrap()) {
                let pratyayas = stem_to_endings
                    .get("_")
                    .expect("`_` ending should be defined");
                for (sup_text, sup_semantics) in pratyayas {
                    let pada_text = inflect_halanta_stem(stem_text, sup_text);

                    if let Pada::Subanta(sup_semantics) = sup_semantics {
                        // Create and insert the corresponding pada.
                        let pada_semantics = Pada::Subanta(Subanta {
                            pratipadika: stem_semantics.clone(),
                            ..sup_semantics.clone()
                        });
                        padas.push((pada_text.clone(), pada_semantics));
                    }
                }
            }
        }
    }
}

fn create_sarvanamas(padas: &mut PadaVec) {
    // Data copied from vidyut-prakriya.
    const SARVANAMA: &[&str] = &[
        // qatara, qatama
        // TODO: actually detect qatarac/qatamac in vidyut-prakriya.
        "katara", "yatara", "tatara", "ekatara", "katama", "yatama", "tatama", "ekatama",
        // sarvAdi
        "sarva", "viSva", "uBa", "uBaya", "qatara", "qatama", "anya", "anyatara", "itara", "tvat",
        "tva", "nema", "sama", "sima", "pUrva", "para", "avara", "dakziRa", "uttara", "apara",
        "aDara", "sva", "antara", "tyad", "tad", "yad", "etad", "idam", "adas", "eka", "dvi",
        "yuzmad", "asmad", "Bavatu~", "kim",
    ];

    let linga_vibhakti_vacana = linga_vibhakti_vacana_options();

    let v = Vyakarana::builder()
        .log_steps(false)
        .is_chandasi(true)
        .build();
    for stem in SARVANAMA {
        let prati = vp::Pratipadika::basic(stem);
        let lingas = vec![Linga::Pum, Linga::Stri, Linga::Napumsaka];

        for (linga, vibhakti, vacana) in &linga_vibhakti_vacana {
            let args = vp::Subanta::new(prati.clone(), *linga, *vibhakti, *vacana);
            let prakriyas = v.derive_subantas(&args);
            for p in prakriyas {
                let morph = Pada::Subanta(Subanta {
                    pratipadika: Pratipadika::Basic {
                        text: stem.to_string(),
                        lingas: lingas.clone(),
                    },
                    linga: Some(Linga::from(*linga)),
                    vibhakti: Some(Vibhakti::from(*vibhakti)),
                    vacana: Some(Vacana::from(*vacana)),
                    is_purvapada: false,
                });
                let text = p.text();
                padas.push((text, morph));
            }
        }
    }
}

/// Creates all tinantas.
///
/// This function generates the following combinations:
///
/// (upasarga, dhatu, sanadi, pada, lakara, purusha, vacana)
///
/// - `upasarga` comes from the Upasargartha-candrika.
/// - `dhatu` comes from the Dhatupatha on ashtadhyayi.com
///
/// TODO: gati, cvi
fn create_tinantas(
    entries: &Vec<DhatuEntry>,
    upasarga_dhatus: &UpasargaDhatuMap,
    padas: &mut PadaVec,
) {
    let all_sanadis = sanadi_options();
    let args = tinanta_options();

    let v = Vyakarana::builder()
        .log_steps(false)
        .is_chandasi(true)
        .build();

    let results: Vec<_> = entries
        .par_iter()
        .flat_map(|entry| {
            let new = Vec::new();
            let upasarga_groups = upasarga_dhatus.get(entry.code()).unwrap_or(&new);
            let mut ret = Vec::new();

            for group in upasarga_groups {
                for sanadi in &all_sanadis {
                    let dhatu = entry
                        .dhatu()
                        .clone()
                        .with_sanadi(sanadi)
                        .with_prefixes(group);

                    for (prayoga, dhatu_pada, lakara, purusha, vacana) in &args {
                        let args = vp::Tinanta::builder()
                            .dhatu(dhatu.clone())
                            .prayoga(*prayoga)
                            .pada(*dhatu_pada)
                            .lakara(*lakara)
                            .purusha(*purusha)
                            .vacana(*vacana)
                            .build()
                            .expect("ok");

                        let pada_prayoga = match (dhatu_pada, prayoga) {
                            (vp::DhatuPada::Parasmai, _) => PadaPrayoga::Parasmaipada,
                            (vp::DhatuPada::Atmane, vp::Prayoga::Kartari) => {
                                PadaPrayoga::AtmanepadaKartari
                            }
                            (vp::DhatuPada::Atmane, _) => PadaPrayoga::AtmanepadaNotKartari,
                        };

                        let prakriyas = v.derive_tinantas(&args);
                        ret.extend(prakriyas.iter().map(|prakriya| {
                            let text = prakriya.text();
                            let semantics = Pada::Tinanta(Tinanta {
                                dhatu: dhatu.clone().into(),
                                purusha: Purusha::from(*purusha),
                                vacana: Vacana::from(*vacana),
                                lakara: Lakara::from(*lakara),
                                pada: PadaPrayoga::from(pada_prayoga),
                            });

                            (text, semantics)
                        }));
                    }
                }
            }

            ret.into_par_iter()
        })
        .collect();

    padas.extend(results);
}

/// Creates all krdantas that form nominals.
///
/// This function generates the following combinations:
///
/// (upasarga, dhatu, sanadi, pada, krt, linga, vibhakti, vacana)
///
/// - `upasarga` comes from the Upasargartha-candrika.
/// - `dhatu` comes from the Dhatupatha on ashtadhyayi.com
///
/// TODO: gati, cvi
fn create_inflected_krdantas(
    entries: &Vec<DhatuEntry>,
    upasarga_dhatus: &UpasargaDhatuMap,
    padas: &mut PadaVec,
) {
    use vp::BaseKrt as VKrt;

    let linga_vibhakti_vacana = linga_vibhakti_vacana_options();
    let all_sanadis = sanadi_options();
    let all_krts = &[
        // Lit
        VKrt::kvasu,
        VKrt::kAnac,
        // nistha
        VKrt::kta,
        VKrt::ktavatu,
        // Lat
        VKrt::Satf,
        VKrt::SAnac,
        // krtya
        VKrt::yat,
        VKrt::Ryat,
        VKrt::kyap,
        VKrt::tavya,
        VKrt::anIyar,
        // Common
        VKrt::Rvul,
        VKrt::lyuw,
        VKrt::tfc,
        // TODO: all all of the others, including unadis.
    ];

    let sat_pratyayas = &[VKrt::Satf, VKrt::SAnac];

    let v = Vyakarana::builder()
        .log_steps(false)
        .is_chandasi(true)
        .build();

    let results: Vec<_> = entries
        .par_iter()
        .flat_map(|entry| {
            let new = Vec::new();
            let upasarga_groups = upasarga_dhatus.get(entry.code()).unwrap_or(&new);
            let mut ret = Vec::new();

            for group in upasarga_groups {
                for sanadi in &all_sanadis {
                    let dhatu = entry
                        .dhatu()
                        .clone()
                        .with_sanadi(sanadi)
                        .with_prefixes(group);

                    for krt in all_krts {
                        for (linga, vibhakti, vacana) in &linga_vibhakti_vacana {
                            let krdanta = vp::Krdanta::builder()
                                .dhatu(dhatu.clone())
                                .krt(*krt)
                                .build()
                                .expect("ok");

                            let args =
                                vp::Subanta::new(krdanta.clone(), *linga, *vibhakti, *vacana);

                            let prakriyas = v.derive_subantas(&args);
                            ret.extend(prakriyas.iter().map(|p| {
                                let text = p.text();
                                let semantics = Pada::Subanta(Subanta {
                                    pratipadika: Pratipadika::Krdanta {
                                        dhatu: dhatu.clone().into(),
                                        krt: Krt::new(*krt),
                                    },
                                    linga: Some(Linga::from(*linga)),
                                    vibhakti: Some(Vibhakti::from(*vibhakti)),
                                    vacana: Some(Vacana::from(*vacana)),
                                    is_purvapada: false,
                                });

                                (text, semantics)
                            }));
                        }
                    }

                    // lrt-sat (karizyan, karizyamARaH, ...)
                    for krt in sat_pratyayas {
                        for (linga, vibhakti, vacana) in &linga_vibhakti_vacana {
                            let krdanta = vp::Krdanta::builder()
                                .dhatu(dhatu.clone())
                                .lakara(vp::Lakara::Lrt)
                                .krt(VKrt::Satf)
                                .build()
                                .expect("ok");

                            let args =
                                vp::Subanta::new(krdanta.clone(), *linga, *vibhakti, *vacana);

                            let prakriyas = v.derive_subantas(&args);
                            ret.extend(prakriyas.iter().map(|p| {
                                let text = p.text();
                                let semantics = Pada::Subanta(Subanta {
                                    pratipadika: Pratipadika::Krdanta {
                                        dhatu: dhatu.clone().into(),
                                        krt: Krt::new(*krt),
                                    },
                                    linga: Some(Linga::from(*linga)),
                                    vibhakti: Some(Vibhakti::from(*vibhakti)),
                                    vacana: Some(Vacana::from(*vacana)),
                                    is_purvapada: false,
                                });

                                (text, semantics)
                            }));
                        }
                    }
                }
            }

            ret.into_par_iter()
        })
        .collect();

    padas.extend(results);
}

/// Creates all krdantas that form avyayas.
///
/// This function generates the following combinations:
///
/// (upasarga, dhatu, sanadi, krt)
///
/// - `upasarga` comes from the Upasargartha-candrika.
/// - `dhatu` comes from the Dhatupatha on ashtadhyayi.com
///
/// TODO: gati, cvi
fn create_avyaya_krdantas(
    entries: &Vec<DhatuEntry>,
    upasarga_dhatus: &UpasargaDhatuMap,
    padas: &mut PadaVec,
) {
    let all_sanadis = sanadi_options();
    let all_krts = &[vp::BaseKrt::ktvA, vp::BaseKrt::tumun];

    let v = Vyakarana::builder()
        .log_steps(false)
        .is_chandasi(true)
        .build();

    let results: Vec<_> = entries
        .par_iter()
        .flat_map(|entry| {
            let new = Vec::new();
            let upasarga_groups = upasarga_dhatus.get(entry.code()).unwrap_or(&new);
            let mut ret = Vec::new();

            for group in upasarga_groups {
                for sanadi in &all_sanadis {
                    let dhatu = entry
                        .dhatu()
                        .clone()
                        .with_sanadi(sanadi)
                        .with_prefixes(group);
                    for krt in all_krts {
                        let args = vp::Krdanta::builder()
                            .dhatu(dhatu.clone().with_sanadi(sanadi))
                            .krt(*krt)
                            .build()
                            .expect("ok");

                        let prakriyas = v.derive_krdantas(&args);
                        ret.extend(prakriyas.iter().map(|p| {
                            let text = p.text();
                            let semantics = Pada::Avyaya(Avyaya {
                                pratipadika: Pratipadika::Krdanta {
                                    dhatu: dhatu.clone().into(),
                                    krt: Krt::new(*krt),
                                },
                            });

                            (text, semantics)
                        }));
                    }
                }
            }

            ret.into_par_iter()
        })
        .collect();

    padas.extend(results);
}

/// Maps a dhatu code (e.g. "01.0001") to all lists of prefixes it might take.
fn parse_upasarga_dhatus(path: &Path) -> Result<UpasargaDhatuMap> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut ret: UpasargaDhatuMap = HashMap::new();
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let upasargas: Vec<_> = r[0].split("-").map(|x| x.to_string()).collect();
        let code = r[2].to_string();
        // the empty Vec is for the default case (no prefixes).
        ret.entry(code).or_insert(vec![Vec::new()]).push(upasargas);
    }

    Ok(ret)
}

fn run(args: Args) -> Result<()> {
    info!("Reading linguistic data ...");

    let data_paths = DataPaths::new(Path::new(&args.input_dir));
    let dhatupatha = Dhatupatha::from_path(&args.dhatupatha)?;
    // let dhatu_entries: Vec<DhatuEntry> = dhatupatha.into_iter().take(200).collect();
    let dhatu_entries: Vec<DhatuEntry> = dhatupatha.into_iter().collect();

    let mut padas = read_padas(&data_paths)?;

    info!("Creating tinantas ...");
    let upasarga_dhatus = parse_upasarga_dhatus(&data_paths.upasarga_dhatus)?;
    create_tinantas(&dhatu_entries, &upasarga_dhatus, &mut padas);

    info!("Creating krdantas (inflected) ...");
    create_inflected_krdantas(&dhatu_entries, &upasarga_dhatus, &mut padas);

    info!("Creating krdantas (avyaya) ...");
    create_avyaya_krdantas(&dhatu_entries, &upasarga_dhatus, &mut padas);

    info!("Creating plain subantas ...");
    create_sarvanamas(&mut padas);

    let stems = read_stems(&data_paths)?;
    let endings = read_sup_endings(&data_paths)?;
    add_nominals(&stems, &endings, &mut padas);

    info!("Sorting keys ...");
    padas.par_sort();

    info!("Inserting entries ...");
    let config = Config::new(&args.output_dir);
    let mut builder = Builder::new(config.kosha())?;
    let mut num_words = 0;
    for (key, pada) in padas {
        builder.insert(&key, &pada)?;
        num_words += 1;
    }

    info!("Finishing build ...");
    builder.finish()?;

    info!("Complete. (Inserted {num_words} entries.)");
    Ok(())
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    if let Err(err) = run(args) {
        println!("{}", err);
        process::exit(1);
    }
}
