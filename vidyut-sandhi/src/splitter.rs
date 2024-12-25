/*!
Utilities for undoing sandhi changes.

*Sandhi* refers to any of the various sound changes that Sanskrit words and morphemes undergo when
they are in contact with each other. For example, the two words *ca iti* would typically combine
into the single chunk *ceti*.

Sandhi is based not only on a term's sounds but also on a term's morphological properties. For
example, the two words *te iti* could either change to *ta iti* or remain as *te iti* depending on
the grammatical number of the first word.

The full set of sandhi changes is complex and elaborate. So to simplify our problem, we deal solely
with the common changes that occur between two words.
*/

use crate::errors::Result;
use crate::sounds;
use crate::sounds::Set;
use compact_str::CompactString;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use std::cmp;
use std::collections::hash_map::Keys;
use std::path::Path;

/// Describes the type of sandhi split that occurred.
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Kind {
    /// A split created by slicing the input string, with no sandhi rules applied. That is,
    /// `split.first` is a *prefix* of the original string.
    Prefix,
    /// A split created by undoing a specific sandhi rule.
    Standard,
}

/// Describes the type of sandhi split that occurred.
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Location {
    /// Indicates that the split occurs within a chunk.
    WithinChunk,
    /// Indicates that the split occurs across a chunk boundary.
    EndOfChunk,
}

/// Models a sandhi split.
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Split {
    first: CompactString,
    second: String,
    location: Location,
    kind: Kind,
}

impl Split {
    /// Creates a new split.
    pub fn new(first: String, second: String, location: Location, kind: Kind) -> Self {
        Self {
            first: CompactString::from(first),
            second,
            location,
            kind,
        }
    }
    /// The first half of the split.
    pub fn first(&self) -> &str {
        &self.first
    }

    /// The second half of the split.
    pub fn second(&self) -> &str {
        &self.second
    }

    /// Whether this split crosses a chunk boundary.
    pub fn is_end_of_chunk(&self) -> bool {
        self.location == Location::EndOfChunk
    }

    /// The kind of split.
    pub fn kind(&self) -> Kind {
        self.kind
    }

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

/// Maps a combination to the two strings (first, second) that created it.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct SplitsMap(FxHashMap<String, Vec<(String, String)>>);

impl SplitsMap {
    /// Creates an empty `SplitsMap`.
    pub fn new() -> Self {
        SplitsMap(FxHashMap::default())
    }

    /// Inserts the given (`key`, `value`) rule.
    pub fn insert(&mut self, key: String, value: (String, String)) {
        if !self.0.contains_key(&key) {
            self.0.insert(key.to_string(), vec![]);
        }
        self.0.get_mut(&key).expect("present").push(value);
    }

    /// Returns an iterator over all keys in the map.
    pub fn keys(&self) -> Keys<'_, String, Vec<(String, String)>> {
        self.0.keys()
    }

    /// Returns all available splits for the given `key`.
    pub fn get_vec(&self, key: &str) -> Option<&Vec<(String, String)>> {
        self.0.get(key)
    }
}

/// Splits Sanskrit words and expressions according to the specified rules.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Splitter {
    map: SplitsMap,
    len_longest_key: usize,
}

impl Splitter {
    /// Creates a splitter from the given map.
    ///
    /// # Example
    ///
    /// ```
    /// use vidyut_sandhi::{Splitter, SplitsMap};
    ///
    /// let mut map: SplitsMap = SplitsMap::new();
    /// map.insert("e".to_string(), ("a".to_string(), "i".to_string()));
    ///
    /// let s: Splitter = Splitter::from_map(map);
    /// ```
    pub fn from_map(map: SplitsMap) -> Self {
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
    /// - `path` - a CSV with columns `first`, `second`, and `result`.
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Use this ugly error matching so that downstream error messages can be more legible.
        let mut rdr = match csv::Reader::from_path(path) {
            Ok(rdr) => rdr,
            Err(e) => {
                if e.is_io_error() {
                    if let csv::ErrorKind::Io(e) = e.into_kind() {
                        return Err(crate::Error::Io(e));
                    } else {
                        panic!("Should be unreachable -- see `is_io_error` docs.");
                    }
                } else {
                    return Err(crate::Error::Csv(e));
                }
            }
        };

        let mut rules = SplitsMap::new();
        for maybe_row in rdr.records() {
            let row = maybe_row?;
            let first = String::from(&row[0]);
            let second = String::from(&row[1]);
            let result = String::from(&row[2]);

            rules.insert(result.clone(), (first.clone(), second.clone()));

            let result_no_spaces = String::from(&row[2]).replace(' ', "");
            if result_no_spaces != result {
                rules.insert(result_no_spaces, (first.clone(), second.clone()));
            }
        }
        if rules.0.is_empty() {
            Err(crate::Error::EmptyFile)
        } else {
            Ok(Splitter::from_map(rules))
        }
    }

