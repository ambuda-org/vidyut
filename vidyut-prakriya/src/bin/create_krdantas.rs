/*!
Creates a test file containing the inputs to `Vyakarana`'s derivation functions and all of the
padas produced by those inputs.
*/
use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::io;
use vidyut_prakriya::args::{BaseKrt, Gana, Krdanta, Linga, Sanadi, Subanta, Vacana, Vibhakti};
use vidyut_prakriya::{Dhatupatha, Vyakarana};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, value_delimiter = ',')]
    sanadi: Vec<Sanadi>,
}

#[derive(Debug, Serialize)]
struct Row<'a> {
    padas: String,
    dhatu: &'a str,
    gana: Gana,
    number: u16,
    sanadi: &'a str,
    krt: BaseKrt,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

/// Creates a collection of (linga, vibhakti, vacana) combinations.
fn linga_vibhakti_vacana_options() -> Vec<(Linga, Vibhakti, Vacana)> {
    let mut ret = Vec::new();
    for linga in Linga::iter() {
        for vibhakti in Vibhakti::iter() {
            for vacana in Vacana::iter() {
                ret.push((linga, vibhakti, vacana))
            }
        }
    }
    ret
}

fn run(dhatupatha: Dhatupatha, args: Args) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let v = Vyakarana::builder().log_steps(false).build();
    let lvv = linga_vibhakti_vacana_options();

    for entry in dhatupatha {
        let dhatu = entry.dhatu().clone().with_sanadi(&args.sanadi);
        let dhatu_text = &dhatu.aupadeshika().expect("mula");
        let sanadi_str = args
            .sanadi
            .iter()
            .map(|x| x.as_str())
            .fold(String::new(), |b, x| b + x + "+");
        let sanadi_str = sanadi_str.trim_end_matches('+');

        for krt in BaseKrt::iter() {
            let krdanta = Krdanta::builder().dhatu(dhatu.clone()).krt(krt).build()?;

            for (linga, vibhakti, vacana) in lvv.iter().copied() {
                let args = Subanta::new(krdanta.clone(), linga, vibhakti, vacana);
                let prakriyas = v.derive_subantas(&args);

                let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                padas.sort();
                padas.dedup();
                let padas = padas.join("|");

                let row = Row {
                    padas,
                    dhatu: dhatu_text,
                    gana: dhatu.gana().expect("ok"),
                    number: entry.number(),
                    sanadi: &sanadi_str,
                    krt,
                    linga,
                    vibhakti,
                    vacana,
                };
                wtr.serialize(row)?;
            }
        }
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
