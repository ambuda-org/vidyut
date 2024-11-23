//! Creates maps between different schemes.   

use crate::scheme::Scheme;
use rustc_hash::{FxHashMap, FxHashSet};

/// A mapping between a span of input text and a span of output text.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Span {
    /// The key of this token.
    pub key: String,
    /// The value of this token.
    pub value: String,
    /// The token type. `kind` controls how this token combines with neighboring tokens.
    pub kind: SpanKind,
}

impl Span {
    /// Creates a new `Span`.
    pub fn new(key: String, text: String, kind: SpanKind) -> Self {
        Self {
            key,
            value: text,
            kind,
        }
    }

    pub fn is_mark(&self) -> bool {
        self.kind == SpanKind::VowelMark
    }

    /// Returns whether this token represents a consonant.
    pub fn is_consonant(&self) -> bool {
        self.kind == SpanKind::Consonant
    }
}

/// Models how a token behaves in relation to other tokens.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum SpanKind {
    /// A consonant. A following vowel is generally a vowel mark.
    Consonant,
    /// A vowel mark, which generally must follow a consonant.
    VowelMark,
    /// An ayogavaha (visarga, anusvara, candrabindu, etc.)
    Ayogavaha,
    /// An accent mark.
    Accent,
    /// Any other token.
    Other,
}

impl SpanKind {
    fn from_devanagari_key(s: &str) -> Self {
        use SpanKind::*;

        const MARK_AA: char = '\u{093e}';
        const MARK_AU: char = '\u{094c}';
        const MARK_L: char = '\u{0962}';
        const MARK_LL: char = '\u{0963}';
        const MARK_PRISHTAMATRA_E: char = '\u{094e}';
        const MARK_AW: char = '\u{094f}';

        const CONS_KA: char = '\u{0915}';
        const CONS_HA: char = '\u{0939}';
        const CONS_QA: char = '\u{0958}';
        const CONS_YYA: char = '\u{095f}';
        const CONS_DDDA: char = '\u{097e}';
        const CONS_BBA: char = '\u{097f}';
        const NUKTA: char = '\u{093c}';

        const CANDRABINDU: char = '\u{0901}';
        const VISARGA: char = '\u{0903}';

        const SVARITA: char = '\u{0951}';
        const ANUDATTA: char = '\u{0952}';

        if let Some(c) = s.chars().last() {
            match c {
                (CONS_KA..=CONS_HA) | (CONS_QA..=CONS_YYA) | CONS_DDDA | CONS_BBA | NUKTA => {
                    Consonant
                }
                (MARK_AA..=MARK_AU) | MARK_PRISHTAMATRA_E | MARK_AW | MARK_L | MARK_LL => VowelMark,
                CANDRABINDU..=VISARGA => Ayogavaha,
                SVARITA | ANUDATTA => Accent,
                _ => Other,
            }
        } else {
            Other
        }
    }
}

/// A one-way mapping from our IR to some `Scheme`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OneWayMapping {
    /// The scheme being mapped to.
    to_scheme: Scheme,
    /// Maps from IR to all options available in the given scheme.
    data: FxHashMap<String, Vec<String>>,
    /// Maps from this scheme's digit chars to their numeric values.
    numeral_to_int: FxHashMap<String, u32>,
    /// The virama, or the empty string if not defined for this scheme.
    pub(crate) virama: String,
    /// The letter representation of the "a" vowel.
    pub(crate) letter_a: String,
}

