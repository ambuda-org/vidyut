/// Parse text received on the command line.
use clap::{Arg, Command};
use log::info;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

use vidyut::context::Context;
use vidyut::io;
use vidyut::parsing;

fn load_context(
    data_paths: &io::DataPaths,
    cache_file: Option<&String>,
) -> Result<Context, Box<dyn Error>> {
    if let Some(path) = cache_file {
        if Path::new(&path).exists() {
            info!("Loading previous snapshot from \"{}\"", &path);
            return match Context::from_snapshot(path) {
                Ok(data) => Ok(data),
                Err(err) => Err(err),
            };
        } else {
            info!("Loading raw data. (Cache file \"{}\" not found.)", path);
        }
    } else {
        info!("Loading raw data. (Too slow? Try setting `--cache-file`.)");
    }

    let ctx = Context::from_paths(data_paths)?;

    if let Some(path) = cache_file {
        info!("Creating snapshot for faster loading next time.");
        ctx.to_snapshot(path)?;
        info!("Wrote snapshot data to \"{}\"", path);
    }

    Ok(ctx)
}

fn parse_text(text: &str, ctx: &Context) {
    info!("Beginning parse: \"{}\"", text);
    let padas = parsing::parse(text, ctx);
    if padas.is_empty() {
        println!("No solutions found for: {}.", text);
    } else {
        for pada in padas {
            println!("  {:?}", pada);
        }
    }
}

fn parse_document(path: &str, ctx: &Context) {
    info!("Beginning parse of document: \"{}\"", path);
    match fs::read_to_string(path) {
        Ok(text) => {
            for block in text.split("\n\n") {
                parse_text(block, ctx)
            }
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}

fn main() {
    env_logger::init();

    let matches = Command::new("Vidyut")
        .version("0.0.1")
        .author("Arun Prasad <ambuda.library@gmail.com>")
        .about("Vidyut: a lightning fast Sanskrit parser.")
        .arg(Arg::new("text"))
        .arg(Arg::new("input-file").short('i').long("input-file"))
        .arg(Arg::new("cache-file").long("cache-file"))
        .get_matches();

    let text = matches.get_one::<String>("text");
    let input_file = matches.get_one::<String>("input-file");
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

    if let Some(path) = input_file {
        parse_document(path, &ctx);
    } else if let Some(text) = text {
        parse_text(text, &ctx);
    } else {
        println!("You must supply some input.");
        process::exit(1);
    }
}
