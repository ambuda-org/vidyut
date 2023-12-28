use crate::aksharas::Weight;
use crate::data::{read_matras, read_vrttas, Matra, Vrttad};
use crate::vrtta::{Jati, Vrtta};
use std::collections::HashMap;
use std::error::Error;

fn to_weights(txt: String) -> Vec<Weight> {
    txt.chars()
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

#[derive(Debug)]
pub struct Chandas {
    vrttas: HashMap<String, Vrtta>,
    jatis: HashMap<String, Jati>,
    anushtup: Vrtta,
}

fn process_data(vrtta_list: Vec<Vrttad>, matra_list: Vec<Matra>) -> Chandas {
    let mut vrttas: HashMap<String, Vrtta> = HashMap::new();
    let mut jatis: HashMap<String, Jati> = HashMap::new();
    let anushtup = Vrtta::new(vec![String::from("anushtup")], vec![]);

    for metre in vrtta_list {
        let name = String::from(&metre.name);
        let names = vec![name.clone()];
        let vrtta: Vrtta;

        vrtta = Vrtta::new(
            names.clone(),
            metre
                .pattern
                .iter()
                .map(|p| to_weights(p.clone()))
                .collect(),
        );

        vrttas.insert(name.clone(), vrtta);
    }

    for metre in matra_list {
        let name = metre.name;
        let pattern: Vec<Vec<usize>> = metre
            .pattern
            .iter()
            .map(|s| {
                s.split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect()
            })
            .collect();

        jatis.insert(name.clone(), Jati::new(vec![name], pattern));
    }

    Chandas {
        vrttas,
        jatis,
        anushtup,
    }
}

impl Chandas {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let vrttas: Vec<Vrttad> = read_vrttas()?;
        let matras: Vec<Matra> = read_matras()?;

        Ok(process_data(vrttas, matras))
    }

    pub fn getall_vrttas(&self) -> Vec<String> {
        self.vrttas.keys().cloned().collect()
    }

    pub fn getall_jati(&self) -> Vec<String> {
        self.jatis.keys().cloned().collect()
    }
}
