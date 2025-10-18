pub(crate) use vidyut_akshara::{Set, Sound};

static HRASVA: Set = Set::from("aiufx");
pub const AC: Set = Set::from("aAiIuUfFxXeEoO");
pub const HAL: Set = Set::from("kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL");

/// Returns whether `c` is a vowel.
pub(crate) fn is_ac(c: Sound) -> bool {
    AC.contains(c)
}

/// Returns whether `c` is a short vowel.
pub(crate) fn is_hrasva(c: Sound) -> bool {
    HRASVA.contains(c)
}

/// Returns whether `c` is a consonant.
pub(crate) fn is_hal(c: Sound) -> bool {
    HAL.contains(c)
}

/// Returns whether `c` is a Sanskrit sound.
pub(crate) fn is_sanskrit(c: Sound) -> bool {
    is_ac(c) || is_hal(c) || matches!(c, 'M' | 'H')
}

/// Returns whether `s` starts with a consonant cluster.
pub(crate) fn is_samyogadi(s: &str) -> bool {
    let mut chars = s.chars();
    if let (Some(x), Some(y)) = (chars.next(), chars.next()) {
        is_hal(x) && is_hal(y)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AC: &str = "aAiIuUfFxXeEoO";
    const HAL: &str = "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL";
    const HRASVA: &str = "aiufx";

    #[test]
    fn test_sounds() {
        for c in AC.chars() {
            assert!(is_sanskrit(c));
            assert!(is_ac(c));
            assert!(!is_hal(c));
        }

        for c in HAL.chars() {
            assert!(is_sanskrit(c));
            assert!(is_hal(c));
            assert!(!is_ac(c));
        }

        for c in HRASVA.chars() {
            assert!(is_ac(c));
            assert!(is_hrasva(c));
        }
    }

    #[test]
    fn test_samyogadi() {
        assert!(is_samyogadi("kra"));
        assert!(!is_samyogadi("ka"));
        assert!(!is_samyogadi("a"));
    }
}
