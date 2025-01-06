use crate::kosha::entries::{PyDhatuEntry, PyPadaEntry, PyPratipadikaEntry};
use crate::utils::py_only_enum;
use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_prakriya::args::{Pratipadika, Subanta, Tinanta};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
use vidyut_prakriya::{Prakriya, Step};

pub mod args;
use args::*;

/// Renders a string Pythonically.
fn py_repr_string(text: &str) -> String {
    if text.contains('\'') {
        format!("{:?}", text)
    } else {
        format!("'{}'", text)
    }
}

/// Defines the source of some rule.
#[pyclass(name = "Source", eq, ord, frozen, hash)]
#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PySource {
    Ashtadhyayi,
    Dhatupatha,
    Kashika,
    Kaumudi,
    Linganushasana,
    Phit,
    Unadipatha,
    Varttika,
}
py_only_enum!(
    PySource,
    Source,
    [
        Ashtadhyayi,
        Dhatupatha,
        Kashika,
        Kaumudi,
        Linganushasana,
        Phit,
        Unadipatha,
        Varttika
    ]
);

/// A sutra from the Ashtadhyayi or a related text.
///
/// Example::
///
///     s = Sutra(source=Source.Ashtadhyayi, code='1.1.1', text='vfdDirAdEc')
#[pyclass(name = "Sutra", get_all, eq, ord)]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PySutra {
    /// The source text that this sutra comes from.
    source: PySource,
    /// The code that represents this sutra.
    code: String,
    /// The text of this sutra. All sutra text is in the SLP1 encoding scheme.
    text: String,
}

#[pymethods]
impl PySutra {
    #[new]
    fn new(source: PySource, code: String, text: String) -> Self {
        Self { source, code, text }
    }

    fn __repr__(&self) -> String {
        format!(
            "Sutra(source={}, code={}, text={})",
            self.source.__repr__(),
            py_repr_string(&self.code),
            py_repr_string(&self.text)
        )
    }
}

/// A step in the derivation.
#[pyclass(name = "Step", eq, get_all, ord)]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyStep {
    /// The source of the rule that was applied. This is almost always `Source.Ashtadhyayi`, but
    /// some derivations may use rules from other sources (e.g. the Dhatupatha).
    pub source: PySource,
    /// The ID of the rule that was applied, e.g. "3.1.68". Most rules are from the Ashtadhyayi,
    /// but a small number of rules are from other sources. See the `source` attribute to learn
    /// where a rule has come from.
    pub code: String,
    /// The result of applying the given rule to the derivation.
    ///
    /// `result` is a list of SLP1 strings.
    pub result: Vec<String>,
}

#[pymethods]
impl PyStep {
    #[new]
    fn new(source: PySource, code: String, result: Vec<String>) -> Self {
        Self {
            source,
            code,
            result,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Step(source={}, code={}, result={:?})",
            self.source.__repr__(),
            py_repr_string(&self.code),
            self.result
        )
    }
}

/// A derivation.
#[pyclass(name = "Prakriya", get_all)]
pub struct PyPrakriya {
    /// The final output of the derivation.
    ///
    /// `text` is an SLP1 string.
    pub text: String,
    /// All of the steps that were applied during the derivation.
    pub history: Vec<PyStep>,
}

fn to_py_history(history: &[Step]) -> Vec<PyStep> {
    use vidyut_prakriya::Rule;

    history
        .iter()
        .map(|x| {
            let (source, code) = match x.rule() {
                Rule::Ashtadhyayi(s) => (PySource::Ashtadhyayi, s),
                Rule::Dhatupatha(s) => (PySource::Dhatupatha, s),
                Rule::Kashika(s) => (PySource::Kashika, s),
                Rule::Kaumudi(s) => (PySource::Kaumudi, s),
                Rule::Linganushasana(s) => (PySource::Linganushasana, s),
                Rule::Phit(s) => (PySource::Phit, s),
                Rule::Unadipatha(s) => (PySource::Unadipatha, s),
                Rule::Varttika(s) => (PySource::Varttika, s),
            };

            PyStep {
                source,
                code: code.to_string(),
                result: x.result().iter().map(|t| t.text().to_string()).collect(),
            }
        })
        .collect()
}

