use entries::{PyDhatuEntry, PyPadaEntry, PyPratipadikaEntry};
use pyo3::exceptions::{PyKeyError, PyOSError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_kosha::entries::{
    BasicPratipadikaEntry, DhatuEntry, KrdantaEntry, PadaEntry, PratipadikaEntry, SubantaEntry,
    TinantaEntry,
};
use vidyut_kosha::{Builder, Kosha};
use vidyut_prakriya::args as vp;

pub mod entries;

/// A compact Sanskrit kosha.
#[pyclass(name = "Kosha")]
pub struct PyKosha(Kosha);

/// A Sanskrit morphological dictionary.
///
/// :class:`~vidyut.kosha.Kosha` aims to provide the same API as you would have if using a
/// read-only ``dict[str, list[PadaEntry]]``. Currently, we support only a few such methods
/// and plan to add more in future releases.
#[pymethods]
impl PyKosha {
    /// Load a `Kosha` instance from the given input path.
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        match Kosha::new(path.clone()) {
            Ok(kosha) => Ok(Self(kosha)),
            Err(_) => Err(PyOSError::new_err(
                "Unknown error. The input file might be missing.",
            )),
        }
    }

    /// Return whether the kosha contains `key`.
    pub fn __contains__(&self, key: String) -> bool {
        self.0.contains_key(&key)
    }

    /// Return all entries with the given `key`, or raise `KeyError` if `key` is missing.
    fn __getitem__(&self, key: String) -> PyResult<Vec<PyPadaEntry>> {
        let ret = self.get(key.clone());
        if ret.is_empty() {
            Err(PyKeyError::new_err(key))
        } else {
            Ok(ret)
        }
    }

    fn __repr__(&self) -> String {
        String::from("Kosha()")
    }

    /// Return all entries with the given `key`.
    pub fn get(&self, key: String) -> Vec<PyPadaEntry> {
        let results = self.0.get_all(&key);
        results.iter().map(|p| p.into()).collect()
    }
}

/// Builder for a `Kosha`.
///
/// Memory usage is linear in the number of unique lemmas.
#[pyclass(name = "Builder")]
pub struct PyBuilder {
    builder: Option<Builder>,
}

/// A quick store for Entry data.
#[derive(Default)]
struct SmallRegistry {
    lingas: Vec<Vec<vp::Linga>>,
}

impl SmallRegistry {
    fn to_dhatu_entry<'a>(&self, entry: &'a PyDhatuEntry) -> DhatuEntry<'a> {
        DhatuEntry::new(entry.dhatu.as_rust(), &entry.clean_text)
    }

    fn to_pratipadika_entry<'a>(
        &'a mut self,
        phit: &'a PyPratipadikaEntry,
    ) -> PyResult<PratipadikaEntry<'a>> {
        let ret = match phit {
            PyPratipadikaEntry::Basic {
                pratipadika,
                lingas,
            } => match pratipadika.as_ref() {
                vp::Pratipadika::Basic(b) => {
                    let lingas_rs: Vec<vp::Linga> = lingas.iter().map(|x| (*x).into()).collect();
                    let i = self.lingas.len();
                    self.lingas.push(lingas_rs);
                    PratipadikaEntry::Basic(BasicPratipadikaEntry::new(b, &self.lingas[i]))
                }
                _ => return Err(PyValueError::new_err("Unsupported pratipadika type")),
            },
            PyPratipadikaEntry::Krdanta {
                dhatu_entry,
                krt,
                prayoga,
                lakara,
            } => PratipadikaEntry::Krdanta(KrdantaEntry::new(
                self.to_dhatu_entry(dhatu_entry),
                vp::BaseKrt::from(*krt).into(),
                prayoga.map(|x| x.into()),
                lakara.map(|x| x.into()),
            )),
        };

        Ok(ret)
    }

    fn to_pada_entry<'a>(&'a mut self, pada: &'a PyPadaEntry) -> PyResult<PadaEntry<'a>> {
        let ret = match pada {
            PyPadaEntry::Subanta {
                pratipadika_entry,
                linga,
                vibhakti,
                vacana,
            } => PadaEntry::Subanta(SubantaEntry::new(
                self.to_pratipadika_entry(pratipadika_entry)?,
                (*linga).into(),
                (*vibhakti).into(),
                (*vacana).into(),
            )),
            PyPadaEntry::Tinanta {
                dhatu_entry,
                prayoga,
                lakara,
                purusha,
                vacana,
            } => PadaEntry::Tinanta(TinantaEntry::new(
                self.to_dhatu_entry(dhatu_entry),
                (*prayoga).into(),
                (*lakara).into(),
                (*purusha).into(),
                (*vacana).into(),
            )),
            PyPadaEntry::Avyaya { pratipadika_entry } => PadaEntry::Avyaya(SubantaEntry::avyaya(
                self.to_pratipadika_entry(pratipadika_entry)?,
            )),
            PyPadaEntry::Unknown() => PadaEntry::Unknown,
        };

        Ok(ret)
    }
}

#[pymethods]
impl PyBuilder {
    /// Create a new builder whose output will be written to `path`.
    ///
    /// If `path` does not exist, the builder will create it.
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        match Builder::new(path) {
            Ok(k) => Ok(Self { builder: Some(k) }),
            Err(_) => Err(PyOSError::new_err(
                "Unknown error. Our guess is that the input file is missing.",
            )),
        }
    }

    /// Insert the given (`key`, `pada`) pair.
    ///
    /// Keys must be inserted in lexicographic order. If a key is received out of order,
    /// this method will raise an `OSError`.
    fn insert(&mut self, key: String, pada: PyPadaEntry) -> PyResult<()> {
        match self.builder {
            Some(ref mut b) => {
                let mut r = SmallRegistry::default();
                let entry: PadaEntry = r.to_pada_entry(&pada)?;

                b.register_pada_entry(&entry);
                match b.insert(&key, &entry) {
                    Ok(()) => Ok(()),
                    Err(e) => Err(PyValueError::new_err(format!("Could not write key. {e}"))),
                }
            }
            None => Err(PyOSError::new_err("Kosha has already been written.")),
        }
    }

    /// Complete the build process.
    ///
    /// If this method is not called, the output data will be invalid.
    fn finish(&mut self) -> PyResult<()> {
        if let Some(x) = self.builder.take() {
            let builder = x;

            match builder.finish() {
                Ok(()) => Ok(()),
                Err(_) => Err(PyOSError::new_err("Could not write kosha.")),
            }
        } else {
            Err(PyOSError::new_err("Kosha has already been written."))
        }
    }
}
