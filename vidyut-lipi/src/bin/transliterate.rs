use clap::Parser;
use std::error::Error;
use vidyut_lipi::transliterate;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Text to transliterate
    #[arg(long)]
    text: String,
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let result = transliterate(
        &args.text,
        vidyut_lipi::Scheme::Iast,
        vidyut_lipi::Scheme::Slp1,
    );

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
