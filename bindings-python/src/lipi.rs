use core::cell::RefCell;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use vidyut_lipi as lipi;
use vidyut_lipi::{Lipika, Scheme};

/// Generates the following boilerplate methods:
/// - `__hash__`
/// - `__str__`
/// - `from_string`, which is the inverse of `__str__`
/// - `From<RustEnum> for PyEnum`
/// - `From<PyEnum> for RustEnum`
///
/// Requirements:
/// - Enum must derive `Hash`
macro_rules! py_enum {
    ($Py:ident, $Rust:ident, [$( $variant:ident ),*]) => {
        impl From<$Rust> for $Py {
            fn from(val: $Rust) -> Self {
                match val {
                    $(
                        $Rust::$variant => $Py::$variant,
                    )*
                }
            }
        }

        impl From<$Py> for $Rust {
            fn from(val: $Py) -> Self {
                match val {
                    $(
                        $Py::$variant => $Rust::$variant,
                    )*
                }
            }
        }

        #[pymethods]
        impl $Py {
            fn __hash__(&self) -> u64 {
                let mut hasher = DefaultHasher::new();
                self.hash(&mut hasher);
                hasher.finish()
            }

            fn __repr__(&self) -> String {
                format!("{}.{}", stringify!{$Rust}, self.name())
            }

            fn __str__(&self) -> String {
                self.name()
            }

            /// The name used to define the `Enum` member.
            ///
            /// (Defined for compatibility with Python enums.)
            #[getter]
            fn name(&self) -> String {
                let ret = match self {
                    $(
                        $Py::$variant => stringify!($variant),
                    )*
                };
                ret.to_string()
            }

            /// Returns all options for this enum.
            #[staticmethod]
            fn choices() -> Vec<$Py> {
                vec![
                    $(
                        $Py::$variant,
                    )*
                ]
            }

            /// Create an enum value from the given string.
            ///
            /// This is the inverse of `__str__`.
            #[staticmethod]
            fn from_string(val: String) -> PyResult<Self> {
                match val.as_str() {
                    $(
                        stringify!($variant) => Ok($Py::$variant),
                    )*
                    _ => Err(PyNotImplementedError::new_err("Could not parse {val}")),
                }
            }

            /// The scheme's ISO 15924 four-letter code, if one exists.
            #[getter]
            pub fn iso_15924_code(&self) -> String {
                $Rust::from(*self).iso_15924_code().to_string()
            }

            /// The scheme's ISO 15924 numeric code, if one exists.
            #[getter]
            pub fn iso_15924_numeric_code(&self) -> u16 {
                $Rust::from(*self).iso_15924_numeric_code()
            }

            /// The scheme's ICU code, if one exists.
            ///
            /// ICU codes are defined in conformance with the values in the ICU4X `Script` impl [here][1].
            #[getter]
            pub fn icu_numeric_code(&self) -> u16 {
                $Rust::from(*self).icu_numeric_code()
            }
        }
    }
}

