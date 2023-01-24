/// Parse text strings received on the command line.
use clap::Parser;
use log::info;
use std::path::PathBuf;
use std::process;

use vidyut_cheda::{Chedaka, Config, Result};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    text: String,
    #[arg(long)]
    data_dir: PathBuf,
}

/*
fn parse_text(text: &str, segmenter: &Segmenter) {
    info!("Beginning parse: \"{}\"", text);
    let padas = segmenter.segment(text);
    if padas.is_empty() {
        println!("No solutions found for \"{}\".", text);
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
*/

fn debug_word(text: &str, segmenter: &Chedaka) -> Result<()> {
    let lex = segmenter.kosha();
    println!("{text}:");
    for packed_pada in lex.get_all(text) {
        let pada = lex.unpack(&packed_pada)?;
        println!("- `{}`, {:?}", pada.lemma(), pada);
    }

    Ok(())
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    info!("Loading raw data from disk.");
    let config = Config::new(&args.data_dir);
    let segmenter = Chedaka::new(config);

    let segmenter = match segmenter {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    match debug_word(&args.text, &segmenter) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
