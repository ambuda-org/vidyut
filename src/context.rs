//! Manages all of the data for the application.

use crate::io;
use crate::lexicon::Lexicon;
use crate::sandhi::Sandhi;
use crate::scoring::Model;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct Context {
    pub sandhi: Sandhi,
    pub lexicon: Lexicon,
    pub model: Model,
}

impl Context {
    pub fn from_paths(paths: &io::DataPaths) -> Result<Context, Box<dyn Error>> {
        Ok(Context {
            sandhi: Sandhi::from_csv(&paths.sandhi_rules)?,
            lexicon: Lexicon {
                padas: io::read_padas(paths)?,
                stems: io::read_stems(paths)?,
                endings: io::read_nominal_endings(paths)?,
            },
            model: Model::from_file(&paths.lemma_counts)?,
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
