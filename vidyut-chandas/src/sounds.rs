use lazy_static::lazy_static;

lazy_static! {
    static ref HRASVA: Set = Set::from("aiufx");
    static ref AC: Set = Set::from("aAiIuUfFxXeEoO");
    static ref HAL: Set = Set::from("kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL");
}

type Sound = char;

// A set of Sanskrit sounds.
//
// This implementation is copied directly from `vidyut_prakriya::sounds`. For details, see the
// comments there.
pub(crate) struct Set([u8; 256]);

impl Set {
    /// Creates an empty set.
    pub(crate) fn new() -> Self {
        Set([0; 256])
    }

    /// Creates a set whose members are the characters in `string`.
    pub(crate) fn from(string: impl AsRef<str>) -> Self {
        let mut res = Self::new();
        for c in string.as_ref().chars() {
            res.0[c as usize] = 1;
        }
        res
    }

    // Returns whether the set contains the given sound.
    pub(crate) fn contains(&self, c: Sound) -> bool {
        self.0[c as usize] == 1
    }
}

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
    AC.contains(c) || HAL.contains(c) || matches!(c, 'M' | 'H')
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
