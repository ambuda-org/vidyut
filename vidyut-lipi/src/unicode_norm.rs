//! Logic for Unicode normalization.
//!
//! Certain sequences of Unicode code points are *equivalent* to each other, meaning that they
//! express essentially the same character. `vidyut-lipi` should be aware of these equivalences,
//! and at a minimum, it should transliterate equivalent Unicode sequences in the same way.
//!
//! `vidyut-lipi` concerns itself with two standard methods of representing Unicode sequences:
//!
//! - *Normalization Form Canonical Composition* (NFC)
//! - *Normalization Form Canonical Decomposition* (NFD)
//!
//!
//! ### Why roll our own code?
//!
//! The canonical crate for working with these representations is [unicode_normalization][1], but
//! the crate is rather large, and we use a small subset of its data for transliteration. So
//! instead, we use our own code for normalizing. (We test our implementation against
//! `unicode_normalization` directly and expect to have equal behavior in our core functions.)
//!
//! Rough size estimates, as of 2024-01-22:
//!
//! - Code without NFC/NFD logic :       ~ 129 KiB wasm
//! - Code with this module:             ~ 134 KiB wasm
//! - Code with `unicode_normalization`: ~ 248 KiB wasm
//!
//! [1]: https://docs.rs/unicode-normalization/latest/unicode_normalization/

use crate::scheme::Scheme;
use rustc_hash::FxHashMap;
use unicode_normalization::UnicodeNormalization;

type Table = &'static [(&'static str, &'static str)];

/// NFD/NFC mapping for Latin.
pub const LATIN_NFD: Table = &[
    // C1 Controls and Latin-1 Supplement (https://unicode.org/charts/PDF/U0080.pdf)
    ("\u{00d1}", "N\u{0303}"), // Ñ
    ("\u{00e8}", "e\u{0300}"), // è
    ("\u{00ea}", "e\u{0302}"), // ê
    ("\u{00f1}", "n\u{0303}"), // ñ
    ("\u{00f2}", "o\u{0300}"), // ò
    ("\u{00f4}", "o\u{0302}"), // ô
    // Latin Extended-A (https://unicode.org/charts/PDF/U0100.pdf)
    ("\u{0100}", "A\u{0304}"), // Ā
    ("\u{0101}", "a\u{0304}"), // ā
    ("\u{0112}", "E\u{0304}"), // Ē
    ("\u{0113}", "e\u{0304}"), // ē
    ("\u{0120}", "G\u{0307}"), // Ġ
    ("\u{0121}", "g\u{0307}"), // ġ
    ("\u{012a}", "I\u{0304}"), // Ī
    ("\u{012b}", "i\u{0304}"), // ī
    ("\u{014c}", "O\u{0304}"), // Ō
    ("\u{014d}", "o\u{0304}"), // ō
    ("\u{015a}", "S\u{0301}"), // Ś
    ("\u{015b}", "s\u{0301}"), // ś
    ("\u{016a}", "U\u{0304}"), // Ū
    ("\u{016b}", "u\u{0304}"), // ū
    ("\u{017c}", "z\u{0307}"), // ż
    ("\u{017e}", "z\u{030c}"), // ž
    // Latin Extended Additional (https://unicode.org/charts/PDF/U1E00.pdf)
    ("\u{1e0c}", "D\u{0323}"),         // Ḍ
    ("\u{1e0d}", "d\u{0323}"),         // ḍ
    ("\u{1e24}", "H\u{0323}"),         // Ḥ
    ("\u{1e25}", "h\u{0323}"),         // ḥ
    ("\u{1e2b}", "h\u{032e}"),         // ḫ
    ("\u{1e32}", "K\u{0323}"),         // Ḳ
    ("\u{1e33}", "k\u{0323}"),         // ḳ
    ("\u{1e36}", "L\u{0323}"),         // Ḷ
    ("\u{1e37}", "l\u{0323}"),         // ḷ
    ("\u{1e38}", "L\u{0323}\u{0304}"), // Ḹ
    ("\u{1e39}", "l\u{0323}\u{0304}"), // ḹ
    ("\u{1e3a}", "L\u{0331}"),         // Ḻ
    ("\u{1e3b}", "l\u{0331}"),         // ḻ
    ("\u{1e40}", "M\u{0307}"),         // Ṁ
    ("\u{1e41}", "m\u{0307}"),         // ṁ
    ("\u{1e42}", "M\u{0323}"),         // Ṃ
    ("\u{1e43}", "m\u{0323}"),         // ṃ
    ("\u{1e44}", "N\u{0307}"),         // Ṅ
    ("\u{1e45}", "n\u{0307}"),         // ṅ
    ("\u{1e46}", "N\u{0323}"),         // Ṇ
    ("\u{1e47}", "n\u{0323}"),         // ṇ
    ("\u{1e48}", "N\u{0331}"),         // Ṉ
    ("\u{1e49}", "n\u{0331}"),         // ṉ
    ("\u{1e5a}", "R\u{0323}"),         // Ṛ
    ("\u{1e5b}", "r\u{0323}"),         // ṛ
    ("\u{1e5c}", "R\u{0323}\u{0304}"), // Ṝ
    ("\u{1e5d}", "r\u{0323}\u{0304}"), // ṝ
    ("\u{1e5e}", "R\u{0331}"),         // Ṟ
    ("\u{1e5f}", "r\u{0331}"),         // ṟ
    ("\u{1e62}", "S\u{0323}"),         // Ṣ
    ("\u{1e63}", "s\u{0323}"),         // ṣ
    ("\u{1e6c}", "T\u{0323}"),         // Ṭ
    ("\u{1e6d}", "t\u{0323}"),         // ṭ
    ("\u{1e89}", "w\u{0323}"),         // ẉ
    ("\u{1e8e}", "Y\u{0307}"),         // Ẏ
    ("\u{1e8f}", "y\u{0307}"),         // ẏ
    ("\u{1e93}", "z\u{0323}"),         // ẓ
    ("\u{1e95}", "z\u{0331}"),         // ž
    ("\u{1e96}", "h\u{0331}"),         // ẖ
];

