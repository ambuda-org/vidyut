//! Evaluates a list of tinantas.
use clap::Parser;
use rayon::prelude::*;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use vidyut_prakriya::args::{Dhatu, Sanadi, Tinanta};
use vidyut_prakriya::dhatupatha;
use vidyut_prakriya::private::check_file_hash;
use vidyut_prakriya::Vyakarana;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    test_cases: PathBuf,

    #[arg(long)]
    hash: String,
}

fn parse_sanadi(val: &str) -> Vec<Sanadi> {
    let results = val
        .split('+')
        .map(|x| x.parse())
        .collect::<Result<Vec<Sanadi>, _>>();
    match results {
        Ok(x) => x,
        Err(_) => Vec::new(),
    }
}

fn test_line(line: &str) -> Result<(), Box<dyn Error>> {
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
    let sanadi = parse_sanadi(&r[4]);
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

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    check_file_hash(&args.test_cases, &args.hash);

    std::fs::read_to_string(&args.test_cases)?
        .lines()
        // Skip the CSV header.
        .skip(1)
        .par_bridge()
        .for_each(|line| match test_line(line) {
            Ok(_) => (),
            Err(_) => println!("ERROR: Row is malformed: {line}"),
        });

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
