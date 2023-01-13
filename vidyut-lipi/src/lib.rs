#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]

//! Hacky transliteration functions that other crates might need.
use std::cmp;

/// Defines the available transliteration schemes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scheme {
    /// SlP1 transliteration.
    Slp1,
    /// IAST transliteration.
    Iast,
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

/// Transliterates the given input text.
///
/// Only the IAST -> SLP1 mapping is defined. All other mappings will panic.
pub fn transliterate(input: &str, from: Scheme, to: Scheme) -> String {
    assert!(from == Scheme::Iast);
    assert!(to == Scheme::Slp1);
    iast_to_slp1(input)
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
