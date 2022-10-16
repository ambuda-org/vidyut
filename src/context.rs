use crate::io;
use crate::padas::{EndingMap, PadaMap, StemMap};
use crate::sandhi::SandhiMap;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct Context {
    pub sandhi_rules: SandhiMap,
    pub pada_map: PadaMap,
    pub stem_map: StemMap,
    pub ending_map: EndingMap,
}

impl Context {
    pub fn from_paths(paths: &io::DataPaths) -> Result<Context, Box<dyn Error>> {
        Ok(Context {
            sandhi_rules: io::read_sandhi_rules(&paths.sandhi_rules)?,
            pada_map: io::read_padas(paths)?,
            stem_map: io::read_stems(paths)?,
            ending_map: io::read_nominal_endings(paths)?,
        })
    }

    /// Read a previous data context to disk.
    ///
    /// Reading this snapshot is about twice as fast as building from scratch.
    pub fn from_snapshot(binary_path: &str) -> Result<Context, Box<bincode::ErrorKind>> {
        // Use BufWriter for better performance.
        // https://stackoverflow.com/questions/43028653
        let mut f = BufReader::new(fs::File::open(binary_path)?);
        bincode::deserialize_from(&mut f)
    }

    /// Dump the current data context to disk.
    pub fn to_snapshot(&self, binary_path: &str) -> Result<(), Box<bincode::ErrorKind>> {
        // Use BufWriter for better performance.
        // https://stackoverflow.com/questions/43028653
        let mut f = BufWriter::new(fs::File::create(binary_path)?);
        bincode::serialize_into(&mut f, &self)
    }
}
