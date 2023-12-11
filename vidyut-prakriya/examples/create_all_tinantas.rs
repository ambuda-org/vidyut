//! Creates a very large list of tinantas. This list includes all combinations of:
//!
//! - Around 2000 dhatus from our dhatupatha
//! - 2 prayogas
//! - 5 sanAdi combinations (none, nic, san, yan, yan-luk)
//! - 10 lakaras
//! - 3 purushas
//! - 3 vacanas
//!
//! These combinations produce around 2000 x 2 x 5 x 10 x 3 x 3 = 1.8 million tinantas.
use serde::Serialize;
use std::error::Error;
use std::io;
use vidyut_prakriya::args::{Lakara, Prayoga, Purusha, Sanadi, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};

#[derive(Debug, Serialize)]
struct Row<'a> {
    padas: String,
    dhatu: &'a str,
    gana: &'static str,
    number: u16,
    prayoga: &'static str,
    lakara: &'static str,
    purusha: &'static str,
    vacana: &'static str,
}

fn run(dhatupatha: Dhatupatha) -> Result<(), Box<dyn Error>> {
    let sanadi_choices = vec![
        vec![],
        vec![Sanadi::san],
        vec![Sanadi::Ric],
        vec![Sanadi::yaN],
        vec![Sanadi::yaNluk],
    ];

    let mut wtr = csv::Writer::from_writer(io::stdout());
    let v = Vyakarana::builder().log_steps(false).build();

    for entry in dhatupatha {
        let dhatu = entry.dhatu();
        for sanadis in &sanadi_choices {
            for prayoga in &[Prayoga::Kartari, Prayoga::Karmani] {
                for lakara in Lakara::iter() {
                    for purusha in Purusha::iter() {
                        for vacana in Vacana::iter() {
                            let tinanta = Tinanta::builder()
                                .dhatu(dhatu.clone().with_sanadi(&sanadis))
                                .prayoga(*prayoga)
                                .purusha(*purusha)
                                .vacana(*vacana)
                                .lakara(*lakara)
                                .build()?;

                            let prakriyas = v.derive_tinantas(&tinanta);
                            if prakriyas.is_empty() {
                                continue;
                            }

                            let dhatu_text = &dhatu.upadesha().expect("ok");
                            let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                            padas.sort();
                            let padas = padas.join("|");

                            let row = Row {
                                padas,
                                dhatu: dhatu_text,
                                gana: dhatu.gana().expect("ok").as_str(),
                                number: entry.number(),
                                lakara: lakara.as_str(),
                                purusha: purusha.as_str(),
                                vacana: vacana.as_str(),
                                prayoga: prayoga.as_str(),
                            };

                            wtr.serialize(row)?;
                        }
                    }
                }
            }
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
