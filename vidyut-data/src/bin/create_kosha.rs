//! Creates a kosha of Sanskrit words and writes the results to disk.
use clap::Parser;
use log::info;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process;
use vidyut_kosha::entries::{DhatuEntry, PadaEntry, PratipadikaEntry, SubantaEntry};
use vidyut_kosha::packing::PackedEntry;
use vidyut_kosha::Builder;
use vidyut_prakriya::args::*;
use vidyut_prakriya::{Dhatupatha, Vyakarana};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type UpasargaDhatuMap = HashMap<String, Vec<Vec<String>>>;
type Pratipadikas = Vec<(String, Pratipadika)>;
type Entries = Vec<(String, PackedEntry)>;
type SmallSubanta = (String, Linga, Vibhakti, Vacana);

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the underlying raw data.
    #[arg(long)]
    input_dir: PathBuf,

    /// Path to a dhatupatha file (e.g. the one used by vidyut-prakriya)
    #[arg(long)]
    dhatupatha: PathBuf,

    /// Path to the Vidyut output directory.
    #[arg(long)]
    output_dir: PathBuf,

    /// (testing) the number of dhatus to use when building the kosha. If unset, use all possible
    /// dhatus.
    #[arg(long)]
    num_dhatus: Option<usize>,

    #[arg(long, value_delimiter = ',')]
    filters: Vec<String>,
}

/// Defines all of the input data paths we need to construct the kosha.
pub struct DataPaths {
    pub avyayas: PathBuf,
    pub pratipadikas: PathBuf,
    pub gati: PathBuf,
    pub upasarga_dhatus: PathBuf,
}

impl DataPaths {
    fn new(base: &Path) -> Self {
        DataPaths {
            avyayas: base.join("indeclinables.csv"),
            pratipadikas: base.join("nominal-stems.csv"),
            gati: base.join("prefix-groups.csv"),
            upasarga_dhatus: base.join("upasarga-dhatus.csv"),
        }
    }
}

/// Creates a preconfigured Vyakarana instance.
fn create_vyakarana() -> Vyakarana {
    Vyakarana::builder()
        .log_steps(false)
        .nlp_mode(true)
        .is_chandasi(true)
        .build()
}

/// Returns all combinations of (linga, vibhakti, vacana)
fn sup_options() -> Vec<(Linga, Vibhakti, Vacana)> {
    let mut ret = Vec::new();
    for linga in Linga::iter() {
        for vibhakti in Vibhakti::iter() {
            for vacana in Vacana::iter() {
                ret.push((linga, vibhakti, vacana))
            }
        }
    }
    ret
}

/// Returns all combinations of (prayoga, lakara, purusha, vacana)
fn tin_options() -> Vec<(Prayoga, Lakara, Purusha, Vacana)> {
    let mut ret = Vec::new();
    for prayoga in Prayoga::iter() {
        if prayoga == Prayoga::Bhave {
            // Duplicates karmani -- skip
            continue;
        }
        for lakara in Lakara::iter() {
            if lakara == Lakara::Let {
                // Noisy -- skip
                continue;
            }
            for purusha in Purusha::iter() {
                for vacana in Vacana::iter() {
                    ret.push((prayoga, lakara, purusha, vacana));
                }
            }
        }
    }
    ret
}

