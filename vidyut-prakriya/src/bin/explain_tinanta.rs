//! Debugging tool that displays all prakriyas available for the given pada and code.
//!
//! Usage:
//!
//! ```ignore
//! $ cargo run --bin explain -- --code 01.0001 --pada BavAmi
//! ```
use clap::Parser;
use compact_str::CompactString;
use std::collections::HashMap;
use std::error::Error;
use vidyut_prakriya::args::{Lakara, Prayoga, Purusha, TinantaArgs, Vacana};
use vidyut_prakriya::dhatupatha;
use vidyut_prakriya::Ashtadhyayi;
use vidyut_prakriya::Prakriya;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    code: String,
    #[arg(long)]
    pada: String,
    #[arg(long)]
    prayoga: Option<Prayoga>,
}

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

const PURUSHA_VACANA: &[(Purusha, Vacana)] = &[
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

fn pretty_print_prakriya(p: &Prakriya, lakara: Lakara, purusha: Purusha, vacana: Vacana) {
    println!("{:?} {:?} {:?}", lakara, purusha, vacana);
    println!("------------------------------");
    for step in p.history() {
        println!("{:<10} | {}", step.rule(), step.result());
    }
    println!("------------------------------");
    for choice in p.rule_choices() {
        println!("{choice:?}");
    }
    println!("------------------------------");
}

fn pretty_print_all_padas_for_dhatu(all_words: HashMap<(&Prayoga, &Lakara), Vec<CompactString>>) {
    for prayoga in PRAYOGAS {
        println!("{prayoga:?}:");
        for lakara in LAKARA {
            let key = (prayoga, lakara);
            if let Some(padas) = all_words.get(&key) {
                let data = padas.join(", ");
                println!("- {lakara:?}: {data}");
            }
        }
        println!();
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let dhatus = dhatupatha::load_all("data/dhatupatha.tsv");

    let mut ordered_words = HashMap::new();
    let a = Ashtadhyayi::new();

    let (gana, number) = match args.code.split_once('.') {
        Some((x, y)) => (x.parse::<u8>()?, y.parse::<u16>()?),
        _ => return Ok(()),
    };

    for (dhatu, dhatu_number) in dhatus?.iter() {
        if !(dhatu.gana == gana && *dhatu_number == number) {
            continue;
        }
        for prayoga in PRAYOGAS {
            for lakara in LAKARA {
                let mut words = vec![];
                for (purusha, vacana) in PURUSHA_VACANA {
                    let tinanta_args = TinantaArgs::builder()
                        .prayoga(*prayoga)
                        .purusha(*purusha)
                        .vacana(*vacana)
                        .lakara(*lakara)
                        .build()?;

                    let ps = a.derive_tinantas(dhatu, &tinanta_args);
                    for p in ps {
                        words.push(p.text());
                        if p.text() == args.pada {
                            pretty_print_prakriya(&p, *lakara, *purusha, *vacana);
                        }
                    }
                }
                ordered_words.insert((prayoga, lakara), words);
            }
        }
    }

    pretty_print_all_padas_for_dhatu(ordered_words);
    Ok(())
}

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
