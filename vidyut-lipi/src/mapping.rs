//! Creates maps between different schemes.   

use crate::scheme::Scheme;
use rustc_hash::{FxHashMap, FxHashSet};

/// Models how a token behaves in relation to other tokens.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum TokenType {
    /// A consonant. A following vowel generally a vowel mark.
    Consonant,
    /// A vowel mark, which generally must follow a consonant.
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
    const NUKTA: u32 = 0x093c;

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
            || code == NUKTA
        {
            TokenType::Consonant
        } else {
            TokenType::Other
        }
    } else {
        TokenType::Other
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OneWayMapping {
    to_scheme: Scheme,
    // Maps from Devanagari to all options available in the given scheme.
    data: FxHashMap<String, Vec<String>>,
    virama: String,
}

impl OneWayMapping {
    fn new(scheme: Scheme) -> Self {
        const VIRAMA: &str = "\u{094d}";
        let mut data = FxHashMap::default();
        let mut virama = String::new();

        for (deva_key, value) in scheme.token_pairs() {
            let key = deva_key.to_string();
            if key == VIRAMA {
                virama += value;
            }
            let vals: &mut Vec<_> = data.entry(key).or_default();
            vals.push(value.to_string());
        }
        if scheme.is_abugida() {
            debug_assert!(
                !virama.is_empty(),
                "Scheme `{scheme:?}` is an abugida but has no virama."
            );
        }
        Self {
            to_scheme: scheme,
            data,
            virama,
        }
    }

    /// Transliterates the given Devanagari key to `self.to_scheme`.
    ///
    /// Assumptions:
    ///
    /// - `deva` is a short key in our intermediate representation (Devanagari).
    /// - There is exactly one reasonable transliteration for `deva`.
    fn transliterate_key(&self, deva: &str) -> Option<String> {
        const VIRAMA: char = '\u{094d}';

        let mut out = String::new();
        let mut deva_char = String::new();
        for d in deva.chars() {
            if d == VIRAMA && self.to_scheme.is_alphabet() {
                out.pop();
                continue;
            }

            deva_char.clear();
            deva_char.push(d);

            let vals = self.data.get(&deva_char)?;
            let v = vals.first()?;
            out.push_str(v);

            let token_type = decide_token_type(&deva_char);
            if self.to_scheme.is_alphabet() && token_type == TokenType::Consonant {
                out.push('a');
            }
        }

        if out.ends_with('a') {
            // quick HACK for new alphabet keys.
            out.pop();
        }

        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }
}

/// Defines a mapping between two schemes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mapping {
    pub(crate) from: Scheme,
    pub(crate) to: Scheme,
    pub(crate) all: FxHashMap<String, String>,
    pub(crate) marks: FxHashMap<String, String>,
    pub(crate) input_virama: String,
    pub(crate) output_virama: String,
    pub(crate) consonants: FxHashMap<String, String>,
    pub(crate) len_longest_key: usize,
}