fn to_py_prakriyas(prakriyas: Vec<Prakriya>) -> Vec<PyPrakriya> {
    prakriyas
        .into_iter()
        .map(|p| PyPrakriya {
            text: p.text(),
            history: to_py_history(p.history()),
        })
        .collect()
}

/// An entry in the Dhatupatha.
#[pyclass(name = "DhatupathaEntry", eq, ord)]
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct PyDhatupathaEntry {
    /// The numeric code associated with this dhatu.
    ///
    /// If you are using the linguistic data that comes with Vidyut, this code is consistent with
    /// the codes used by ashtadhyayi.com.
    #[pyo3(get)]
    code: String,
    /// The dhatu itself.
    #[pyo3(get)]
    dhatu: PyDhatu,
    /// The meaning of this dhatu.
    #[pyo3(get)]
    artha: String,
}

#[pymethods]
impl PyDhatupathaEntry {
    #[new]
    fn new(code: String, dhatu: PyDhatu, artha: String) -> Self {
        Self { code, dhatu, artha }
    }

    fn __repr__(&self) -> String {
        format!(
            "Entry(code={}, dhatu={}, artha={})",
            py_repr_string(&self.code),
            self.dhatu.__repr__(),
            py_repr_string(&self.artha)
        )
    }
}

/// Provides an interface to various data files.
#[pyclass(name = "Data")]
pub struct PyData(PathBuf);

#[pymethods]
impl PyData {
    /// Create a new dhatupatha instance from the given `path`.
    ///
    /// `path` should point to a 3-column TSV with columns `code`, `dhatu`, and `artha`.
    ///
    /// - `code` should be a number in the format "X.Y", such as "01.0001".
    ///
    /// - `dhatu` should be the dhatu`s aupadeshika form transliterated as SLP1.
    ///   Svaras and nasal vowels must be indicated explictly.
    ///
    /// - `artha` is an arbitrary string.
    ///
    /// Exceptions:
    ///
    /// - :exc:`FileNotFoundError` if the file does not exist.
    /// - :exc:`ValueError` if the function cannot parse the input file.
    #[new]
    pub fn new(path: PathBuf) -> PyResult<Self> {
        if !path.exists() {
            let message = format!("No such file: '{}'", path.display());
            return Err(PyFileNotFoundError::new_err(message));
        }
        Ok(Self(path))
    }

    /// Return all entries defined in the Dhatupatha.
    pub fn load_dhatu_entries(&self) -> PyResult<Vec<PyDhatupathaEntry>> {
        let path = self.0.join("dhatupatha.tsv");
        match Dhatupatha::from_path(&path) {
            Ok(dhatupatha) => {
                let mut ret = Vec::new();
                for entry in dhatupatha.iter() {
                    ret.push(PyDhatupathaEntry {
                        code: entry.code().to_string(),
                        dhatu: entry.dhatu().clone().into(),
                        artha: entry.artha().to_string(),
                    });
                }
                Ok(ret)
            }
            Err(e) => {
                let message = format!(
                    "Could not parse file '{}'. Error was: `{}`",
                    path.display(),
                    e
                );
                Err(PyValueError::new_err(message))
            }
        }
    }

