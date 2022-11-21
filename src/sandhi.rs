//! Splits Sanskrit expressions according to a list of sandhi rules. Our splitting algorithm is
//! naive but exhaustive.

use crate::sounds;
use lazy_static::lazy_static;
use multimap::MultiMap;
use regex::Regex;
use std::cmp;
use std::error::Error;
use std::path::Path;

/// Maps a combination to the two strings (first, second) that created it.
pub type SandhiMap = MultiMap<String, (String, String)>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SplitKind {
    /// A split created by slicing the input string, with no sandhi rules applied. As a result,
    /// `split.first` is a **prefix** of the original string.
    Prefix,
    /// A split created by undoing a specific sandhi rule.
    Standard,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Split {
    pub first: String,
    pub second: String,
    pub is_end_of_chunk: bool,
    pub kind: SplitKind,
}

impl Split {
    /// Returns whether a given sandhi split is OK according to some basic heuristics.
    ///
    /// Our sandhi splitting logic overgenerates, and some of its outputs are not phonetically valid.
    /// For most use cases, we recommend filtering the results of `split` with this function.
    pub fn is_valid(&self) -> bool {
        is_good_first(&self.first) && is_good_second(&self.second)
    }

    /// Returns whether this split would lead to infinite recursion.
    ///
    /// We avoid sandhi splits like `AnandaH -> a AnandaH` because they are recursive; if we accept
    /// them, we would likewise accept `a AnandaH -> a a AnandaH` and so on.
    pub fn is_recursive(&self, remaining: &str) -> bool {
        self.second == remaining
    }
}

pub struct Sandhi {
    map: MultiMap<String, (String, String)>,
    len_longest_key: usize,
}

impl Sandhi {
    pub fn from_map(map: SandhiMap) -> Self {
        let len_longest_key = map
            .keys()
            .map(|x| x.len())
            .max()
            .expect("Sandhi map is empty");
        Self {
            map,
            len_longest_key,
        }
    }

    /// Creates a map from sandhi combinations to the sounds that created them.
    ///
    /// # Arguments
    ///
    /// - `path` - C TSV with columns `first`, `second`, `result`, and `type`.
    pub fn from_csv(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut rules = SandhiMap::new();

        let mut rdr = csv::Reader::from_path(path)?;
        for maybe_row in rdr.records() {
            let row = maybe_row?;
            let first = String::from(&row[0]);
            let second = String::from(&row[1]);
            let result = String::from(&row[2]);
            let type_ = &row[3];
            if type_ == "internal" {
                continue;
            }

            rules.insert(result.clone(), (first.clone(), second.clone()));

            let result_no_spaces = String::from(&row[2]).replace(' ', "");
            if result_no_spaces != result {
                rules.insert(result_no_spaces, (first.clone(), second.clone()));
            }
        }
        Ok(Sandhi::from_map(rules))
    }

    /// Yield all possible ways to split `input` at the given index `i`.
    ///
    /// The `first` field in the split is guaranteed to be non-empty.
    pub fn split_at(&self, input: &str, i: usize) -> Vec<Split> {
        let mut res = Vec::new();

        // First, push the default split where no sandhi rule is applied.
        //
        // We mark this split separately so that callers can match on it and handle it differently.
        // For example, if `input[..i]` is not in a prefix tree of Sanskrit words, then for all j >
        // 1, `input[..j]` will likewise not be in the tree. This principle does not apply for
        // "normal" sandhi splits.
        res.push(Split {
            first: input[..i + 1].to_string(),
            // Trim leading whitespace on the remainder for consistency later on.
            second: input[i + 1..].trim_start().to_string(),
            // We are at the end of a chunk if and only if the next sound is not a Sanskrit sound
            // (or and avagraha).
            is_end_of_chunk: !&input[i + 1..].starts_with(sounds::is_sanskrit),
            kind: SplitKind::Prefix,
        });

        // Special case for `sa` and `eza`.
        let first = &input[..i + 1];
        if first == "sa" || first == "eza" {
            let second = input[i + 1..].trim_start().to_string();
            res.push(Split {
                first: first.to_string() + "s",
                second,
                is_end_of_chunk: true,
                kind: SplitKind::Standard,
            });
        }

        // Also consider the special case where we are at the end of the input. Here, we want to_string
        // allow a word-final visarga to be either "s" or "r".
        // FIXME: this is a hack.
        if i + 1 == input.len() && input.ends_with('H') {
            res.push(Split {
                first: visarga_to_s(input),
                second: "".to_string(),
                is_end_of_chunk: true,
                kind: SplitKind::Standard,
            });
            res.push(Split {
                first: visarga_to_r(input),
                second: "".to_string(),
                is_end_of_chunk: true,
                kind: SplitKind::Standard,
            });
        }

        for j in i..cmp::min(input.len(), i + self.len_longest_key + 1) {
            let combination = &input[i..j];
            let is_end_of_chunk = combination.contains(' ');

            if let Some(pairs) = self.map.get_vec(combination) {
                for (f, s) in pairs {
                    let first = String::from(&input[0..i]) + f;
                    let mut second = String::from(s) + &input[j..];

                    // Trim leading whitespace since it might still be present in the remaining
                    // text (e.g. "kaH sa" where the rule is "H" -> ("s", "").
                    // FIXME: think through this behavior carefully and simplify it.
                    second = second.trim_start().to_string();

                    res.push(Split {
                        first,
                        second,
                        is_end_of_chunk,
                        kind: SplitKind::Standard,
                    });
                }
            }
        }
        res
    }

    /// Temporary function until we migrate to split_at everywhere.
    pub fn split_all(&self, input: &str) -> Vec<Split> {
        let mut splits = Vec::new();
        for i in 0..input.len() {
            // Break on non-sounds so that `first` is a continuous chunk.
            if !input[i..].starts_with(sounds::is_sanskrit) {
                break;
            }
            splits.extend(self.split_at(input, i));
        }
        splits
    }
}

/// Hackily converts a word ending with a visarga to end with an `s`.
fn visarga_to_s(s: &str) -> String {
    let n = s.len();
    String::from(&s[0..n - 1]) + "s"
}

/// Hackily converts a word ending with a visarga to end with an `r`.
fn visarga_to_r(s: &str) -> String {
    let n = s.len();
    String::from(&s[0..n - 1]) + "r"
}

/// Returns whether the first item in a sandhi split is OK according to some basic heuristics.
fn is_good_first(text: &str) -> bool {
    lazy_static! {
        // Must not end with a double vowel.
        static ref RE_DOUBLE_AC: Regex = Regex::new(r"[aAiIuUfFxXeEoO]{2}$").unwrap();
        // Must not end with a double consonant (exceptions: yrlv).
        static ref RE_DOUBLE_HAL: Regex = Regex::new(
            // non-yaN + hal
            r"[kKgGNcCjJYwWqQRtTdDnpPbBmzSsh][kKgGNcCjJYwWqQRtTdDnpPbBmyrlvzSsh]$").unwrap();
    }

    if RE_DOUBLE_AC.is_match(text) || RE_DOUBLE_HAL.is_match(text) {
        false
    } else {
        match text.chars().last() {
            // Vowels, standard consonants, and "s" and "r"
            Some(c) => "aAiIuUfFxXeEoOHkNwRtpnmsr".contains(c),
            None => true,
        }
    }
}

/// Returns whether the second item in a sandhi split is OK according to some basic heuristics.
fn is_good_second(text: &str) -> bool {
    lazy_static! {
        // Initial yrlv must not be followed by sparsha.
        static ref RE_YAN: Regex = Regex::new(r"^[yrlv][kKgGNcCjJYwWqQRtTdDnpPbBm]").unwrap();
        // Must not start with a double vowel.
        static ref RE_DOUBLE_AC: Regex = Regex::new(r"^[aAiIuUfFxXeEoO]{2}").unwrap();
    }
    if RE_DOUBLE_AC.is_match(text) {
        // "afRin" is acceptable, but skip otherwise.
        text.starts_with("afR")
    } else {
        !RE_YAN.is_match(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use multimap::multimap;

    fn splits(items: Vec<(&str, &str, bool, SplitKind)>) -> Vec<Split> {
        items
            .iter()
            .map(|(f, s, c, k)| Split {
                first: f.to_string(),
                second: s.to_string(),
                is_end_of_chunk: *c,
                kind: k.clone(),
            })
            .collect()
    }

    #[test]
    fn test_visarga_to_s() {
        assert_eq!(visarga_to_s("naraH"), "naras".to_string());
    }

    #[test]
    fn test_split_at_start_of_chunk() {
        let rules = multimap![
            "ar".to_string() => ("a".to_string(), "f".to_string()),
            "ar".to_string() => ("a".to_string(), "F".to_string()),
            "ar".to_string() => ("A".to_string(), "f".to_string()),
            "ar".to_string() => ("A".to_string(), "F".to_string()),
        ];
        let sandhi = Sandhi {
            map: rules,
            len_longest_key: 2,
        };

        let expected: Vec<Split> = splits(vec![
            ("a", "rka", false, SplitKind::Prefix),
            ("a", "fka", false, SplitKind::Standard),
            ("a", "Fka", false, SplitKind::Standard),
            ("A", "fka", false, SplitKind::Standard),
            ("A", "Fka", false, SplitKind::Standard),
        ]);

        assert_eq!(sandhi.split_at("arka", 0), expected);
    }

    #[test]
    fn test_split_at_middle_of_chunk() {
        let rules = multimap![
            "e".to_string() => ("a".to_string(), "i".to_string()),
            "e".to_string() => ("a".to_string(), "I".to_string()),
            "e".to_string() => ("A".to_string(), "i".to_string()),
            "e".to_string() => ("A".to_string(), "I".to_string()),
        ];
        let sandhi = Sandhi {
            map: rules,
            len_longest_key: 1,
        };

        let expected: Vec<Split> = splits(vec![
            ("ce", "ti", false, SplitKind::Prefix),
            ("ca", "iti", false, SplitKind::Standard),
            ("ca", "Iti", false, SplitKind::Standard),
            ("cA", "iti", false, SplitKind::Standard),
            ("cA", "Iti", false, SplitKind::Standard),
        ]);

        assert_eq!(sandhi.split_at("ceti", 1), expected);
    }

    #[test]
    fn test_split_at_end_of_chunk_trims_whitespace() {
        let rules = multimap![
            "o".to_string() => ("a".to_string(), "u".to_string()),
        ];
        let sandhi = Sandhi {
            map: rules,
            len_longest_key: 1,
        };

        let expected: Vec<Split> = splits(vec![("nare", "ca", true, SplitKind::Prefix)]);

        assert_eq!(sandhi.split_at("nare ca", 3), expected);
    }

    #[test]
    fn test_split_at_end_of_input() {
        let rules = multimap![
            "e".to_string() => ("a".to_string(), "i".to_string()),
        ];
        let sandhi = Sandhi {
            map: rules,
            len_longest_key: 1,
        };

        let expected: Vec<Split> = splits(vec![
            ("devaH", "", true, SplitKind::Prefix),
            ("devas", "", true, SplitKind::Standard),
            ("devar", "", true, SplitKind::Standard),
        ]);

        assert_eq!(sandhi.split_at("devaH", 4), expected);
    }

    #[test]
    fn test_is_good_first() {
        for word in vec![
            "rAma", "rAjA", "iti", "nadI", "maDu", "gurU", "pitf", "F", "laBate", "vE", "aho",
            "narO", "naraH", "vAk", "rAw", "prAN", "vit", "narAn", "anuzWup", "naram",
        ] {
            assert!(is_good_first(word));
        }
        for word in &["PalaM", "zaz", "vAc"] {
            assert!(!is_good_first(word));
        }
    }

    #[test]
    fn test_is_good_second() {
        for word in &[
            "yogena",
            "rAma",
            "leKaH",
            "vE",
            "kArtsnyam",
            "vraja",
            "vyajanam",
            "afRin",
        ] {
            assert!(is_good_second(word));
        }
        for word in &["rmakzetre", "lga", "aitad", "fasya", "Fasya"] {
            assert!(!is_good_second(word), "failed: {}", word);
        }
    }
}
