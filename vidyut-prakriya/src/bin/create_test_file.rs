/*!
Creates a test file containing the inputs to `Ashtadhyayi`'s derivation functions and all of the
padas produced by those inputs.
*/
use serde::Serialize;
use std::error::Error;
use std::io;
use vidyut_prakriya::args::{Lakara, Prayoga, Purusha, TinantaArgs, Vacana};
use vidyut_prakriya::{Ashtadhyayi, Dhatupatha};

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

fn run(d: Dhatupatha) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let a = Ashtadhyayi::builder().log_steps(false).build();

    for entry in d {
        let dhatu = entry.dhatu();
        for lakara in Lakara::iter() {
            for (purusha, vacana) in TIN_SEMANTICS {
                let prayoga = Prayoga::Kartari;
                let tinanta_args = TinantaArgs::builder()
                    .prayoga(prayoga)
                    .purusha(*purusha)
                    .vacana(*vacana)
                    .lakara(*lakara)
                    .build()?;

                let prakriyas = a.derive_tinantas(dhatu, &tinanta_args);

                let dhatu_text = &dhatu.upadesha();
                let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                padas.sort();
                let padas = padas.join("|");

                let row = Row {
                    padas,
                    dhatu: dhatu_text,
                    gana: dhatu.gana().as_str(),
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