/// Creates subanta endings for tricky pratipadikas.
///
/// TODO: improve the rest of the code so that we can delete this function.
fn create_subanta_endings() -> HashMap<String, Vec<(String, Linga, Vibhakti, Vacana)>> {
    let mut ret = HashMap::new();

    fn safe(s: &str) -> Slp1String {
        Slp1String::from(s).expect("static")
    }

    fn mula(s: &str, gana: Gana) -> Dhatu {
        Dhatu::mula(safe(s), gana)
    }

    let havis = Krdanta::new(mula("hu\\", Gana::Juhotyadi), Unadi::isi);
    let dhanus = Krdanta::new(mula("Dana~\\", Gana::Juhotyadi), Unadi::usi);
    let bhagavat = Taddhitanta::new(Pratipadika::basic(safe("Baga")), Taddhita::matup);
    let hanumat = Taddhitanta::new(Pratipadika::basic(safe("hanu")), Taddhita::matup);
    let mahat = Krdanta::new(mula("maha~", Gana::Bhvadi), Unadi::ati).with_require("mahat");

    let pratipadikas: &[(&str, &str, Pratipadika)] = &[
        ("is", "havis", havis.into()),
        ("us", "Danus", dhanus.into()),
        ("vat", "Bagavat", bhagavat.into()),
        ("mat", "hanumat", hanumat.into()),
        ("mahat", "mahat", mahat.into()),
    ];
    let lvv = sup_options();

    let v = create_vyakarana();
    for (ending_type, sample, phit) in pratipadikas {
        let mut endings = Vec::new();
        for (linga, vibhakti, vacana) in lvv.iter().copied() {
            let sup = Subanta::new(phit.clone(), linga, vibhakti, vacana);
            let prakriyas = v.derive_subantas(&sup);
            for p in prakriyas {
                let text = p.text();
                let offset = sample.len() - ending_type.len();
                let ending = &text[offset..];
                endings.push((ending.to_string(), linga, vibhakti, vacana));
            }
        }
        ret.insert(ending_type.to_string(), endings);
    }

    ret
}

/// Creates all standard combinations of (upasarga x dhatu x sanadi)
fn create_all_dhatus(
    builder: &mut Builder,
    dhatupatha_path: &Path,
    upasarga_dhatu_path: &Path,
) -> Result<Vec<Dhatu>> {
    let sanadis = {
        use Sanadi::*;
        vec![
            vec![],
            vec![Ric],
            vec![san],
            vec![yaN],
            vec![yaNluk],
            vec![Ric, san],
            vec![san, Ric],
        ]
    };

    // Load mula dhatus and the upasarga combinations they support.
    let dhatupatha = Dhatupatha::from_path(dhatupatha_path)?;
    let mut upasarga_dhatus: UpasargaDhatuMap = HashMap::new();
    {
        let mut rdr = csv::Reader::from_path(upasarga_dhatu_path)?;
        for maybe_row in rdr.records() {
            let r = maybe_row?;
            let upasargas: Vec<_> = r[0].split("-").map(|x| x.to_string()).collect();
            let code = r[2].to_string();
            // the empty Vec is for the default case (no prefixes).
            upasarga_dhatus
                .entry(code)
                .or_insert(vec![Vec::new()])
                .push(upasargas);
        }
    }

    // Create the final list of dhatus.
    let v = Vyakarana::new();
    let mut ret = Vec::new();
    let no_upasargas = vec![Vec::new()];
    for entry in &dhatupatha {
        let upasarga_groups = upasarga_dhatus.get(entry.code()).unwrap_or(&no_upasargas);

        for sanadi in &sanadis {
            for prefixes in upasarga_groups {
                let dhatu = entry
                    .dhatu()
                    .clone()
                    .with_sanadi(sanadi)
                    .with_prefixes(prefixes);
                let prakriyas = v.derive_dhatus(&dhatu);
                if let Some(p) = prakriyas.first() {
                    let text = p.text();
                    let entry = DhatuEntry::new(&dhatu, &text);
                    builder.register_dhatu_entry(&entry);

                    // Add valid dhatus only.
                    ret.push(dhatu);
                }
            }
        }
    }

    Ok(ret)
}