/// NFD/NFC mapping for Devanagari.
/// Spec: <https://unicode.org/charts/PDF/U0900.pdf>
///
/// (other Devanagari NFC/NFD combinations are usually exempt.)
pub const DEVANAGARI_NFD: Table = &[
    ("\u{0929}", "\u{0928}\u{093c}"), // na
    ("\u{0931}", "\u{0930}\u{093c}"), // ra
    ("\u{0934}", "\u{0933}\u{093c}"), // La
    ("\u{0958}", "\u{0915}\u{093c}"), // ka
    ("\u{0959}", "\u{0916}\u{093c}"), // kha
    ("\u{095a}", "\u{0917}\u{093c}"), // ga
    ("\u{095b}", "\u{091c}\u{093c}"), // ja
    ("\u{095c}", "\u{0921}\u{093c}"), // Da
    ("\u{095d}", "\u{0922}\u{093c}"), // Dha
    ("\u{095e}", "\u{092b}\u{093c}"), // pha
    ("\u{095f}", "\u{092f}\u{093c}"), // ya
];

/// Characters that should not be created during NFD --> NFC.
#[cfg(target_arch = "wasm32")]
pub const DEVANAGARI_COMPOSITION_EXCLUSIONS: &[&str] = &[
    "\u{0958}", // ka
    "\u{0959}", // kha
    "\u{095a}", // ga
    "\u{095b}", // ja
    "\u{095c}", // Da
    "\u{095d}", // Dha
    "\u{095e}", // pha
    "\u{095f}", // ya
];

