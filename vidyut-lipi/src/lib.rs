//! Hacky transliteration functions that other crates might need.
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

use rustc_hash::FxHashMap;
use wasm_bindgen::prelude::wasm_bindgen;

mod schemes;
pub mod wasm;

type Pair = (&'static str, &'static str);

/// A method of encoding text.
///
/// Schemes vary on various dimensions, including:
///
/// - writing system (alphabet vs. abugida)
/// - text encoding (ASCII vs. Unicode)
/// - support for Sanskrit (complete vs. partial)
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[wasm_bindgen]
pub enum Scheme {
    /// Bengali script.
    ///
    /// https://unicode.org/charts/PDF/U0980.pdf
    Bengali,

    /// Brahmi script.
    ///
    /// https://unicode.org/charts/PDF/U11000.pdf
    Brahmi,

    /// Devanagari script.
    ///
    /// https://unicode.org/charts/PDF/U0900.pdf
    Devanagari,

    /// Gujarati script.
    ///
    /// https://unicode.org/charts/PDF/U0A80.pdf
    Gujarati,

    /// Gurmukhi script.
    ///
    /// https://unicode.org/charts/PDF/U0A00.pdf
    Gurmukhi,

    /// Grantha script.
    ///
    /// http://www.unicode.org/charts/PDF/U11300.pdf
    Grantha,

    /// Kannada script.
    ///
    /// https://unicode.org/charts/PDF/U0C80.pdf
    Kannada,

    /// Malayalam script.
    ///
    /// https://unicode.org/charts/PDF/U0D00.pdf
    Malayalam,

    /// Oriya script.
    ///
    /// https://unicode.org/charts/PDF/U0B00.pdf
    Oriya,

    /// Sinhala script.
    ///
    /// https://unicode.org/charts/PDF/U0D80.pdf
    Sinhala,

    /// Tamil script.
    ///
    /// https://unicode.org/charts/PDF/U0B80.pdf
    Tamil,

    /// Tibetan script.
    ///
    /// https://unicode.org/charts/PDF/U0F00.pdf
    // Tibetan,

    /// Telugu script.
    ///
    /// https://unicode.org/charts/PDF/U0C00.pdf
    Telugu,

    /// Harvard-Kyoto transliteration.
    ///
    /// TODO: find documentation link for HK.
    HarvardKyoto,

    /// ITRANS transliteration.
    ///
    /// https://www.aczoom.com/itrans/online/itrans6/itrans-tables-unicode.pdf
    Itrans,

    /// IAST transliteration.
    ///
    /// TODO: find documentation link for IAST.
    Iast,

    /// SLP1 transliteration.
    ///
    /// https://www.sanskritlibrary.org/pub/SLP1LiesAppendixB.pdf
    Slp1,

    /// Velthuis transliteration.
    ///
    /// https://mirrors.mit.edu/CTAN/language/devanagari/velthuis/doc/manual.pdf
    Velthuis,
}

impl Scheme {
    fn token_pairs(&self) -> &[Pair] {
        match self {
            Scheme::Bengali => schemes::BENGALI,
            Scheme::Brahmi => schemes::BRAHMI,
            Scheme::Devanagari => schemes::DEVANAGARI,
            Scheme::Gujarati => schemes::GUJARATI,
            Scheme::Gurmukhi => schemes::GURMUKHI,
            Scheme::Grantha => schemes::GRANTHA,
            Scheme::Kannada => schemes::KANNADA,
            Scheme::Malayalam => schemes::MALAYALAM,
            Scheme::Oriya => schemes::ORIYA,
            Scheme::Sinhala => schemes::SINHALA,
            Scheme::Tamil => schemes::TAMIL,
            Scheme::Telugu => schemes::TELUGU,
            // Scheme::Tibetan => schemes::TIBETAN,
            Scheme::Slp1 => schemes::SLP1,
            Scheme::HarvardKyoto => schemes::HK,
            Scheme::Itrans => schemes::ITRANS,
            Scheme::Iast => schemes::IAST,
            Scheme::Velthuis => schemes::VELTHUIS,
        }
    }

    /// Returns whether this scheme represents an abugida.
    pub fn is_abugida(&self) -> bool {
        use Scheme::*;

        // Use an exhaustive match (no `_`) so that we explicitly account for all schemes.
        match self {
            // Abugidas are all `true`.
            Bengali | Brahmi | Devanagari | Gujarati | Gurmukhi | Grantha | Kannada | Malayalam
            | Oriya | Sinhala | Tamil | Telugu => true,

            // Alphabets are all `false`.
            HarvardKyoto | Itrans | Iast | Slp1 | Velthuis => false,
        }
    }

    /// Returns whether this scheme represents an alphabet.
    pub fn is_alphabet(&self) -> bool {
        !self.is_abugida()
    }

