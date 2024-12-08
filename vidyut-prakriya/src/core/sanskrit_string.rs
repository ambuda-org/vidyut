use crate::core::errors::{Error, Result};
use crate::sounds::AL;

#[allow(unused)]
struct SanskritString(String);

#[allow(unused)]
impl SanskritString {
    pub(crate) fn new() -> Self {
        Self(String::new())
    }

    pub(crate) fn from(text: impl AsRef<str>) -> Result<Self> {
        Self::from_inner(text.as_ref())
    }

    fn from_inner(text: &str) -> Result<Self> {
        // All chars must be ASCII and valid SLP1.
        //
        // (The `is_ascii` check is just to be explicit.)
        let is_valid = text.is_ascii()
            && text
                .chars()
                .all(|c| AL.contains(c) || matches!(c, '\\' | '^' | '~'));

        if is_valid {
            Ok(Self(String::from(text)))
        } else {
            Err(Error::ParseError(
                "Input text must be an ASCII string in SLP1 encoding with no whitespace."
                    .to_string(),
            ))
        }
    }

    // Accessors

    pub fn get(&self, i: usize) -> Option<char> {
        self.0.as_bytes().get(i).map(|x| *x as char)
    }

    pub fn get_rev(&self, i: usize) -> Option<char> {
        self.0.bytes().rev().nth(i).map(|x| x as char)
    }

    pub fn adi(&self) -> Option<char> {
        self.0.bytes().next().map(|x| x as char)
    }

    pub fn antya(&self) -> Option<char> {
        self.0.bytes().last().map(|x| x as char)
    }

    pub fn upadha(&self) -> Option<char> {
        self.get_rev(1)
    }

    pub fn has_antya(&self, c: char) -> bool {
        assert!(c.is_ascii());
        self.0.bytes().last() == Some(c as u8)
    }
}
