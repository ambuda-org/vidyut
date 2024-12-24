//! Evaluates a list of tinantas.
use clap::Parser;
use rayon::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use vidyut_prakriya::args::{
    BaseKrt, Dhatu, Gana, Krdanta, Lakara, Linga, Prayoga, Purusha, Sanadi, Subanta, Tinanta,
    Vacana, Vibhakti,
};
use vidyut_prakriya::dhatupatha;
use vidyut_prakriya::private::check_file_hash;
use vidyut_prakriya::Vyakarana;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    test_cases: PathBuf,

    #[arg(long)]
    data_type: String,

    #[arg(long)]
    hash: String,
}

#[derive(Debug, Deserialize)]
struct TinantaRow {
    padas: String,
    dhatu: String,
    gana: Gana,
    number: u16,
    sanadi: String,
    prayoga: Prayoga,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
}

#[derive(Debug, Deserialize)]
struct KrdantaRow {
    padas: String,
    dhatu: String,
    gana: Gana,
    number: u16,
    sanadi: String,
    krt: BaseKrt,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

#[derive(Debug, Deserialize)]
struct DhatuRow {
    results: String,
    dhatu: String,
    gana: Gana,
    number: u16,
    sanadi: String,
}

fn parse_sanadi(val: &str) -> Result<Vec<Sanadi>, vidyut_prakriya::Error> {
    if val.is_empty() {
        Ok(Vec::new())
    } else {
        val.split('+')
            .map(|x| x.parse())
            .collect::<Result<Vec<Sanadi>, _>>()
    }
}

fn test_tinanta(r: Result<TinantaRow, csv::Error>) -> Result<(), Box<dyn Error>> {
    let r = r?;

    let expected: Vec<_> = r.padas.split('|').collect();
    let aupadeshika = &r.dhatu;
    let dhatu = dhatupatha::create_dhatu(aupadeshika, r.gana, r.number)?;
    let sanadi = parse_sanadi(&r.sanadi)?;
    let prayoga = r.prayoga;
    let lakara = r.lakara;
    let purusha = r.purusha;
    let vacana = r.vacana;

    // TODO: this is very clumsy!
    let mut builder = Dhatu::builder()
        .aupadeshika(dhatu.aupadeshika().expect("ok"))
        .gana(dhatu.gana().expect("ok"))
        .prefixes(dhatu.prefixes())
        .sanadi(&sanadi);

    if let Some(x) = dhatu.antargana() {
        builder = builder.antargana(x);
    }

    let dhatu = builder.build()?;

    let tinanta_args = Tinanta::builder()
        .dhatu(dhatu.clone())
        .prayoga(prayoga)
        .purusha(purusha)
        .vacana(vacana)
        .lakara(lakara)
        .build()?;

    let v = Vyakarana::builder().log_steps(false).build();
    let prakriyas = v.derive_tinantas(&tinanta_args);
    let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    actual.sort();
    actual.dedup();

    if expected != actual {
        let lakara = r.lakara;
        let purusha = r.purusha;
        let vacana = r.vacana;
        let code = format!("{:0>2}.{:0>4}", r.gana, r.number);
        let aupadeshika = dhatu.aupadeshika().expect("ok");

        let mut out = std::io::stdout().lock();
        writeln!(
            out,
            "[ FAIL ]  {code:<10} {aupadeshika:<10} {lakara:<10} {purusha:<10} {vacana:<10}"
        )?;
        writeln!(out, "          Expected: {:?}", expected)?;
        writeln!(out, "          Actual  : {:?}", actual)?;
    }

    Ok(())
}

fn test_krdanta(r: &KrdantaRow) -> Result<(), Box<dyn Error>> {
    let expected: Vec<_> = r.padas.split('|').filter(|x| !x.is_empty()).collect();

    let aupadeshika = &r.dhatu;
    let gana = r.gana;
    let number = r.number;
    let sanadi = parse_sanadi(&r.sanadi)?;
    let krt: BaseKrt = r.krt;
    let linga: Linga = r.linga;
    let vibhakti: Vibhakti = r.vibhakti;
    let vacana: Vacana = r.vacana;

    let v = Vyakarana::builder().log_steps(false).build();
    let dhatu = dhatupatha::create_dhatu(aupadeshika, gana, number)?.with_sanadi(&sanadi);
    let krdanta = Krdanta::builder().dhatu(dhatu.clone()).krt(krt).build()?;
    let subanta = Subanta::new(krdanta, linga, vibhakti, vacana);
    let prakriyas = v.derive_subantas(&subanta);
    let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    actual.sort();
    actual.dedup();

    if expected != actual {
        let code = format!("{:0>2}.{:0>4}", gana, number);
        let aupadeshika = dhatu.aupadeshika().expect("ok");

        let mut out = std::io::stdout().lock();
        writeln!(out, "[ FAIL ]  {code:<10} {aupadeshika:<10} {krt:<10}")?;
        writeln!(out, "          Expected: {:?}", expected)?;
        writeln!(out, "          Actual  : {:?}", actual)?;
    }

    Ok(())
}

fn test_dhatu(r: Result<DhatuRow, csv::Error>) -> Result<(), Box<dyn Error>> {
    let r = r?;

    let expected: Vec<_> = r.results.split('|').filter(|x| !x.is_empty()).collect();
    let aupadeshika = &r.dhatu;
    let gana: Gana = r.gana;
    let number: u16 = r.number;
    let sanadi = parse_sanadi(&r.sanadi)?;

    let dhatu = dhatupatha::create_dhatu(aupadeshika, gana, number)?.with_sanadi(&sanadi);

    let v = Vyakarana::builder().log_steps(false).build();
    let prakriyas = v.derive_dhatus(&dhatu);
    let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    actual.sort();
    actual.dedup();

    if expected != actual {
        let code = format!("{gana:0>2}.{number:0>4}");
        let aupadeshika = dhatu.aupadeshika().expect("ok");

        let mut out = std::io::stdout().lock();
        writeln!(out, "[ FAIL ]  {code:<10} {aupadeshika:<10} {sanadi:?}")?;
        writeln!(out, "          Expected: {expected:?}")?;
        writeln!(out, "          Actual  : {actual:?}")?;
    }

    Ok(())
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    check_file_hash(&args.test_cases, &args.hash);

    if args.data_type == "krdanta" {
        // Reading krdantas into a vec first leads to a 3x speedup over using par_bridge.
        //
        // Benchmark:
        //
        // time ../target/release/test_results \
        //   --data-type krdanta \
        //   --test-cases test-files/krdantas-basic.csv \
        //   --hash "5965c75ef9897df91907148e8b548abf359d4ac89615c073ba4a2c61af9fff54"
        //
        // Before:   Executed in 43.96 secs
        // After:    Executed in 14.03 secs
        let mut r = csv::Reader::from_path(&args.test_cases)?;
        let krdantas: Vec<KrdantaRow> = r.deserialize().filter_map(Result::ok).collect();
        krdantas.par_iter().for_each(|row| {
            match test_krdanta(row) {
                Ok(()) => (),
                Err(e) => println!("ERROR: Row is malformed: {e}"),
            };
        });
    } else if args.data_type == "tinanta" {
        let mut r = csv::Reader::from_path(&args.test_cases)?;
        r.deserialize().par_bridge().for_each(|row| {
            match test_tinanta(row) {
                Ok(()) => (),
                Err(e) => println!("ERROR: Row is malformed: {e}"),
            };
        });
    } else if args.data_type == "dhatu" {
        let mut r = csv::Reader::from_path(&args.test_cases)?;
        r.deserialize().par_bridge().for_each(|row| {
            match test_dhatu(row) {
                Ok(()) => (),
                Err(e) => println!("ERROR: Row is malformed: {e}"),
            };
        });
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
