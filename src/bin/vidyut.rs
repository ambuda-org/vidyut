/// Parse text strings received on the command line.
use clap::{Arg, Command};
use log::info;
use std::fs;
use std::path::Path;
use std::process;

use vidyut::config::Config;
use vidyut::segmenting::Segmenter;

fn parse_text(text: &str, segmenter: &Segmenter) {
    info!("Beginning parse: \"{}\"", text);
    let padas = segmenter.segment(text);
    if padas.is_empty() {
        println!("No solutions found for: {}.", text);
    } else {
        for (i, pada) in padas.iter().enumerate() {
            println!(
                "[{}] {} : {}, {:?}",
                i,
                pada.text,
                pada.lemma(),
                pada.semantics
            );
        }
    }
}

fn parse_document(path: &str, segmenter: &Segmenter) {
    info!("Beginning parse of document: \"{}\"", path);
    match fs::read_to_string(path) {
        Ok(text) => {
            for block in text.split("\n\n") {
                parse_text(block, segmenter)
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
        .version("0.1.0")
        .author("Arun Prasad <ambuda.library@gmail.com>")
        .about("Vidyut: a lightning fast Sanskrit toolkit.")
        .arg(Arg::new("text"))
        .arg(Arg::new("input-file").short('i').long("input-file"))
        .get_matches();

    let text = matches.get_one::<String>("text");
    let input_file = matches.get_one::<String>("input-file");

    info!("Loading raw data from disk.");
    let config = Config::new(Path::new("data/vidyut-0.1.0"));
    let segmenter = Segmenter::new(config);

    let segmenter = match segmenter {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    if let Some(path) = input_file {
        parse_document(path, &segmenter);
    } else if let Some(text) = text {
        parse_text(text, &segmenter);
    } else {
        println!("You must supply some input.");
        process::exit(1);
    }
}
