//! Creates an FST lexicon using our raw linguistic data.
//!
//! The slowest part of this process is `add_nominals`, which inflects almost 200,000 nominal
//! stems with all of the endings they allow.
use clap::Parser;
use log::info;
use multimap::MultiMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process;

use vidyut::config::Config;
use vidyut::io;
use vidyut::lexicon::Builder;
use vidyut::old_lexicon::PadaMap;
use vidyut::semantics::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the underlying raw data.
    #[arg(short, long)]
    input_dir: PathBuf,

    /// Path to the Vidyut output directory.
    #[arg(short, long)]
    output_dir: PathBuf,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Generates all nominal padas and adds them to the pada map.
fn add_nominals(paths: &io::DataPaths, padas: &mut PadaMap) -> Result<()> {
    let stems = io::read_stems(paths)?;
    let endings = io::read_nominal_endings(paths)?;

    let stem_to_endings = endings
        .iter_all()
        .flat_map(|(ending, vs)| {
            vs.iter()
                .map(|(stem, pada)| (stem.clone(), (ending.clone(), pada.clone())))
        })
        .collect::<MultiMap<String, (String, Pada)>>();

    // For all stems, ...
    for (stem_text, all_stem_semantics) in stems.iter_all() {
        // And all stem endings ...
        for (stem_ending, sup_pratyayas) in stem_to_endings.iter_all() {
            // If the stem ends in this ending ...
            if let Some(prefix) = stem_text.strip_suffix(stem_ending) {
                // Then for all pratyayas that the ending allows, ...
                for (sup_text, sup_semantics) in sup_pratyayas {
                    let pada_text = prefix.to_string() + sup_text;

                    if let Pada::Subanta(sup_semantics) = sup_semantics {
                        for stem_semantics in all_stem_semantics {
                            // Create and insert the corresponding pada.
                            let pada_semantics = Pada::Subanta(Subanta {
                                pratipadika: stem_semantics.clone(),
                                ..sup_semantics.clone()
                            });
                            padas.insert(pada_text.clone(), pada_semantics);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn run(args: Args) -> Result<()> {
    info!("Reading linguistic data.");
    let data_paths = io::DataPaths::from_dir(Path::new(&args.input_dir));
    let mut padas = io::read_padas(&data_paths)?;

    info!("Generating nominals.");
    add_nominals(&data_paths, &mut padas)?;

    info!("Sorting linguistic data for FST insertion.");
    let mut padas: Vec<_> = padas.into_iter().collect();
    padas.sort_by(|x, y| x.0.cmp(&y.0));

    info!("Adding terms to FST builder.");
    let config = Config::new(&args.output_dir);
    let mut builder = Builder::new(config.lexicon())?;
    for (key, pada_vec) in padas {
        // FIXME:
        // - support prefix groups
        for pada in pada_vec {
            if let Pada::PrefixGroup(_) = pada {
            } else {
                // debug!("Inserting {} {:?}", key, pada);
                builder.insert(&key, &pada)?;
            }
        }
    }

    info!("Building FST.");
    let _fst = builder.into_lexicon();

    info!("Complete.");
    Ok(())
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    if let Err(err) = run(args) {
        println!("{}", err);
        process::exit(1);
    }
}