fn create_bare_krdantas(all_dhatus: &[Dhatu]) -> Vec<Krdanta> {
    use BaseKrt as K;

    let all_krts: Vec<BaseKrt> = K::iter()
        .filter(|k| !k.is_duplicate() && !k.is_avyaya())
        .collect();

    let v = create_vyakarana();

    let all_krdantas: Vec<_> = all_dhatus
        .par_chunks(1 + all_dhatus.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();
            for dhatu in chunk {
                for krt in all_krts.iter().copied() {
                    let prayoga_lakara: &[(Option<Prayoga>, Option<Lakara>)] = match krt {
                        K::Satf => &[
                            (Some(Prayoga::Kartari), Some(Lakara::Lat)),
                            (Some(Prayoga::Kartari), Some(Lakara::Lrt)),
                        ],
                        K::SAnac => &[
                            (Some(Prayoga::Kartari), Some(Lakara::Lat)),
                            (Some(Prayoga::Kartari), Some(Lakara::Lrt)),
                            (Some(Prayoga::Karmani), Some(Lakara::Lat)),
                            (Some(Prayoga::Karmani), Some(Lakara::Lrt)),
                        ],
                        _ => &[(None, None)],
                    };

                    for (prayoga, lakara) in prayoga_lakara.iter().copied() {
                        let mut builder = Krdanta::builder().dhatu(dhatu.clone()).krt(krt);
                        if let (Some(prayoga), Some(lakara)) = (prayoga, lakara) {
                            builder = builder.prayoga(prayoga).lakara(lakara);
                        }
                        let krdanta = builder.build().expect("ok");

                        // Only add valid krdantas, to avoid clogging the registry.
                        let prakriyas = v.derive_krdantas(&krdanta);
                        if !prakriyas.is_empty() {
                            ret.push(krdanta)
                        }
                    }
                }
            }

            ret.into_par_iter()
        })
        .collect();
    all_krdantas
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
fn create_all_tinantas(builder: &Builder, all_dhatus: &[Dhatu]) -> Entries {
    let plpv = tin_options();
    let v = create_vyakarana();

    all_dhatus
        .par_chunks(1 + all_dhatus.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for dhatu in chunk {
                let dhatu = dhatu.clone();
                for (prayoga, lakara, purusha, vacana) in plpv.iter().copied() {
                    let args = Tinanta::new(dhatu.clone(), prayoga, lakara, purusha, vacana);

                    let prakriyas = v.derive_tinantas(&args);
                    for prakriya in prakriyas {
                        let text = prakriya.text();
                        let entry = PadaEntry::Tinanta((&args).into());
                        let packed_entry = builder.pack(&entry).expect("ok");
                        ret.push((text, packed_entry))
                    }
                }
            }

            ret.into_par_iter()
        })
        .collect()
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
fn create_inflected_krt_subantas(builder: &mut Builder, all_krdantas: &[Krdanta]) -> Entries {
    let v = create_vyakarana();
    let lvv = sup_options();
    let paradigms: Vec<(Krdanta, Vec<SmallSubanta>)> = all_krdantas
        .par_chunks(1 + all_krdantas.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for krdanta in chunk {
                for variant in v.derive_krdantas(krdanta) {
                    // Restrict all padas to use the same rule choices as this pratipadika variant.
                    // This properly groups padas by their common prefix and lets us generate a
                    // useful paradigm.
                    let v_variant = create_vyakarana()
                        .into_builder()
                        .rule_choices(variant.rule_choices().to_vec())
                        .build();

                    let mut padas = Vec::new();
                    for (linga, vibhakti, vacana) in lvv.iter().copied() {
                        let args = Subanta::new(krdanta.clone(), linga, vibhakti, vacana);

                        let prakriyas = v_variant.derive_subantas(&args);
                        for prakriya in &prakriyas {
                            let text = prakriya.text();
                            padas.push((text, linga, vibhakti, vacana));
                        }
                    }
                    ret.push((krdanta.clone(), padas));
                }
            }

            ret.into_par_iter()
        })
        .collect();

    let mut entries = Vec::new();
    for (krdanta, padas) in paradigms {
        let phit: Pratipadika = krdanta.into();
        let phit_entry = (&phit).try_into().expect("ok");
        builder.register_pratipadika_entry(&phit_entry);

        // Prefixed subantas.
        let (key, value) = builder
            .register_subanta_paradigm(&(&phit).try_into().expect("ok"), &padas)
            .expect("ok");

        entries.push((key, value));
    }

    entries
}

