use clap::Parser;
use std::error::Error;
use vidyut_lipi::{transliterate, Mapping, Scheme};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Text to transliterate
    #[arg(long)]
    text: String,
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mapping = Mapping::new(Scheme::Iast, Scheme::Slp1);
    let result = transliterate(&args.text, &mapping);

    println!("{}", result);

    Ok(())
}

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
