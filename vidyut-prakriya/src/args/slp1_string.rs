use crate::core::errors::{Error, Result};
use crate::sounds::{AC, AL};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// `SanskritString` is a validated `String` that contains only ASCII characters that conform to
/// the SLP1 encoding for Sanskrit text. Specifically, it makes the following guarantees:
///
/// - All characters in the string are ASCII.
/// - Specifically, all characters are part of the SLP1 encoding scheme. All characters not in the
///   list provided below, including whitespace, will cause an error.
/// - `~` must follow a vowel.
/// - `\\` and `^` must follow either a vowel or `~`.
///
/// List of accepted SLP1 characters:
///
/// - `a` -- *a*
/// - `A` -- *ā*
/// - `i` -- *i*
/// - `I` -- *ī*
/// - `u` -- *u*
/// - `U` -- *ū*
/// - `f` -- *r̥*
/// - `F` -- *r̥̄*
/// - `x` -- *l̥*
/// - `X` -- *l̥̄*
/// - `e` -- *ē*
/// - `E` -- *ai*
/// - `o` -- *ō*
/// - `O` -- *au*
/// - `M` -- *ṁ* (*anusvāra*)
/// - `H` -- *ḥ* (*visarga*)
/// - `k` -- *k*
/// - `K` -- *kh*
/// - `g` -- *g*
/// - `G` -- *gh*
/// - `N` -- *ṅ*
/// - `c` -- *c*
/// - `C` -- *ch*
/// - `j` -- *j*
/// - `J` -- *jh*
/// - `Y` -- *ñ*
/// - `w` -- *ṭ*
/// - `W` -- *ṭh*
/// - `q` -- *ḍ*
/// - `Q` -- *ḍh*
/// - `R` -- *ṇ*
/// - `t` -- *t*
/// - `T` -- *th*
/// - `d` -- *d*
/// - `D` -- *dh*
/// - `n` -- *n*
/// - `p` -- *p*
/// - `P` -- *ph*
/// - `b` -- *b*
/// - `B` -- *bh*
/// - `m` -- *m*
/// - `y` -- *y*
/// - `r` -- *r*
/// - `l` -- *l*
/// - `v` -- *v*
/// - `S` -- *ś*
/// - `z` -- *ṣ*
/// - `s` -- *s*
/// - `h` -- *h*
/// - `\\` -- *anudātta* on the preceding vowel
/// - `^` -- *svarita* on the preceding vowel
/// - `~` -- *anunāsikatva* on the preceding vowel.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Slp1String(pub(crate) String);

impl Slp1String {
    /// Tries to initialize a new `Slp1String`.
    ///
    /// ### Usage
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// use vidyut_prakriya::args::*;
    ///
    /// let bhu = Slp1String::from("BU")?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn from(text: impl AsRef<str>) -> Result<Self> {
        Self::from_inner(text.as_ref())
    }

    fn from_inner(text: &str) -> Result<Self> {
        let bytes = text.as_bytes();
        for (i, c) in text.char_indices() {
            if !c.is_ascii() {
                return Err(Error::ParseError(format!("Char `{c}` is not ASCII")));
            }

            if !AL.contains(c) && !matches!(c, '\\' | '^' | '~' | 'M' | 'H') {
                return Err(Error::ParseError(format!(
                    "Char `{c}` is ASCII but not valid SLP1."
                )));
            }

            if matches!(c, '\\' | '^' | '~') {
                if i == 0 {
                    return Err(Error::ParseError(format!(
                        "Char `{c}` must follow a vowel but is the first char in the string."
                    )));
                }

                let prev = bytes[i - 1] as char;
                let prev_is_vowel = AC.contains(prev);
                if c == '~' && !prev_is_vowel {
                    return Err(Error::ParseError(format!(
                        "Char `~` must follow a vowel but follows `{prev}`."
                    )));
                } else if !prev_is_vowel && prev != '~' {
                    return Err(Error::ParseError(format!(
                        "Char `{c}` must follow a vowel or `~` but follows `{prev}`."
                    )));
                }
            }
        }

        Ok(Self(String::from(text)))
    }

    // Accessors (experimental)

    #[allow(unused)]
    pub(crate) fn get(&self, i: usize) -> Option<char> {
        self.0.as_bytes().get(i).map(|x| *x as char)
    }

    #[allow(unused)]
    pub(crate) fn get_rev(&self, i: usize) -> Option<char> {
        self.0.bytes().rev().nth(i).map(|x| x as char)
    }

    #[allow(unused)]
    pub(crate) fn adi(&self) -> Option<char> {
        self.0.bytes().next().map(|x| x as char)
    }

    #[allow(unused)]
    pub(crate) fn antya(&self) -> Option<char> {
        self.0.bytes().last().map(|x| x as char)
    }

    #[allow(unused)]
    pub(crate) fn upadha(&self) -> Option<char> {
        self.get_rev(1)
    }

    #[allow(unused)]
    pub(crate) fn has_antya(&self, c: char) -> bool {
        assert!(c.is_ascii());
        self.0.bytes().last() == Some(c as u8)
    }
}

impl std::ops::Deref for Slp1String {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for Slp1String {
    type Error = Error;
    fn try_from(val: String) -> Result<Slp1String> {
        Slp1String::from(&val)
    }
}

impl TryFrom<&str> for Slp1String {
    type Error = Error;
    fn try_from(val: &str) -> Result<Slp1String> {
        Slp1String::from(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_cases() {
        let success_cases = &[
            "nArAyaRAya", // Sample text.
            "",           // Empty strings OK.
            "aH",         // Visarga
            "aM",         // Anusvara
            "ka~",        // Anunasika
            "ka\\",       // Anudatta
            "ka^",        // Svarita
            "ka~\\",      // Anunasika + anudatta
            "ka~^",       // Anunasika + svarita
        ];
        for case in success_cases {
            assert!(Slp1String::from(case).is_ok(), "Should be Ok: {case}")
        }

        let failure_cases = &[
            "अ",     // Not ASCII
            "!",     // Not SLP1
            " ",     // Not SLP1
            "5",     // Not SLP1
            "\\",    // Must follow vowel.
            "^",     // Must follow vowel.
            "~",     // Must follow vowel.
            "b\\",   // Must follow vowel or `~`.
            "b^",    // Must follow vowel or `~`.
            "b~",    // Must follow vowel.
            "ba\\~", // Must follow vowel.
            "ba^~",  // Must follow vowel.
        ];
        for case in failure_cases {
            assert!(Slp1String::from(case).is_err(), "Should be Err: {case}")
        }
    }
}
