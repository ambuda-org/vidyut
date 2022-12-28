/*!
Creates a test file containing the inputs to `Ashtadhyayi`'s derivation functions and all of the
padas produced by those inputs.
*/
use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::io;
use std::path::Path;
use vidyut_prakriya::args::{Dhatu, KrdantaArgs, Krt};
use vidyut_prakriya::dhatupatha as D;
use vidyut_prakriya::Ashtadhyayi;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    krt: Krt,
}

#[derive(Debug, Serialize)]
struct Row<'a> {
    pratipadikas: String,
    dhatu: &'a str,
    gana: u8,
    number: u16,
    krt: &'static str,
}

fn run(dhatus: Vec<(Dhatu, u16)>, args: Args) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let a = Ashtadhyayi::builder().log_steps(false).build();

    for (dhatu, number) in dhatus {
        let krt = args.krt;
        let krdanta_args = KrdantaArgs::builder().krt(krt).build()?;

        let prakriyas = a.derive_krdantas(&dhatu, &krdanta_args);

        let dhatu_text = &dhatu.upadesha;
        let mut pratipadikas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        pratipadikas.sort();
        let pratipadikas = pratipadikas.join("|");

        let row = Row {
            pratipadikas,
            dhatu: dhatu_text,
            gana: dhatu.gana,
            number,
            krt: krt.as_str(),
        };
        wtr.serialize(row)?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    let dhatus = match D::load_all(Path::new("data/dhatupatha.tsv")) {
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
