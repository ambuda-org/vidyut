/*!
Creates a test file containing the inputs to `Vyakarana`'s derivation functions and all of the
padas produced by those inputs.
*/
use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::io;
use vidyut_prakriya::args::{BaseKrt, Krdanta};
use vidyut_prakriya::{Dhatupatha, Vyakarana};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    krt: BaseKrt,
}

#[derive(Debug, Serialize)]
struct Row<'a> {
    pratipadikas: String,
    dhatu: &'a str,
    gana: &'static str,
    number: u16,
    krt: &'static str,
}

fn run(d: Dhatupatha, args: Args) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let v = Vyakarana::builder().log_steps(false).build();

    for entry in d {
        let dhatu = entry.dhatu();
        let krt = args.krt;
        let krdanta = Krdanta::builder().dhatu(dhatu.clone()).krt(krt).build()?;

        let prakriyas = v.derive_krdantas(&krdanta);

        let dhatu_text = &dhatu.upadesha().expect("ok");
        let mut pratipadikas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        pratipadikas.sort();
        pratipadikas.dedup();
        let pratipadikas = pratipadikas.join("|");

        let row = Row {
            pratipadikas,
            dhatu: dhatu_text,
            gana: dhatu.gana().expect("ok").as_str(),
            number: entry.number(),
            krt: krt.as_str(),
        };
        wtr.serialize(row)?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    let dhatus = match Dhatupatha::from_path("data/dhatupatha.tsv") {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    match run(dhatus, args) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
