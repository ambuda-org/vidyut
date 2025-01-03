//! Creates and runs snapshot tests.
//!
//! Snapshot tests run across time and confirm that the snapshot from an earlier version of the
//! code is consistent with a snapshot from a newer version. Snapshot tests don't prove
//! correctness; they only show that output is consistent across time.
use clap::{Args, Parser, Subcommand};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use vidyut_prakriya::args::{
    BaseKrt, Dhatu, Krdanta, Lakara, Linga, Prayoga, Purusha, Sanadi, Subanta, Tinanta, Vacana,
    Vibhakti,
};
use vidyut_prakriya::{Dhatupatha, Vyakarana};

#[derive(Parser)]
#[command(author, version, about)]
struct MainArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Create(CreateArgs),
    Print(PrintArgs),
    Validate(ValidateArgs),
}

/// Creates snapshots and writes them to disk.
#[derive(Debug, Args)]
struct CreateArgs {
    /// Output directory for snapshot data.
    #[arg(long)]
    output_dir: PathBuf,
}

/// Pretty-prints snapshots to the terminal.
#[derive(Debug, Args)]
struct PrintArgs {
    /// "tinantas", "krdantas", or "dhatus"
    #[arg(long)]
    test_type: String,

    /// Path to a test file.
    #[arg(long)]
    test_file: PathBuf,

    /// The aupadeshika to filter on.
    #[arg(long)]
    aupadeshika: Option<String>,
}

/// Validate the latest build against snapshots on disk.
#[derive(Debug, Args)]
struct ValidateArgs {
    /// "tinantas", "krdantas", or "dhatus"
    #[arg(long)]
    test_type: String,

    /// Paths to test files.
    #[arg(short = 'f', long)]
    test_files: Vec<PathBuf>,

