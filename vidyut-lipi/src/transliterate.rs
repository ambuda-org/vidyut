use crate::scheme::Scheme;
use rustc_hash::FxHashMap;

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
struct OneWayMapping {
    // Maps from Devanagari to all options available in the given scheme.
    data: FxHashMap<String, Vec<String>>,
    virama: String,
}

impl OneWayMapping {
    fn from_scheme(scheme: Scheme) -> Self {
        const VIRAMA: &str = "\u{094d}";
        let mut data = FxHashMap::default();
        let mut virama = String::new();

        for (key, value) in scheme.token_pairs() {
            let key = key.to_string();
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
        Self { data, virama }
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

impl Mapping {
    /// Creates a mappping between the given `Scheme`s.
    pub fn new(from_scheme: Scheme, to_scheme: Scheme) -> Mapping {
        let from = OneWayMapping::from_scheme(from_scheme);
        let to = OneWayMapping::from_scheme(to_scheme);

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

    fn get(&self, key: &str) -> Option<&String> {
        self.all.get(key)
    }
}

/// Transliterates from an abugida.
fn transliterate_from_abugida(input: &str, mapping: &Mapping) -> String {
    let chars: Vec<char> = input.chars().collect();
    let is_to_alpha = mapping.to.is_alphabet();

    let mut output = String::new();
    let mut i = 0;
    let mut key = String::new();
    let mut had_consonant = false;
    while i < chars.len() {
        let mut o: Option<&String> = None;

        let mut key_len_in_chars = 0;
        for len_key in (1..=mapping.len_longest_key).rev() {
            let j = std::cmp::min(i + len_key, chars.len());
            key.clear();
            key.extend(&chars[i..j]);
            key_len_in_chars = j - i;

            o = mapping.get(&key);
            if o.is_some() {
                break;
            }
        }

        match o {
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

        // Add length in *chars*, not in *bytes*. Otherwise we get weird output.
        debug_assert!(key_len_in_chars > 0);
        i += key_len_in_chars;
    }

    output
}

/// Transliterates from an alphabet.
fn transliterate_from_alphabet(input: &str, mapping: &Mapping) -> String {
    let chars: Vec<char> = input.chars().collect();
    let is_to_abugida = mapping.to.is_abugida();
    let is_itrans = mapping.from == Scheme::Itrans;

    let mut output = String::new();
    let mut i = 0;
    let mut key = String::new();
    let mut had_consonant = false;
    while i < chars.len() {
        let mut o: Option<&String> = None;

        let mut key_len_in_chars = 0;
        for len_key in (1..=mapping.len_longest_key).rev() {
            let j = std::cmp::min(i + len_key, chars.len());
            key.clear();
            key.extend(&chars[i..j]);
            key_len_in_chars = j - i;

            o = mapping.get(&key);
            if o.is_some() {
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
                // ITRANS: `\` skips the next character.
                if is_itrans && chars[i] == '\\' {
                    if let Some(c) = chars.get(i + 1) {
                        output.push(*c);
                    }
                    i += 2;
                    continue;
                }

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

/// Transliterates the given input text using the given `Mapping`.
///
/// For most use cases, we recommend using the API on `Lipika` instead.
pub fn transliterate(input: impl AsRef<str>, mapping: &Mapping) -> String {
    if mapping.from.is_abugida() {
        transliterate_from_abugida(input.as_ref(), mapping)
    } else {
        transliterate_from_alphabet(input.as_ref(), mapping)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mapping = Mapping::new(Scheme::HarvardKyoto, Scheme::Devanagari);
        let t = |s| transliterate(s, &mapping);
        assert_eq!(t("namaskRtya"), "नमस्कृत्य");
    }
}
