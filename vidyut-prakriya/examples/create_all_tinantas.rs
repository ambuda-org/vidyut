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
//! Usage:
//!
//!     cargo run --release --example create_all_tinantas -- --output-scheme Devanagari
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
    /// If set, the output scheme to use.
    ///
    /// Any scheme name accepted by `vidyut-prakriya` is valid. Examples: `Devanagari`, `Iso15919`,
    /// `Slp1`.
    ///
    /// (Default: `Slp1`)
    #[arg(long)]
    output_scheme: Option<String>,
}

#[derive(Debug, Serialize)]
struct Row<'a> {
    padas: String,
    dhatu: &'a str,
    gana: &'static str,
    number: u16,
    sanadi: String,
    prayoga: Prayoga,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
}

fn create_output_string(
    lipika: &mut Lipika,
    mut items: Vec<String>,
    output_scheme: Scheme,
) -> String {
    items.sort();
    if output_scheme != Scheme::Slp1 {
        for s in items.iter_mut() {
            *s = lipika.transliterate(&s, Scheme::Slp1, output_scheme);
        }
    }
    items.join("|")
}

fn run(dhatupatha: Dhatupatha, args: Args) -> Result<(), Box<dyn Error>> {
    let sanadi_choices = vec![
        vec![],
        vec![Sanadi::san],
        vec![Sanadi::Ric],
        vec![Sanadi::yaN],
        vec![Sanadi::yaNluk],
    ];

    let v = Vyakarana::builder().log_steps(false).build();
    let mut lipika = Lipika::new();
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let output_scheme: Scheme = match args.output_scheme {
        Some(s) => s.parse()?,
        None => Scheme::Slp1,
    };

    for sanadis in &sanadi_choices {
        for entry in &dhatupatha {
            let dhatu = entry.dhatu().clone().with_sanadi(&sanadis);
            let sanadi_text: Vec<_> = sanadis.iter().map(|x| x.as_str()).collect();
            let sanadi_text = sanadi_text.join("-");

            for prayoga in [Prayoga::Kartari, Prayoga::Karmani] {
                for lakara in Lakara::iter() {
                    for purusha in Purusha::iter() {
                        for vacana in Vacana::iter() {
                            let tinanta = Tinanta::builder()
                                .dhatu(dhatu.clone())
                                .prayoga(prayoga)
                                .purusha(purusha)
                                .vacana(vacana)
                                .lakara(lakara)
                                .build()?;

                            let prakriyas = v.derive_tinantas(&tinanta);
                            if prakriyas.is_empty() {
                                continue;
                            }

                            let dhatu_text = &dhatu.aupadeshika().expect("ok");
                            let padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                            let padas = create_output_string(&mut lipika, padas, output_scheme);

                            let row = Row {
                                padas,
                                dhatu: dhatu_text,
                                gana: dhatu.gana().expect("ok").as_str(),
                                number: entry.number(),
                                sanadi: sanadi_text.clone(),
                                lakara,
                                purusha,
                                vacana,
                                prayoga,
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
