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
use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::io;
use vidyut_lipi::{Lipika, Scheme};
use vidyut_prakriya::args::{Lakara, Prayoga, Purusha, Sanadi, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};

/// Command line arguments.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// If set, the output scheme to use. Supported options are: slp1, devanagari, iast.
    ///
    /// (Default: slp1)
    #[arg(long)]
    output_scheme: Option<String>,
}

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

fn create_pada_string(mut padas: Vec<String>, output_scheme: Scheme) -> String {
    padas.sort();
    let mut lipika = Lipika::new();
    if output_scheme != Scheme::Slp1 {
        for s in padas.iter_mut() {
            *s = lipika.transliterate(&s, Scheme::Slp1, output_scheme);
        }
    }
    padas.join("|")
}

fn run(dhatupatha: Dhatupatha, args: Args) -> Result<(), Box<dyn Error>> {
    let sanadi_choices = vec![
        vec![],
        vec![Sanadi::san],
        vec![Sanadi::Ric],
        vec![Sanadi::yaN],
        vec![Sanadi::yaNluk],
    ];

    let mut wtr = csv::Writer::from_writer(io::stdout());
    let v = Vyakarana::builder().log_steps(false).build();

    let output_scheme = match args.output_scheme {
        Some(x) => match x.as_str() {
            "devanagari" => Scheme::Devanagari,
            "iast" => Scheme::Iast,
            "slp1" => Scheme::Slp1,
            // We should handle this with an error, but it's easier to default to SLP1.
            _ => Scheme::Slp1,
        },
        None => Scheme::Slp1,
    };

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
                            let padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                            let padas = create_pada_string(padas, output_scheme);

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