/// A method of encoding text.
///
/// Schemes vary on various dimensions, including:
///
/// - writing system (alphabet vs. abugida)
/// - text encoding (ASCII vs. Unicode)
/// - support for Sanskrit (complete vs. partial)
#[pyclass(name = "Scheme", module = "lipi", eq, eq_int, ord)]
#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyScheme {
    /// Assamese script.
    ///
    /// The Assamese script uses the same characters as the Bengali script, with a few minor
    /// differences.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0980.pdf>
    Assamese,

    /// Balinese script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1B00.pdf>
    Balinese,

    /// Bengali script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0980.pdf>
    Bengali,

    /// Bhaiksuki script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11C00.pdf>
    Bhaiksuki,

    /// Brahmi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11000.pdf>
    Brahmi,

    /// Burmese script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1000.pdf>
    Burmese,

    /// Cham script.
    ///
    /// <https://unicode.org/charts/PDF/UAA00.pdf>
    Cham,

    /// Devanagari script.
    ///
    /// Docs:
    /// - <https://unicode.org/charts/PDF/U0900.pdf>
    /// - <https://unicode.org/charts/PDF/UA8E0.pdf> (Devanagari Extended)
    /// - <https://unicode.org/charts/PDF/U11B00.pdf> (Devanagari Extended-A)
    /// - <https://unicode.org/charts/PDF/U1CD0.pdf> (Vedic Extensions)
    Devanagari,

    /// Dogra script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11800.pdf>
    Dogra,

    /// Grantha script.
    ///
    /// Docs:
    /// - <http://www.unicode.org/charts/PDF/U11300.pdf>
    /// - <https://unicode.org/L2/L2009/09372-grantha.pdf>
    Grantha,

    /// Gujarati script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0A80.pdf>
    Gujarati,

    /// Gunjala Gondi script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11D60.pdf>
    GunjalaGondi,

    /// Gurmukhi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0A00.pdf>
    Gurmukhi,

    /// Javanese script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/UA980.pdf>
    Javanese,

    /// Kaithi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11080.pdf>
    Kaithi,

    /// Kannada script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0C80.pdf>
    Kannada,

    /// Kharoshthi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U10A00.pdf>
    Kharoshthi,

    /// Khmer script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1780.pdf>
    Khmer,

    /// Khudawadi script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U112B0.pdf>
    Khudawadi,

    /// Lepcha script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1C00.pdf>
    // Lepcha,

    /// Limbu script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U1900.pdf>
    Limbu,

    /// Mahajani script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11150.pdf>
    /// Mahajani,

    /// Malayalam script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0D00.pdf>
    Malayalam,

    /// Meetei script, known as Meetei Mayek.
    ///
    /// Docs: <https://unicode.org/charts/PDF/UABC0.pdf>
    MeeteiMayek,

    /// Masaram Gondi script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11D00.pdf>
    MasaramGondi,

    /// Modi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11600.pdf>
    Modi,

    /// Mon script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1000.pdf>
    Mon,

    /// Lao script.
    ///
    /// Docs:
    /// - <https://unicode.org/charts/PDF/U0E80.pdf>
    /// - <https://www.unicode.org/wg2/docs/n4861-17106r-lao-for-pali.pdf>
    // Lao,

    /// Nandinagari script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U119A0.pdf>
    Nandinagari,

    /// Newa script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11400.pdf>
    Newa,

    /// Odia script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0B00.pdf>
    Odia,

    /// Ol Chiki (Santali) script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U1C50.pdf>
    OlChiki,

    /// `Phags-pa script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/UA840.pdf>
    // PhagsPa,

    /// Saurashtra script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/UA880.pdf>
    Saurashtra,

    /// Sharada script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11180.pdf>
    Sharada,

    /// Siddham script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11580.pdf>
    Siddham,

    /// Sinhala script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0D80.pdf>
    Sinhala,

    /// Soyombo script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11A50.pdf>
    Soyombo,

    /// Tai Tham script (Lanna)
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1A20.pdf>
    TaiTham,

    /// Takri script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11680.pdf>
    Takri,

    /// Tamil script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0B80.pdf>
    Tamil,

    /// Telugu script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0C00.pdf>
    Telugu,

    /// Thai script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0E00.pdf>
    Thai,

    /// Tibetan script.
    ///
    /// **Status: buggy and partial.**
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0F00.pdf>
    Tibetan,

    /// Tirhuta script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11480.pdf>
    Tirhuta,

    /// Zanabazar Square script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11A00.pdf>
    ZanabazarSquare,

    /// Baraha transliteration.
    ///
    /// Docs:
    /// - <https://baraha.com/help//Keyboards/dev-phonetic.htm> (Baraha North)
    /// - <https://baraha.com/help/special-symbols.htm>
    BarahaSouth,

    /// Harvard-Kyoto transliteration.
    ///
    /// TODO: find documentation link for HK.
    HarvardKyoto,

    /// IAST transliteration.
    ///
    /// TODO: find documentation link for IAST.
    Iast,

    /// ISO 15919 transliteration.
    ///
    /// TODO: find a free documentation link for ISO 15919.
    Iso15919,

    /// ITRANS 5.3 transliteration.
    ///
    /// Docs:
    /// - <https://www.aczoom.com/itrans/> (official ITRANS site for version 5.3)
    /// - <https://www.aczoom.com/itrans/html/dvng/node3.html> (DEVNAG table)
    /// - <http://www.sanskritweb.net/itrans/itmanual2003.pdf> (Itranslator 2003 manual)
    ///
    /// ITRANS appears in various versions, some of which conflict with each other. Version 5.3
    /// seems to be the most widely used, and it is supported by software like Itranslator 2003.
    Itrans,

    /// SLP1 transliteration.
    ///
    /// Docs: <https://www.sanskritlibrary.org/pub/SLP1LiesAppendixB.pdf>
    Slp1,

    /// Velthuis transliteration.
    ///
    /// Docs:
    /// - <https://mirrors.mit.edu/CTAN/language/devanagari/velthuis/doc/manual.pdf> (Devanagari)
    /// - <https://ctan.math.illinois.edu/language/bengali/pandey/doc/bengdoc.pdf> (Bengali)
    Velthuis,

    /// WX transliteration.
    ///
    /// TODO: find documentation link for WX.
    Wx,
}

