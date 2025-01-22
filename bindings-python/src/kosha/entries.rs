use pyo3::prelude::*;

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

fn py_repr_option_string(option: &Option<String>) -> String {
    match option {
        Some(text) => py_repr_string(&text),
        None => String::from("None"),
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

    /// The meaning of this dhatu's *mūla* as an SLP1 string.
    ///
    /// We have meaning strings only for the ~2000 *mūla* dhatus from the Dhatupatha. Any roots
    /// derived from these ~2000 will share their `artha` with the dhatu they come from.
    ///
    /// Examples:
    ///
    /// - `BU` --> `sattAyAm`
    /// - `aBiBU` --> `sattAyAm`
    /// - `aBibuBUza` --> `sattAyAm`
    pub(crate) artha_sa: Option<String>,
    pub(crate) artha_en: Option<String>,
    pub(crate) artha_hi: Option<String>,
    pub(crate) karmatva: Option<String>,
    pub(crate) ittva: Option<String>,
    pub(crate) pada: Option<String>,
}

#[pymethods]
impl PyDhatuEntry {
    /// Create a new `DhatuEntry`.
    #[new]
    #[pyo3(signature = (dhatu, clean_text, *, artha_sa = None, artha_en = None, artha_hi = None,
        karmatva = None, ittva = None, pada = None))]
    fn new(
        dhatu: PyDhatu,
        clean_text: String,
        artha_sa: Option<String>,
        artha_en: Option<String>,
        artha_hi: Option<String>,
        karmatva: Option<String>,
        ittva: Option<String>,
        pada: Option<String>,
    ) -> Self {
        Self {
            dhatu,
            clean_text,
            artha_sa,
            artha_en,
            artha_hi,
            karmatva,
            ittva,
            pada,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "DhatuEntry(dhatu={}, clean_text={}, artha_sa={})",
            self.dhatu.__repr__(),
            py_repr_string(&self.clean_text),
            py_repr_option_string(&self.artha_sa),
        )
    }

    /// Convert this entry to a :class:`~vidyut.prakriya.Dhatu`.
    pub fn to_prakriya_args(&self) -> PyDhatu {
        self.dhatu.clone()
    }
}

impl<'a> From<&DhatuEntry<'a>> for PyDhatuEntry {
    fn from(val: &DhatuEntry<'a>) -> PyDhatuEntry {
        Self {
            dhatu: val.dhatu().into(),
            clean_text: val.clean_text().to_string(),
            artha_sa: val.artha_sa().map(|x| x.to_string()),
            artha_en: val.artha_en().map(|x| x.to_string()),
            artha_hi: val.artha_hi().map(|x| x.to_string()),
            ittva: None,
            karmatva: None,
            pada: None,
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

    /// Returns the lingas that this *prātipadika* is allowed to use.
    ///
    /// If empty, lingas might not yet be implemented for this *prātipadika* type.
    #[getter]
    pub fn lingas(&self) -> Vec<PyLinga> {
        match self {
            Self::Basic { lingas, .. } => lingas.clone(),
            Self::Krdanta { krt, .. } => vp::BaseKrt::from(*krt)
                .lingas()
                .iter()
                .map(|x| (*x).into())
                .collect(),
        }
    }

    #[getter]
    pub fn is_avyaya(&self) -> bool {
        match self {
            Self::Basic { pratipadika, .. } => pratipadika.is_avyaya(),
            Self::Krdanta { krt, .. } => vp::BaseKrt::from(*krt).is_avyaya(),
        }
    }

    /// Convert this entry to a :class:`~vidyut.prakriya.Pratipadika`.
    pub fn to_prakriya_args(&self) -> PyPratipadika {
        use PyPratipadikaEntry as PE;
        match self {
            PE::Basic { pratipadika, .. } => pratipadika.clone(),
            PE::Krdanta {
                dhatu_entry,
                krt,
                prayoga: _,
                lakara: _,
            } => PyPratipadika::krdanta(dhatu_entry.dhatu.clone(), krt.clone()),
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
/// A `PadaEntry` is a simple dataclass that models either a Subanta or a Tinanta. These types
/// are constructed by `Kosha` directly, and we strongly encourage you to avoid creating these
/// types for yourself unless you are creating a Kosha for yourself.
///
/// The `PadaEntry.Subanta` constructor creates a *subanta*:
///
/// .. testcode::
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
/// `PadaEntry.Subanta` can also create an *avyaya*:
///
/// .. testcode::
///
///     from vidyut.kosha import PratipadikaEntry, PadaEntry
///     from vidyut.prakriya import Pratipadika
///
///     ca = Pratipadika.basic("ca", is_avyaya=True)
///     ca_entry = PratipadikaEntry.Basic(pratipadika=ca, lingas=[])
///     pada = PadaEntry.Subanta(pratipadika_entry=ca_entry)
///
///     assert pada.lemma == "ca"
///     assert pada.is_avyaya
///
/// The `PadaEntry.Tinanta` constructor creates a *tinanta*:
///
/// .. testcode::
///
///     from vidyut.kosha import DhatuEntry, PadaEntry
///     from vidyut.prakriya import Dhatu, Prayoga, Lakara, Purusha, Vacana, Gana
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
#[pyclass(name = "PadaEntry", get_all, eq, ord)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PyPadaEntry {
    #[pyo3(constructor = (*, pratipadika_entry, linga = None, vibhakti = None, vacana = None))]
    Subanta {
        pratipadika_entry: PyPratipadikaEntry,
        linga: Option<PyLinga>,
        vibhakti: Option<PyVibhakti>,
        vacana: Option<PyVacana>,
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
                    linga.map(|x| x.__repr__()).unwrap_or("None".to_string()),
                    vibhakti.map(|x| x.__repr__()).unwrap_or("None".to_string()),
                    vacana.map(|x| x.__repr__()).unwrap_or("None".to_string()),
                )
            }
        }
    }

    /// The lemma used by this *pada*.
    ///
    /// The lemma is either a *dhātu* or a simple *prātipadika*.
    #[getter]
    pub fn lemma(&self) -> Option<String> {
        match self {
            Self::Tinanta { dhatu_entry, .. } => Some(dhatu_entry.clean_text.clone()),
            Self::Subanta {
                pratipadika_entry, ..
            } => Some(pratipadika_entry.lemma()),
        }
    }

    #[getter]
    pub fn is_avyaya(&self) -> bool {
        if let PyPadaEntry::Subanta {
            pratipadika_entry, ..
        } = self
        {
            pratipadika_entry.is_avyaya()
        } else {
            false
        }
    }

    /// Convert this entry to a :class:`~vidyut.prakriya.Pada`.
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
                dhatu_pada: None,
                skip_at_agama: false,
            },
        };

        Ok(ret)
    }
}

impl<'a> From<&PadaEntry<'a>> for PyPadaEntry {
    fn from(val: &PadaEntry) -> PyPadaEntry {
        match val {
            PadaEntry::Subanta(s) => PyPadaEntry::Subanta {
                pratipadika_entry: s.pratipadika_entry().into(),
                linga: Some(s.linga().into()),
                vibhakti: Some(s.vibhakti().into()),
                vacana: Some(s.vacana().into()),
            },
            PadaEntry::Tinanta(t) => PyPadaEntry::Tinanta {
                dhatu_entry: t.dhatu_entry().into(),
                prayoga: t.prayoga().into(),
                lakara: t.lakara().into(),
                purusha: t.purusha().into(),
                vacana: t.vacana().into(),
            },
        }
    }
}
