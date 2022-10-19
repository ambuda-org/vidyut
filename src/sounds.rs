//! Utility functions for checking Sanskrit sounds.

use lazy_static::lazy_static;
use regex::Regex;

/// Returns whether the given character is a Sanskrit sound or *avagraha*.
///
/// We use this function to find boundaries between Sanskrit words. Non-Sanskrit sounds include:
/// - spaces
/// - other punctuation characters (|, ||, numbers)
/// - characters or symbols from non-SLP1 encodings
pub fn is_sanskrit(c: char) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"[aAiIuUfFxXeEoOkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL']").unwrap();
    }
    // Avoid `to_string`, which will create a new string on the heap.
    let mut buf = [0u8; 4];
    let s = c.encode_utf8(&mut buf);
    RE.is_match(s)
}

pub fn is_ac(c: char) -> bool {
    lazy_static! {
        // Matches all non-sounds at the beginning of the string.
        static ref RE: Regex = Regex::new(r"[aAiIuUfFxXeEoO]").unwrap();
    }
    let mut buf = [0u8; 4];
    let s = c.encode_utf8(&mut buf);
    RE.is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sanskrit() {
        for c in "aAiIuUfFxXeEoOkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL'".chars() {
            assert!(is_sanskrit(c));
        }
        for c in "0123456789,.![]|".chars() {
            assert!(!is_sanskrit(c));
        }
    }
}
