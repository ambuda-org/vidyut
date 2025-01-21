use entries::{PyDhatuEntry, PyPadaEntry, PyPratipadikaEntry};
use pyo3::exceptions::{PyKeyError, PyOSError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_kosha::entries::{
    BasicPratipadikaEntry, DhatuEntry, DhatuMeta, KrdantaEntry, PadaEntry, PratipadikaEntry,
    SubantaEntry, TinantaEntry,
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
            Err(e) => Err(PyOSError::new_err(format!(
                "Could not load kosha. Error was: {e:?}"
            ))),
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

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __repr__(&self) -> String {
        String::from("Kosha()")
    }

    /// Return all entries with the given `key`.
    pub fn get(&self, key: String) -> Vec<PyPadaEntry> {
        let results = self.0.get_all(&key);
        results.iter().map(|p| p.into()).collect()
    }

    /// Return an iterator over all dhatus in the kosha.
    ///
    /// This method is inefficient because it creates a copy of the kosha's internal
    /// dhatu list. But since the dhatu list contains ~40,000 entries, this is not
    /// very slow in practice.
    fn dhatus(slf: PyRef<'_, Self>) -> PyResult<Py<DhatuEntryIter>> {
        // Inefficient, but works for now.
        let dhatus: Vec<PyDhatuEntry> = slf.0.dhatus().map(|x| (&x).into()).collect();
        let iter = DhatuEntryIter {
            inner: dhatus.into_iter(),
        };
        Py::new(slf.py(), iter)
    }

    /// Return an iterator over all pratipadikas in the kosha.
    ///
    /// This method is inefficient because it creates a copy of the kosha's internal
    /// pratipadika list. The pratipadika list contains more than a million entries
    /// in our official data release, so this method may be slow on some machines.
    fn pratipadikas(slf: PyRef<'_, Self>) -> PyResult<Py<PratipadikaEntryIter>> {
        // Inefficient, but works for now.
        let dhatus: Vec<PyPratipadikaEntry> = slf.0.pratipadikas().map(|x| (&x).into()).collect();
        let iter = PratipadikaEntryIter {
            inner: dhatus.into_iter(),
        };
        Py::new(slf.py(), iter)
    }
}

#[pyclass]
struct DhatuEntryIter {
    inner: std::vec::IntoIter<PyDhatuEntry>,
}

#[pymethods]
impl DhatuEntryIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyDhatuEntry> {
        slf.inner.next()
    }
}

#[pyclass]
struct PratipadikaEntryIter {
    inner: std::vec::IntoIter<PyPratipadikaEntry>,
}

#[pymethods]
impl PratipadikaEntryIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyPratipadikaEntry> {
        slf.inner.next()
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
    dhatu_meta: Vec<DhatuMeta>,
}

impl SmallRegistry {
    fn to_dhatu_entry<'a>(&'a mut self, entry: &'a PyDhatuEntry) -> DhatuEntry<'a> {
        let mut builder = DhatuMeta::builder();

        builder = builder.clean_text(entry.clean_text.to_string());
        if let Some(s) = &entry.artha_sa {
            builder = builder.artha_sa(s.to_string());
        }
        if let Some(s) = &entry.artha_en {
            builder = builder.artha_en(s.to_string());
        }
        if let Some(s) = &entry.artha_hi {
            builder = builder.artha_hi(s.to_string());
        }
        if let Some(s) = &entry.ittva {
            builder = builder.ittva(s.to_string());
        }
        if let Some(s) = &entry.karmatva {
            builder = builder.karmatva(s.to_string());
        }
        if let Some(s) = &entry.pada {
            builder = builder.pada(s.to_string());
        }

        let meta = builder.build().expect("clean_text defined");
        self.dhatu_meta.push(meta);

        let m = self.dhatu_meta.last().expect("just pushed");
        DhatuEntry::new(entry.dhatu.as_rust()).with_meta(m)
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
            } => {
                let rs_phit_entry = self.to_pratipadika_entry(pratipadika_entry)?;
                if let (Some(li), Some(vi), Some(va)) = (linga, vibhakti, vacana) {
                    PadaEntry::Subanta(SubantaEntry::new(
                        rs_phit_entry,
                        (*li).into(),
                        (*vi).into(),
                        (*va).into(),
                    ))
                } else {
                    PadaEntry::Subanta(SubantaEntry::avyaya(rs_phit_entry))
                }
            }
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
