use multimap::MultiMap;
use regex::Regex;
use std::cmp;
use std::error::Error;

type SandhiMap = MultiMap<String, (String, String)>;

pub fn read_rules(tsv_path: &str) -> Result<SandhiMap, Box<dyn Error>> {
    let mut rules = MultiMap::new();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(tsv_path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let first = String::from(&row[0]);
        let second = String::from(&row[1]);

        let result = String::from(&row[2]);
        rules.insert(result.clone(), (first.clone(), second.clone()));

        let result_no_spaces = String::from(&row[2]).replace(" ", "");
        if result_no_spaces != result {
            rules.insert(result_no_spaces, (first.clone(), second.clone()));
        }
    }
    Ok(rules)
}

pub fn split(input: &str, rules: SandhiMap) -> Vec<(String, String)> {
    let mut res = Vec::new();
    let len_longest_key = rules.keys().map(|x| x.len()).max().expect("Map is empty");
    let len_input = input.len();

    // When iterating, prefer making the first item as long as possible, as longer
    // items are easier to rule out.
    for i in (1..=len_input).rev() {
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
                        res.push((first, second))
                    }
                }
                None => continue,
            }
        }
    }
    res
}

fn is_good_first(text: &str) -> bool {
    match text.chars().last() {
        // Vowels, standard consonants, and "s" and "r"
        Some(c) => "aAiIuUfFxXeEoOHkNwRtpnmsr".contains(c),
        None => true,
    }
}

fn is_good_second(text: &str) -> bool {
    // Initial yrlv must not be followed by sparsha.
    let r = Regex::new(r"^[yrlv][kKgGNcCjJYwWqQRtTdDnpPbBm]").unwrap();
    return !r.is_match(text);
}

pub fn is_good_split(text: &str, first: &str, second: &str) -> bool {
    let is_recursive = text == second;
    is_good_first(&first) && is_good_second(&second) && !is_recursive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let mut rules = SandhiMap::new();
        rules.insert("e".to_string(), ("a".to_string(), "i".to_string()));
        let expected: Vec<(String, String)> = vec![
            ("ceti", ""),
            ("cet", "i"),
            ("ce", "ti"),
            ("c", "eti"),
            ("ca", "iti"),
        ]
        .iter()
        .map(|&(f, s)| (f.to_string(), s.to_string()))
        .collect();

        assert_eq!(split("ceti", rules), expected);
    }

    #[test]
    fn test_is_good_first() {
        for word in vec![
            "rAma", "rAjA", "iti", "nadI", "maDu", "gurU", "pitf", "F", "laBate", "vE", "aho",
            "narO", "naraH", "vAk", "rAw", "prAN", "vit", "narAn", "anuzWup", "naram",
        ] {
            assert!(is_good_first(word));
        }
        for word in vec!["PalaM", "zaz", "vAc"] {
            assert!(!is_good_first(word));
        }
    }

    #[test]
    fn test_has_valid_start() {
        for word in vec![
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
        for word in vec!["rmakzetre"] {
            assert!(!is_good_second(word));
        }
    }
}