    /// Return all sutras used as rules.
    ///
    /// Sutras come from various sources, including:
    ///
    /// - the gana-sutras in the Dhatupatha
    /// - the Ashtadhyayi
    /// - the Unadipatha
    /// - various varttikas
    pub fn load_sutras(&self) -> PyResult<Vec<PySutra>> {
        let mut sutras = Vec::new();

        for (filename, source) in &[
            ("dhatupatha-ganasutras.tsv", PySource::Dhatupatha),
            ("sutrapatha.tsv", PySource::Ashtadhyayi),
            ("varttikas.tsv", PySource::Varttika),
            ("unadipatha.tsv", PySource::Unadipatha),
        ] {
            let path = self.0.join(filename);
            let text = std::fs::read_to_string(path)?;

            for line in text.lines() {
                let fields: Vec<_> = line.split('\t').collect();
                if let &[code, text] = &fields[..] {
                    sutras.push(PySutra::new(*source, code.to_string(), text.to_string()))
                } else {
                    return Err(PyValueError::new_err(format!(
                        "Could not parse `{}`",
                        filename
                    )));
                }
            }
        }
        Ok(sutras)
    }
}

#[derive(FromPyObject)]
pub enum Derivable {
    Dhatu(PyDhatu),
    Pratipadika(PyPratipadika),
    Pada(PyPada),
    DhatuEntry(PyDhatuEntry),
    PratipadikaEntry(PyPratipadikaEntry),
    PadaEntry(PyPadaEntry),
}

/// An interface to the rules of the Ashtadhyayi.
///
/// Options:
///
/// - `log_steps` (default: ``True``) -- If set, log each step in the prakriya. Disable this if you
///   want extra speed.
/// - `is_chandasi` (default: ``False``) -- If set, use rules tagged with *chandasi*, *mantre*,
///   etc.
/// - `use_svaras` (default: ``False``) -- If set, use rules that add svaras to the output.
/// - `nlp_mode` (default: ``False``) -- If set, preserve the final `s`/`r` of words. This behavior
///   is useful for certain applications in natural language processing.
#[pyclass(name = "Vyakarana")]
#[derive(Default)]
pub struct PyVyakarana(Vyakarana);

impl PyVyakarana {
    fn derive_dhatus(&self, dhatu: &PyDhatu) -> Vec<PyPrakriya> {
        let args = dhatu.as_ref();
        let results = self.0.derive_dhatus(args);
        to_py_prakriyas(results)
    }

    /// Return all pratipadikas that can be derived from the given arguments.
    ///
    /// This function is meant mainly for krdantas and taddhitantas. If a pratipadika cannot be
    /// derived, the function returns an empty list.
    fn derive_pratipadikas(&self, pratipadika: &PyPratipadika) -> Vec<PyPrakriya> {
        let args = pratipadika.as_ref();
        match args {
            Pratipadika::Basic(_b) => {
                let results = self.0.derive_pratipadikas(args);
                to_py_prakriyas(results)
            }
            Pratipadika::Krdanta(k) => {
                let results = self.0.derive_krdantas(k);
                to_py_prakriyas(results)
            }
            Pratipadika::Taddhitanta(t) => {
                let results = self.0.derive_taddhitantas(t);
                to_py_prakriyas(results)
            }
            _ => Vec::new(),
        }
    }

    pub fn derive_tinantas(
        &self,
        dhatu: PyDhatu,
        prayoga: PyPrayoga,
        lakara: PyLakara,
        purusha: PyPurusha,
        vacana: PyVacana,
        skip_at_agama: bool,
    ) -> Vec<PyPrakriya> {
        let args = PyPada::Tinanta {
            dhatu,
            prayoga,
            lakara,
            purusha,
            vacana,
            skip_at_agama,
        };
        self.derive_padas(args)
    }

    pub fn derive_subantas(
        &self,
        pratipadika: PyPratipadika,
        linga: PyLinga,
        vibhakti: PyVibhakti,
        vacana: PyVacana,
        is_avyaya: bool,
    ) -> Vec<PyPrakriya> {
        let args = PyPada::Subanta {
            pratipadika,
            linga,
            vibhakti,
            vacana,
            is_avyaya,
        };
        self.derive_padas(args)
    }

