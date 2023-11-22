use std::collections::HashMap;
use crate::aksharas::Weight;
use crate::vrtta::{Vrtta, Jati};
use crate::data::{read_json_vrtta, read_json_matra, VrttaData, MatraData, StringOrList};


fn to_weights(txt: String) -> Vec<Weight> {
    txt.chars().map(|c|
    
        match c {
            'X' => Weight::X,
            'L' => Weight::L,
            'G' => Weight::G,
            _ => {
                eprintln!("ERROR: Received JSON of wrong format!");
                std::process::exit(1);
            }
        }
    
    ).collect()
}

#[derive(Debug)]
pub struct Chandas {
    vrttas: HashMap<String, Vrtta>,
    jatis: HashMap<String, Jati>,
    anushtup: Vrtta,
}

fn process_data(data_vrttas: VrttaData, data_matras: MatraData) -> Chandas {
    let mut vrttas: HashMap<String, Vrtta> = HashMap::new();
    let mut jatis: HashMap<String, Jati> = HashMap::new();
    let anushtup = Vrtta::new(vec![String::from("anushtup")], vec![]);

    for metre in data_vrttas.metres {
        let name = String::from(&metre.name);
        let names = vec![name.clone()];
        let vrtta: Vrtta;

        match metre.pattern {
            StringOrList::List(pattern) => {
                vrtta = Vrtta::new(names.clone(), pattern.iter().map(
                    |p| to_weights(p.clone())
                ).collect());
            }
            StringOrList::String(pattern) => {
                vrtta = Vrtta::new(names.clone(), vec![to_weights(pattern)]);
            }
        }

        vrttas.insert(name.clone(), vrtta);
    }

    for metre in data_matras.metres {
        let name = metre.name;
        let pattern: Vec<Vec<usize>> = metre.pattern.regex
            .iter()
            .map(
                |s| s.split_whitespace()
                    .filter_map(
                        |n| n.parse().ok()
                    ).collect()
            ).collect();

        jatis.insert(name.clone(), Jati::new(vec![name], pattern));
    }

    Chandas { vrttas, jatis, anushtup }
}

impl Chandas {
    
    pub fn init_from_text(vrtta_json: &str, jati_json: &str, anushtup_json: &str) -> Self {
        let data_vrttas = serde_json::from_str(&vrtta_json).expect("Unable to parse");
        let data_matras = serde_json::from_str(&jati_json).expect("Unable to parse");
        
        return process_data(data_vrttas, data_matras);
    }

    pub fn init() -> Self {

        let data_vrttas: VrttaData = read_json_vrtta();
        let data_matras: MatraData = read_json_matra();

        return process_data(data_vrttas, data_matras);
    }

    pub fn getall_vrttas(&self) -> Vec<String> {
        self.vrttas.keys().cloned().collect()
    }

    pub fn getall_jati(&self) -> Vec<String> {
        self.jatis.keys().cloned().collect()
    }
}

