use std::collections::HashMap;

use crate::vrtta::{Vrtta, Jati};

#[derive(Debug)]
pub struct Chandas {
    vrttas: HashMap<String, Vrtta>,
    jatis: HashMap<String, Jati>,
    anushtup: Vrtta,
}

impl Chandas {
    pub fn init_from_text(vrtta_json: &str, jati_json: &str, anushtup_json: &str) -> Self {
        todo!("Need to write the code to parse the json input (essentially our json files) using serde or something.")
    }

    pub fn init() -> Self {
        todo!("We init by reading our json files.")
    }

    pub fn getall_vrttas(&self) -> Vec<String> {
        self.vrttas.keys().cloned().collect()
    }

    pub fn getall_jati(&self) -> Vec<String> {
        self.jatis.keys().cloned().collect()
    }
}

