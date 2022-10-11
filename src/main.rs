/// Parse text received on the command line.
use clap::{Arg, Command};
use log::info;
use std::error::Error;
use std::path::Path;
use std::process;

mod io;
mod padas;
mod parsing;
mod sandhi;
mod scoring;
mod semantics;

fn load_context(data_paths: &io::DataPaths, cache_file: Option<&String>) -> Result<io::Context, Box<dyn Error>> {
    if let Some(path) = cache_file {
        if Path::new(&path).exists() {
            info!("Loading previous snapshot from \"{}\"", &path);
            return match io::read_snapshot(&path) {
                Ok(data) => Ok(data),
                Err(err) => Err(err),
            }
        } else {
            info!("Loading raw data. (Cache file \"{}\" not found.)", path);
        }
    } else {
        info!("Loading raw data. (Too slow? Try setting `--cache-file`.)");
    }

    let ctx = io::read_all_data(data_paths)?;

    if let Some(path) = cache_file {
        info!("Creating snapshot for faster loading next time.");
        io::write_snapshot(&ctx, &path)?;
        info!("Wrote snapshot data to \"{}\"", path);
    }

    Ok(ctx)
}

fn main() {
    env_logger::init();

    let matches = Command::new("Vidyut")
        .version("0.0.1")
        .author("Arun Prasad")
        .about("A fast Sanskrit parser")
        .arg(Arg::new("text"))
        .arg(
            Arg::new("cache-file")
                .long("cache-file")
        )
        .get_matches();

    let text = matches.get_one::<String>("text").expect("required");
    let cache_file = matches.get_one::<String>("cache-file");

    let data_paths = io::DataPaths {
        indeclinables: "data/indeclinables.csv".to_string(),
        nominal_endings_compounded: "data/nominal-endings-compounded.csv".to_string(),
        nominal_endings_inflected: "data/nominal-endings-inflected.csv".to_string(),
        nominal_stems: "data/nominal-stems.csv".to_string(),
        participle_stems: "data/participle-stems.csv".to_string(),
        prefix_groups: "data/prefix-groups.csv".to_string(),
        prefixed_roots: "data/prefixed-roots.csv".to_string(),
        pronouns: "data/pronouns.csv".to_string(),
        sandhi_rules: "data/sandhi-rules.csv".to_string(),
        unprefixed_roots: "data/unprefixed-roots.csv".to_string(),
        verb_endings: "data/verb-endings.csv".to_string(),
        verb_prefixes: "data/verb-prefixes.csv".to_string(),
        verbal_indeclinables: "data/verbal-indeclinables.csv".to_string(),
        verbs: "data/verbs.csv".to_string(),
    };

    let ctx = match load_context(&data_paths, cache_file) {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    for phrase in text.split(';') {
        info!("Beginning parse: \"{}\"", text);
        let padas = parsing::parse(phrase, &ctx);
        println!("{:?}", padas);
    }
}
