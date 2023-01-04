//! Creates an FST lexicon using our raw linguistic data.
//!
//! The slowest part of this process is `add_nominals`, which inflects almost 200,000 nominal
//! stems with all of the endings they allow.
use clap::Parser;
use log::info;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process;

use vidyut_cheda::generator;
use vidyut_cheda::io;
use vidyut_cheda::Config;
use vidyut_kosha::Builder;

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

fn run(args: Args) -> Result<()> {
    info!("Reading linguistic data.");
    let data_paths = io::DataPaths::from_dir(Path::new(&args.input_dir));
    let mut padas = io::read_padas(&data_paths)?;

    info!("Generating nominals.");
    let stems = io::read_stems(&data_paths)?;
    let endings = io::read_nominal_endings(&data_paths)?;
    generator::add_nominals(&stems, &endings, &mut padas);

    info!("Sorting linguistic data for FST insertion.");
    let mut padas: Vec<_> = padas.into_iter().collect();
    padas.sort_by(|x, y| x.0.cmp(&y.0));

    info!("Adding terms to FST builder.");
    let config = Config::new(&args.output_dir);
    let mut builder = Builder::new(config.lexicon())?;
    for (key, pada_vec) in padas {
        for pada in pada_vec {
            // debug!("Inserting {} {:?}", key, pada);
            builder.insert(&key, &pada)?;
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