    /// Yields all possible ways to split `input` at the given index `i`.
    ///
    /// The `first` field in the split is guaranteed to be non-empty.
    ///
    /// # Example
    ///
    /// ```
    /// use vidyut_sandhi::{Splitter, SplitsMap};
    ///
    /// let mut map: SplitsMap = SplitsMap::new();
    /// map.insert("e".to_string(), ("a".to_string(), "i".to_string()));
    /// let s: Splitter = Splitter::from_map(map);
    ///
    /// let input = "ceti";
    /// for split in s.split_at(input, 1) {
    ///   println!("{} -> {} {}", input, split.first(), split.second());
    /// }
    /// ```
    pub fn split_at(&self, input: &str, i: usize) -> Vec<Split> {
        let mut res = Vec::new();

        // First, push the default split where no sandhi rule is applied.
        //
        // We mark this split separately so that callers can match on it and handle it differently.
        // For example, if `input[..i]` is not in a prefix tree of Sanskrit words, then for all j >
        // 1, `input[..j]` will likewise not be in the tree. This principle does not apply for
        // "normal" sandhi splits.
        res.push(Split {
            first: CompactString::from(&input[..i + 1]),
            // Trim leading whitespace on the remainder for consistency later on.
            second: input[i + 1..].trim_start().to_string(),
            // We are at the end of a chunk if and only if the next sound is not a Sanskrit sound
            // (or and avagraha).
            location: if input[i + 1..].starts_with(sounds::is_sanskrit) {
                Location::WithinChunk
            } else {
                Location::EndOfChunk
            },
            kind: Kind::Prefix,
        });

        // Special case for `sa` and `eza`.
        let first = &input[..i + 1];
        if first == "sa" || first == "eza" {
            let second = input[i + 1..].trim_start().to_string();
            res.push(Split {
                first: CompactString::from(first) + "s",
                second,
                location: Location::EndOfChunk,
                kind: Kind::Standard,
            });
        }

        // Also consider the special case where we are at the end of the input. Here, we want to_string
        // allow a word-final visarga to be either "s" or "r".
        // FIXME: this is a hack.
        if i + 1 == input.len() && input.ends_with('H') {
            res.push(Split {
                first: visarga_to_s(input),
                second: "".to_string(),
                location: Location::EndOfChunk,
                kind: Kind::Standard,
            });
            res.push(Split {
                first: visarga_to_r(input),
                second: "".to_string(),
                location: Location::EndOfChunk,
                kind: Kind::Standard,
            });
        }

        for j in i..cmp::min(input.len(), i + self.len_longest_key + 1) {
            let combination = &input[i..j];
            let location = if combination.contains(' ') {
                Location::EndOfChunk
            } else {
                Location::WithinChunk
            };

            if let Some(pairs) = self.map.get_vec(combination) {
                for (f, s) in pairs {
                    let first = CompactString::from(&input[0..i]) + f;
                    let mut second = String::from(s) + &input[j..];

                    // Trim leading whitespace since it might still be present in the remaining
                    // text (e.g. "kaH sa" where the rule is "H" -> ("s", "").
                    // FIXME: think through this behavior carefully and simplify it.
                    second = second.trim_start().to_string();

                    res.push(Split {
                        first,
                        second,
                        location,
                        kind: Kind::Standard,
                    });
                }
            }
        }
        res
    }

