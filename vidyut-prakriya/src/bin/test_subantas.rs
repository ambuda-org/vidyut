use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use vidyut_prakriya::args::{Pratipadika, SubantaArgs};
use vidyut_prakriya::private::check_file_hash;
use vidyut_prakriya::Ashtadhyayi;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    test_cases: PathBuf,

    #[arg(long)]
    hash: Option<String>,
}

#[derive(Debug, Clone)]
struct ArgumentError {
    message: &'static str,
}

use std::fmt;
impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    if let Some(hash) = args.hash {
        check_file_hash(&args.test_cases, &hash);
    }

    let a = Ashtadhyayi::builder().log_steps(false).build();
    let mut num_matches = 0;
    let mut n = 0;

    let mut rdr = csv::Reader::from_path(args.test_cases)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let padas = &r[0];
        let linga = &r[1].parse()?;
        let vibhakti = &r[2].parse()?;
        let vacana = &r[3].parse()?;

        let pratipadika = Pratipadika::builder()
            .text(&r[4])
            .is_nyap(r[5].parse()?)
            .is_dhatu(r[6].parse()?)
            .build()?;

        let args = SubantaArgs::builder()
            .linga(*linga)
            .vibhakti(*vibhakti)
            .vacana(*vacana)
            .build()?;
        let prakriyas = a.derive_subantas(&pratipadika, &args);

        let mut expected: Vec<_> = padas.split('|').collect();
        expected.sort();
        expected.dedup();

        let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        actual.sort();
        actual.dedup();

        n += 1;
        if expected == actual {
            num_matches += 1;
        } else {
            println!(
                "[ FAIL ]  {:<10} {linga:?} {vibhakti:?} {vacana:?}",
                pratipadika.text()
            );
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
