//! Parse text strings received on the command line.
use clap::Parser;
use log::info;
use std::path::PathBuf;
use std::process;

use vidyut_cheda::{Chedaka, Result};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// The text to parse.
    #[arg(long)]
    text: String,
    /// Path to config data.
    #[arg(long)]
    data_dir: PathBuf,
}

fn run(args: Args) -> Result<()> {
    info!("Loading chedaka.");
    let chedaka = Chedaka::new(&args.data_dir).unwrap();

    let kosha = chedaka.kosha();
    for entry in kosha.get_all(&args.text) {
        println!("kosha: {:#?}", entry);
    }

    /*
    println!(
        "log prob: {:?}",
        chedaka
            .model()
            .lemma_log_probability(&args.text, POSTag::Avyaya)
    );

    let ret = chedaka.run(&args.text);
    println!("{:#?}", ret);
    */

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
