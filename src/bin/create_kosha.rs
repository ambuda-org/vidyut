//! Creates a kosha of Sanskrit words and writes the results to disk.
use clap::Parser;
use log::info;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process;
use vidyut_kosha::entries::{DhatuEntry, PadaEntry, PratipadikaEntry, SubantaEntry};
use vidyut_kosha::packing::PackedEntry;
use vidyut_kosha::Builder;
use vidyut_prakriya::args::*;
use vidyut_prakriya::{Dhatupatha, Vyakarana};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type UpasargaDhatuMap = HashMap<String, Vec<Vec<String>>>;
type Pratipadikas = Vec<(String, PratipadikaEntry)>;
type Padas = Vec<(String, PackedEntry)>;

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
}

/// Defines all of the input data paths we need to construct the FST.
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

/// Creates a preconfigured Vyakarana instance.
fn create_vyakarana() -> Vyakarana {
    Vyakarana::builder()
        .log_steps(false)
        .nlp_mode(true)
        .is_chandasi(true)
        .build()
}

/// Creates all standard combinations of (upasarga x dhatu x sanadi)
fn create_all_dhatus(
    dhatupatha_path: &Path,
    upasarga_dhatu_path: &Path,
) -> Result<Vec<DhatuEntry>> {
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
    let dhatupatha = Dhatupatha::from_path(&dhatupatha_path)?;
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
    let new = Vec::new();
    for entry in &dhatupatha {
        let upasarga_groups = upasarga_dhatus.get(entry.code()).unwrap_or(&new);

        for sanadi in &sanadis {
            for prefixes in upasarga_groups {
                let dhatu = entry
                    .dhatu()
                    .clone()
                    .with_sanadi(&sanadi)
                    .with_prefixes(prefixes);
                let prakriyas = v.derive_dhatus(&dhatu);
                if let Some(p) = prakriyas.iter().next() {
                    // Add valid dhatus only.
                    ret.push(DhatuEntry::new(dhatu, p.text()));
                }
            }
        }
    }

    Ok(ret)
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
fn create_all_tinantas(builder: &Builder, all_dhatus: &[DhatuEntry]) -> Padas {
    let plpv = tin_options();
    let v = create_vyakarana();

    all_dhatus
        .par_chunks(all_dhatus.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for entry in chunk {
                let dhatu = entry.dhatu().clone();
                for (prayoga, lakara, purusha, vacana) in plpv.iter().copied() {
                    let args = Tinanta::new(dhatu.clone(), prayoga, lakara, purusha, vacana);

                    let prakriyas = v.derive_tinantas(&args);
                    for prakriya in prakriyas {
                        let text = prakriya.text();
                        let semantics = PadaEntry::Tinanta(args.clone());
                        let packed_semantics = builder.pack(&semantics).expect("ok");
                        ret.push((text, packed_semantics))
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
fn create_inflected_krt_subantas(builder: &mut Builder, all_dhatus: &[DhatuEntry]) -> Padas {
    use BaseKrt as K;
    const ALL_KRTS: &[K] = &[
        // Lit
        K::kvasu,
        K::kAnac,
        // nistha
        K::kta,
        K::ktavatu,
        // Lat
        K::Satf,
        K::SAnac,
        // krtya
        K::yat,
        K::Ryat,
        K::kyap,
        K::tavya,
        K::anIyar,
        // Common
        K::Rvul,
        K::lyuw,
        K::tfc,
        // TODO: add all of the others, including unadis.
    ];

    let v = create_vyakarana();
    let mut all_krdantas: Vec<PratipadikaEntry> = Vec::new();
    {
        for entry in all_dhatus {
            let dhatu = entry.dhatu();
            for krt in ALL_KRTS.iter().copied() {
                let prayoga_lakara: &[(Option<Prayoga>, Option<Lakara>)] = match krt {
                    K::Satf | K::SAnac => &[
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

                    // Keep only krdantas that are morphologically valid to avoid filling
                    // `register_pratipadikas` with junk.
                    let prakriyas = v.derive_krdantas(&krdanta);
                    if !prakriyas.is_empty() {
                        all_krdantas.push(PratipadikaEntry::new(krdanta.into(), vec![]));
                    }
                }
            }
        }
        builder.register_pratipadikas(&all_krdantas);
    }
    info!("- Created {} krdanta pratipadikas.", all_krdantas.len());

    let lvv = sup_options();
    all_krdantas
        .par_chunks(all_krdantas.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for entry in chunk {
                let krdanta = entry.pratipadika();
                for (linga, vibhakti, vacana) in lvv.iter().copied() {
                    let args = Subanta::new(krdanta.clone(), linga, vibhakti, vacana);

                    let prakriyas = v.derive_subantas(&args);
                    for prakriya in &prakriyas {
                        let text = prakriya.text();

                        let subanta = Subanta::new(krdanta.clone(), linga, vibhakti, vacana);
                        let pada = PadaEntry::Subanta(subanta.into());
                        let packed_semantics = builder.pack(&pada).expect("valid");
                        ret.push((text, packed_semantics))
                    }
                }
            }

            ret.into_par_iter()
        })
        .collect()
}

fn create_avyaya_krt_subantas(builder: &mut Builder, all_dhatus: &[DhatuEntry]) -> Padas {
    use BaseKrt as K;
    const AVYAYA_KRTS: &[K] = &[K::ktvA, K::tumun];

    let mut all_krdantas: Vec<PratipadikaEntry> = Vec::new();
    {
        for entry in all_dhatus {
            let dhatu = entry.dhatu();
            for krt in AVYAYA_KRTS.iter().copied() {
                let krdanta = Krdanta::new(dhatu.clone(), krt);
                let entry = PratipadikaEntry::new(krdanta.into(), vec![]);
                all_krdantas.push(entry);
            }
        }
        builder.register_pratipadikas(&all_krdantas);
    }
    info!(
        "- Created {} krdanta avyaya pratipadikas.",
        all_krdantas.len()
    );

    let v = create_vyakarana();
    all_krdantas
        .par_chunks(all_krdantas.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for entry in chunk {
                let krdanta = entry.pratipadika();
                let args = Subanta::avyaya(krdanta.clone());

                let prakriyas = v.derive_subantas(&args);
                for prakriya in &prakriyas {
                    let text = prakriya.text();
                    let semantics = PadaEntry::Avyaya(SubantaEntry::new(
                        Subanta::avyaya(krdanta.clone()),
                        PratipadikaEntry::new(krdanta.clone(), vec![]),
                    ));
                    let packed_semantics = builder.pack(&semantics).expect("valid");
                    ret.push((text, packed_semantics))
                }
            }

            ret.into_par_iter()
        })
        .collect()
}

fn read_basic_pratipadikas(builder: &mut Builder, path: &Path) -> Result<Pratipadikas> {
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
        let lingas = parse_stem_linga(&r[1]);
        let pratipadika =
            if lingas == &[Linga::Stri] && (stem.ends_with('A') || stem.ends_with('I')) {
                // senA, devI, ...
                Pratipadika::nyap(stem)
            } else {
                Pratipadika::basic(stem)
            };
        let entry = PratipadikaEntry::new(pratipadika, lingas.to_vec());
        ret.push((r[0].to_string(), entry));
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
        let entry = PratipadikaEntry::new(Pratipadika::basic(safe), Vec::new());
        ret.push((s.to_string(), entry));
    }

    for (_, entry) in &ret {
        builder.register_pratipadikas(&[entry.clone()]);
    }

    Ok(ret)
}

fn create_basic_subantas(builder: &Builder, all_pratipadikas: &Pratipadikas) -> Padas {
    let v = create_vyakarana();
    let lvv = sup_options();

    // For pratipadikas that are tedious to construct, e.g. havis, Danus, ...
    let ending_table = create_subanta_endings();

    all_pratipadikas
        .par_chunks(all_pratipadikas.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for (_, entry) in chunk {
                let prati = entry.pratipadika();
                let b = match prati {
                    Pratipadika::Basic(b) => b,
                    _ => continue,
                };

                // Shortcut logic for tricky pratipadikas (aunadika, taddhitanta, ...)
                let mut used_shortcut = false;
                for (ending_type, endings) in ending_table.iter() {
                    if b.text().ends_with(ending_type) {
                        let offset = b.text().len() - ending_type.len();
                        // For tricky pratipadikas that aren't fully supported yet.
                        let prefix = &b.text()[..offset];

                        for (ending, linga, vibhakti, vacana) in endings {
                            let args = Subanta::new(prati.clone(), *linga, *vibhakti, *vacana);
                            let entry = PadaEntry::Subanta(args.into());
                            let packed_semantics = builder.pack(&entry).expect("valid");

                            let key = prefix.to_string() + ending;
                            ret.push((key, packed_semantics));
                        }
                        used_shortcut = true;
                        break;
                    }
                }
                if used_shortcut {
                    continue;
                }

                // For all other pratipadikas. Ideally, this should be the branch we use for
                // all subantas.
                for (linga, vibhakti, vacana) in lvv.iter().copied() {
                    let args = Subanta::new(prati.clone(), linga, vibhakti, vacana);

                    let prakriyas = v.derive_subantas(&args);
                    for prakriya in &prakriyas {
                        let text = prakriya.text();
                        let entry = PadaEntry::Subanta(args.clone().into());
                        let packed_semantics = builder.pack(&entry).expect("valid");
                        ret.push((text, packed_semantics))
                    }
                }
            }
            ret.into_par_iter()
        })
        .collect()
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
        let entry = PratipadikaEntry::new(pratipadika, vec![]);
        ret.push((r[0].to_string(), entry));
    }

    for (_, value) in &ret {
        builder.register_pratipadikas(&[value.clone()]);
    }

    Ok(ret)
}

/// Adds avyayas scraped from the MW dictionary.
fn create_avyayas(builder: &Builder, avyaya_pratipadikas: &Pratipadikas) -> Padas {
    let v = create_vyakarana();
    avyaya_pratipadikas
        .par_chunks(avyaya_pratipadikas.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for (_, entry) in chunk {
                let prati = entry.pratipadika();
                let args = Subanta::new(prati.clone(), Linga::Pum, Vibhakti::Prathama, Vacana::Eka);

                let prakriyas = v.derive_subantas(&args);
                for prakriya in &prakriyas {
                    let text = prakriya.text();
                    let entry = PadaEntry::Avyaya(args.clone().into());
                    let packed_semantics = builder.pack(&entry).expect("valid");
                    ret.push((text, packed_semantics))
                }
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
        let entry = PratipadikaEntry::new(pratipadika, vec![]);
        ret.push((r[0].to_string(), entry));
    }
    for (_, value) in &ret {
        builder.register_pratipadikas(&[value.clone()]);
    }

    Ok(ret)
}

fn run(args: Args) -> Result<()> {
    let paths = DataPaths::new(&args.input_dir);

    info!("Generating words.");
    let mut padas = Vec::new();
    let mut builder = Builder::new(&args.output_dir)?;
    {
        let all_dhatus = create_all_dhatus(&args.dhatupatha, &paths.upasarga_dhatus)?;
        builder.register_dhatus(&all_dhatus);
        info!("- Created {} dhatus.", all_dhatus.len());

        let all_tinantas = create_all_tinantas(&builder, &all_dhatus);
        info!("- Created {} tinantas.", all_tinantas.len());
        padas.extend(all_tinantas);

        let all_krt_subantas = create_inflected_krt_subantas(&mut builder, &all_dhatus);
        info!(
            "- Created {} inflected krdanta padas.",
            all_krt_subantas.len()
        );
        padas.extend(all_krt_subantas);

        let all_krt_avyayas = create_avyaya_krt_subantas(&mut builder, &all_dhatus);
        info!("- Created {} avyaya krdanta padas.", all_krt_avyayas.len());
        padas.extend(all_krt_avyayas);

        let basic_pratipadikas = read_basic_pratipadikas(&mut builder, &paths.pratipadikas)?;
        info!("- Loaded {} basic pratipadikas.", basic_pratipadikas.len());

        let basic_subantas = create_basic_subantas(&builder, &basic_pratipadikas);
        info!("- Created {} basic subantas.", basic_subantas.len());
        padas.extend(basic_subantas);

        let basic_avyayas = read_basic_avyayas(&mut builder, &paths.avyayas)?;
        info!("- Loaded {} basic avyayas.", basic_avyayas.len());
        let basic_avyayas = create_avyayas(&builder, &basic_avyayas);
        info!("- Created {} basic avyayas.", basic_avyayas.len());
        padas.extend(basic_avyayas);

        let gati = read_gatis(&mut builder, &paths.gati)?;
        info!("- Loaded {} gati prefixes.", gati.len());
        let gati = create_avyayas(&builder, &gati);
        info!("- Created {} gati prefixes.", gati.len());
        padas.extend(gati);
    }

    info!("Sorting keys.");
    padas.par_sort();

    info!("Inserting entries.");
    let mut num_entries = 0;
    for (key, packed_pada) in padas {
        builder.insert_packed(&key, &packed_pada)?;
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
