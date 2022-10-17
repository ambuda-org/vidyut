//! Splits Sanskrit expressions according to a list of sandhi rules. Our splitting algorithm is
//! naive but exhaustive.

use lazy_static::lazy_static;
use multimap::MultiMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::error::Error;

/// Maps a combination to the two strings (first, second) that created it.
pub type SandhiMap = MultiMap<String, (String, String)>;

#[derive(Serialize, Deserialize)]
pub struct Sandhi {
    map: MultiMap<String, (String, String)>,
}

impl Sandhi {
    pub fn from_map(map: SandhiMap) -> Sandhi {
        Sandhi { map }
    }

    /// Creates a map from sandhi combinations to the sounds that created them.
    ///
    /// # Arguments
    ///
    /// - `path` - C TSV with columns `first`, `second`, `result`, and `type`.
    pub fn from_csv(path: &str) -> Result<Self, Box<dyn Error>> {
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
        Ok(Sandhi { map: rules })
    }

    pub fn split(&self, raw_input: &str) -> Vec<(String, String)> {
        split_sandhi(raw_input, &self.map)
    }
}

/// Hackily converts a word ending with a visarga to end with an `s`.
fn visarga_to_s(s: &str) -> String {
    let n = s.len();
    String::from(&s[0..n - 1]) + "s"
}

/// Yield all possible splits (a, b) that can be made on `raw_input` with `rules`.
fn split_sandhi(raw_input: &str, rules: &SandhiMap) -> Vec<(String, String)> {
    lazy_static! {
        // Matches all non-sonuds at the beginning of the string.
        static ref RE_NOT_SOUND: Regex = Regex::new(r"^[^a-zA-Z]+").unwrap();
    }
    let mut res = Vec::new();
    let len_longest_key = rules.keys().map(|x| x.len()).max().expect("Map is empty");

    // Sanitize by removing leading chars that aren't Sanskrit sounds.
    let input = RE_NOT_SOUND.replace(raw_input, "");
    let len_input = input.len();

    // Order is not important here, since downstream logic will reorder the results here based on
    // each item's score.
    for i in 1..len_input {
        // Chunk boundary -- return.
        let cur_char = &input[i - 1..i];
        if RE_NOT_SOUND.is_match(cur_char) {
            // HACK for visarga
            return res;
        }

        // Default: split as-is, no sandhi.
        res.push((
            String::from(&input[0..i]),
            String::from(&input[i..len_input]),
        ));

        for j in i..cmp::min(len_input, i + len_longest_key + 1) {
            let combination = &input[i..j];
            // println!("{}-{} : {}", i, j, combination);
            match rules.get_vec(combination) {
                Some(pairs) => {
                    for (f, s) in pairs {
                        let first = String::from(&input[0..i]) + f;
                        let second = String::from(s) + &input[j..len_input];

                        if first.ends_with('H') {
                            res.push((visarga_to_s(&first), second.clone()));
                        }
                        res.push((first, second))
                    }
                }
                None => continue,
            }
        }
    }

    // If we reached this line, then the input is one big chunk. So, include that chunk as-is in
    // case the chunk is a singnle word.
    res.push((input.to_string(), "".to_string()));
    // HACK for visarga
    if input.ends_with('H') {
        res.push((visarga_to_s(&input), "".to_string()));
    }
    res
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

/// Returns whether a given sandhi split is OK according to some basic heuristics.
///
/// Our sandhi splitting logic overgenerates, and some of its outputs are not phonetically valid.
/// For most use cases, we recommend filtering the results of `split` with this function.
pub fn is_good_split(text: &str, first: &str, second: &str) -> bool {
    // To avoid recursion, require that `second` is not just a repeat of the inital state.
    let is_recursive = text == second;
    is_good_first(first) && is_good_second(second) && !is_recursive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visarga_to_s() {
        assert_eq!(visarga_to_s("naraH"), "naras".to_string());
    }

    #[test]
    fn test_split_single_chunk() {
        let mut rules = SandhiMap::new();
        rules.insert("e".to_string(), ("a".to_string(), "i".to_string()));
        let expected: Vec<(String, String)> = vec![
            ("c", "eti"),
            ("ca", "iti"),
            ("ce", "ti"),
            ("cet", "i"),
            ("ceti", ""),
        ]
        .iter()
        .map(|&(f, s)| (f.to_string(), s.to_string()))
        .collect();

        assert_eq!(split_sandhi("ceti", &rules), expected);
    }

    #[test]
    fn test_split_two_chunks() {
        let mut dummy = SandhiMap::new();
        dummy.insert("e".to_string(), ("a".to_string(), "i".to_string()));

        assert!(
            split_sandhi("aham iti", &dummy).contains(&("aham".to_string(), " iti".to_string()))
        );
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