fn create_avyaya_krt_subantas(builder: &mut Builder, all_dhatus: &[Dhatu]) -> Entries {
    use BaseKrt as K;
    const AVYAYA_KRTS: &[K] = &[K::ktvA, K::tumun];

    let mut all_krdantas: Vec<Krdanta> = Vec::new();
    for dhatu in all_dhatus {
        for krt in AVYAYA_KRTS.iter().copied() {
            let krdanta = Krdanta::new(dhatu.clone(), krt);
            let phit: Pratipadika = krdanta.clone().into();
            builder.register_pratipadika_entry(&(&phit).try_into().expect("ok"));
            all_krdantas.push(krdanta);
        }
    }
    info!(
        "Created {} krdanta avyaya pratipadikas.",
        all_krdantas.len()
    );

    let v = create_vyakarana();
    all_krdantas
        .par_chunks(1 + all_krdantas.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for krdanta in chunk {
                let args = Subanta::avyaya(krdanta.clone());

                let prakriyas = v.derive_subantas(&args);
                for prakriya in &prakriyas {
                    let text = prakriya.text();
                    let entry = PadaEntry::Avyaya((&args).try_into().expect("ok"));
                    let packed_semantics = builder.pack(&entry).expect("valid");
                    ret.push((text, packed_semantics))
                }
            }

            ret.into_par_iter()
        })
        .collect()
}

fn read_basic_pratipadikas(
    builder: &mut Builder,
    all_dhatus: &[Dhatu],
    path: &Path,
) -> Result<Pratipadikas> {
    let mut ret = Pratipadikas::new();

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

    // Skip krdantas that we're generating elsewhere.
    let v = create_vyakarana();
    let krdanta_phits: HashSet<_> = create_bare_krdantas(all_dhatus)
        .iter()
        .flat_map(|k| v.derive_krdantas(k))
        .map(|p| p.text())
        .collect();

    let mut num_krdanta_collisions = 0;
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let lingas = parse_stem_linga(&r[1]);

        // Weird '|' characters in input, all seem to be SLP1 `Q`.
        let stem = r[0].to_string().replace('|', "Q");
        // Skip `L` for now.
        if stem.contains('L') {
            continue;
        }
        // Skip strange pratipadikas.
        if stem.ends_with('M') {
            continue;
        }

        if krdanta_phits.contains(&stem) && lingas.len() == 3 {
            num_krdanta_collisions += 1;
            continue;
        }

        let stem = Slp1String::from(stem).expect("ok");
        let pratipadika =
            if lingas == &[Linga::Stri] && (stem.ends_with('A') || stem.ends_with('I')) {
                // senA, devI, ...
                Pratipadika::nyap(stem)
            } else {
                Pratipadika::basic(stem)
            };

        match pratipadika {
            Pratipadika::Basic(ref b) => {
                let entry = PratipadikaEntry::basic(&b, lingas);
                builder.register_pratipadika_entry(&entry);
            }
            _ => panic!("unexpected pratipadika type"),
        }
        ret.push((r[0].to_string(), pratipadika));
    }

    let sankhyas = &[
        "saptan",
        "azwan",
        "navan",
        "daSan",
        "ekAdaSan",
        "dvAdaSan",
        "trayodaSan",
        "caturdaSan",
        "paYcadaSan",
        "zoqaSan",
        "saptadaSan",
        "azwAdaSan",
    ];
    for s in sankhyas {
        let safe = Slp1String::from(s).expect("ok");
        let phit = Pratipadika::basic(safe);
        let entry = (&phit).try_into().expect("ok");

        builder.register_pratipadika_entry(&entry);
        ret.push((s.to_string(), phit));
    }

    info!("- Skipped {num_krdanta_collisions} pratipadikas (likely krdantas).");
    Ok(ret)
}