py_enum!(
    PyScheme,
    Scheme,
    [
        Assamese,
        Balinese,
        Bengali,
        Bhaiksuki,
        Brahmi,
        Burmese,
        Cham,
        Devanagari,
        Dogra,
        Grantha,
        Gujarati,
        GunjalaGondi,
        Gurmukhi,
        Javanese,
        Kaithi,
        Kannada,
        Kharoshthi,
        Khmer,
        Khudawadi,
        Limbu,
        Malayalam,
        MeeteiMayek,
        MasaramGondi,
        Modi,
        Mon,
        Nandinagari,
        Newa,
        Odia,
        OlChiki,
        Saurashtra,
        Sharada,
        Siddham,
        Sinhala,
        Soyombo,
        TaiTham,
        Takri,
        Tamil,
        Telugu,
        Thai,
        Tibetan,
        Tirhuta,
        ZanabazarSquare,
        BarahaSouth,
        HarvardKyoto,
        Iast,
        Iso15919,
        Itrans,
        Slp1,
        Velthuis,
        Wx
    ]
);

/// Return the scheme used by `input_text`, or ``None`` if the scheme could not be decided.
///
/// `detect` analyzes the input string by applying various heuristic tests. For non-ASCII scripts,
/// `detect` checks whether characters are in a specific unicode range. For ASCII scripts, `detect`
/// checks for bigrams and trigrams associated with specific encodings. (For example, `R^i` is
/// indicative of ITRANS.) Currently, `detect` returns the first match found and does not do any
/// kind of scoring, ranking, statistical modeling, etc.
///
/// `detect` is ideal for interfaces where the user would otherwise need to manually choose which
/// input encoding to use. `detect` removes some of this user friction by making a reasonable guess
/// about the user's query.
///
/// `detect` is best used on text that is mostly or entirely in one `Scheme`. For text that uses
/// multiple schemes, we recommend splitting the text into smaller chunks and running `detect` on
/// these chunks individually. For greater accuracy, we recommend using a more sophisticated
/// approach than this function provides.
#[pyfunction]
pub fn detect(input_text: &str) -> Option<PyScheme> {
    lipi::detect(input_text).map(PyScheme::from)
}

/// Transliterates `input_text` from `source` to `dest`.
///
/// `source` and `dest` must be instances of :class:`~vidyut.lipi.Scheme`.
#[pyfunction]
pub fn transliterate(input_text: &str, source: PyScheme, dest: PyScheme) -> String {
    thread_local! {
        static LIPIKA: RefCell<Lipika> = RefCell::new(Lipika::new());
    };
    LIPIKA.with_borrow_mut(|lipika| lipika.transliterate(input_text, source.into(), dest.into()))
}
