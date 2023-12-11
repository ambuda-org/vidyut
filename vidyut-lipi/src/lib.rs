//! Hacky transliteration functions that other crates might need.
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

use std::cmp;

/// Defines the available transliteration schemes.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Scheme {
    /// SlP1 transliteration.
    Slp1,
    /// IAST transliteration.
    Iast,
    /// Devanagari.
    Devanagari,
}

fn map_char(cur: &str) -> Option<&'static str> {
    let val = match cur {
        "ā" => "A",
        "ī" => "I",
        "ū" => "U",
        "ṛ" => "f",
        "ṝ" => "F",
        "ḷ" => "x",
        "ḹ" => "X",
        "ai" => "E",
        "au" => "O",
        "ṃ" => "M",
        "ḥ" => "H",
        "ṅ" => "N",
        "kh" => "K",
        "gh" => "G",
        "ch" => "C",
        "jh" => "J",
        "ñ" => "Y",
        "ṭ" => "w",
        "ṭh" => "W",
        "ḍ" => "q",
        "ḍh" => "Q",
        "th" => "T",
        "dh" => "D",
        "ph" => "P",
        "bh" => "B",
        "ṇ" => "R",
        "ś" => "S",
        "ṣ" => "z",
        "ḻ" => "L",
        &_ => return None,
    };
    Some(val)
}

/// Hackily transliterate from IAST to SLP1.
fn iast_to_slp1(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut ret = String::new();
    let mut i = 0;
    while i < chars.len() {
        let mut next: Option<&str> = None;
        let mut offset = 0;

        // Search for matches against our mapping. The longest IAST glyph has two characters,
        // so search up to length 2. Start with 2 first so that we match greedily.
        for j in [2, 1] {
            let limit = cmp::min(i + j, chars.len());
            let cur = String::from_iter(&chars[i..limit]);
            offset = limit - i;

            next = map_char(cur.as_str());
            if let Some(_s) = next {
                break;
            }
        }

        match next {
            Some(s) => {
                ret += s;
                i += offset;
            }
            None => {
                // Use the original character as-is.
                ret += &String::from_iter(&chars[i..=i]);
                i += 1;
            }
        }
    }
    ret
}

fn slp1_to_devanagari(text: &str) -> String {
    const VIRAMA: char = '\u{094D}';

    let mut ret = String::new();
    for c in text.chars() {
        let out = match c {
            'a' => "अ",
            'A' => "आ",
            'i' => "इ",
            'I' => "ई",
            'u' => "उ",
            'U' => "ऊ",
            'f' => "ऋ",
            'F' => "ॠ",
            'x' => "ऌ",
            'X' => "ॡ",
            'e' => "ए",
            'E' => "ऐ",
            'o' => "ओ",
            'O' => "औ",
            '~' => "\u{0901}",
            'M' => "\u{0902}",
            'H' => "\u{0903}",
            'k' => "क",
            'K' => "ख",
            'g' => "ग",
            'G' => "घ",
            'N' => "ङ",
            'c' => "च",
            'C' => "छ",
            'j' => "ज",
            'J' => "झ",
            'Y' => "ञ",
            'w' => "ट",
            'W' => "ठ",
            'q' => "ड",
            'Q' => "ढ",
            'R' => "ण",
            't' => "त",
            'T' => "थ",
            'd' => "द",
            'D' => "ध",
            'n' => "न",
            'p' => "प",
            'P' => "फ",
            'b' => "ब",
            'B' => "भ",
            'm' => "म",
            'y' => "य",
            'r' => "र",
            'l' => "ल",
            'v' => "व",
            'S' => "श",
            'z' => "ष",
            's' => "स",
            'h' => "ह",
            'L' => "ळ",
            other => {
                ret.push(other);
                continue;
            }
        };

        let vowel_mark = match c {
            'a' => Some(""),
            'A' => Some("\u{093E}"),
            'i' => Some("\u{093F}"),
            'I' => Some("\u{0940}"),
            'u' => Some("\u{0941}"),
            'U' => Some("\u{0942}"),
            'f' => Some("\u{0943}"),
            'F' => Some("\u{0944}"),
            'x' => Some("\u{0962}"),
            'X' => Some("\u{0963}"),
            'e' => Some("\u{0947}"),
            'E' => Some("\u{0948}"),
            'o' => Some("\u{094B}"),
            'O' => Some("\u{094C}"),
            _ => None,
        };

        if ret.chars().last() == Some(VIRAMA) && vowel_mark.is_some() {
            // Pop virama and add.
            ret.pop();
            ret += vowel_mark.expect("ok");
        } else {
            ret += out;
        }

        let is_consonant = "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL".contains(c);
        if is_consonant {
            ret.push(VIRAMA);
        }
    }
    ret
}

/// Transliterates the given input text.
///
/// ### Panics
///
/// Only the IAST -> SLP1 and SLP1 -> Devanagari mappings are defined. All other mappings will
/// panic.
pub fn transliterate(input: &str, from: Scheme, to: Scheme) -> String {
    use Scheme::*;
    if from == Iast && to == Slp1 {
        iast_to_slp1(input)
    } else if from == Slp1 && to == Devanagari {
        slp1_to_devanagari(input)
    } else {
        panic!("Unsupported scheme combination: {from:?} -> {to:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_slp1() {
        let t = |s| transliterate(s, Scheme::Iast, Scheme::Slp1);

        assert_eq!(t("a ā i ī u ū ṛ ṝ ḷ ḹ"), "a A i I u U f F x X");
        assert_eq!(t("e ai o au ṃ ḥ"), "e E o O M H");
        assert_eq!(t("k kh g gh ṅ"), "k K g G N");
        assert_eq!(t("c ch j jh ñ"), "c C j J Y");
        assert_eq!(t("ṭ ṭh ḍ ḍh ṇ"), "w W q Q R");
        assert_eq!(t("t th d dh n"), "t T d D n");
        assert_eq!(t("p ph b bh m"), "p P b B m");
        assert_eq!(t("y r l v"), "y r l v");
        assert_eq!(t("ś ṣ s h ḻ"), "S z s h L");

        assert_eq!(t("vāgarthāviva saṃpṛktau"), "vAgarTAviva saMpfktO");
    }
}