fn create_basic_subantas(builder: &mut Builder, all_pratipadikas: &Pratipadikas) -> Entries {
    let v = create_vyakarana();
    let lvv = sup_options();

    // For pratipadikas that are tedious to construct, e.g. havis, Danus, ...
    let ending_table = create_subanta_endings();

    let paradigms: Vec<(Pratipadika, Vec<SmallSubanta>)> = all_pratipadikas
        .par_chunks(1 + all_pratipadikas.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for (_, phit) in chunk {
                let b = match phit {
                    Pratipadika::Basic(b) => b,
                    _ => continue,
                };

                // Shortcut logic for tricky pratipadikas (aunadika, taddhitanta, ...)
                let mut used_shortcut = false;
                for (ending_type, endings) in ending_table.iter() {
                    if b.text().ends_with(ending_type) {
                        let offset = b.text().len() - ending_type.len();
                        let prefix = &b.text()[..offset];

                        let mut padas: Vec<SmallSubanta> = Vec::new();
                        for (ending, linga, vibhakti, vacana) in endings {
                            let text = prefix.to_string() + ending;
                            padas.push((text, *linga, *vibhakti, *vacana))
                        }
                        ret.push((phit.clone(), padas));
                        used_shortcut = true;
                        break;
                    }
                }
                if used_shortcut {
                    continue;
                }

                for variant in &v.derive_pratipadikas(phit) {
                    // Restrict all padas to use the same rule choices as this pratipadika variant.
                    // This properly groups padas by their common prefix and lets us generate a
                    // useful paradigm.
                    let v_variant = create_vyakarana()
                        .into_builder()
                        .rule_choices(variant.rule_choices().to_vec())
                        .build();

                    // For all other pratipadikas. Ideally, this should be the branch we use for
                    // all subantas.
                    let mut padas: Vec<SmallSubanta> = Vec::new();
                    for (linga, vibhakti, vacana) in lvv.iter().copied() {
                        let args = Subanta::new(phit.clone(), linga, vibhakti, vacana);
                        let prakriyas = v_variant.derive_subantas(&args);

                        for prakriya in &prakriyas {
                            let text = prakriya.text();
                            padas.push((text, linga, vibhakti, vacana))
                        }
                    }
                    ret.push((phit.clone(), padas));
                }
            }
            ret.into_par_iter()
        })
        .collect();

    let mut entries = Vec::new();
    for (phit, padas) in paradigms {
        // Subantas only.
        for pada in padas {
            let entry = SubantaEntry::new((&phit).try_into().expect("ok"), pada.1, pada.2, pada.3);
            let packed_entry = builder.pack(&entry.into()).expect("ok");
            entries.push((pada.0, packed_entry));
        }

        /*
        // Prefixed subantas.
        let (key, value) = builder
            .register_subanta_paradigm(&entry, &padas)
            .expect("ok");
        entries.push((key, value));
        */
    }

    entries
}

fn read_basic_avyayas(builder: &mut Builder, path: &Path) -> Result<Pratipadikas> {
    let mut ret = Pratipadikas::new();

    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        // Weird '|' characters in input, all seem to be SLP1 `Q`.
        let stem = r[0].to_string().replace('|', "Q");
        // Skip `L` for now.
        if stem.contains('L') {
            continue;
        }

        let stem = Slp1String::from(stem).expect("ok");
        let pratipadika = Pratipadika::basic(stem.clone());
        ret.push((r[0].to_string(), pratipadika));
    }

    for (_, value) in &ret {
        builder.register_pratipadika_entry(&value.try_into().expect("ok"));
    }

    Ok(ret)
}

