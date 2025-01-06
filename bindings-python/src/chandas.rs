use crate::utils::py_enum;
use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_chandas::{Akshara, Chandas, Jati, Vrtta, VrttaPada, VrttaWeight};

/// A Sanskrit syllable.
///
/// An akshara follows the following rules:
///
/// - It must contain exactly one vowel.
/// - It must end with a vowel, an anusvara, or a visarga.
/// - It must not start with an anusvara or visarga.
///
/// Together, these three rurles mean that an input string has exactly one division into aksharas.
#[pyclass(name = "Akshara", get_all, eq, ord)]
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PyAkshara {
    text: String,
    weight: String,
}

impl From<Akshara> for PyAkshara {
    fn from(a: Akshara) -> Self {
        Self {
            text: a.text().to_string(),
            weight: a.weight().to_string(),
        }
    }
}

#[pymethods]
impl PyAkshara {
    #[new]
    #[pyo3(signature = (*, text, weight))]
    fn new(text: String, weight: String) -> Self {
        PyAkshara { text, weight }
    }

    /// The text that composes this akshara.
    #[getter]
    fn text(&self) -> &str {
        &self.text
    }

    /// The weight of this akshara.
    #[getter]
    fn weight(&self) -> &str {
        &self.weight
    }

    fn __repr__(&self) -> String {
        format!("Akshara(text='{}', weight='{}')", self.text, self.weight)
    }
}

/// Represents a weight pattern as part of some Vrtta.
#[pyclass(name = "VrttaWeight", eq, eq_int)]
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum PyVrttaWeight {
    G,
    L,
    Any,
}

py_enum!(PyVrttaWeight, VrttaWeight, [G, L, Any]);

/// Represents a pada (quarter) of some Vrtta.
#[pyclass(name = "VrttaPada", eq, ord)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PyVrttaPada {
    weights: Vec<PyVrttaWeight>,
    yati: Vec<usize>,
}

#[pymethods]
impl PyVrttaPada {
    /// Create a `VrttaPada` by parsing the input `text`.
    ///
    /// Raises `ValueError` if parsing fails.
    #[staticmethod]
    #[pyo3(signature = (text))]
    pub fn from_string(text: String) -> PyResult<Self> {
        match VrttaPada::try_from(text.as_str()) {
            Ok(v) => Ok(v.into()),
            Err(_) => Err(PyValueError::new_err("Could not parse string.")),
        }
    }
}

impl From<VrttaPada> for PyVrttaPada {
    fn from(val: VrttaPada) -> Self {
        PyVrttaPada {
            weights: val.weights().into_iter().map(|w| (*w).into()).collect(),
            yati: val.yati().to_vec(),
        }
    }
}

/// Models a *vṛtta*, which defines a specific pattern of light and heavy syllables.
#[pyclass(name = "Vrtta", eq, ord)]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyVrtta {
    #[pyo3(get)]
    name: String,
    padas: Vec<PyVrttaPada>,
}

#[pymethods]
impl PyVrtta {
    #[new]
    #[pyo3(signature = (name, padas))]
    pub fn new(name: String, padas: Vec<PyVrttaPada>) -> Self {
        Self { name, padas }
    }

    pub fn __repr__(&self) -> String {
        format!("Vrtta(name=\'{}\', padas={:?})", self.name, self.padas)
    }

    /// The padas thac constitute this vrtta.
    #[getter]
    pub fn padas(&self) -> Vec<PyVrttaPada> {
        self.padas.clone()
    }
}

impl From<Vrtta> for PyVrtta {
    fn from(v: Vrtta) -> Self {
        Self {
            name: v.name().to_string(),
            padas: v.padas().iter().map(|p| p.clone().into()).collect(),
        }
    }
}

/// Models a *jāti*, which defines a specific pattern of *mātrā*s (morae).
#[pyclass(name = "Jati", eq)]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyJati {
    #[pyo3(get)]
    name: String,
    matras: Vec<i32>,
}

#[pymethods]
impl PyJati {
    #[new]
    #[pyo3(signature = (name, matras))]
    pub fn new(name: String, matras: Vec<i32>) -> Self {
        Self { name, matras }
    }

    pub fn __repr__(&self) -> String {
        format!("Jati(name=\'{}\', matras={:?})", self.name, self.matras)
    }

    /// The matras that constitute this jati.
    pub fn matras(&self) -> Vec<i32> {
        self.matras.clone()
    }
}

impl From<Jati> for PyJati {
    fn from(val: Jati) -> Self {
        Self {
            name: val.name().clone(),
            matras: val.matras().to_vec(),
        }
    }
}

/// Models a match against our database or meters.
#[pyclass(name = "Match", get_all, eq)]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PyMatch {
    pub padya: Option<String>,
    pub aksharas: Vec<Vec<PyAkshara>>,
}

#[pymethods]
impl PyMatch {
    /// The name of the matching meter, if one could be found.
    #[getter]
    fn padya(&self) -> Option<String> {
        self.padya.clone()
    }

    /// All of the aksharas of the input text.
    #[getter]
    fn aksharas(&self) -> Vec<Vec<PyAkshara>> {
        self.aksharas.clone()
    }

    fn __repr__(&self) -> String {
        match &self.padya {
            Some(s) => format!("MatchResult(name=\'{}\')", s),
            None => "MatchResult(name=None)".to_string(),
        }
    }
}

/// A metrical classifier.
#[pyclass(name = "Chandas")]
pub struct PyChandas {
    chandas: Chandas,
}

#[pymethods]
impl PyChandas {
    /// Initialize `Chandas` by reading the necessary data from the file at `path`.
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        match Chandas::from_file(path) {
            Ok(chandas) => Ok(PyChandas { chandas }),
            Err(_) => Err(PyIOError::new_err("Could not read data from path.")),
        }
    }

    /// Initialize `Chandas` from the input string.
    ///
    /// `text` should be a TSV with three columns:
    ///
    /// 1. The meter name.
    /// 2. The meter type, e.g. `vrtta` (Currently unused)
    /// 3. A string containing the characters `G`, `L`, `/`, and `|`, where `G` is guru, `L` is
    ///    laghu, `/` marks a pada boundary, and `|` marks yati.
    #[staticmethod]
    fn from_text(text: String) -> PyResult<Self> {
        match Chandas::from_text(text) {
            Ok(chandas) => Ok(PyChandas { chandas }),
            Err(_) => Err(PyIOError::new_err("Could not parse text.")),
        }
    }

    /// Classify the input string against an internal list of meters.
    ///
    /// `text` should be an SLP1 string.
    pub fn classify(&self, text: &str) -> PyResult<PyMatch> {
        let res = self.chandas.classify(text);

        let padya_name = res.padya().as_ref().map(|v| v.name().to_string());
        let mut aksharas = Vec::new();
        for row in res.aksharas() {
            let new_row: Vec<_> = row.iter().map(|a| a.clone().into()).collect();
            aksharas.push(new_row);
        }
        Ok(PyMatch {
            padya: padya_name,
            aksharas,
        })
    }

    /// Return all vrttas defined on this classifier.
    #[getter]
    pub fn vrttas(&self) -> Vec<PyVrtta> {
        self.chandas
            .vrttas()
            .iter()
            .map(|x| x.clone().into())
            .collect()
    }

    /// Return all jatis defined on this classifier.
    #[getter]
    pub fn jatis(&self) -> Vec<PyJati> {
        self.chandas
            .jatis()
            .iter()
            .map(|x| x.clone().into())
            .collect()
    }
}