impl Mapping {
    /// Creates a mappping between the given `Scheme`s.
    ///
    ///
    /// ### Approach
    ///
    /// We start with two mappings: one from `A` to `X`, and one from `B` to `X`. Here `A` is our
    /// input scheme, `B` is our output scheme, and `X` is our intermediate representation.
    ///
    /// If we reverse our `B to `X` mapping to get an `X` to `B` mapping, we can join these two
    /// mappings to get an `A` to `B` mapping. This approach is workable but will two cases:
    ///
    /// 1. A mapping `a --> x` without a corresponding `x --> b`. For example, consider `| --> ळ`,
    ///    where `|` is an SLP1 character and `ळ` is not defined in B. In this case, we
    ///    transliterate `x` to scheme `B` then programmatically create a new `a --> b` mapping.
    ///
    /// 2. A mapping `x --> b` without a corresponding `a --> b`. For example, consider "ळ --> |`,
    ///    where `|` is again an SLP1 character and `ळ` is not defined in A. In this case, we
    ///    transliterate `x` to scheme `A` then programmatically create a new `a --> b` mapping.
    pub fn new(from: Scheme, to: Scheme) -> Mapping {
        // Since `new` is a public API, our parameter names use `from` and `to`. Internally, use
        // `a` and `b`, per our doc comments above.

        let a_map = OneWayMapping::new(from);
        let b_map = OneWayMapping::new(to);

        let mut all = FxHashMap::default();
        let mut marks = FxHashMap::default();
        let mut consonants = FxHashMap::default();
        let mut seen_b: FxHashSet<&str> = FxHashSet::default();

        // Iterate over `from.token_pairs()` so that we maintain a predictable input order.
        for (deva_key, a) in from.token_pairs() {
            let token_type = decide_token_type(deva_key);
            let bs = match b_map.data.get(*deva_key) {
                Some(bs) => bs,
                None => continue,
            };
            let b = match bs.first() {
                Some(b) => b,
                None => continue,
            };

            match token_type {
                TokenType::VowelMark => {
                    marks.insert(a.to_string(), b.to_string());
                }
                TokenType::Consonant => {
                    consonants.insert(a.to_string(), b.to_string());
                }
                TokenType::Other => (),
            }

            // Insert only the first match seen. Consequences:
            //
            // - If a sound maps to both a vowel and a vowel mark, we insert the vowel mark,
            //   which comes first in our representation.
            //
            // - If a sound has alternates, we store only the first.
            if !all.contains_key(*a) {
                all.insert(a.to_string(), b.to_string());
                seen_b.insert(b);
            }
        }

        for (deva_key, a) in from.token_pairs() {
            let token_type = decide_token_type(deva_key);
            if !all.contains_key(*a) && b_map.data.get(*deva_key).is_none() {
                // Mapping `a --> x` doesn't have a corresponding `x --> b`.
                // So, create one.
                let new_b = match b_map.transliterate_key(deva_key) {
                    Some(s) => s,
                    None => continue,
                };

                match token_type {
                    TokenType::VowelMark => {
                        marks.insert(a.to_string(), new_b.clone());
                    }
                    TokenType::Consonant => {
                        consonants.insert(a.to_string(), new_b.clone());
                    }
                    TokenType::Other => (),
                }

                println!("{a} --> {deva_key} --> [{new_b}] ({from:?} --> {to:?})");
                all.insert(a.to_string(), new_b);
            }
        }

        for (deva_key, b) in to.token_pairs() {
            if seen_b.contains(b) {
                continue;
            }

            let new_a = match a_map.transliterate_key(deva_key) {
                Some(s) => s,
                None => continue,
            };

            let token_type = decide_token_type(deva_key);

            if !new_a.is_empty() && !all.contains_key(&new_a) {
                match token_type {
                    TokenType::VowelMark => {
                        marks.insert(new_a.clone(), b.to_string());
                    }
                    TokenType::Consonant => {
                        consonants.insert(new_a.clone(), b.to_string());
                    }
                    TokenType::Other => (),
                }

                println!("[{new_a}] --> {deva_key} --> {b} ({from:?} --> {to:?})");
                all.insert(new_a, b.to_string());
            }
        }

        let len_longest_key = all.keys().map(|a| a.len()).max().unwrap_or(0);

        Self {
            from,
            to,
            all,
            marks,
            consonants,
            input_virama: a_map.virama,
            output_virama: b_map.virama,
            len_longest_key,
        }
    }

    /// The source scheme.
    pub fn from(&self) -> Scheme {
        self.from
    }

    /// The destination scheme.
    pub fn to(&self) -> Scheme {
        self.to
    }

    pub(crate) fn get(&self, key: &str) -> Option<&String> {
        self.all.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Scheme::*;

    #[test]
    fn test_decide_token_type() {
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
        // Nukta
        assert!(is_consonant("क\u{093c}"));

        assert!(is_other("१"));
    }

    #[test]
    fn test_one_way_mapping() {
        let slp1 = OneWayMapping::new(Iast);
        assert_eq!(slp1.transliterate_key("ळ्ह"), Some("ḻh".to_string()));

        let deva = OneWayMapping::new(Devanagari);
        assert_eq!(deva.transliterate_key("ळ्ह"), Some("ळ्ह".to_string()));
    }

    #[test]
    fn test_mapping() {
        let m = Mapping::new(Devanagari, Itrans);
        assert_eq!(m.from(), Devanagari);
        assert_eq!(m.to(), Itrans);

        let assert_has = |m: &Mapping, x: &str, y: &str| {
            assert_eq!(m.get(x), Some(&y.to_string()));
        };

        let m = Mapping::new(Devanagari, Itrans);
        assert_has(&m, "आ", "A");
        assert_has(&m, "\u{093e}", "A");
        assert_has(&m, "ए", "e");
        assert_has(&m, "\u{0947}", "e");

        let m = Mapping::new(Bengali, Itrans);
        assert_has(&m, "\u{09be}", "A");
        assert_has(&m, "\u{09c7}", "e");

        let m = Mapping::new(Velthuis, Devanagari);
        assert_has(&m, "R", "\u{095c}");
        assert_has(&m, "Rh", "\u{095d}");
    }
}