/// NFD/NFC mapping for Bengali.
/// Spec: <https://unicode.org/charts/PDF/U0980.pdf>
pub const BENGALI_NFD: Table = &[
    ("\u{09cb}", "\u{09c7}\u{09be}"), // vowel sign o
    ("\u{09cc}", "\u{09c7}\u{09d7}"), // vowel sign au
    ("\u{09dc}", "\u{09a1}\u{09bc}"), // letter rra
    ("\u{09dd}", "\u{09a2}\u{09bc}"), // letter rha
    ("\u{09df}", "\u{09af}\u{09bc}"), // letter yya
];

/// Characters that should not be created during NFD --> NFC.
#[cfg(target_arch = "wasm32")]
pub const BENGALI_COMPOSITION_EXCLUSIONS: &[&str] = &["\u{09dc}", "\u{09dd}", "\u{09df}"];

/// Spec: <https://unicode.org/charts/PDF/U1000.pdf>
pub const MYANMAR_NFD: Table = &[
    ("\u{1026}", "\u{1025}\u{102e}"), // uu
];

/// NFD/NFC mapping for Balinese.
/// Spec: <https://unicode.org/charts/PDF/U1B00.pdf>
pub const BALINESE_NFD: Table = &[
    ("\u{1b06}", "\u{1b05}\u{1b35}"),
    ("\u{1b08}", "\u{1b07}\u{1b35}"),
    ("\u{1b0a}", "\u{1b09}\u{1b35}"),
    ("\u{1b0c}", "\u{1b0b}\u{1b35}"),
    ("\u{1b0e}", "\u{1b0d}\u{1b35}"),
    ("\u{1b12}", "\u{1b11}\u{1b35}"),
    ("\u{1b3b}", "\u{1b3a}\u{1b35}"),
    ("\u{1b3d}", "\u{1b3c}\u{1b35}"),
    ("\u{1b40}", "\u{1b3e}\u{1b35}"),
    ("\u{1b41}", "\u{1b3f}\u{1b35}"),
];

/// Spec: <http://www.unicode.org/charts/PDF/U11300.pdf>
pub const GRANTHA_NFD: Table = &[
    ("\u{1134b}", "\u{11347}\u{1133e}"), // vowel sign oo
    ("\u{1134c}", "\u{11347}\u{11357}"), // vowel sign au
];

/// Spec: <https://unicode.org/charts/PDF/U0A00.pdf>
pub const GURMUKHI_NFD: Table = &[
    ("\u{0a33}", "\u{0a32}\u{0a3c}"), // letter lla
    ("\u{0a36}", "\u{0a38}\u{0a3c}"), // letter sha
    ("\u{0a59}", "\u{0a16}\u{0a3c}"), // letter khha
    ("\u{0a5a}", "\u{0a17}\u{0a3c}"), // letter ghha
    ("\u{0a5b}", "\u{0a1c}\u{0a3c}"), // letter za
    ("\u{0a5e}", "\u{0a2b}\u{0a3c}"), // letter fa
];

/// Spec: <https://unicode.org/charts/PDF/U0A00.pdf>
#[cfg(target_arch = "wasm32")]
pub const GURMUKHI_COMPOSITION_EXCLUSIONS: &[&str] = &[
    "\u{0a33}", "\u{0a36}", "\u{0a59}", "\u{0a5a}", "\u{0a5b}", "\u{0a5e}",
];

/// Spec: <https://unicode.org/charts/PDF/U0C80.pdf>
pub const KANNADA_NFD: Table = &[
    ("\u{0cc0}", "\u{0cbf}\u{0cd5}"),        // vowel sign ii
    ("\u{0cc7}", "\u{0cc6}\u{0cd5}"),        // vowel sign ee
    ("\u{0cc8}", "\u{0cc6}\u{0cd6}"),        // vowel sign ai
    ("\u{0cca}", "\u{0cc6}\u{0cc2}"),        // vowel sign o
    ("\u{0ccb}", "\u{0cc6}\u{cc2}\u{0cd5}"), // vowel sign oo
];

