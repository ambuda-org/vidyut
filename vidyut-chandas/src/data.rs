use std::error::Error;
use std::fs;

#[derive(Clone, Debug)]
pub struct Vrttad {
    pub name: String,
    pub pattern: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Matra {
    pub name: String,
    pub pattern: Vec<String>,
}

pub fn read_vrttas() -> Result<Vec<Vrttad>, Box<dyn Error>> {
    let path = "./data/vrtta.tsv";
    let data = fs::read_to_string(path)?;

    let mut ret = Vec::new();
    for line in data.lines() {
        let fields: Vec<_> = line.split("\t").collect();
        debug_assert_eq!(fields.len(), 2);

        let name = fields[0];
        let pattern_str = fields[1];
        ret.push(Vrttad {
            name: name.to_string(),
            pattern: pattern_str.split("/").map(String::from).collect(),
        })
    }

    Ok(ret)
}

pub fn read_matras() -> Result<Vec<Matra>, Box<dyn Error>> {
    let path = "./data/matra.tsv";
    let data = fs::read_to_string(path)?;

    let mut ret = Vec::new();
    for line in data.lines() {
        let fields: Vec<_> = line.split("\t").collect();
        debug_assert_eq!(fields.len(), 2);

        let name = fields[0];
        let pattern_str = fields[1];
        ret.push(Matra {
            name: name.to_string(),
            pattern: pattern_str.split("/").map(String::from).collect(),
        })
    }

    Ok(ret)
}
