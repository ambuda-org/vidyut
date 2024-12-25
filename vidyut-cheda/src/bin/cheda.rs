/// Parse text strings received on the command line.
use clap::Parser;
use log::info;
use std::path::PathBuf;
use std::process;

use vidyut_cheda::{Chedaka, Config, Result};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value = "")]
    word: String,
    #[arg(long, default_value = "")]
    phrase: String,
    #[arg(long)]
    data_dir: PathBuf,
}

trait Debugger {
    fn debug_word(&self, text: &str) -> Result<()>;
    fn debug_phrase(&self, text: &str) -> Result<()>;
}

impl Debugger for Chedaka {
    /// Prints all interpretations of a word.
    fn debug_word(&self, text: &str) -> Result<()> {
        let lex = self.kosha();
        println!("word={text}:");
        for packed_pada in lex.get_all(text) {
            let pada = lex.unpack(&packed_pada)?;
            println!("- `{}`, {:?}", pada.lemma(), pada);
        }
        Ok(())
    }

    /// Prints all interpretations of a phrase.
    fn debug_phrase(&self, text: &str) -> Result<()> {
        let ret = self.run(text)?;
        println!("phrase={text}:");
        println!("{ret:?}");
        Ok(())
    }
}

fn run(args: Args) -> Result<()> {
    info!("Loading raw data from disk.");
    let config = Config::new(&args.data_dir);
    let c = Chedaka::new(config).unwrap();

    if !args.word.is_empty() {
        c.debug_word(&args.word)?;
    }

    if !args.phrase.is_empty() {
        c.debug_phrase(&args.phrase)?;
    }

    Ok(())
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    match run(args) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