/// Spec: <https://unicode.org/charts/PDF/U0D00.pdf>
pub const MALAYALAM_NFD: Table = &[
    ("\u{0d4a}", "\u{0d46}\u{0d3e}"), // vowel sign o
    ("\u{0d4b}", "\u{0d47}\u{0d3e}"), // vowel sign oo
    ("\u{0d4c}", "\u{0d46}\u{0d57}"), // vowel sign au
];

/// Spec: <https://unicode.org/charts/PDF/U0B00.pdf>
pub const ORIYA_NFD: Table = &[
    ("\u{0b48}", "\u{0b47}\u{0b56}"), // vowel sign ai
    ("\u{0b4b}", "\u{0b47}\u{0b3e}"), // vowel sign o
    ("\u{0b4c}", "\u{0b47}\u{0b57}"), // vowel sign au
    ("\u{0b5c}", "\u{0b21}\u{0b3c}"), // letter rra
    ("\u{0b5d}", "\u{0b22}\u{0b3c}"), // letter rha
];

#[cfg(target_arch = "wasm32")]
pub const ORIYA_COMPOSITION_EXCLUSIONS: &[&str] = &["\u{0b5c}", "\u{0b5d}"];

/// Spec: <https://unicode.org/charts/PDF/U11580.pdf>
pub const SIDDHAM_NFD: Table = &[
    ("\u{115ba}", "\u{115b8}\u{115af}"), // vowel sign o
    ("\u{115bb}", "\u{115b9}\u{115af}"), // vowel sign au
];

/// Spec: <https://unicode.org/charts/PDF/U0D80.pdf>
pub const SINHALA_NFD: Table = &[
    ("\u{0dda}", "\u{0dd9}\u{0dca}"),         // vowel sign ee
    ("\u{0ddc}", "\u{0dd9}\u{0dcf}"),         // vowel sign o
    ("\u{0ddd}", "\u{0dd9}\u{0dcf}\u{0dca}"), // vowel sign oo
    ("\u{0dde}", "\u{0dd9}\u{0ddf}"),         // vowel sign au
];

/// Spec: <https://unicode.org/charts/PDF/U0B80.pdf>
pub const TAMIL_NFD: Table = &[
    ("\u{0b94}", "\u{0b92}\u{0bd7}"), // letter au
    ("\u{0bca}", "\u{0bc6}\u{0bbe}"), // vowel sign o
    ("\u{0bcb}", "\u{0bc7}\u{0bbe}"), // vowel sign oo
    ("\u{0bcc}", "\u{0bc6}\u{0bd7}"), // vowel sign au
];

/// Spec: <https://unicode.org/charts/PDF/U0C00.pdf>
pub const TELUGU_NFD: Table = &[
    ("\u{0c48}", "\u{0c46}\u{0c56}"), // vowel sign ai
];

pub const KAITHI_NFD: Table = &[
    ("\u{1109a}", "\u{11099}\u{110ba}"), // Letter dddha
    ("\u{1109c}", "\u{1109b}\u{110ba}"), // Letter rha
    ("\u{110ab}", "\u{110a5}\u{110ba}"), // Letter va
];

/// Spec: <https://www.unicode.org/charts/PDF/U11480.pdf>
pub const TIRHUTA_NFD: Table = &[
    ("\u{114bb}", "\u{114b9}\u{114ba}"), // vowel sign ai
    ("\u{114bc}", "\u{114b9}\u{114b0}"), // vowel sign o
    ("\u{114be}", "\u{114b9}\u{114bd}"), // vowel sign au
];

/// Converts `s` to its NFC representation.
///
/// Only characters that appear in one of our `Scheme`s will be converted. All other characters
/// will be left as-is.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn to_nfc(s: &str) -> String {
    // `.collect()` seems not to pre-allocate.
    //
    // Result: 8.27s --> 7.89s (-5%)
    let mut ret = String::with_capacity(s.len());
    for c in s.nfc() {
        ret.push(c);
    }
    ret
}

