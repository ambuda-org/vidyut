//! Generates most of the common sandhi rules that occur between two *pada*s.
use clap::Parser;
use std::path::{Path, PathBuf};
use vidyut_cheda::Config;
use vidyut_cheda::Result;
use vidyut_sandhi::{generate_rules, Rule};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the output data directory.
    #[arg(short, long)]
    data_dir: PathBuf,
}

fn write_rules(rules: &[Rule], path: &Path) -> Result<()> {
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
    let config = Config::new(&args.data_dir);

    if let Err(err) = write_rules(&rules, config.sandhi()) {
        println!("{}", err);
        std::process::exit(1);
    }
}
