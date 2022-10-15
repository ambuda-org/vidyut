use lazy_static::lazy_static;
use regex::Regex;

#[allow(dead_code)]
pub fn is_sanskrit(c: char) -> bool {
    lazy_static! {
        // Matches all non-sonuds at the beginning of the string.
        static ref RE: Regex = Regex::new(r"[aAiIuUfFxXeEoOkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL']").unwrap();
    }
    RE.is_match(&String::from(c))
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
