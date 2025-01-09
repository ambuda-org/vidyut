//! Test the production setup.

use clap::Parser;
use log::info;
use rayon::prelude::*;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use vidyut_kosha::entries::{KrdantaEntry, PadaEntry, PratipadikaEntry};
use vidyut_kosha::Kosha;
use vidyut_prakriya::args::{
    Krdanta, Lakara, Linga, Pratipadika, Prayoga, Purusha, Subanta, Tinanta, Vacana, Vibhakti,
};
use vidyut_prakriya::Vyakarana;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // The base directory from which we read our model.
    #[arg(long)]
    data_dir: PathBuf,
}

/// Returns all combinations of (linga, vibhakti, vacana)
fn sup_options() -> Vec<(Linga, Vibhakti, Vacana)> {
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

/// Returns all combinations of (prayoga, lakara, purusha, vacana)
fn tin_options() -> Vec<(Prayoga, Lakara, Purusha, Vacana)> {
    let mut ret = Vec::new();
    for prayoga in Prayoga::iter() {
        if prayoga == Prayoga::Bhave {
            // Duplicates karmani -- skip
            continue;
        }
        for lakara in Lakara::iter() {
            if lakara == Lakara::Let {
                // Noisy -- skip
                continue;
            }
            for purusha in Purusha::iter() {
                for vacana in Vacana::iter() {
                    ret.push((prayoga, lakara, purusha, vacana));
                }
            }
        }
    }
    ret
}

/// Creates a preconfigured Vyakarana instance.
fn create_vyakarana() -> Vyakarana {
    Vyakarana::builder()
        .log_steps(false)
        .nlp_mode(true)
        .is_chandasi(true)
        .build()
}

fn test_tinantas(kosha: &Kosha) {
    info!("Testing tinantas.");

    let dhatus: Vec<_> = kosha.dhatus().collect();
    assert!(dhatus.len() > 10_000);

    let v = create_vyakarana();
    let plpv = tin_options();

    let statuses: Vec<bool> = dhatus
        .par_chunks(1 + dhatus.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();
            for entry in chunk {
                let dhatu = entry.dhatu().clone();

                let mut missing = Vec::new();
                for (prayoga, lakara, purusha, vacana) in plpv.iter().copied() {
                    let args = Tinanta::new(dhatu.clone(), prayoga, lakara, purusha, vacana);

                    let prakriyas = v.derive_tinantas(&args);
                    for prakriya in prakriyas {
                        let text = prakriya.text();
                        let actuals = kosha.get_all(&text);
                        let found = actuals.iter().any(|entry| {
                            if let PadaEntry::Tinanta(t) = entry {
                                Tinanta::from(t) == args
                            } else {
                                false
                            }
                        });
                        if !found {
                            missing.push(text);
                        }
                    }
                }

                let all_passed = missing.is_empty();
                if !all_passed {
                    let mut out = std::io::stdout().lock();
                    writeln!(out, "[ FAIL ]  {:?}", dhatu).expect("ok");
                    writeln!(out, "    missing: {:?}", missing).expect("ok");
                }

                ret.push(all_passed);
            }
            ret.into_par_iter()
        })
        .collect();

    let num_passed = statuses.iter().filter(|x| **x).count();
    println!("- {} / {} dhatus passed", num_passed, statuses.len());
}

fn test_krdantas(kosha: &Kosha) {
    let krdantas: Vec<KrdantaEntry> = kosha
        .pratipadikas()
        .filter_map(|p| match p {
            PratipadikaEntry::Krdanta(k) => Some(k),
            _ => None,
        })
        .collect();
    assert!(krdantas.len() > 1_000_000);

    let v = create_vyakarana();
    let lvv = sup_options();

    println!("Testing {} krdantas.", krdantas.len());
    let statuses: Vec<_> = krdantas
        .par_chunks(1 + krdantas.len() / 100)
        .flat_map(|chunk| {
            let mut ret = Vec::new();

            for krdanta in chunk {
                let mut missing = Vec::new();
                let phit: Pratipadika = Krdanta::from(krdanta).into();
                for (linga, vibhakti, vacana) in lvv.iter().copied() {
                    let args = Subanta::new(phit.clone(), linga, vibhakti, vacana);

                    let prakriyas = v.derive_subantas(&args);
                    for prakriya in &prakriyas {
                        let text = prakriya.text();
                        if text != "BUtas" {
                            continue;
                        }

                        let actuals = kosha.get_all(&text);

                        let found = actuals.iter().any(|entry| {
                            if let PadaEntry::Subanta(s) = entry {
                                Subanta::from(s) == args
                            } else {
                                false
                            }
                        });
                        if !found {
                            missing.push(text);
                        }
                    }
                }

                let all_passed = missing.is_empty();
                if !all_passed {
                    let mut out = std::io::stdout().lock();
                    writeln!(out, "[ FAIL ]  {:?}", krdanta).expect("ok");
                    writeln!(out, "    missing: {:?}", missing).expect("ok");
                }

                ret.push(all_passed);
            }

            ret.into_par_iter()
        })
        .collect();

    let num_passed = statuses.iter().filter(|x| **x).count();
    println!("- {} / {} krdantas passed.", num_passed, statuses.len());
}

fn test_subantas(k: &Kosha) {
    println!("Testing basic subantas.");

    let keys = vec![
        ("devas", "deva"),
        ("senA", "senA"),
        ("agnis", "agni"),
        ("devI", "devI"),
        ("gurus", "guru"),
        ("vaDUs", "vaDU"),
        ("kartA", "kartf"),
        ("rAs", "rE"),
        ("dyOs", "div"),
        ("nOs", "nO"),
        ("AtmA", "Atman"),
        ("manasA", "manas"),
        ("havizA", "havis"),
        ("DanurByAm", "Danus"),
        ("hanumAn", "hanumat"),
        ("Bagavantam", "Bagavat"),
        // Consonant stems
        ("vAk", "vAc"),
        ("vit", "vid"),
        ("kakup", "kakuB"),
        // Irregular subantas
        ("mahAn", "mahat"),
        ("trayas", "tri"),
        ("zaRRAm", "zaz"),
        ("sapta", "saptan"),
        ("azwa", "azwan"),
        ("daSa", "daSan"),
        ("pitaras", "pitf"),
        ("mAtaras", "mAtf"),
        ("BrAtaras", "BrAtf"),
        ("panTAnam", "paTin"),
        ("patyus", "pati"),
        ("yUnAm", "yuvan"),
    ];

    let mut i = 0;
    for (key, lemma) in &keys {
        let entries = k.get_all(key);
        let found = !entries.is_empty();
        let has_lemma = entries.iter().any(|x| &x.lemma().unwrap() == lemma);

        if found && has_lemma {
            i += 1;
        } else {
            println!("FAILED: {key} (found = {found}, has_lemma={has_lemma})");
        }
    }
    let n = keys.len();
    println!("- {i} / {n} subantas passed.");
}

fn run_tests(args: Args) -> Result<(), Box<dyn Error>> {
    let kosha = Kosha::new(args.data_dir)?;
    test_tinantas(&kosha);
    test_krdantas(&kosha);
    test_subantas(&kosha);
    Ok(())
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    if let Err(e) = run_tests(args) {
        println!("Test runner failed with: `{}`", e);
        std::process::exit(1);
    }
}
