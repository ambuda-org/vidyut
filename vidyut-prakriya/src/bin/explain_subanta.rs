//! Debugging tool that displays all prakriyas available for the given pada and code.
//!
//! Usage:
//!
//! ```ignore
//! $ cargo run --bin explain -- --code 01.0001 --pada BavAmi
//! ```
use clap::Parser;
use std::error::Error;
use vidyut_prakriya::args::{Linga, SubantaArgs, Vacana, Vibhakti};
use vidyut_prakriya::Ashtadhyayi;
use vidyut_prakriya::Prakriya;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    pratipadika: String,
    #[arg(long)]
    linga: Linga,
    #[arg(long)]
    pada: String,
}

fn pretty_print_prakriya(p: &Prakriya, args: &SubantaArgs) {
    println!("------------------------------");
    println!(
        "{:?} {:?} {:?}",
        args.linga(),
        args.vacana(),
        args.vibhakti()
    );
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

const VIBHAKTIS: &[Vibhakti] = &[
    Vibhakti::Prathama,
    Vibhakti::Dvitiya,
    Vibhakti::Trtiya,
    Vibhakti::Caturthi,
    Vibhakti::Panchami,
    Vibhakti::Sasthi,
    Vibhakti::Saptami,
    Vibhakti::Sambodhana,
];

const VACANAS: &[Vacana] = &[Vacana::Eka, Vacana::Dvi, Vacana::Bahu];

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut padas = vec![];
    let a = Ashtadhyayi::new();

    for vibhakti in VIBHAKTIS {
        for vacana in VACANAS {
            let subanta_args = SubantaArgs::builder()
                .linga(args.linga)
                .vibhakti(*vibhakti)
                .vacana(*vacana)
                .build()?;
            let prakriyas = a.derive_subantas(&args.pratipadika, &subanta_args);
            for p in prakriyas {
                let text = p.text();
                if text == args.pada {
                    pretty_print_prakriya(&p, &subanta_args);
                }
                padas.push(text);
            }
        }
    }

    let data = padas.join(", ");
    println!("{}", data);

    Ok(())
}

fn main() {
    let args = Args::parse();
    run(args).ok();
}