fn create_sarvanamas(builder: &mut Builder) -> Entries {
    let mut sarva_adi: Vec<&str> = vidyut_prakriya::ganapatha::SARVADI
        .items()
        .iter()
        .filter(|x| !matches!(**x, "qatara" | "qatama" | "Batavu~"))
        .map(|x| *x)
        .collect();

    sarva_adi.extend(&[
        "katara", "katama", "yatara", "yatama", "tatara", "tatama", "ekatara", "ekatama",
    ]);

    // TODO: Bavatu~
    let mut ret = Vec::new();
    let v = create_vyakarana();
    let lvv = sup_options();
    for text in vidyut_prakriya::ganapatha::SARVADI.items() {
        let phit = Pratipadika::basic(Slp1String::from(text).expect("ascii"));
        let b = match phit {
            Pratipadika::Basic(ref b) => b,
            _ => panic!("impossible"),
        };
        let entry = PratipadikaEntry::basic(b, &[]);
        builder.register_pratipadika_entry(&entry);

        for (linga, vibhakti, vacana) in lvv.iter().copied() {
            let args = Subanta::new(phit.clone(), linga, vibhakti, vacana);
            let prakriyas = v.derive_subantas(&args);

            for prakriya in &prakriyas {
                let text = prakriya.text();
                let entry =
                    SubantaEntry::new((&phit).try_into().expect("ok"), linga, vibhakti, vacana);
                let packed_entry = builder.pack(&entry.into()).expect("ok");

                // Not implemented in vidyut-prakriya yet.
                match text.as_str() {
                    "mAm" => ret.push(("mA".to_string(), packed_entry)),
                    "AvAm" | "AvayoH" => ret.push(("nO".to_string(), packed_entry)),
                    "asmAn" | "asmaByam" | "asmAkam" => ret.push(("naH".to_string(), packed_entry)),
                    "mahyam" | "mama" => ret.push(("me".to_string(), packed_entry)),

                    "tvAm" => ret.push(("tvA".to_string(), packed_entry)),
                    "yuvAm" | "yuvayoH" => ret.push(("vAm".to_string(), packed_entry)),
                    "yuzmAn" | "yuzmaByam" | "yuzmAkam" => {
                        ret.push(("vaH".to_string(), packed_entry))
                    }
                    "tuByam" | "tava" => ret.push(("te".to_string(), packed_entry)),
                    _ => (),
                }

                ret.push((text, packed_entry));
            }
        }
    }

    ret
}

/// Adds avyayas scraped from the MW dictionary.
fn create_avyayas(builder: &Builder, avyaya_pratipadikas: &Pratipadikas) -> Entries {
    let _v = create_vyakarana();
    avyaya_pratipadikas
        .par_chunks(1 + avyaya_pratipadikas.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for (_, prati) in chunk {
                let x = match prati {
                    Pratipadika::Basic(b) => b,
                    _ => panic!("basic only"),
                };

                let text = x.text();
                // TODO: add spelling variants later. For now, buggy.
                let args = Subanta::avyaya(prati.clone());
                let entry = PadaEntry::Avyaya((&args).try_into().expect("ok"));

                let packed_entry = builder.pack(&entry).expect("ok");
                ret.push((text.to_string(), packed_entry))
            }
            ret.into_par_iter()
        })
        .collect()
}

fn read_gatis(builder: &mut Builder, path: &Path) -> Result<Pratipadikas> {
    let mut ret = Pratipadikas::new();

    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        // Weird '|' characters in input, all seem to be SLP1 `Q`.
        let stem = r[0].to_string().replace('|', "Q");
        // Skip `L` for now.
        if stem.contains('L') {
            continue;
        }

        // Keep only gati.
        if !matches!(stem.chars().last().expect("present"), 'A' | 'I' | 'U') {
            continue;
        }
        // Avoid prefix chains and upasargas.
        if r[1].contains('-') || &r[0] == "A" {
            continue;
        }

        let stem = Slp1String::from(stem).expect("ok");
        let pratipadika = Pratipadika::basic(stem.clone());
        ret.push((r[0].to_string(), pratipadika));
    }
    for (_, value) in &ret {
        builder.register_pratipadika_entry(&value.try_into().expect("ok"));
    }

    Ok(ret)
}

