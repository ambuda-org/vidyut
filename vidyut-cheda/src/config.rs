//! Config options for Vidyut.
use crate::errors::Result;
use std::path::{Path, PathBuf};

/// Stores config options and file paths for the utilities in Vidyut.
pub struct Config {
    /// Path to a CSV of sandhi rules.
    sandhi: PathBuf,
    /// Base path to kosha data, including `Dhatu` and `Pratipadika` tables.
    kosha: PathBuf,
    /// Path to lemma counts.
    model: PathBuf,
}

impl Config {
    /// Creates a new config object from the given base path.
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        let base_dir = base_dir.as_ref();
        Config {
            sandhi: base_dir.join("sandhi-rules.csv"),
            kosha: base_dir.join("kosha"),
            model: base_dir.join("model"),
        }
    }

    /// Creates all necesary directories, if they don't exist.
    pub fn create_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(self.kosha())?;
        std::fs::create_dir_all(self.model())?;
        Ok(())
    }

    /// Path to sandhi rules.
    pub fn sandhi(&self) -> &Path {
        &self.sandhi
    }

    /// Path to kosha data.
    pub fn kosha(&self) -> &Path {
        &self.kosha
    }

    /// Path to model data
    pub fn model(&self) -> &Path {
        &self.model
    }

    /// Path to model transitions data
    pub fn model_transitions(&self) -> PathBuf {
        self.model.join("transitions.csv")
    }

    /// Path to model emissions data
    pub fn model_emissions(&self) -> PathBuf {
        self.model.join("emissions.csv")
    }

    /// Path to model lemma counts data
    pub fn model_lemma_counts(&self) -> PathBuf {
        self.model.join("lemma-counts.csv")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // Check that the basic path structure is reasonable.
        let base = Path::new("/tmp/path/vidyut-0.1.0");
        let config = Config::new(base);
        assert!(config.sandhi().starts_with(base));
        assert!(config.kosha().starts_with(base));

        let model_path = config.model();
        assert!(config.model_emissions().starts_with(model_path));
        assert!(config.model_transitions().starts_with(model_path));
        assert!(config.model_lemma_counts().starts_with(model_path));
    }
}