    /// Returns whether this scheme supports all sounds in post-Vedic Sanskrit.
    ///
    /// This check excludes accent and other vedic symbols.
    #[allow(unused)]
    pub(crate) fn supports_basic_sanskrit(&self) -> bool {
        use Scheme::*;

        matches!(
            self,
            Devanagari | Gujarati | Grantha | Kannada | Malayalam | Oriya | Sinhala | Telugu
        )
    }
}

/// Defines a mapping between two schemes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mapping {
    from: Scheme,
    to: Scheme,
    all: FxHashMap<String, String>,
    marks: FxHashMap<String, String>,
    input_virama: String,
    output_virama: String,
    consonants: FxHashMap<String, String>,
    len_longest_key: usize,
}

struct OneWayMapping {
    // Maps from Devanagari to all options available in the given scheme.
    data: FxHashMap<String, Vec<String>>,
    virama: String,
}

fn create_kv_map(pairs: &[Pair]) -> OneWayMapping {
    const VIRAMA: &str = "\u{094d}";

    let mut data = FxHashMap::default();
    let mut virama = String::new();
    for (key, value) in pairs {
        let key = key.to_string();
        if key == VIRAMA {
            virama += value;
        }
        let vals: &mut Vec<_> = data.entry(key).or_default();
        vals.push(value.to_string());
    }
    OneWayMapping { data, virama }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum TokenType {
    /// A consonant. A following vowel becomes a vowel mark.
    Consonant,
    /// A vowel mark, which must follow a consonant.
    VowelMark,
    /// Any other token.
    Other,
}

fn decide_token_type(s: &str) -> TokenType {
    const MARK_AA: u32 = 0x093e;
    const MARK_AU: u32 = 0x094c;
    const MARK_L: u32 = 0x0962;
    const MARK_LL: u32 = 0x0963;
    const MARK_PRISHTAMATRA_E: u32 = 0x094e;
    const MARK_AW: u32 = 0x094f;

    const CONS_KA: u32 = 0x0915;
    const CONS_HA: u32 = 0x0939;
    const CONS_QA: u32 = 0x0958;
    const CONS_YYA: u32 = 0x095f;
    const CONS_DDDA: u32 = 0x097e;
    const CONS_BBA: u32 = 0x097f;

    // const VIRAMA: u32 = 0x094d;

    if let Some(c) = s.chars().last() {
        let code = c as u32;
        if (code >= MARK_AA && code <= MARK_AU)
            || code == MARK_PRISHTAMATRA_E
            || code == MARK_AW
            || code == MARK_L
            || code == MARK_LL
        {
            TokenType::VowelMark
        } else if (code >= CONS_KA && code <= CONS_HA)
            || (code >= CONS_QA && code <= CONS_YYA)
            || code == CONS_DDDA
            || code == CONS_BBA
        // || code == VIRAMA
        {
            TokenType::Consonant
        } else {
            TokenType::Other
        }
    } else {
        TokenType::Other
    }
}

impl Mapping {
    /// Creates a mappping between the given `Scheme`s.
    fn new(from_scheme: Scheme, to_scheme: Scheme) -> Mapping {
        let from = create_kv_map(from_scheme.token_pairs());
        let to = create_kv_map(to_scheme.token_pairs());

        let mut all = FxHashMap::default();
        let mut marks = FxHashMap::default();
        let mut consonants = FxHashMap::default();

        // Iterate over token pairs so that we maintain the input order.
        for (deva_key, f) in from_scheme.token_pairs() {
            let token_type = decide_token_type(deva_key);
            let to_values = match to.data.get(*deva_key) {
                Some(x) => x,
                None => continue,
            };
            let t = match to_values.first() {
                Some(x) => x,
                None => continue,
            };

            match token_type {
                TokenType::VowelMark => {
                    marks.insert(f.to_string(), t.to_string());
                }
                TokenType::Consonant => {
                    consonants.insert(f.to_string(), t.to_string());
                }
                TokenType::Other => (),
            }

            // Insert only the first match seen. Consequences:
            //
            // - If a sound maps to both a vowel and a vowel mark, we insert the vowel mark,
            //   which comes first in our representation.
            //
            // - If a sound has alternates, we store only the first.
            if !all.contains_key(*f) {
                all.insert(f.to_string(), t.to_string());
            }
        }

        let len_longest_key = all.keys().map(|x| x.len()).max().unwrap_or(0);

        Self {
            from: from_scheme,
            to: to_scheme,
            all,
            marks,
            consonants,
            input_virama: from.virama,
            output_virama: to.virama,
            len_longest_key,
        }
    }

    /// The input scheme for this mapping.
    pub fn from(&self) -> Scheme {
        self.from
    }

    /// The output scheme for this mapping.
    pub fn to(&self) -> Scheme {
        self.to
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.all.get(key)
    }
}

/// Transliterates from an abugida.
fn transliterate_from_abugida(input: &str, mapping: Mapping) -> String {
    let chars: Vec<char> = input.chars().collect();
    let is_to_alpha = mapping.to.is_alphabet();

    let mut output = String::new();
    let mut i = 0;
    let mut key = String::new();
    let mut had_consonant = false;
    while i < chars.len() {
        key.clear();
        key.extend(&chars[i..=i]);

        match mapping.get(&key) {
            Some(s) => {
                if had_consonant
                    && (mapping.marks.contains_key(&key) || key == mapping.input_virama)
                {
                    // Pop implicit "a" vowel.
                    output.pop();
                }

                output += s;

                if is_to_alpha && mapping.consonants.contains_key(&key) {
                    // Add implicit "a" vowel.
                    output += "a";
                    had_consonant = true;
                }
            }
            None => {
                output.push_str(&key);
            }
        }
        i += 1;
    }

    output
}

/// Transliterates from an alphabet.
fn transliterate_from_alphabet(input: &str, mapping: Mapping) -> String {
    let chars: Vec<char> = input.chars().collect();
    let is_to_abugida = mapping.to.is_abugida();

    let mut output = String::new();
    let mut i = 0;
    let mut key = String::new();
    let mut had_consonant = false;
    while i < chars.len() {
        let mut o: Option<&String> = None;

        let mut key_len_in_chars = 0;
        for j in (1..=mapping.len_longest_key).rev() {
            key_len_in_chars = j;
            let limit = std::cmp::min(i + j, chars.len());
            key.clear();
            key.extend(&chars[i..limit]);

            o = mapping.get(&key);
            if let Some(_s) = o {
                break;
            }
        }

        match o {
            Some(o) => {
                if had_consonant {
                    if let Some(mark) = mapping.marks.get(&key) {
                        if is_to_abugida {
                            output.pop();
                        }
                        output += mark;
                    } else if key == "a" && is_to_abugida {
                        output.pop();
                    } else {
                        output += o;
                    }
                } else {
                    output += o;
                }

                had_consonant = mapping.consonants.contains_key(&key);
                if had_consonant && is_to_abugida {
                    output += &mapping.output_virama;
                }
            }
            None => {
                // Use the original character as-is.
                output.push_str(&key);
                had_consonant = false;
            }
        }

        // Add length in *chars*, not in *bytes*. Otherwise we get weird output.
        debug_assert!(key_len_in_chars > 0);
        i += key_len_in_chars;
    }
    output
}

/// Transliterates the given input text.
pub fn transliterate(input: &str, from: Scheme, to: Scheme) -> String {
    let mapping = Mapping::new(from, to);

    if from.is_abugida() {
        transliterate_from_abugida(input, mapping)
    } else {
        transliterate_from_alphabet(input, mapping)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schemes() {
        let mark_aa = "\u{093e}";

        let slp1 = Scheme::Slp1.token_pairs();
        assert!(slp1.contains(&("आ", "A")));
        assert!(slp1.contains(&(mark_aa, "A")));

        let hk = Scheme::HarvardKyoto.token_pairs();
        assert!(hk.contains(&("आ", "A")));
        assert!(hk.contains(&(mark_aa, "A")));

        let deva = Scheme::Devanagari.token_pairs();
        assert!(deva.contains(&("आ", "आ")));
        assert!(deva.contains(&(mark_aa, mark_aa)));

        let deva = Scheme::Devanagari;
        assert_ne!(deva.is_abugida(), deva.is_alphabet());
    }

    #[test]
    fn test_decide_char_type() {
        let is_mark = |c| decide_token_type(c) == TokenType::VowelMark;
        let is_consonant = |c| decide_token_type(c) == TokenType::Consonant;
        let is_other = |c| decide_token_type(c) == TokenType::Other;

        assert!(is_mark("\u{093e}"));
        assert!(is_mark("\u{093f}"));
        assert!(is_mark("\u{094b}"));
        assert!(is_mark("\u{094c}"));
        assert!(is_mark("\u{094e}"));
        assert!(is_mark("\u{094f}"));

        assert!(is_consonant("क"));
        assert!(is_consonant("ख"));
        assert!(is_consonant("स"));
        assert!(is_consonant("ह"));
        // Consonant clusters
        assert!(is_consonant("क्ष"));
        assert!(is_consonant("ज्ञ"));

        assert!(is_other("१"));
    }

    #[test]
    fn test_mapping() {
        let assert_has = |m: &Mapping, x: &str, y: &str| {
            assert_eq!(m.get(x), Some(&y.to_string()));
        };

        let m = Mapping::new(Scheme::Devanagari, Scheme::Itrans);
        assert_has(&m, "आ", "A");
        assert_has(&m, "\u{093e}", "A");
        assert_has(&m, "ए", "e");
        assert_has(&m, "\u{0947}", "e");

        let m = Mapping::new(Scheme::Bengali, Scheme::Itrans);
        assert_has(&m, "\u{09be}", "A");
        assert_has(&m, "\u{09c7}", "e");
    }

    #[test]
    fn test_transliterate() {
        let t = |s| transliterate(s, Scheme::HarvardKyoto, Scheme::Devanagari);
        assert_eq!(t("namaskRtya"), "नमस्कृत्य");
    }
}
