use crate::utils::py_only_enum;
use pyo3::prelude::*;

use crate::prakriya::args::{
    PyDhatu, PyKrt, PyLakara, PyLinga, PyPratipadika, PyPrayoga, PyPurusha, PyVacana, PyVibhakti,
};
use vidyut_kosha::entries::*;
use vidyut_prakriya::args as vp;

/// Renders a string Pythonically.
fn py_repr_string(text: &str) -> String {
    if text.contains('\'') {
        format!("{:?}", text)
    } else {
        format!("'{}'", text)
    }
}

/// A verb root.
#[pyclass(name = "DhatuEntry", get_all, eq, ord)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyDhatuEntry {
    dhatu: PyDhatu,

    /// The human-readable text representation of this dhatu.
    ///
    /// Examples:
    ///
    /// - `qukf\\Y` --> `kf`
    /// - `vidi~` --> `vind`
    clean_text: String,
}

#[pymethods]
impl PyDhatuEntry {
    /// Create a new `DhatuEntry`.
    #[new]
    #[pyo3(signature = (*, dhatu, clean_text))]
    fn new(dhatu: PyDhatu, clean_text: String) -> Self {
        Self { dhatu, clean_text }
    }

    #[getter]
    fn dhatu(&self) -> PyDhatu {
        self.dhatu.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "DhatuEntry(dhatu={}, clean_text={})",
            self.dhatu.__repr__(),
            py_repr_string(&self.clean_text)
        )
    }
}

impl<'a> From<&DhatuEntry<'a>> for PyDhatuEntry {
    fn from(val: &DhatuEntry<'a>) -> PyDhatuEntry {
        Self {
            dhatu: val.dhatu().into(),
            clean_text: val.clean_text().to_string(),
        }
    }
}

/// A *prātipadika* with its metadata.
#[pyclass(name = "PratipadikaEntry", get_all, eq, ord)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PyPratipadikaEntry {
    #[pyo3(constructor = (*, pratipadika, lingas))]
    Basic {
        pratipadika: PyPratipadika,
        lingas: Vec<PyLinga>,
    },

    #[pyo3(constructor = (*, dhatu_entry, krt, prayoga = None, lakara = None))]
    Krdanta {
        dhatu_entry: PyDhatuEntry,
        krt: PyKrt,
        prayoga: Option<PyPrayoga>,
        lakara: Option<PyLakara>,
    },
}

#[pymethods]
impl PyPratipadikaEntry {
    /// The lemma that corresponds to this *prātipadika*.
    #[getter]
    pub fn lemma(&self) -> String {
        match &self {
            Self::Basic { pratipadika, .. } => pratipadika.text.clone(),
            Self::Krdanta { dhatu_entry, .. } => dhatu_entry.clean_text.clone(),
        }
    }

    pub fn __repr__(&self) -> String {
        match self {
            Self::Basic {
                pratipadika,
                lingas,
            } => {
                let mut ret = format!(
                    "PratipadikaEntry.Basic(pratipadika={}, lingas=[",
                    &pratipadika.__repr__(),
                );
                let mut pushed = false;
                for linga in lingas {
                    if pushed {
                        ret.push_str(", ");
                    }
                    ret.push_str(&linga.__repr__());
                    pushed = true;
                }
                ret.push_str("])");
                ret
            }
            Self::Krdanta {
                dhatu_entry,
                krt,
                prayoga,
                lakara,
            } => {
                format!(
                    "PratipadikaEntry.Krdanta(dhatu_entry={}, krt={}, prayoga={}, lakara={})",
                    dhatu_entry.__repr__(),
                    krt.__repr__(),
                    prayoga
                        .map(|x| x.__repr__())
                        .unwrap_or(String::from("None")),
                    lakara.map(|x| x.__repr__()).unwrap_or(String::from("None")),
                )
            }
        }
    }
}

impl<'a> From<&PratipadikaEntry<'a>> for PyPratipadikaEntry {
    fn from(val: &PratipadikaEntry<'a>) -> PyPratipadikaEntry {
        match val {
            PratipadikaEntry::Basic(b) => PyPratipadikaEntry::Basic {
                pratipadika: {
                    let phit: vp::Pratipadika = b.pratipadika().clone().into();
                    phit.into()
                },
                lingas: b.lingas().iter().map(|x| (*x).into()).collect(),
            },
            PratipadikaEntry::Krdanta(k) => PyPratipadikaEntry::Krdanta {
                dhatu_entry: k.dhatu_entry().into(),
                krt: {
                    let base_krt = match k.krt() {
                        vp::Krt::Base(b) => b.into(),
                        // TODO: unadi not supported yet anywhere. So for now, use `ac` to pass
                        // compilation. Once we add unadis, this should be supported properly.
                        vp::Krt::Unadi(_u) => vp::BaseKrt::ac,
                    };
                    base_krt.into()
                },
                prayoga: k.prayoga().map(|x| x.into()),
                lakara: k.lakara().map(|x| x.into()),
            },
        }
    }
}

