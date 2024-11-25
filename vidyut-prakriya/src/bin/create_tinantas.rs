/*!
Creates a test file containing the inputs to `Vyakarana`'s derivation functions and all of the
padas produced by those inputs.
*/
use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::io;
use vidyut_prakriya::args::{Dhatu, Lakara, Muladhatu, Prayoga, Purusha, Sanadi, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    prayoga: Option<Prayoga>,
    #[arg(long, value_delimiter = ',')]
    sanadi: Vec<Sanadi>,
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
    gana: &'static str,
    number: u16,
    sanadi: String,
    prayoga: &'static str,
    lakara: &'static str,
    purusha: &'static str,
    vacana: &'static str,
}

fn to_mula(dhatu: &Dhatu) -> &Muladhatu {
    match dhatu {
        Dhatu::Mula(m) => m,
        _ => panic!("unknown dhatu"),
    }
}

fn run(d: Dhatupatha, args: Args) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let v = Vyakarana::builder().log_steps(false).build();

    let sanadi = args.sanadi;
    for entry in d {
        let dhatu = to_mula(entry.dhatu());

        // Add sanadi to the dhatu.
        let mut builder = Dhatu::builder()
            .upadesha(dhatu.upadesha())
            .prefixes(dhatu.prefixes())
            .gana(dhatu.gana())
            .sanadi(&sanadi);
        if let Some(x) = dhatu.antargana() {
            builder = builder.antargana(x);
        }
        let dhatu = builder.build()?;
        let mula = to_mula(&dhatu);

        for prayoga in PRAYOGAS {
            // Filter prayoga based on args
            if let Some(p) = args.prayoga {
                if *prayoga != p {
                    continue;
                }
            }

            for lakara in LAKARA {
                for (purusha, vacana) in TIN_SEMANTICS {
                    let tinanta = Tinanta::builder()
                        .dhatu(dhatu.clone())
                        .prayoga(*prayoga)
                        .purusha(*purusha)
                        .vacana(*vacana)
                        .lakara(*lakara)
                        .build()?;

                    let prakriyas = v.derive_tinantas(&tinanta);
                    let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                    padas.sort();
                    padas.dedup();
                    let padas = padas.join("|");
                    if padas.is_empty() {
                        continue;
                    }

                    let sanadi_str = dhatu
                        .sanadi()
                        .iter()
                        .map(|x| x.as_str())
                        .fold(String::new(), |b, x| b + x + "+");
                    let sanadi_str = sanadi_str.trim_end_matches('+');

                    let dhatu_text = mula.upadesha();
                    let row = Row {
                        padas,
                        dhatu: dhatu_text,
                        gana: mula.gana().as_str(),
                        number: entry.number(),
                        sanadi: sanadi_str.to_string(),
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
