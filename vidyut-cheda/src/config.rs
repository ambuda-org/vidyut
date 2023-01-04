//! Config options for Vidyut.
use std::error::Error;
use std::path::{Path, PathBuf};

/// Stores config options and file paths for the utilities in Vidyut.
pub struct Config {
    /// Path to a CSV of sandhi rules.
    sandhi: PathBuf,
    /// Base path to lexicon data, including `Dhatu` and `Pratipadika` tables.
    lexicon: PathBuf,
    /// Path to lemma counts.
    model: PathBuf,
}

impl Config {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        let base_dir = base_dir.as_ref();
        Config {
            sandhi: base_dir.join("sandhi-rules.csv"),
            lexicon: base_dir.join("lexicon"),
            model: base_dir.join("model"),
        }
    }

    pub fn create_dirs(&self) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(self.lexicon())?;
        std::fs::create_dir_all(self.model())?;
        Ok(())
    }

    pub fn sandhi(&self) -> &Path {
        &self.sandhi
    }

    pub fn lexicon(&self) -> &Path {
        &self.lexicon
    }

    pub fn model(&self) -> &Path {
        &self.model
    }

    pub fn model_transitions(&self) -> PathBuf {
        self.model.join("transitions.csv")
    }

    pub fn model_emissions(&self) -> PathBuf {
        self.model.join("emissions.csv")
    }

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
        assert!(config.lexicon().starts_with(base));

        let model_path = config.model();
        assert!(config.model_emissions().starts_with(model_path));
        assert!(config.model_transitions().starts_with(model_path));
        assert!(config.model_lemma_counts().starts_with(model_path));
    }
}
