//! Evaluates a list of tinantas.
use clap::Parser;
use rayon::prelude::*;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use vidyut_prakriya::args::{BaseKrt, Krdanta};
use vidyut_prakriya::args::{Dhatu, Gana, Sanadi, Tinanta};
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

fn parse_sanadi(val: &str) -> Result<Vec<Sanadi>, vidyut_prakriya::Error> {
    if val.is_empty() {
        Ok(Vec::new())
    } else {
        val.split('+')
            .map(|x| x.parse())
            .collect::<Result<Vec<Sanadi>, _>>()
    }
}

fn test_tinanta(line: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(line.as_bytes());
    let mut r = csv::StringRecord::new();
    assert!(reader.read_record(&mut r).unwrap());

    let expected: Vec<_> = r[0].split('|').collect();

    let upadesha = &r[1];
    let gana = &r[2];
    let number = &r[3];
    let dhatu = dhatupatha::create_dhatu(upadesha, gana.parse()?, number.parse()?)?;
    let sanadi = parse_sanadi(&r[4])?;
    let prayoga = r[5].parse()?;
    let lakara = r[6].parse()?;
    let purusha = r[7].parse()?;
    let vacana = r[8].parse()?;

    // TODO: this is very clumsy!
    let mut builder = Dhatu::builder()
        .upadesha(dhatu.upadesha().expect("ok"))
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
        let lakara = &r[5];
        let purusha = &r[6];
        let vacana = &r[7];
        let code = format!("{:0>2}.{:0>4}", gana, number);
        let upadesha = dhatu.upadesha().expect("ok");

        let mut out = std::io::stdout().lock();
        writeln!(
            out,
            "[ FAIL ]  {code:<10} {upadesha:<10} {lakara:<10} {purusha:<10} {vacana:<10}"
        )?;
        writeln!(out, "          Expected: {:?}", expected)?;
        writeln!(out, "          Actual  : {:?}", actual)?;
    }

    Ok(())
}

fn test_krdanta(line: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(line.as_bytes());
    let mut r = csv::StringRecord::new();
    assert!(reader.read_record(&mut r).unwrap());

    let expected: Vec<_> = r[0].split('|').collect();

    let upadesha = &r[1];
    let gana = &r[2];
    let number = &r[3];
    let dhatu = dhatupatha::create_dhatu(upadesha, gana.parse()?, number.parse()?)?;

    let krt: BaseKrt = r[4].parse()?;

    let krdanta = Krdanta::builder().dhatu(dhatu.clone()).krt(krt).build()?;

    let v = Vyakarana::builder().log_steps(false).build();
    let prakriyas = v.derive_krdantas(&krdanta);
    let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    actual.sort();
    actual.dedup();

    if expected != actual {
        let krt = &r[4];
        let code = format!("{:0>2}.{:0>4}", gana, number);
        let upadesha = dhatu.upadesha().expect("ok");

        let mut out = std::io::stdout().lock();
        writeln!(out, "[ FAIL ]  {code:<10} {upadesha:<10} {krt:<10}")?;
        writeln!(out, "          Expected: {:?}", expected)?;
        writeln!(out, "          Actual  : {:?}", actual)?;
    }

    Ok(())
}

fn test_dhatu(line: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(line.as_bytes());
    let mut r = csv::StringRecord::new();
    assert!(reader.read_record(&mut r).unwrap());

    let expected: Vec<_> = r[0].split('|').filter(|x| !x.is_empty()).collect();
    let aupadeshika = &r[1];
    let gana: Gana = r[2].parse()?;
    let number: u16 = r[3].parse()?;
    let sanadi = parse_sanadi(&r[4])?;

    let dhatu = dhatupatha::create_dhatu(aupadeshika, gana, number)?.with_sanadi(&sanadi);

    let v = Vyakarana::builder().log_steps(false).build();
    let prakriyas = v.derive_dhatus(&dhatu);
    let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    actual.sort();
    actual.dedup();

    if expected != actual {
        let code = format!("{gana:0>2}.{number:0>4}");
        let upadesha = dhatu.upadesha().expect("ok");

        let mut out = std::io::stdout().lock();
        writeln!(out, "[ FAIL ]  {code:<10} {upadesha:<10} {sanadi:?}")?;
        writeln!(out, "          Expected: {expected:?}")?;
        writeln!(out, "          Actual  : {actual:?}")?;
    }

    Ok(())
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    check_file_hash(&args.test_cases, &args.hash);

    let file = std::fs::read_to_string(&args.test_cases)?;

    if args.data_type == "krdanta" {
        file.lines()
            // Skip CSV header.
            .skip(1)
            .par_bridge()
            .for_each(|line| match test_krdanta(line) {
                Ok(_) => (),
                Err(_) => println!("ERROR: Row is malformed: {line}"),
            });
    } else if args.data_type == "tinanta" {
        file.lines()
            .skip(1)
            .par_bridge()
            .for_each(|line| match test_tinanta(line) {
                Ok(_) => (),
                Err(_) => println!("ERROR: Row is malformed: {line}"),
            });
    } else if args.data_type == "dhatu" {
        file.lines()
            .skip(1)
            .par_bridge()
            .for_each(|line| match test_dhatu(line) {
                Ok(_) => (),
                Err(_) => println!("ERROR: Row is malformed: {line}"),
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
