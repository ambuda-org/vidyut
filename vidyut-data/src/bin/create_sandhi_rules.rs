//! Generates most of the common sandhi rules that occur between two *pada*s.

use clap::Parser;
use std::error::Error;
use std::path::{Path, PathBuf};
use vidyut_sandhi::{generate_rules, Rule};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Where to write our rules.
    #[arg(short, long)]
    output_path: PathBuf,
}

fn write_rules(rules: &[Rule], path: &Path) -> Result<(), Box<dyn Error>> {
    let mut w = csv::Writer::from_path(path)?;
    w.write_record(["first", "second", "result"])?;
    for r in rules {
        w.write_record([r.first(), r.second(), r.result()])?;
    }
    w.flush()?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let rules = generate_rules();

    if let Err(err) = write_rules(&rules, &args.output_path) {
        println!("{}", err);
        std::process::exit(1);
    }
}
