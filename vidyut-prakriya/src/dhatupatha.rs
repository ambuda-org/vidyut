/*!
Utility functions for working with the Dhatupatha file included in this crate.
*/

use crate::args::{Antargana, ArgumentError, Dhatu};
use std::error::Error;
use std::path::Path;

fn maybe_find_antargana(gana: u8, number: u16) -> Option<Antargana> {
    if gana == 6 && (93..=137).contains(&number) {
        // Check number explicitly because some roots are duplicated within tudAdi
        // but outside this gana (e.g. juq).
        Some(Antargana::Kutadi)
    } else if gana == 10 && (192..=236).contains(&number) {
        // Need to check range explicitly because some of these roots appear
        // multiple times in the gana, e.g. lakza~
        Some(Antargana::Akusmiya)
    } else {
        None
    }
}

/// Resolve a specific lookup code against our version of the Dhatupatha.
pub fn resolve(upadesha: &str, gana: &str, number: &str) -> Result<Dhatu, ArgumentError> {
    let gana = match gana.parse() {
        Ok(x) => x,
        Err(_) => {
            return Err(ArgumentError::new(&format!(
                "Could not parse gana: `{}`",
                gana
            )))
        }
    };
    let number = match number.parse() {
        Ok(x) => x,
        Err(_) => {
            return Err(ArgumentError::new(&format!(
                "Could not parse dhatu number: `{}`",
                gana
            )))
        }
    };

    let mut builder = Dhatu::builder().upadesha(upadesha).gana(gana);
    if let Some(x) = maybe_find_antargana(gana, number) {
        builder = builder.antargana(x);
    }
    builder.build()
}

/// Loads a list of dhatus from the given path.
pub fn load_all(path: impl AsRef<Path>) -> Result<Vec<(Dhatu, u16)>, Box<dyn Error>> {
    let mut res = vec![];
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let code = &r[0];
        let upadesha = &r[1];

        if upadesha == "-" {
            continue;
        }
        if let Some((gana, number)) = code.split_once('.') {
            let dhatu = resolve(upadesha, gana, number)?;
            res.push((dhatu, number.parse()?));
        }
    }
    Ok(res)
}
