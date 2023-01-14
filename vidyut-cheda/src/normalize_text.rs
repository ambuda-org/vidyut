use lazy_static::lazy_static;
use regex::Regex;

/// Creates a normalized version of `text` that is easier to process.
///
/// We normalize as follows:
/// 1. Separate all input into three kinds of spans: text, whitespace, and symbols.
/// 2. Delete all whitespace spans.
/// 3. Separate all remaining spans with a single " ".
pub fn normalize(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"([a-zA-Z']+)|(\s+)|([^a-zA-Z']+)").expect("always defined");
    }

    let mut ret = RE
        .find_iter(text)
        .map(|m| m.as_str())
        .filter(|s| !s.trim().is_empty())
        .fold(String::new(), |s, x| s + x + " ");
    ret.pop();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let cases = vec![
            ("devaH", "devaH"),
            ("devo gacchati", "devo gacchati"),
            ("devo 'gniH", "devo 'gniH"),
        ];
        for (input, output) in cases {
            assert_eq!(normalize(input), output.to_string());
        }
    }

    #[test]
    fn test_whitespace() {
        let cases = vec![
            ("      deva iti", "deva iti"),
            ("deva iti      ", "deva iti"),
            ("  deva   iti  ", "deva iti"),
            ("  deva\t\niti ", "deva iti"),
        ];
        for (input, output) in cases {
            assert_eq!(normalize(input), output.to_string());
        }
    }

    #[test]
    fn test_punctuation() {
        let cases = vec![
            ("deva!", "deva !"),
            ("deva--iti", "deva -- iti"),
            (":deva12345iti!", ": deva 12345 iti !"),
            (":deva  12345iti!", ": deva 12345 iti !"),
        ];
        for (input, output) in cases {
            assert_eq!(normalize(input), output.to_string());
        }
    }
}
