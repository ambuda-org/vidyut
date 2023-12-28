use crate::vrtta::{Jati, Vrtta};
use std::error::Error;
use std::fs;

pub fn read_vrttas() -> Result<Vec<Vrtta>, Box<dyn Error>> {
    let path = "./data/vrtta.tsv";
    let data = fs::read_to_string(path)?;

    let mut ret = Vec::new();
    for line in data.lines() {
        ret.push(line.try_into()?);
    }

    Ok(ret)
}

pub fn read_matras() -> Result<Vec<Jati>, Box<dyn Error>> {
    let path = "./data/matra.tsv";
    let data = fs::read_to_string(path)?;

    let mut ret = Vec::new();
    for line in data.lines() {
        ret.push(line.try_into()?);
    }

    Ok(ret)
}