impl OneWayMapping {
    fn new(scheme: Scheme) -> Self {
        const DEVA_VIRAMA: &str = "\u{094d}";
        let mut data = FxHashMap::default();
        let mut virama = String::new();

        let mut unicode_alts = FxHashMap::default();
        for (nfc, nfd) in scheme.unicode_nfd_pairs() {
            unicode_alts.insert(nfc, vec![nfc, nfd]);
            unicode_alts.insert(nfd, vec![nfc, nfd]);
        }

        for (deva_key, value) in scheme.token_pairs() {
            let key = deva_key.to_string();
            if key == DEVA_VIRAMA {
                virama += value;
            }
            let vals: &mut Vec<_> = data.entry(key).or_default();

            let value = value.to_string();
            if !vals.contains(&value) {
                vals.push(value.to_string());
            }

            // Also add Unicode equivalents.
            if let Some(vs) = unicode_alts.get(&value.as_str()) {
                for v in vs {
                    let v = v.to_string();
                    if !vals.contains(&v) {
                        vals.push(v);
                    }
                }
            }
        }

        let letter_a = match data.get("अ") {
            Some(vs) => vs.iter().next().expect("present").clone(),
            None => String::new(),
        };

        // Checks
        // ------
        if scheme.is_abugida() {
            debug_assert!(
                !virama.is_empty(),
                "Scheme `{scheme:?}` is an abugida but has no virama."
            );
        }

        // Numerals
        // --------
        let mut numeral_to_int = FxHashMap::default();
        const DIGITS: &[(&str, u32)] = &[
            ("\u{0966}", 0),
            ("\u{0967}", 1),
            ("\u{0968}", 2),
            ("\u{0969}", 3),
            ("\u{096a}", 4),
            ("\u{096b}", 5),
            ("\u{096c}", 6),
            ("\u{096d}", 7),
            ("\u{096e}", 8),
            ("\u{096f}", 9),
        ];
        for (deva_key, value) in scheme.token_pairs() {
            for (digit, num) in DIGITS {
                if deva_key == digit {
                    numeral_to_int.insert(value.to_string(), *num);
                }
            }
        }

        // Also include Grantha powers of ten so that `transliterate` can detect that these are
        // numerals.
        if scheme == Scheme::Grantha {
            numeral_to_int.insert("௰".to_string(), 10);
            numeral_to_int.insert("௱".to_string(), 100);
            numeral_to_int.insert("௲".to_string(), 1000);
        }

        Self {
            to_scheme: scheme,
            data,
            numeral_to_int,
            virama,
            letter_a,
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

            let token_kind = SpanKind::from_devanagari_key(&deva_char);
            if self.to_scheme.is_alphabet() && token_kind == SpanKind::Consonant {
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

    pub(crate) fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.data.get(key)
    }

    #[allow(unused)]
    pub(crate) fn dump(&self) {
        let mut items: Vec<_> = self.data.iter().collect();
        items.sort_by(|x, y| x.0.cmp(y.0));
        for (k, vs) in items {
            let k_codes: Vec<_> = k.chars().map(|c| c as u32).collect();
            for v in vs {
                let v_codes: Vec<_> = v.chars().map(|c| c as u32).collect();
                println!("{k} ({k_codes:x?}) --> {v} ({v_codes:x?})");
            }
        }
    }
}

/// Defines a mapping between two schemes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mapping {
    pub(crate) from: Scheme,
    pub(crate) to: Scheme,
    pub(crate) all: FxHashMap<String, Span>,
    pub(crate) marks: FxHashMap<String, String>,

    pub(crate) from_map: OneWayMapping,
    pub(crate) to_map: OneWayMapping,

    pub(crate) numeral_to_int: FxHashMap<String, u32>,
    pub(crate) int_to_numeral: FxHashMap<u32, String>,

    tokens_by_first_char: FxHashMap<char, Vec<Span>>,
}

impl Mapping {
    /// Creates a mappping between the given `Scheme`s.
    ///
    /// ### Usage
    ///
    /// ```
    /// use vidyut_lipi::{Mapping, Scheme};
    /// use Scheme::*;
    ///
    /// let m = Mapping::new(HarvardKyoto, Devanagari);
    /// assert_eq!(m.from(), HarvardKyoto);
    /// assert_eq!(m.to(), Devanagari);
    /// ```
    ///
    /// ### Approach
    ///
    /// We start with two mappings: one from `A` to `X`, and one from `B` to `X`. Here `A` is our
    /// input scheme, `B` is our output scheme, and `X` is our intermediate representation.
    ///
    /// If we reverse our `B` to `X` mapping to get an `X` to `B` mapping, we can join these two
    /// mappings to get an `A` to `B` mapping. This approach is workable but needs extra support
    /// for these two cases:
    ///
    /// 1. A mapping `a --> x` without a corresponding `x --> b`. For example, consider `| --> ळ`,
    ///    where `|` is an SLP1 character and `ळ` is not defined in B. In this case, we
    ///
    /// 2. A mapping `x --> b` without a corresponding `a --> x`. For example, consider `ळ --> |`,
    ///    where `|` is again an SLP1 character and `ळ` is not defined in A. In this case, we
    ///    transliterate `x` to scheme `A` then programmatically create a new `a --> b` mapping.
    pub fn new(from: Scheme, to: Scheme) -> Mapping {
        // Since `new` is a public API, our parameter names use `from` and `to`. Internally, use
        // `a` and `b`, per our doc comments above.

        let a_map = OneWayMapping::new(from);
        let b_map = OneWayMapping::new(to);

        let mut all = FxHashMap::default();
        let mut marks = FxHashMap::default();
        let mut seen_b: FxHashSet<&str> = FxHashSet::default();

        // Iterate over `from.token_pairs()` so that we maintain a predictable input order.
        for (deva_key, _) in from.token_pairs() {
            // But, use the values in `a_map` instead of the values from `token_pairs` so that we
            // pick up Unicode equivalents.
            for a in a_map.get(deva_key).expect("present") {
                let bs = match b_map.get(deva_key) {
                    Some(bs) => bs,
                    None => continue,
                };
                let b = match bs.first() {
                    Some(b) => b,
                    None => continue,
                };

                let token_kind = SpanKind::from_devanagari_key(deva_key);
                if token_kind == SpanKind::VowelMark {
                    marks.insert(a.to_string(), b.to_string());
                }

                // Insert only the first match seen. Consequences:
                //
                // - If a sound maps to both a vowel and a vowel mark, we insert the vowel mark,
                //   which comes first in our representation.
                //
                // - If a sound has alternates, we store only the first.
                if !all.contains_key(a) {
                    all.insert(
                        a.to_string(),
                        Span::new(a.to_string(), b.to_string(), token_kind),
                    );
                    seen_b.insert(b);
                }
            }
        }

        for (deva_key, a) in from.token_pairs() {
            let token_kind = SpanKind::from_devanagari_key(deva_key);
            if !all.contains_key(*a) && b_map.get(deva_key).is_none() {
                // Mapping `a --> x` doesn't have a corresponding `x --> b`.
                // So, create one.
                let new_b = match b_map.transliterate_key(deva_key) {
                    Some(s) => s,
                    None => continue,
                };

                if token_kind == SpanKind::VowelMark {
                    marks.insert(a.to_string(), new_b.clone());
                }
                all.insert(a.to_string(), Span::new(a.to_string(), new_b, token_kind));
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

            let token_kind = SpanKind::from_devanagari_key(deva_key);

            if !new_a.is_empty() && !all.contains_key(&new_a) {
                if token_kind == SpanKind::VowelMark {
                    marks.insert(new_a.clone(), b.to_string());
                }
                all.insert(
                    new_a.to_string(),
                    Span::new(new_a.to_string(), b.to_string(), token_kind),
                );
            }
        }

        let mut int_to_numeral = FxHashMap::default();
        for (k, v) in &b_map.numeral_to_int {
            int_to_numeral.insert(*v, k.to_string());
        }
        // Take length in *chars*, not in *bytes*.
        // (Using chars over bytes offers a ~3x speedup in the core transliterate loop.)
        let numeral_to_int = a_map.numeral_to_int.clone();

        let mut tokens_by_first_char = FxHashMap::default();
        for t in all.values() {
            if let Some(first_char) = t.key.chars().next() {
                debug_assert!(!t.key.is_empty());
                tokens_by_first_char
                    .entry(first_char)
                    .or_insert(Vec::new())
                    .push(t.clone());
            }
        }

        Self {
            from,
            to,
            all,
            marks,
            from_map: a_map,
            to_map: b_map,
            numeral_to_int,
            int_to_numeral,
            tokens_by_first_char,
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

    pub(crate) fn get(&self, key: &str) -> Option<&Span> {
        self.all.get(key)
    }

    pub(crate) fn spans_starting_with(&self, c: char) -> &[Span] {
        match self.tokens_by_first_char.get(&c) {
            Some(v) => v,
            None => &[],
        }
    }

    /// Dumps this mapping's data to stdout.
    #[allow(unused)]
    pub(crate) fn dump(&self) {
        let mut items: Vec<_> = self.all.iter().collect();
        items.sort_by(|x, y| x.0.cmp(y.0));
        for (k, v) in items {
            let k_codes: Vec<_> = k.chars().map(|c| c as u32).collect();
            let v_codes: Vec<_> = v.value.chars().map(|c| c as u32).collect();
            println!("{k} ({k_codes:x?}) --> {} ({v_codes:x?})", v.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Scheme::*;

    #[test]
    fn test_decide_token_type() {
        let is_mark = |c| SpanKind::from_devanagari_key(c) == SpanKind::VowelMark;
        let is_consonant = |c| SpanKind::from_devanagari_key(c) == SpanKind::Consonant;
        let is_other = |c| SpanKind::from_devanagari_key(c) == SpanKind::Other;

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
    fn test_one_way_mapping_basic() {
        let itrans = OneWayMapping::new(Itrans);
        assert_eq!(itrans.data.get("ळ").unwrap(), &vec!["L"]);
        assert_eq!(itrans.data.get("\u{0916}\u{093c}").unwrap(), &vec!["K"]);
    }

    #[test]
    fn test_one_way_mapping_with_unicode_decompositions() {
        let itrans = OneWayMapping::new(Devanagari);
        // Map to both NFD and composed, preferring NFD.
        assert_eq!(
            itrans.data.get("\u{0916}\u{093c}").unwrap(),
            &vec!["\u{0916}\u{093c}", "\u{0959}"]
        );
    }

    #[test]
    fn test_one_way_mapping_transliterate_key() {
        let iast = OneWayMapping::new(Iast);
        assert_eq!(iast.transliterate_key("ळ्ह"), Some("ḻh".to_string()));

        let deva = OneWayMapping::new(Devanagari);
        assert_eq!(deva.transliterate_key("ळ्ह"), Some("ळ्ह".to_string()));
    }

    #[test]
    fn test_mapping() {
        let other = |x: &str, y: &str| Span::new(x.to_string(), y.to_string(), SpanKind::Other);
        let mark = |x: &str, y: &str| Span::new(x.to_string(), y.to_string(), SpanKind::VowelMark);

        let m = Mapping::new(Devanagari, Itrans);

        assert_eq!(m.from(), Devanagari);
        assert_eq!(m.to(), Itrans);

        let assert_has = |m: &Mapping, x: &str, y: &Span| {
            assert_eq!(m.get(x).unwrap(), y);
        };

        let m = Mapping::new(Devanagari, Itrans);
        assert_has(&m, "आ", &other("आ", "A"));
        assert_has(&m, "\u{093e}", &mark("\u{093e}", "A"));
        assert_has(&m, "ए", &other("ए", "e"));
        assert_has(&m, "\u{0947}", &mark("\u{0947}", "e"));

        let m = Mapping::new(Bengali, Itrans);
        assert_has(&m, "\u{09be}", &mark("\u{09be}", "A"));
        assert_has(&m, "\u{09c7}", &mark("\u{09c7}", "e"));
    }

    #[test]
    fn test_mapping_with_unicode_decompositions() {
        // Maps to NFD
        let m = Mapping::new(Velthuis, Devanagari);
        let cons = |x: &str, y: &str| Span::new(x.to_string(), y.to_string(), SpanKind::Consonant);
        assert_eq!(m.get("R").unwrap(), &cons("R", "\u{0921}\u{093c}"));
        assert_eq!(m.get("Rh").unwrap(), &cons("Rh", "\u{0922}\u{093c}"));

        // Maps from NFD and composed
        let m = Mapping::new(Devanagari, Velthuis);

        let deva = OneWayMapping::new(Devanagari);
        assert_eq!(
            deva.data.get("\u{0921}\u{093c}").unwrap(),
            &vec!["\u{0921}\u{093c}", "\u{095c}"]
        );

        let velthuis = OneWayMapping::new(Velthuis);
        assert_eq!(velthuis.data.get("\u{0921}\u{093c}").unwrap(), &vec!["R"]);
        assert_eq!(velthuis.data.get("\u{095c}"), None);

        assert_eq!(
            m.get("\u{0921}\u{093c}").unwrap(),
            &cons("\u{0921}\u{093c}", "R")
        );
        assert_eq!(m.get("\u{095c}").unwrap(), &cons("\u{095c}", "R"));
        assert_eq!(m.get("\u{095d}").unwrap(), &cons("\u{095d}", "Rh"));
    }
}
