use crate::data::{read_matras, read_vrttas, Jati, Vrtta};
use std::error::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Chandas {
    vrttas: Vec<Vrtta>,
    jatis: Vec<Jati>,
}

impl Chandas {
    fn new(vrttas: Vec<Vrtta>, jatis: Vec<Jati>) -> Chandas {
        Self { vrttas, jatis }
    }

    pub fn from_files() -> Result<Self, Box<dyn Error>> {
        let vrttas: Vec<Vrtta> = read_vrttas()?;
        let matras: Vec<Jati> = read_matras()?;

        Ok(Self::new(vrttas, matras))
    }

    pub fn vrttas(&self) -> &Vec<Vrtta> {
        &self.vrttas
    }

    pub fn jatis(&self) -> &Vec<Jati> {
        &self.jatis
    }
}
