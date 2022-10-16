use lazy_static::lazy_static;
/// Splits Sanskrit expressions according to a list of sandhi rules.
use multimap::MultiMap;
use regex::Regex;
use std::cmp;

pub type SandhiMap = MultiMap<String, (String, String)>;

fn visarga_to_s(s: &str) -> String {
    let n = s.len();
    String::from(&s[0..n - 1]) + "s"
}

/// Returns all possible splits for the given input.
pub fn split(raw_input: &str, rules: &SandhiMap) -> Vec<(String, String)> {
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
    match text.chars().last() {
        // Vowels, standard consonants, and "s" and "r"
        Some(c) => "aAiIuUfFxXeEoOHkNwRtpnmsr".contains(c),
        None => true,
    }
}

/// Returns whether the second item in a sandhi split is OK according to some basic heuristics.
fn is_good_second(text: &str) -> bool {
    lazy_static! {
        // Initial yrlv must not be followed by sparsha.
        static ref RE: Regex = Regex::new(r"^[yrlv][kKgGNcCjJYwWqQRtTdDnpPbBm]").unwrap();
    }
    !RE.is_match(text)
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

        assert_eq!(split("ceti", &rules), expected);
    }

    #[test]
    fn test_split_two_chunks() {
        let mut dummy = SandhiMap::new();
        dummy.insert("e".to_string(), ("a".to_string(), "i".to_string()));

        assert!(split("aham iti", &dummy).contains(&("aham".to_string(), " iti".to_string())));
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
    fn test_has_valid_start() {
        for word in &[
            "yogena",
            "rAma",
            "leKaH",
            "vE",
            "kArtsnyam",
            "vraja",
            "vyajanam",
        ] {
            assert!(is_good_second(word));
        }
        for word in &["rmakzetre", "lga"] {
            assert!(!is_good_second(word));
        }
    }
}
