//! A simple debugger that creates a form from the input arguments.

use clap::Parser;
use std::error::Error;
use vidyut_prakriya::args::*;
use vidyut_prakriya::{Dhatupatha, Prakriya, Vyakarana};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    kind: String,

    #[arg(long)]
    prayoga: Option<Prayoga>,
    #[arg(long)]
    lakara: Option<Lakara>,
    #[arg(long)]
    purusha: Option<Purusha>,
    #[arg(long)]
    vacana: Option<Vacana>,

    #[arg(long)]
    pratipadika: Option<String>,
    #[arg(long)]
    vibhakti: Option<Vibhakti>,
    #[arg(long)]
    linga: Option<Linga>,
}

/// Prints the `prakriyas` provided.
fn print_prakriyas(prakriyas: &[Prakriya]) {
    for p in prakriyas {
        println!("{}", p.text());
        println!("---------------------------");
        for step in p.history() {
            let code = step.rule().code();
            let terms: Vec<_> = step
                .result()
                .iter()
                .map(|x| x.text())
                .filter(|x| !x.is_empty())
                .collect();
            let result = terms.join(" + ");
            println!("{:<10} | {}", code, result);
        }
        println!("---------------------------");
        println!("\n");
    }
}

fn run(_dhatupatha: Dhatupatha, args: Args) -> Result<(), Box<dyn Error>> {
    let v = Vyakarana::new();

    if args.kind == "subanta" {
        (|| {
            let sup = Subanta::new(
                Pratipadika::basic(Slp1String::from(args.pratipadika?).expect("ok")),
                args.linga?,
                args.vibhakti?,
                args.vacana?,
            );
            let prakriyas = v.derive_subantas(&sup);
            print_prakriyas(&prakriyas);

            Some(())
        })();
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let dhatupatha = match Dhatupatha::from_path("data/dhatupatha.tsv") {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    match run(dhatupatha, args) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