/// WASM-only version of `to_nfc`.
///
/// The `unicode_normalization` implementation of this logic is substantially faster (which
/// motivates using it in non-WASM builds) but also much larger (which motivates avoiding it in
/// WASM builds).
#[cfg(target_arch = "wasm32")]
pub(crate) fn to_nfc(s: &str) -> String {
    let mut map = FxHashMap::default();
    let mut len_longest_key = 0;
    for scheme in Scheme::iter() {
        for (nfc, nfd) in scheme.unicode_nfd_pairs() {
            if scheme.unicode_composition_exclusions().contains(nfc) {
                continue;
            }
            map.insert(nfd.to_string(), nfc.to_string());
            len_longest_key = std::cmp::max(len_longest_key, nfd.chars().count());
        }
    }

    let nfd = to_nfd(s);
    let nfd_chars: Vec<_> = nfd.chars().collect();

    let mut ret = String::new();
    let mut key = String::new();
    let mut o: Option<&String> = None;
    let mut i = 0;
    while i < nfd_chars.len() {
        let mut key_len_in_chars = 0;
        for len_key in (1..=len_longest_key).rev() {
            let j = std::cmp::min(i + len_key, nfd_chars.len());
            key.clear();
            key.extend(&nfd_chars[i..j]);
            key_len_in_chars = j - i;

            o = map.get(&key);
            if o.is_some() {
                break;
            }
        }

        match o {
            Some(s) => ret.push_str(s),
            None => ret.push(nfd_chars[i]),
        }

        i += key_len_in_chars;
    }

    ret
}

/// Converts the given string to its NFD representation.
///
/// Our version of `to_nfd` supports only those characters that are part of a `Scheme`. All other
/// characters are left unchanged.
#[allow(unused)]
pub(crate) fn to_nfd(s: &str) -> String {
    let mut map: FxHashMap<String, String> = FxHashMap::default();

    for scheme in Scheme::iter() {
        for (nfc, nfd) in scheme.unicode_nfd_pairs() {
            map.insert(nfc.to_string(), nfd.to_string());
        }
    }

    let mut ret = String::new();
    for c in s.chars() {
        let mut temp = [0u8; 4];
        let char_str = c.encode_utf8(&mut temp);
        if let Some(nfd) = map.get(char_str) {
            ret.push_str(nfd)
        } else {
            ret.push_str(char_str);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scheme;
    use unicode_normalization::UnicodeNormalization;

    #[test]
    fn test_to_nfc() {
        for scheme in Scheme::iter() {
            println!("Testing: {scheme:?}");
            for (_, token) in scheme.token_pairs() {
                let expected: String = token.nfc().collect();
                let actual = to_nfc(&token);
                let t_codes: Vec<_> = token.chars().map(|c| c as u32).collect();
                let e_codes: Vec<_> = expected.chars().map(|c| c as u32).collect();
                let a_codes: Vec<_> = actual.chars().map(|c| c as u32).collect();
                assert_eq!(
                    expected, actual,
                    "NFC of {scheme:?} token `{token}` ({t_codes:x?}) should be `{expected}` ({e_codes:x?}), was `{actual}` ({a_codes:x?})"
                );
            }
        }
    }

    #[test]
    fn test_to_nfd() {
        for scheme in Scheme::iter() {
            println!("Testing: {scheme:?}");
            for (_, token) in scheme.token_pairs() {
                let expected: String = token.nfd().collect();
                let actual = to_nfd(&token);
                let t_codes: Vec<_> = token.chars().map(|c| c as u32).collect();
                let e_codes: Vec<_> = expected.chars().map(|c| c as u32).collect();
                let a_codes: Vec<_> = actual.chars().map(|c| c as u32).collect();
                assert_eq!(
                    expected, actual,
                    "NFD of {scheme:?} token `{token}` ({t_codes:x?}) should be `{expected}` ({e_codes:x?}), was `{actual}` ({a_codes:x?})"
                );
            }
        }
    }
}
