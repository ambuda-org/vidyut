//! Evaluates a list of tinantas.
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use vidyut_prakriya::args::{Dhatu, Sanadi, TinantaArgs};
use vidyut_prakriya::dhatupatha;
use vidyut_prakriya::private::check_file_hash;
use vidyut_prakriya::Ashtadhyayi;

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

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    check_file_hash(&args.test_cases, &args.hash);

    let a = Ashtadhyayi::builder().log_steps(false).build();

    let mut rdr = csv::Reader::from_path(&args.test_cases)?;

    let mut num_matches = 0;
    let mut n = 0;

    for maybe_row in rdr.records() {
        let r = maybe_row?;
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
            .upadesha(dhatu.upadesha())
            .gana(dhatu.gana())
            .prefixes(dhatu.prefixes())
            .sanadi(&sanadi);

        if let Some(x) = dhatu.antargana() {
            builder = builder.antargana(x);
        }

        let dhatu = builder.build()?;

        let tinanta_args = TinantaArgs::builder()
            .prayoga(prayoga)
            .purusha(purusha)
            .vacana(vacana)
            .lakara(lakara)
            .build()?;

        let prakriyas = a.derive_tinantas(&dhatu, &tinanta_args);
        let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        actual.sort();
        actual.dedup();

        n += 1;
        if expected == actual {
            num_matches += 1;
        } else {
            let lakara = &r[5];
            let purusha = &r[6];
            let vacana = &r[7];
            let code = format!("{:0>2}.{:0>4}", gana, number);
            let upadesha = dhatu.upadesha();
            println!("[ FAIL ]  {code:<10} {upadesha:<10} {lakara:<10} {purusha:<10} {vacana:<10}");
            println!("          Expected: {:?}", expected);
            println!("          Actual  : {:?}", actual);
        }
    }

    let pct = 100_f32 * (num_matches as f32) / (n as f32);
    println!("{num_matches} / {n} tests pass ({pct:.2}%)");
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
