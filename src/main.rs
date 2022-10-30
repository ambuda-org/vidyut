/// Parse text strings received on the command line.
use clap::{Arg, Command};
use log::info;
use std::fs;
use std::path::Path;
use std::process;

use vidyut::io;
use vidyut::parsing::Parser;

fn parse_text(text: &str, parser: &Parser) {
    info!("Beginning parse: \"{}\"", text);
    let padas = parser.parse(text);
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

fn parse_document(path: &str, parser: &Parser) {
    info!("Beginning parse of document: \"{}\"", path);
    match fs::read_to_string(path) {
        Ok(text) => {
            for block in text.split("\n\n") {
                parse_text(block, parser)
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
        .arg(Arg::new("cache-file").long("cache-file"))
        .get_matches();

    let text = matches.get_one::<String>("text");
    let input_file = matches.get_one::<String>("input-file");

    let data_paths = io::DataPaths::from_dir(Path::new("data"));

    info!("Loading raw data from disk.");
    let parser = Parser::from_paths(&data_paths);
    let parser = match parser {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    if let Some(path) = input_file {
        parse_document(path, &parser);
    } else if let Some(text) = text {
        parse_text(text, &parser);
    } else {
        println!("You must supply some input.");
        process::exit(1);
    }
}
