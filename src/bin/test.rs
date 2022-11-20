//! Test the production setup.

use clap::Parser;
use std::error::Error;
use vidyut::config::Config;
use vidyut::lexicon::Lexicon;
use vidyut::segmenting::Segmenter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // The base directory from which we read our model.
    #[arg(long)]
    data_dir: PathBuf,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn test_lexicon_tinantas(lex: &Lexicon) -> Result<()> {
    let keys = vec![
        // Basic lakaras (kartari, karmani/bhAve)
        "nayati",
        "ninAya",
        "netA",
        "nezyati",
        "nayatu",
        "anayat",
        // "nIyAt",
        "nayet",
        "anEzIt",
        // "anezyat",

        "nIyate",
        "nIyatAm",
        "anIyata",
        "nIyeta",

        // san dhAtus (kartari, karmani/bhAve)
        "ninIzati",
        "ninIzatu",
        "aninIzat",
        "ninIzet",

        "ninIzyate",
        "ninIzyatAm",
        "aninIzyata",
        "ninIzyeta",

        // Nic dhAtus (kartari, karmani/bhAve)
        "nAyayati",
        "nAyayatu",
        "anAyayat",
        "nAyayet",

        "nAyyate",
        "nAyyatAm",
        "anAyyata",
        "nAyyeta",

        // TODO: yaG
    ];

    for key in keys {
        assert!(lex.contains_key(key), "{key}");
    }
    Ok(())
}

fn test_lexicon_subantas(lex: &Lexicon) -> Result<()> {
    let keys = vec![
        "devas",
        "senA",
        "agnis",
        "devI",
        "gurus",
        "vaDUs",
        "kartA",
        "rEs",
        // "dyOs",
        "nOs",

        "AtmA",
        "gacCan",
        "manas",
        "havis",
        "Danus",
        "hanumAn",
        "BagavAn",
        "cakfvAn",

        "mahAn",
        "mahAntam",
    ];

    for key in keys {
        assert!(lex.contains_key(key), "{key}");
    }
    Ok(())
}

fn run_tests(args: Args) -> Result<()> {
    let config = Config::new(&args.data_dir);
    let segmenter = Segmenter::new(config)?;
    let lex = segmenter.lexicon();

    test_lexicon_tinantas(lex)?;
    test_lexicon_subantas(lex)?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run_tests(args) {
        println!("{}", e);
        std::process::exit(1);
    }
}
