//! Defines Python bindings for `vidyut_cheda`.
use vidyut_cheda::{Chedaka, Error, Model, ModelBuilder};

use crate::kosha::entries::PyPadaEntry;
use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;

/// A token.
#[pyclass(name = "Token", get_all)]
pub struct PyToken {
    /// The token text.
    pub text: String,
    /// Other information associated with the token.
    pub data: PyPadaEntry,
}

#[pymethods]
impl PyToken {
    #[getter]
    fn lemma(&self) -> Option<String> {
        self.data.lemma()
    }

    fn __repr__(&self) -> String {
        format!(
            "Token<(text=\'{}\', lemma='{}', info={})>",
            self.text,
            self.lemma().unwrap_or_default(),
            self.data.__repr__()
        )
    }
}

/// A Sanskrit segmentation engine.
#[pyclass(name = "Chedaka")]
pub struct PyChedaka {
    chedaka: Chedaka,
}

#[pymethods]
impl PyChedaka {
    /// Initialize `Chedaka` by reading the necessary data from the directory at `path`.
    ///
    /// This constructor raises a ValueError if the initialiation fails.
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        match Chedaka::new(&path) {
            Ok(chedaka) => Ok(PyChedaka { chedaka }),
            Err(e) => Err(WrappedError(e).into()),
        }
    }

    /// Parse the given SLP1 input and return a list of `Token` objects.
    pub fn run(&self, slp1_text: &str) -> PyResult<Vec<PyToken>> {
        let tokens = match self.chedaka.run(slp1_text) {
            Ok(tokens) => tokens,
            Err(e) => return Err(WrappedError(e).into()),
        };

        let mut ret = Vec::new();
        for token in tokens {
            ret.push(PyToken {
                text: token.text().to_string(),
                data: token.data().into(),
            });
        }

        Ok(ret)
    }
}

#[pyclass(name = "Model")]
pub struct PyModel(Model);

#[pyclass(name = "ModelBuilder")]
pub struct PyModelBuilder(ModelBuilder);

#[pymethods]
impl PyModelBuilder {
    #[new]
    fn new() -> Self {
        Self(ModelBuilder::new())
    }

    /// Writes the model to disk.
    fn write_model(&self, base_path: PathBuf) -> PyResult<()> {
        match self.0.write_model(&base_path) {
            Ok(_) => Ok(()),
            Err(_) => Err(PyOSError::new_err("Could not write model.")),
        }
    }
}

struct WrappedError(Error);

impl From<Error> for WrappedError {
    fn from(e: Error) -> Self {
        Self(e)
    }
}

impl From<WrappedError> for PyErr {
    fn from(e: WrappedError) -> Self {
        match e.0 {
            Error::Io(e) => PyOSError::new_err(format!("{}", e)),
            e => PyValueError::new_err(format!("{}", e)),
        }
    }
}
