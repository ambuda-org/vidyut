use std::fmt;

/// A Sanskrit sound.
pub type Sound = char;

/// A set of Sanskrit sounds.
///
/// Internally, a `Set` is just a 128-byte array where `array[i]` is 1 if the char with `u8`
/// value `i` is present in the set and 0 otherwise.
pub struct Set([u8; 128]);

impl Set {
    /// Creates an empty set.
    pub const fn new() -> Self {
        Set([0; 128])
    }

    /// Creates a set whose members are the characters in `string`.
    pub const fn from(text: &str) -> Self {
        let mut res = Set([0; 128]);
        let mut i = 0;
        while i < text.len() {
            let c = text.as_bytes()[i] as char;
            res.0[c as usize] = 1;
            i += 1;
        }
        res
    }

    pub const fn add(&mut self, c: Sound) {
        self.0[c as usize] = 1;
    }

    pub const fn extend(&mut self, other: &Set) {
        let mut i = 0;
        while i < self.0.len() {
            self.0[i] |= other.0[i];
            i += 1;
        }
    }

    /// Returns whether the set contains the given sound.
    pub const fn contains(&self, c: Sound) -> bool {
        self.0[c as usize] == 1
    }

    pub fn contains_any(&self, s: &str) -> bool {
        s.as_bytes().iter().any(|c| self.0[*c as usize] == 1)
    }
}

impl fmt::Display for Set {
    /// Returns all chars in this set in their traditional Sanskrit order.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        for c in "aAiIuUfFxXeEoOMHkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh".chars() {
            if self.contains(c) {
                ret.push(c);
            }
        }
        write!(f, "{ret}")
    }
}

impl Default for Set {
    fn default() -> Self {
        Self::new()
    }
}
