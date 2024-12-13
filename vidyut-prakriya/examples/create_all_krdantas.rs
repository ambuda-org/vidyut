//! Creates a very large list of krdantas. This list includes all combinations of:
//!
//! - Around 2000 dhatus from our dhatupatha
//! - 5 sanAdi combinations (none, nic, san, yan, yan-luk)
//! - Around 120 krt-pratyayas from the `Krt` enum, including variants like sya-Satf and sya-SAnac.
//!
//! These combinations produce around 2000 x 5 x 120 = 1.2 million krdantas.
//!
//! Usage:
//!
//!     cargo run --release --example create_all_krdantas -- --output-scheme Devanagari
use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::io;
use vidyut_lipi::{Lipika, Scheme};
use vidyut_prakriya::args::{BaseKrt, Krdanta, Lakara, Prayoga, Sanadi};
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
    krdantas: String,
    dhatu: &'a str,
    gana: &'static str,
    number: u16,
    sanadi: String,
    prayoga: Option<Prayoga>,
    lakara: Option<Lakara>,
    krt: BaseKrt,
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

    let sat_prayoga_lakara = &[
        (Some(Prayoga::Kartari), Some(Lakara::Lat)),
        (Some(Prayoga::Kartari), Some(Lakara::Lrt)),
        (Some(Prayoga::Karmani), Some(Lakara::Lat)),
        (Some(Prayoga::Karmani), Some(Lakara::Lrt)),
    ];

    for sanadis in &sanadi_choices {
        for entry in &dhatupatha {
            let dhatu = entry.dhatu().clone().with_sanadi(&sanadis);
            for krt in BaseKrt::iter() {
                let prayoga_lakara: &[(Option<Prayoga>, Option<Lakara>)] = match krt {
                    BaseKrt::Satf | BaseKrt::SAnac => sat_prayoga_lakara,
                    _ => &[(None, None)],
                };

                for (prayoga, lakara) in prayoga_lakara.iter().copied() {
                    let mut builder = Krdanta::builder().dhatu(dhatu.clone()).krt(krt);
                    if let (Some(prayoga), Some(lakara)) = (prayoga, lakara) {
                        builder = builder.prayoga(prayoga).lakara(lakara);
                    }
                    let krdanta = builder.build()?;

                    let prakriyas = v.derive_krdantas(&krdanta);
                    if prakriyas.is_empty() {
                        continue;
                    }

                    let dhatu_text = &dhatu.aupadeshika().expect("ok");
                    let krdantas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                    let krdantas = create_output_string(&mut lipika, krdantas, output_scheme);
                    let sanadi_text: Vec<_> = sanadis.iter().map(|x| x.as_str()).collect();
                    let sanadi_text = sanadi_text.join("-");

                    let row = Row {
                        krdantas,
                        dhatu: dhatu_text,
                        gana: dhatu.gana().expect("ok").as_str(),
                        number: entry.number(),
                        sanadi: sanadi_text,
                        prayoga,
                        lakara,
                        krt,
                    };

                    wtr.serialize(row)?;
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