    /// Paths to hashes of test files.
    #[arg(short = 'h', long)]
    hashes: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct KrdantaResults {
    krdanta: Krdanta,
    padas: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct TinantaResults {
    dhatu: Dhatu,
    padas: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DhatuResults {
    dhatu: Dhatu,
    expected: Vec<String>,
}

/// Hashes the given input file.
fn calculate_sha256_file_hash(path: &Path) -> std::io::Result<String> {
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

/// Checks the file hash of the given test file.
pub fn check_file_hash(test_path: &Path, expected_hash: &str) -> bool {
    // Check that the test file is as expected.
    let actual_hash = match calculate_sha256_file_hash(test_path.as_ref()) {
        Ok(x) => x,
        Err(err) => {
            println!("We could not create a hash for {}", test_path.display());
            println!("Error was: {}", err);
            return false;
        }
    };
    if actual_hash != expected_hash {
        println!();
        println!("The test file has test cases that differ from the ones we were expecting.");
        println!("We know this because the test file has an unexpected hash value:");
        println!();
        println!("    Path to test file: {}", test_path.display());
        println!("    Expected hash    : {}", expected_hash);
        println!("    Actual hash      : {}", actual_hash);
        println!();
        println!(
            "If you are intentionally trying to change the test file -- for example, because you"
        );
        println!("are changing the implementation of some rule -- then please open `Makefile` and");
        println!(
            "replace the hash value in the `test_all` command with the `Actual hash` value above."
        );
        println!();
        println!(
            "If you have not changed any core code, please file a GitHub issue so that we can help"
        );
        println!("you debug the issue (https://github.com/ambuda-org/vidyut/issues/).");
        println!();
        false
    } else {
        true
    }
}

/// Creates a collection of (prayoga, lakara, purusha, vacana) combinations.
fn prayoga_lakara_purusha_vacana() -> Vec<(Prayoga, Lakara, Purusha, Vacana)> {
    let mut ret = Vec::new();
    for prayoga in Prayoga::iter() {
        for lakara in Lakara::iter() {
            if lakara == Lakara::Let {
                continue;
            }
            for purusha in Purusha::iter() {
                for vacana in Vacana::iter() {
                    ret.push((prayoga, lakara, purusha, vacana))
                }
            }
        }
    }
    ret
}

/// Creates a collection of (linga, vibhakti, vacana) combinations.
fn linga_vibhakti_vacana_options() -> Vec<(Linga, Vibhakti, Vacana)> {
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

/// Writes MessagePack data to disk with compression.
fn write_and_compress<T: Serialize>(
    results: &[T],
    output_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let out = rmp_serde::to_vec(&results)?;
    let file = std::fs::File::create(output_file)?;
    let writer = BufWriter::new(file);

    let mut z = ZlibEncoder::new(writer, Compression::default());
    z.write_all(&out)?;
    z.finish()?;

    Ok(())
}

/// Reads MessagePack data from disk with decompression.
fn read_and_decompress<T: DeserializeOwned>(input_file: &Path) -> Result<Vec<T>, Box<dyn Error>> {
    let raw_encoded = std::fs::read(&input_file)?;
    let mut z = ZlibDecoder::new(raw_encoded.as_slice());
    let mut raw_decoded = Vec::new();
    z.read_to_end(&mut raw_decoded)?;

    let results = rmp_serde::from_read(raw_decoded.as_slice())?;
    Ok(results)
}

fn create_tinanta_results(
    v: &Vyakarana,
    dhatu: &Dhatu,
    plvp: &[(Prayoga, Lakara, Purusha, Vacana)],
) -> TinantaResults {
    let mut pada_groups = Vec::new();
    for (prayoga, lakara, purusha, vacana) in plvp.iter().copied() {
        let tinanta = Tinanta::new(dhatu.clone(), prayoga, lakara, purusha, vacana);
        let prakriyas = v.derive_tinantas(&tinanta);
        let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        padas.sort();
        padas.dedup();

        let padas = padas.join("|");
        pada_groups.push(padas);
    }

    TinantaResults {
        dhatu: dhatu.clone(),
        padas: pada_groups,
    }
}

fn create_tinantas_file(
    dhatupatha: &Dhatupatha,
    sanadi: &[Sanadi],
    output_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let v = Vyakarana::builder().log_steps(false).build();
    let plvp = prayoga_lakara_purusha_vacana();

    let entries: Vec<_> = dhatupatha.iter().collect();
    let all_results: Vec<_> = entries
        .par_chunks(entries.len() / 50)
        .flat_map(|group| {
            let ret: Vec<TinantaResults> = group
                .iter()
                .map(|entry| {
                    let dhatu = entry.dhatu().clone().with_sanadi(sanadi);
                    create_tinanta_results(&v, &dhatu, &plvp)
                })
                .collect();
            ret
        })
        .collect();

    write_and_compress(&all_results, &output_file)
}

fn validate_tinantas(expected: &TinantaResults) -> Result<(), Box<dyn Error>> {
    let v = Vyakarana::builder().log_steps(false).build();
    let plpv = prayoga_lakara_purusha_vacana();
    let actual = create_tinanta_results(&v, &expected.dhatu, &plpv);

    if expected != &actual {
        let mut out = std::io::stdout().lock();
        writeln!(out, "[ FAIL ]  {:?}", expected.dhatu)?;
        for (e, a) in expected.padas.iter().zip(actual.padas) {
            writeln!(out, "        Expected {:?}, Actual {:?}", e, a)?;
        }
    }

    Ok(())
}

fn create_krdanta_results(
    v: &Vyakarana,
    krdanta: &Krdanta,
    lvv: &[(Linga, Vibhakti, Vacana)],
) -> KrdantaResults {
    let mut pada_groups = Vec::new();
    for (linga, vibhakti, vacana) in lvv.iter().copied() {
        let args = Subanta::new(krdanta.clone(), linga, vibhakti, vacana);
        let prakriyas = v.derive_subantas(&args);

        let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        padas.sort();
        padas.dedup();
        let padas = padas.join("|");

        if padas.is_empty() {
            // `padas` will be empty for any subanta -- skip.
            break;
        } else {
            pada_groups.push(padas);
        }
    }

    KrdantaResults {
        krdanta: krdanta.clone(),
        padas: pada_groups,
    }
}

fn create_krdantas_file(
    dhatupatha: &Dhatupatha,
    sanadi: &[Sanadi],
    output_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let v = Vyakarana::builder().log_steps(false).build();
    let lvv = linga_vibhakti_vacana_options();

    let entries: Vec<_> = dhatupatha.iter().collect();
    let all_results: Vec<_> = entries
        .par_chunks(entries.len() / 50)
        .flat_map(|group| {
            let mut ret = Vec::new();
            for entry in group {
                let dhatu = entry.dhatu().clone().with_sanadi(sanadi);
                for krt in BaseKrt::iter() {
                    // sya-Satf, sya-SAnac, yak-SAnac, yak-sya-SAnac
                    let prayoga_lakara: &[(Option<Prayoga>, Option<Lakara>)] = match krt {
                        BaseKrt::Satf | BaseKrt::SAnac => &[
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
                        ret.push(create_krdanta_results(&v, &krdanta, &lvv))
                    }
                }
            }
            ret
        })
        .collect();

    write_and_compress(&all_results, &output_file)
}

fn validate_krdantas(expected: &KrdantaResults) -> Result<(), Box<dyn Error>> {
    let v = Vyakarana::builder().log_steps(false).build();
    let lvv = linga_vibhakti_vacana_options();
    let actual = create_krdanta_results(&v, &expected.krdanta, &lvv);

    if expected != &actual {
        let mut out = std::io::stdout().lock();
        writeln!(out, "[ FAIL ]  {:?}", expected.krdanta)?;
        for (e, a) in expected.padas.iter().zip(actual.padas) {
            writeln!(out, "        Expected {:?}, Actual {:?}", e, a)?;
        }
    }

    Ok(())
}

fn create_dhatus_file(dhatupatha: &Dhatupatha, output_file: &Path) -> Result<(), Box<dyn Error>> {
    let v = Vyakarana::builder().log_steps(false).build();

    let sanadi: Vec<Vec<Sanadi>> = vec![
        vec![],
        vec![Sanadi::san],
        vec![Sanadi::Ric],
        vec![Sanadi::yaN],
        vec![Sanadi::yaNluk],
        vec![Sanadi::san, Sanadi::Ric],
        vec![Sanadi::Ric, Sanadi::san],
    ];

    let mut all_results = Vec::new();

    for s in sanadi {
        for entry in dhatupatha {
            let dhatu = entry.dhatu().clone().with_sanadi(&s);

            let prakriyas = v.derive_dhatus(&dhatu);
            let mut results: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
            results.sort();
            results.dedup();

            all_results.push(DhatuResults {
                dhatu: dhatu.clone(),
                expected: results,
            });
        }
    }

    write_and_compress(&all_results, &output_file)
}

fn validate_dhatus(r: &DhatuResults) -> Result<(), Box<dyn Error>> {
    let expected = &r.expected;

    let v = Vyakarana::builder().log_steps(false).build();
    let prakriyas = v.derive_dhatus(&r.dhatu);
    let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    actual.sort();
    actual.dedup();

    if expected != &actual {
        let mut out = std::io::stdout().lock();
        writeln!(out, "[ FAIL ]  {:?}", r.dhatu)?;
        writeln!(out, "          Expected: {expected:?}")?;
        writeln!(out, "          Actual  : {actual:?}")?;
    }

    Ok(())
}

/// Creates all snapshot data and writes it to disk.
///
/// Our output format is a compressed MessagePack file, which seems to do much better than
/// alternatives like CBOR, JSON, etc.
fn create_snapshots(args: CreateArgs) -> Result<(), Box<dyn Error>> {
    use Sanadi::*;

    let dhatupatha = Dhatupatha::from_path("data/dhatupatha.tsv")?;

    let sanadis = &[
        (vec![], "basic"),
        (vec![Ric], "nic"),
        (vec![san], "san"),
        (vec![yaN], "yan"),
        (vec![yaNluk], "yan-luk"),
        (vec![san, Ric], "san-nic"),
        (vec![Ric, san], "nic-san"),
    ];

    let dir = args.output_dir;

    for (sanadis, s_name) in sanadis {
        let filename = format!("tinantas-{s_name}.msgpack");
        println!("Creating {filename}.");
        create_tinantas_file(&dhatupatha, &sanadis, &dir.join(filename))?;
    }

    for (sanadis, s_name) in sanadis {
        let filename = format!("krdantas-{s_name}.msgpack");
        println!("Creating {filename}.");
        create_krdantas_file(&dhatupatha, &sanadis, &dir.join(filename))?;
    }

    let filename = "dhatus.msgpack";
    println!("Creating {filename}.");
    create_dhatus_file(&dhatupatha, &dir.join(filename))?;

    Ok(())
}

/// Pretty-prints data from a given snapshot file.
fn print_snapshots(args: PrintArgs) -> Result<(), Box<dyn Error>> {
    match args.test_type.as_str() {
        "krdantas" => {
            let cases: Vec<KrdantaResults> = read_and_decompress(&args.test_file)?;
            for c in cases {
                if let Some(aupadeshika) = &args.aupadeshika {
                    if c.krdanta.dhatu().aupadeshika().expect("present") != aupadeshika {
                        continue;
                    }
                }
                println!("{c:#?}");
            }
        }
        "tinantas" => {
            let cases: Vec<TinantaResults> = read_and_decompress(&args.test_file)?;
            for c in cases {
                if let Some(aupadeshika) = &args.aupadeshika {
                    if c.dhatu.aupadeshika().expect("present") != aupadeshika {
                        continue;
                    }
                }
                println!("{c:#?}");
            }
        }
        "dhatus" => {
            let cases: Vec<DhatuResults> = read_and_decompress(&args.test_file)?;
            for c in cases {
                if let Some(aupadeshika) = &args.aupadeshika {
                    if c.dhatu.aupadeshika().expect("present") != aupadeshika {
                        continue;
                    }
                }
                println!("{c:#?}");
            }
        }
        _ => {
            panic!("Unknown test type: {:?}", args.test_type);
        }
    }

    Ok(())
}

/// Runs tests against a snapshot file.
fn validate_snapshots(args: ValidateArgs) -> Result<(), Box<dyn Error>> {
    assert_eq!(args.test_files.len(), args.hashes.len());

    for (input_file, hash) in args.test_files.iter().zip(args.hashes) {
        let ok = check_file_hash(&input_file, &hash);
        if !ok {
            continue;
        }
        println!("Testing: {input_file:?}");

        if args.test_type == "krdantas" {
            let cases: Vec<KrdantaResults> = read_and_decompress(input_file)?;
            cases.par_iter().for_each(|krdanta| {
                validate_krdantas(krdanta).expect("ok");
            });
        } else if args.test_type == "tinantas" {
            let cases: Vec<TinantaResults> = read_and_decompress(input_file)?;
            cases.par_iter().for_each(|tinanta| {
                validate_tinantas(tinanta).expect("ok");
            });
        } else if args.test_type == "dhatus" {
            let cases: Vec<DhatuResults> = read_and_decompress(input_file)?;
            cases.par_iter().for_each(|dhatu| {
                validate_dhatus(dhatu).expect("ok");
            });
        } else {
            panic!("Unknown test type: {:?}", args.test_type);
        }
    }

    Ok(())
}

fn main() {
    let args = MainArgs::parse();
    let ret = match args.command {
        Command::Create(args) => create_snapshots(args),
        Command::Print(args) => print_snapshots(args),
        Command::Validate(args) => validate_snapshots(args),
    };

    match ret {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