/*
impl<'a> From<&'a PyPratipadikaEntry> for PratipadikaEntry<'a> {
    fn from(val: &PyPratipadikaEntry) -> PratipadikaEntry {
        match val {
            PyPratipadikaEntry::Basic {
                pratipadika,
                lingas,
            } => PratipadikaEntry::Basic(BasicPratipadikaEntry::new(pratipadika.as_ref(), &[]),
            PyPratipadikaEntry::Krdanta { dhatu_entry, krt, prayoga, lakara } => {

            }
        }
    }
}
*/

/// An entry in the kosha.
#[pyclass(name = "PadaEntry", get_all, eq, ord)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PyPadaEntry {
    Unknown(),

    #[pyo3(constructor = (*, pratipadika_entry, linga, vibhakti, vacana))]
    Subanta {
        pratipadika_entry: PyPratipadikaEntry,
        linga: PyLinga,
        vibhakti: PyVibhakti,
        vacana: PyVacana,
    },

    #[pyo3(constructor = (*, pratipadika_entry))]
    Avyaya {
        pratipadika_entry: PyPratipadikaEntry,
    },

    #[pyo3(constructor = (*, dhatu_entry, prayoga, lakara, purusha, vacana))]
    Tinanta {
        dhatu_entry: PyDhatuEntry,
        prayoga: PyPrayoga,
        lakara: PyLakara,
        purusha: PyPurusha,
        vacana: PyVacana,
    },
}

#[pymethods]
impl PyPadaEntry {
    #[getter]
    pub fn lemma(&self) -> Option<String> {
        match self {
            Self::Tinanta { dhatu_entry, .. } => Some(dhatu_entry.clean_text.clone()),
            Self::Subanta {
                pratipadika_entry, ..
            } => Some(pratipadika_entry.lemma()),
            Self::Avyaya {
                pratipadika_entry, ..
            } => Some(pratipadika_entry.lemma()),
            Self::Unknown() => None,
        }
    }

    pub fn __repr__(&self) -> String {
        match self {
            PyPadaEntry::Tinanta {
                dhatu_entry,
                prayoga,
                lakara,
                purusha,
                vacana,
            } => {
                format!("PadaEntry.Tinanta(dhatu_entry={}, prayoga={}, lakara={}, purusha={}, vacana={})",
                    dhatu_entry.__repr__(),
                    prayoga.__repr__(),
                    lakara.__repr__(),
                    purusha.__repr__(),
                    vacana.__repr__(),
                )
            }
            PyPadaEntry::Subanta {
                pratipadika_entry,
                linga,
                vibhakti,
                vacana,
            } => {
                format!(
                    "PadaEntry.Subanta(pratipadika_entry={}, linga={}, vibhakti={}, vacana={})",
                    pratipadika_entry.__repr__(),
                    linga.__repr__(),
                    vibhakti.__repr__(),
                    vacana.__repr__(),
                )
            }
            PyPadaEntry::Avyaya { pratipadika_entry } => {
                format!(
                    "PadaEntry.Avyaya(pratipadika_entry={})",
                    pratipadika_entry.__repr__(),
                )
            }
            PyPadaEntry::Unknown() => String::from("PadaEntry.Unknown()"),
        }
    }
}

impl<'a> From<&PadaEntry<'a>> for PyPadaEntry {
    fn from(val: &PadaEntry) -> PyPadaEntry {
        match val {
            PadaEntry::Subanta(s) => PyPadaEntry::Subanta {
                pratipadika_entry: s.pratipadika_entry().into(),
                linga: s.linga().into(),
                vibhakti: s.vibhakti().into(),
                vacana: s.vacana().into(),
            },
            PadaEntry::Tinanta(t) => PyPadaEntry::Tinanta {
                dhatu_entry: t.dhatu_entry().into(),
                prayoga: t.prayoga().into(),
                lakara: t.lakara().into(),
                purusha: t.purusha().into(),
                vacana: t.vacana().into(),
            },
            PadaEntry::Avyaya(a) => PyPadaEntry::Avyaya {
                pratipadika_entry: a.pratipadika_entry().into(),
            },
            PadaEntry::Unknown => PyPadaEntry::Unknown(),
        }
    }
}

/*
impl<'a> From<&'a PyPadaEntry> for PadaEntry<'a> {
    fn from(val: &PyPadaEntry) -> PadaEntry {
        match val {
            PyPadaEntry::Subanta {
                pratipadika_entry,
                linga,
                vibhakti,
                vacana,
            } => PadaEntry::Subanta(SubantaEntry::new(
                pratipadika_entry.into(),
                *linga.into(),
                *vibhakti.into(),
                *vacana.into(),
            )),
            PyPadaEntry::Tinanta {
                dhatu_entry,
                prayoga,
                lakara,
                purusha,
                vacana,
            } => PadaEntry::Tinanta(TinantaEntry::new(
                *dhatu_entry.into(),
                *prayoga.into(),
                *lakara.into(),
                *purusha.into(),
                *vacana.into(),
            )),
            PyPadaEntry::Avyaya { pratipadika_entry } => {
                PadaEntry::Avyaya(SubantaEntry::avyaya(pratipadika_entry.into()))
            }
            PyPadaEntry::Unknown() => PadaEntry::Unknown,
        }
    }
}
*/
