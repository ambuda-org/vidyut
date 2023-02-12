//! Evaluates a list of krdantas.
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use vidyut_prakriya::args::KrdantaArgs;
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

        let krt = r[4].parse()?;

        let krdanta_args = KrdantaArgs::builder().krt(krt).build()?;

        let prakriyas = a.derive_krdantas(&dhatu, &krdanta_args);
        let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        actual.sort();
        actual.dedup();

        n += 1;
        if expected == actual {
            num_matches += 1;
        } else {
            let krt = &r[4];
            let code = format!("{:0>2}.{:0>4}", gana, number);
            let upadesha = dhatu.upadesha();
            println!("[ FAIL ]  {code:<10} {upadesha:<10} {krt:<10}");
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
