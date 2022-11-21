//! Utility functions for checking Sanskrit sounds.

use lazy_static::lazy_static;
use regex::Regex;

fn match_char(c: &char, re: &Regex) -> bool {
    // Avoid `to_string`, which will create a new string on the heap.
    let mut buf = [0u8; 4];
    let s = c.encode_utf8(&mut buf);
    re.is_match(s)
}

/// Returns whether the given character is a Sanskrit sound or *avagraha*.
///
/// We use this function to find boundaries between Sanskrit words. Non-Sanskrit sounds include:
/// - spaces
/// - other punctuation characters (|, ||, numbers)
/// - characters or symbols from non-SLP1 encodings
pub fn is_sanskrit(c: char) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"[aAiIuUfFxXeEoOMHkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL']").unwrap();
    }
    match_char(&c, &RE)
}

pub fn is_ac(c: char) -> bool {
    lazy_static! {
        // Matches all non-sounds at the beginning of the string.
        static ref RE: Regex = Regex::new(r"[aAiIuUfFxXeEoO]").unwrap();
    }
    match_char(&c, &RE)
}

pub fn is_ghosha(c: char) -> bool {
    lazy_static! {
        // Matches all voiced sounds the beginning of the string.
        static ref RE: Regex = Regex::new(r"[aAiIuUfFxXeEoOgGNjJYqQRdDnbBmyrlvh]").unwrap();
    }
    match_char(&c, &RE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sanskrit() {
        for c in "aAiIuUfFxXeEoOMHkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL'".chars() {
            assert!(is_sanskrit(c));
        }
        for c in "0123456789,.![]|".chars() {
            assert!(!is_sanskrit(c));
        }
    }

    #[test]
    fn test_is_ac() {
        for c in "aAiIuUfFxXeEoO".chars() {
            assert!(is_ac(c));
        }
        for c in "kKgGnSzsh0123456789 '+".chars() {
            assert!(!is_ac(c));
        }
    }

    #[test]
    fn test_is_ghosha() {
        for c in "aAiIuUfFxXeEoOgGnjJYqQRdDnbBmyrlvh".chars() {
            assert!(is_ghosha(c));
        }
        for c in "kKcCwWtTpPSzs".chars() {
            assert!(!is_ghosha(c));
        }
    }
}