    /// Return all padas that can be derived from the given arguments.
    fn derive_padas(&self, pada: PyPada) -> Vec<PyPrakriya> {
        match pada {
            PyPada::Subanta {
                pratipadika,
                linga,
                vibhakti,
                vacana,
                is_avyaya,
            } => {
                let args = if is_avyaya {
                    Subanta::avyaya(pratipadika.as_ref())
                } else {
                    Subanta::new(
                        pratipadika.as_ref(),
                        linga.into(),
                        vibhakti.into(),
                        vacana.into(),
                    )
                };
                let results = self.0.derive_subantas(&args);
                to_py_prakriyas(results)
            }
            PyPada::Tinanta {
                dhatu,
                prayoga,
                lakara,
                purusha,
                vacana,
                skip_at_agama,
            } => {
                let tin_args = Tinanta::builder()
                    .dhatu(dhatu.as_rust().clone())
                    .prayoga(prayoga.into())
                    .purusha(purusha.into())
                    .vacana(vacana.into())
                    .lakara(lakara.into())
                    .skip_at_agama(skip_at_agama)
                    .build()
                    .expect("should have all required fields");

                let results = self.0.derive_tinantas(&tin_args);
                to_py_prakriyas(results)
            }
        }
    }
}

#[pymethods]
impl PyVyakarana {
    /// Create an interface to the system.
    ///
    /// Options:
    /// - `log_steps` (default: `True`) -- If set, log each step in the prakriya. Disable this if
    ///     you want extra speed.
    /// - `is_chandasi` (default: `False`) -- If set, use rules tagged with *chandasi*, *mantre*,
    ///     etc.
    /// - `use_svaras` (default: `False`) -- If set, use rules that add svaras to the output.
    /// - `nlp_mode` (default: `False`) -- If set, preserve the final `s`/`r` of words. This
    ///     behavior is useful for certain applications in natural language processing.
    #[new]
    #[pyo3(signature = (*, log_steps=true, is_chandasi=false, use_svaras=false, nlp_mode=false))]
    pub fn new(log_steps: bool, is_chandasi: bool, use_svaras: bool, nlp_mode: bool) -> Self {
        Self(
            Vyakarana::builder()
                .log_steps(log_steps)
                .is_chandasi(is_chandasi)
                .use_svaras(use_svaras)
                .nlp_mode(nlp_mode)
                .build(),
        )
    }

    fn __repr__(&self) -> String {
        #[allow(unused)]
        fn repr_py_bool(v: bool) -> &'static str {
            if v {
                "True"
            } else {
                "False"
            }
        }
        format!("Vyakarana()")
    }

    /// Return all prakriyas that can be derived from the input arguments.
    ///
    /// `args` must be one of the following types:
    ///
    /// - :class:`~vidyut.prakriya.Dhatu`
    /// - :class:`~vidyut.prakriya.Pratipadika`
    /// - :class:`~vidyut.prakriya.Pada`
    /// - :class:`~vidyut.kosha.DhatuEntry`
    /// - :class:`~vidyut.kosha.PratipadikaEntry`
    /// - :class:`~vidyut.kosha.PadaEntry`
    ///
    /// Exceptions:
    ///
    /// - :exc:`TypeError` if `args` is not one of the types above.
    /// - :exc:`ValueError` if `args` is `PadaEntry.Unknown()`.
    ///
    /// Results are returned as a list of :class:`~vidyut.prakriya.Prakriya` objects.
    pub fn derive(&self, args: Derivable) -> PyResult<Vec<PyPrakriya>> {
        let ret = match args {
            Derivable::Dhatu(d) => self.derive_dhatus(&d),
            Derivable::Pratipadika(p) => self.derive_pratipadikas(&p),
            Derivable::Pada(p) => self.derive_padas(p),
            Derivable::DhatuEntry(d) => self.derive_dhatus(&d.to_prakriya_args()),
            Derivable::PratipadikaEntry(p) => self.derive_pratipadikas(&p.to_prakriya_args()),
            Derivable::PadaEntry(p) => self.derive_padas(p.to_prakriya_args()?),
        };
        Ok(ret)
    }
}
