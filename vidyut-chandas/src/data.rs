use crate::aksharas::Weight;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vrtta {
    pub name: String,
    pub weights: Vec<Vec<Weight>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Jati {
    pub name: String,
    pub pattern: Vec<Vec<usize>>,
}

fn to_weights(text: &str) -> Vec<Weight> {
    text.chars()
        .map(|c| match c {
            'X' => Weight::X,
            'L' => Weight::L,
            'G' => Weight::G,
            _ => {
                eprintln!("ERROR: Received JSON of wrong format!");
                std::process::exit(1);
            }
        })
        .collect()
}

fn to_counts(text: &str) -> Vec<usize> {
    text.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

pub fn read_vrttas() -> Result<Vec<Vrtta>, Box<dyn Error>> {
    let path = "./data/vrtta.tsv";
    let data = fs::read_to_string(path)?;

    let mut ret = Vec::new();
    for line in data.lines() {
        let fields: Vec<_> = line.split("\t").collect();
        debug_assert_eq!(fields.len(), 2);

        let name = fields[0];
        let pattern_str = fields[1];
        ret.push(Vrtta {
            name: name.to_string(),
            weights: pattern_str.split("/").map(to_weights).collect(),
        })
    }

    Ok(ret)
}

pub fn read_matras() -> Result<Vec<Jati>, Box<dyn Error>> {
    let path = "./data/matra.tsv";
    let data = fs::read_to_string(path)?;

    let mut ret = Vec::new();
    for line in data.lines() {
        let fields: Vec<_> = line.split("\t").collect();
        debug_assert_eq!(fields.len(), 2);

        let name = fields[0];
        let pattern_str = fields[1];
        ret.push(Jati {
            name: name.to_string(),
            pattern: pattern_str.split("/").map(to_counts).collect(),
        })
    }

    Ok(ret)
}
