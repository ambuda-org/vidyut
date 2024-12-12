/*!
Creates a test file containing the inputs to `Vyakarana`'s derivation functions and all of the
padas produced by those inputs.
*/
use serde::Serialize;
use std::error::Error;
use std::io;
use vidyut_prakriya::args::Sanadi;
use vidyut_prakriya::{Dhatupatha, Vyakarana};

#[derive(Debug, Serialize)]
struct Row<'a> {
    results: String,
    dhatu: &'a str,
    gana: &'static str,
    number: u16,
    sanadi: String,
}

fn run(dhatupatha: Dhatupatha) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let v = Vyakarana::builder().log_steps(false).build();

    let sanadi: Vec<Vec<Sanadi>> = vec![
        vec![],
        vec![Sanadi::san],
        vec![Sanadi::Ric],
        vec![Sanadi::yaN],
        vec![Sanadi::yaNluk],
        vec![Sanadi::san, Sanadi::Ric],
        vec![Sanadi::Ric, Sanadi::san],
    ];

    for s in sanadi {
        for entry in &dhatupatha {
            let dhatu = entry.dhatu().clone().with_sanadi(&s);

            let prakriyas = v.derive_dhatus(&dhatu);
            let dhatu_text = &dhatu.aupadeshika().expect("ok");
            let mut results: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
            results.sort();
            results.dedup();
            let results = results.join("|");

            let sanadi_str = dhatu
                .sanadi()
                .iter()
                .map(|x| x.as_str())
                .fold(String::new(), |b, x| b + x + "+");
            let sanadi_str = sanadi_str.trim_end_matches('+');

            let row = Row {
                results,
                dhatu: dhatu_text,
                gana: dhatu.gana().expect("ok").as_str(),
                number: entry.number(),
                sanadi: sanadi_str.to_string(),
            };
            wtr.serialize(row)?;
        }
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    let dhatus = match Dhatupatha::from_path("data/dhatupatha.tsv") {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    match run(dhatus) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