fn run(args: Args) -> Result<()> {
    let paths = DataPaths::new(&args.input_dir);

    let filters = if args.filters.is_empty() {
        vec![
            "krdantas".to_string(),
            "tinantas".to_string(),
            "basic".to_string(),
            "avyayas".to_string(),
        ]
    } else {
        args.filters
    };

    let mut entries = Vec::new();
    let mut builder = Builder::new(&args.output_dir)?;
    {
        let mut all_dhatus =
            create_all_dhatus(&mut builder, &args.dhatupatha, &paths.upasarga_dhatus)?;
        if let Some(n) = args.num_dhatus {
            all_dhatus.drain(n..);
        }

        info!("Setup");
        info!("---------------------");
        info!("Created {} dhatus.", all_dhatus.len());

        let bare_krdantas = create_bare_krdantas(&all_dhatus);
        info!("Created {} basic krdantas.", bare_krdantas.len());

        if filters.iter().any(|x| x == "krdantas") {
            info!("");
            info!("Krdantas");
            info!("---------------------");
            let all_krt_subantas = create_inflected_krt_subantas(&mut builder, &bare_krdantas);
            info!("Created {} krdantas + subantas.", all_krt_subantas.len());
            entries.extend(all_krt_subantas);

            let all_krt_avyayas = create_avyaya_krt_subantas(&mut builder, &all_dhatus);
            info!("Created {} avyaya krdanta padas.", all_krt_avyayas.len());
            entries.extend(all_krt_avyayas);
        }

        if filters.iter().any(|x| x == "tinantas") {
            info!("");
            info!("Tinantas");
            info!("---------------------");
            let all_tinantas = create_all_tinantas(&builder, &all_dhatus);
            info!("Created {} tinantas.", all_tinantas.len());
            entries.extend(all_tinantas);
        }

        if filters.iter().any(|x| x == "basic") {
            info!("");
            info!("Basic pratipadikas");
            info!("---------------------");
            let basic_pratipadikas =
                read_basic_pratipadikas(&mut builder, &all_dhatus, &paths.pratipadikas)?;
            info!("Loaded {} basic pratipadikas.", basic_pratipadikas.len());

            let basic_subantas = create_basic_subantas(&mut builder, &basic_pratipadikas);
            info!(
                "Created {} basic pratipadikas + subantas.",
                basic_subantas.len()
            );
            entries.extend(basic_subantas);

            let sarvanamas = create_sarvanamas(&mut builder);
            entries.extend(sarvanamas);
        }

        if filters.iter().any(|x| x == "avyayas") {
            info!("");
            info!("Avyayas");
            info!("---------------------");
            let basic_avyayas = read_basic_avyayas(&mut builder, &paths.avyayas)?;
            info!("Loaded {} basic avyayas.", basic_avyayas.len());
            let basic_avyayas = create_avyayas(&builder, &basic_avyayas);
            info!("Created {} basic avyayas.", basic_avyayas.len());
            entries.extend(basic_avyayas);

            let gati = read_gatis(&mut builder, &paths.gati)?;
            info!("Loaded {} gati prefixes.", gati.len());
            let gati = create_avyayas(&builder, &gati);
            info!("Created {} gati prefixes.", gati.len());
            entries.extend(gati);
        }
    }

    info!("");
    info!("Insertion");
    info!("---------------------");
    info!("Sorting keys.");
    entries.par_sort();

    info!("Inserting entries.");
    let mut num_entries = 0;
    for (key, packed_entry) in entries {
        builder.insert_packed(&key, &packed_entry)?;
        num_entries += 1;
    }

    info!("Finishing build.");
    builder.finish()?;

    info!("Complete. (Inserted {num_entries} entries.)");
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