    /// Finds all splits of the given string.
    #[warn(deprecated)]
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
fn visarga_to_s(s: &str) -> CompactString {
    let n = s.len();
    CompactString::from(&s[0..n - 1]) + "s"
}

/// Hackily converts a word ending with a visarga to end with an `r`.
fn visarga_to_r(s: &str) -> CompactString {
    let n = s.len();
    CompactString::from(&s[0..n - 1]) + "r"
}

/// Returns whether the first item in a sandhi split is OK according to some basic heuristics.
fn is_good_first(text: &str) -> bool {
    lazy_static! {
        static ref AC: Set = Set::from("aAiIuUfFxXeEoO");
        static ref SPARSHA: Set = Set::from("kKgGNcCjJYwWqQRtTdDnpPbBm");
        static ref HAL: Set = Set::from("kKgGNcCjJYwWqQRtTdDnpPbBmyrlvzSsh");
        // Vowels, standard consonants, and "s" and "r"
        static ref VALID_FINALS: Set = Set::from("aAiIuUfFxXeEoOHkNwRtpnmsr");
    }
    let mut chars = text.chars().rev();
    if let (Some(y), Some(x)) = (chars.next(), chars.next()) {
        if (AC.contains(x) && AC.contains(y)) || (HAL.contains(x) && HAL.contains(y)) {
            return false;
        }
    }
    match text.chars().last() {
        Some(c) => VALID_FINALS.contains(c),
        None => true,
    }
}

/// Returns whether the second item in a sandhi split is OK according to some basic heuristics.
fn is_good_second(text: &str) -> bool {
    lazy_static! {
        static ref YAN: Set = Set::from("yrlv");
        static ref AC: Set = Set::from("aAiIuUfFxXeEoO");
        static ref SPARSHA: Set = Set::from("kKgGNcCjJYwWqQRtTdDnpPbBm");
    }
    let mut chars = text.chars();
    if let (Some(x), Some(y)) = (chars.next(), chars.next()) {
        if AC.contains(x) && AC.contains(y) {
            // Must not start with a double vowel.
            // But, "afRin" is acceptable.
            text.starts_with("afR")
        } else {
            // Initial yrlv must not be followed by sparsha.
            !(YAN.contains(x) && SPARSHA.contains(y))
        }
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn splits(items: Vec<(&str, &str, Location, Kind)>) -> Vec<Split> {
        items
            .iter()
            .map(|(f, s, location, kind)| Split {
                first: CompactString::from(*f),
                second: s.to_string(),
                location: *location,
                kind: *kind,
            })
            .collect()
    }

    #[test]
    fn test_visarga_to_s() {
        assert_eq!(visarga_to_s("naraH"), "naras".to_string());
    }

    #[test]
    fn test_split_at_start_of_chunk() {
        let mut rules = SplitsMap::new();
        rules.insert("ar".to_string(), ("a".to_string(), "f".to_string()));
        rules.insert("ar".to_string(), ("a".to_string(), "F".to_string()));
        rules.insert("ar".to_string(), ("A".to_string(), "f".to_string()));
        rules.insert("ar".to_string(), ("A".to_string(), "F".to_string()));

        let sandhi = Splitter {
            map: rules,
            len_longest_key: 2,
        };

        let expected: Vec<Split> = splits(vec![
            ("a", "rka", Location::WithinChunk, Kind::Prefix),
            ("a", "fka", Location::WithinChunk, Kind::Standard),
            ("a", "Fka", Location::WithinChunk, Kind::Standard),
            ("A", "fka", Location::WithinChunk, Kind::Standard),
            ("A", "Fka", Location::WithinChunk, Kind::Standard),
        ]);

        assert_eq!(sandhi.split_at("arka", 0), expected);
    }

    #[test]
    fn test_split_at_middle_of_chunk() {
        let mut rules = SplitsMap::new();
        rules.insert("e".to_string(), ("a".to_string(), "i".to_string()));
        rules.insert("e".to_string(), ("a".to_string(), "I".to_string()));
        rules.insert("e".to_string(), ("A".to_string(), "i".to_string()));
        rules.insert("e".to_string(), ("A".to_string(), "I".to_string()));

        let sandhi = Splitter {
            map: rules,
            len_longest_key: 1,
        };

        let expected: Vec<Split> = splits(vec![
            ("ce", "ti", Location::WithinChunk, Kind::Prefix),
            ("ca", "iti", Location::WithinChunk, Kind::Standard),
            ("ca", "Iti", Location::WithinChunk, Kind::Standard),
            ("cA", "iti", Location::WithinChunk, Kind::Standard),
            ("cA", "Iti", Location::WithinChunk, Kind::Standard),
        ]);

        assert_eq!(sandhi.split_at("ceti", 1), expected);
    }

    #[test]
    fn test_split_at_end_of_chunk_trims_whitespace() {
        let mut rules = SplitsMap::new();
        rules.insert("o".to_string(), ("a".to_string(), "u".to_string()));

        let sandhi = Splitter {
            map: rules,
            len_longest_key: 1,
        };

        let expected: Vec<Split> = splits(vec![("nare", "ca", Location::EndOfChunk, Kind::Prefix)]);

        assert_eq!(sandhi.split_at("nare ca", 3), expected);
    }

    #[test]
    fn test_split_at_end_of_input() {
        let mut rules = SplitsMap::new();
        rules.insert("e".to_string(), ("a".to_string(), "i".to_string()));

        let sandhi = Splitter {
            map: rules,
            len_longest_key: 1,
        };

        let expected: Vec<Split> = splits(vec![
            ("devaH", "", Location::EndOfChunk, Kind::Prefix),
            ("devas", "", Location::EndOfChunk, Kind::Standard),
            ("devar", "", Location::EndOfChunk, Kind::Standard),
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
