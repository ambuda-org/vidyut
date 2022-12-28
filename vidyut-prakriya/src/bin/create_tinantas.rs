/*!
Creates a test file containing the inputs to `Ashtadhyayi`'s derivation functions and all of the
padas produced by those inputs.
*/
use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::io;
use std::path::Path;
use vidyut_prakriya::args::{Dhatu, Lakara, Prayoga, Purusha, TinantaArgs, Vacana};
use vidyut_prakriya::dhatupatha as D;
use vidyut_prakriya::Ashtadhyayi;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    prayoga: Option<Prayoga>,
}

// TODO: reuse with other binaries?
const LAKARA: &[Lakara] = &[
    Lakara::Lat,
    Lakara::Lit,
    Lakara::Lut,
    Lakara::Lrt,
    Lakara::Lot,
    Lakara::Lan,
    Lakara::AshirLin,
    Lakara::VidhiLin,
    Lakara::Lun,
    Lakara::Lrn,
];

const TIN_SEMANTICS: &[(Purusha, Vacana)] = &[
    (Purusha::Prathama, Vacana::Eka),
    (Purusha::Prathama, Vacana::Dvi),
    (Purusha::Prathama, Vacana::Bahu),
    (Purusha::Madhyama, Vacana::Eka),
    (Purusha::Madhyama, Vacana::Dvi),
    (Purusha::Madhyama, Vacana::Bahu),
    (Purusha::Uttama, Vacana::Eka),
    (Purusha::Uttama, Vacana::Dvi),
    (Purusha::Uttama, Vacana::Bahu),
];

const PRAYOGAS: &[Prayoga] = &[Prayoga::Kartari, Prayoga::Karmani];

#[derive(Debug, Serialize)]
struct Row<'a> {
    padas: String,
    dhatu: &'a str,
    gana: u8,
    number: u16,
    prayoga: &'static str,
    lakara: &'static str,
    purusha: &'static str,
    vacana: &'static str,
}

fn run(dhatus: Vec<(Dhatu, u16)>, args: Args) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let a = Ashtadhyayi::builder().log_steps(false).build();

    for (dhatu, number) in dhatus {
        for prayoga in PRAYOGAS {
            // Filter prayoga based on args
            if let Some(p) = args.prayoga {
                if *prayoga != p {
                    continue;
                }
            }

            for lakara in LAKARA {
                for (purusha, vacana) in TIN_SEMANTICS {
                    let tinanta_args = TinantaArgs::builder()
                        .prayoga(*prayoga)
                        .purusha(*purusha)
                        .vacana(*vacana)
                        .lakara(*lakara)
                        .build()?;

                    let prakriyas = a.derive_tinantas(&dhatu, &tinanta_args);

                    let dhatu_text = &dhatu.upadesha;
                    let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                    padas.sort();
                    let padas = padas.join("|");

                    let row = Row {
                        padas,
                        dhatu: dhatu_text,
                        gana: dhatu.gana,
                        number,
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
