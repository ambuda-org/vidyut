use pyo3::{exceptions::PyValueError, prelude::*};

use crate::prakriya::args::{
    PyDhatu, PyKrt, PyLakara, PyLinga, PyPada, PyPratipadika, PyPrayoga, PyPurusha, PyVacana,
    PyVibhakti,
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
    pub(crate) dhatu: PyDhatu,

    /// The human-readable text representation of this dhatu.
    ///
    /// Examples:
    ///
    /// - `qukf\\Y` --> `kf`
    /// - `vidi~` --> `vind`
    pub(crate) clean_text: String,
}

#[pymethods]
impl PyDhatuEntry {
    /// Create a new `DhatuEntry`.
    #[new]
    #[pyo3(signature = (*, dhatu, clean_text))]
    fn new(dhatu: PyDhatu, clean_text: String) -> Self {
        Self { dhatu, clean_text }
    }

    fn __repr__(&self) -> String {
        format!(
            "DhatuEntry(dhatu={}, clean_text={})",
            self.dhatu.__repr__(),
            py_repr_string(&self.clean_text)
        )
    }

    #[getter]
    fn dhatu(&self) -> PyDhatu {
        self.dhatu.clone()
    }

    /// Convert this entry to a `Dhatu`.
    pub fn to_prakriya_args(&self) -> PyDhatu {
        self.dhatu.clone()
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

    /// The lemma that corresponds to this *prātipadika*.
    ///
    /// The lemma is either a *dhātu* or a simple *prātipadika*.
    #[getter]
    pub fn lemma(&self) -> String {
        match &self {
            Self::Basic { pratipadika, .. } => pratipadika.text.clone(),
            Self::Krdanta { dhatu_entry, .. } => dhatu_entry.clean_text.clone(),
        }
    }

    /// Convert this entry to a `Pratipadika`.
    pub fn to_prakriya_args(&self) -> PyPratipadika {
        use PyPratipadikaEntry as PE;
        match self {
            PE::Basic { pratipadika, .. } => pratipadika.clone(),
            PE::Krdanta {
                dhatu_entry,
                krt,
                prayoga: _,
                lakara: _,
            } => PyPratipadika::krdanta(dhatu_entry.dhatu().clone(), krt.clone()),
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
*/

/// An entry in the kosha.
///
/// A `PadaEntry` is a simple dataclass that has one of four types. These types are
/// constructed by `Kosha` directly, but you can create them yourself if you so choose.
///
/// The `PadaEntry.Subanta` constructor creates a *subanta*::
///
///     from vidyut.kosha import PratipadikaEntry, PadaEntry
///     from vidyut.prakriya import Pratipadika, Linga, Vibhakti, Vacana
///
///     rama = Pratipadika.basic("rAma")
///     rama_entry = PratipadikaEntry.Basic(pratipadika=rama, lingas=[Linga.Pum])
///     pada = PadaEntry.Subanta(
///         pratipadika_entry=rama_entry,
///         linga=Linga.Pum,
///         vibhakti=Vibhakti.Prathama,
///         vacana=Vacana.Eka)
///
///     assert pada.lemma == "rAma"
///
/// The `PadaEntry.Tinanta` constructor creates a *tinanta*::
///
///     from vidyut.kosha import DhatuEntry, PadaEntry
///     from vidyut.prakriya import Dhatu, Prayoga, Lakara, Purusha, Vacana
///
///     gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
///     gam_entry = DhatuEntry(dhatu=gam, clean_text="gam")
///     pada = PadaEntry.Tinanta(
///         dhatu_entry=gam_entry,
///         prayoga=Prayoga.Kartari,
///         lakara=Lakara.Lat,
///         purusha=Purusha.Prathama,
///         vacana=Vacana.Eka)
///
///     assert pada.lemma == "gam"
///
/// The `PadaEntry.Avyaya` constructor creates an *avyaya*::
///
///     from vidyut.kosha import PratipadikaEntry, PadaEntry
///     from vidyut.prakriya import Pratipadika
///
///     ca = Pratipadika.basic("ca")
///     ca_entry = PratipadikaEntry.Basic(pratipadika=ca, lingas=[])
///     pada = PadaEntry.Avyaya(pratipadika_entry=ca_entry)
///
///     assert pada.lemma == "ca"
///
/// The `PadaEntry.Unknown` constructor indicates that the semantics are unknown::
///
///     unk = PadaEntry.Unknown()
///     assert unk.lemma is None
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
    /// The lemma used by this *pada*.
    ///
    /// The lemma is either aa *dhātu* or a simple *prātipadika*.
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

    /// Convert this entry to a `Pada`.
    pub fn to_prakriya_args(&self) -> PyResult<PyPada> {
        PyPada::try_from(self)
    }
}

impl TryFrom<&PyPadaEntry> for PyPada {
    type Error = PyErr;

    fn try_from(val: &PyPadaEntry) -> PyResult<PyPada> {
        use PyPadaEntry as PE;
        let ret = match val {
            PE::Subanta {
                pratipadika_entry,
                linga,
                vibhakti,
                vacana,
            } => PyPada::Subanta {
                pratipadika: pratipadika_entry.to_prakriya_args(),
                linga: *linga,
                vibhakti: *vibhakti,
                vacana: *vacana,
                is_avyaya: false,
            },
            PE::Tinanta {
                dhatu_entry,
                prayoga,
                lakara,
                purusha,
                vacana,
            } => PyPada::Tinanta {
                dhatu: dhatu_entry.to_prakriya_args(),
                prayoga: *prayoga,
                lakara: *lakara,
                purusha: *purusha,
                vacana: *vacana,
                skip_at_agama: false,
            },
            PE::Avyaya { pratipadika_entry } => {
                PyPada::make_avyaya(pratipadika_entry.to_prakriya_args())
            }
            _ => return Err(PyValueError::new_err("Unknown PadaEntry type")),
        };

        Ok(ret)
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
