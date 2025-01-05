use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use std::path::PathBuf;

use vidyut_sandhi::{Split, Splitter};

/// Models a sandhi split.
#[pyclass(name = "Split", eq, hash, frozen, ord)]
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PySplit(Split);

#[pymethods]
impl PySplit {
    /// The first part of the split.
    #[getter]
    fn first(&self) -> &str {
        self.0.first()
    }

    /// The second part of the split.
    #[getter]
    fn second(&self) -> &str {
        self.0.second()
    }

    /// Whether the split is phonetically valid according to some basic heuristics.
    #[getter]
    fn is_valid(&self) -> bool {
        self.0.is_valid()
    }
}

/// A sandhi splitter.
#[pyclass(name = "Splitter")]
pub struct PySplitter(Splitter);

#[pymethods]
impl PySplitter {
    /// Create a sandhi splitter from the given `path`.
    ///
    /// `path` should be a CSV with columns `first`, `second`, and `result`.
    #[staticmethod]
    fn from_csv(path: PathBuf) -> PyResult<Self> {
        match Splitter::from_csv(path) {
            Ok(s) => Ok(Self(s)),
            Err(e) => Err(PyOSError::new_err(format!("Error: {:?}", e))),
        }
    }

    /// Return all possible ways to split `text` at the given `index`.
    ///
    /// The `first` field in the split is guaranteed to be non-empty.
    fn split_at(&self, text: String, index: usize) -> Vec<PySplit> {
        self.0
            .split_at(&text, index)
            .into_iter()
            .map(PySplit)
            .collect()
    }
}
