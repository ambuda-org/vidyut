//! Debugging tool that displays all prakriyas available for the given pada and code.
//!
//! Usage:
//!
//! ```ignore
//! $ cargo run --bin explain -- --code 01.0001 --pada BavAmi
//! ```
use clap::Parser;
use std::error::Error;
use vidyut_prakriya::args::{KrdantaArgs, Krt};
use vidyut_prakriya::dhatupatha;
use vidyut_prakriya::Ashtadhyayi;
use vidyut_prakriya::Prakriya;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    code: String,
    #[arg(long)]
    krt: Krt,
}

fn pretty_print_prakriya(p: &Prakriya, args: &KrdantaArgs) {
    println!("------------------------------");
    println!("{:?}", args.krt(),);
    println!("------------------------------");
    for step in p.history() {
        println!("{:<10} | {}", step.rule(), step.result());
    }
    println!("------------------------------");
    for choice in p.rule_choices() {
        println!("{choice:?}");
    }
    println!("------------------------------");
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let dhatus = dhatupatha::load_all("data/dhatupatha.tsv");
    let (gana, number) = match args.code.split_once('.') {
        Some((x, y)) => (x.parse::<u8>()?, y.parse::<u16>()?),
        _ => return Ok(()),
    };

    for (dhatu, dhatu_number) in dhatus?.iter() {
        if !(dhatu.gana == gana && *dhatu_number == number) {
            continue;
        }

        let mut pratipadikas = vec![];
        let a = Ashtadhyayi::new();

        let krdanta_args = KrdantaArgs::builder().krt(args.krt).build()?;
        let prakriyas = a.derive_krdantas(dhatu, &krdanta_args);
        for p in prakriyas {
            let text = p.text();
            pretty_print_prakriya(&p, &krdanta_args);
            pratipadikas.push(text);
        }

        let data = pratipadikas.join(", ");
        println!("{}", data);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    run(args).ok();
}
