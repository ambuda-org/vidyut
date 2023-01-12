//! This script generates most of the common sandhi rules that occur between two *pada*s. We aim
//! here for high coverage without getting lost in minor exceptions.
//!
//! A future iteration might clean up this script to remove all of the `panic!`s.
use std::error::Error;
use std::io;
use vidyut_sandhi::{generate_rules, Rule};

fn write_rules(rules: &[Rule]) -> Result<(), Box<dyn Error>> {
    let mut w = csv::Writer::from_writer(io::stdout());
    w.write_record(["first", "second", "result"])?;
    for r in rules {
        w.write_record([r.first(), r.second(), r.result()])?;
    }
    w.flush()?;
    Ok(())
}

fn main() {
    let rules = generate_rules();
    if let Err(err) = write_rules(&rules) {
        println!("{}", err);
        std::process::exit(1);
    }
}
