use crate::aksharas::{Akshara, Weight};
use lazy_static::lazy_static;

// A set of Sanskrit sounds.
//
// This implementation is copied directly from `vidyut_prakriya::sounds`. For details, see the
// comments there.

type Sound = char;
//hrasva Vowels
const HRASVA: &str = "aiufx";
//dIrgha Vowels
const DIRGHA: &str = "AIUeEoOFX";
//Consonants
const HAL: &str = "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh";
//Anusvara or Visarga
const OTHERS: &str = "MH";
//Sanskrit
const SANSKRIT: &str = "aAiIuUfFxXeEoOMHkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL";

pub struct Set([u8; 256]);

impl Set {
    /// Creates an empty set.
    pub fn new() -> Self {
        Set([0; 256])
    }

    /// Creates a set whose members are the characters in `string`.
    pub fn from(string: impl AsRef<str>) -> Self {
        let mut res = Self::new();
        for c in string.as_ref().chars() {
            res.0[c as usize] = 1;
        }
        res
    }

    // Returns whether the set contains the given sound.
    pub fn contains(&self, c: Sound) -> bool {
        self.0[c as usize] == 1
    }
}

// Helper functions to identify the type of the sanskrit sound....

pub fn is_hrasva(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(HRASVA);
    }
    CHARS.contains(c)
}

pub fn is_dirgha(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(DIRGHA);
    }
    CHARS.contains(c)
}

pub fn is_hal(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(HAL);
    }
    CHARS.contains(c)
}

pub fn is_special(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(OTHERS);
    }
    CHARS.contains(c)
}

// Copied from sounds.rs in vidyut-sandhi crate
// Returns whether the given character is a Sanskrit sound or not.
// Non-Sanskrit sounds include:
// s - avagraha
// - spaces
// - other punctuation characters (|, ||, numbers)
// - characters or symbols from non-SLP1 encodings
pub fn is_sanskrit(c: char) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(SANSKRIT);
    }
    CHARS.contains(c)
}

pub fn clean(line: &str) -> String {
    let mut cleaned = String::new();

    for i in line.chars() {
        if is_sanskrit(i) {
            cleaned.push(i);
        }
    }

    cleaned
}

pub fn to_aksharas(text: impl AsRef<str>, separator: Option<&str>) -> Vec<Vec<Akshara>> {
    let lines = text
        .as_ref()
        .split(separator.unwrap_or("\n"))
        .filter(|s| !s.is_empty())
        .map(|a| {
            let s = clean(a);
            let line: Vec<char> = s.chars().collect();
            let mut aksharas: Vec<Akshara> = Vec::new();
            let LEN = line.len();
            let mut to_push = String::new();

            for mut i in 0..LEN {
                let curr = line[i];

                // If it is a vowel
                if is_hrasva(curr) || is_dirgha(curr) {
                    to_push.push(curr);

                    if i != LEN - 1 {
                        // Check if the next one is a special character. If so push it also
                        if is_special(line[i + 1]) {
                            to_push.push(line[i + 1]);
                            aksharas.push(Akshara::new(to_push.clone(), Weight::G));
                            to_push.clear();
                            i += 1;
                        } else if is_dirgha(curr) {
                            aksharas.push(Akshara::new(to_push.clone(), Weight::G));
                            to_push.clear();
                        }
                        // If it is a hrasva check if a conjunct consonant follows
                        else if i != LEN - 2 {
                            if is_hal(line[i + 1]) && is_hal(line[i + 2]) {
                                aksharas.push(Akshara::new(to_push.clone(), Weight::G));
                                to_push.clear();
                            } else {
                                aksharas.push(Akshara::new(to_push.clone(), Weight::L));
                                to_push.clear();
                            }
                        } else {
                            // Vowel at second last position. Just push whatever follows also
                            to_push.push(line[i + 1]);
                            if is_dirgha(curr) || is_special(line[i + 1]) {
                                aksharas.push(Akshara::new(to_push.clone(), Weight::G));
                            } else {
                                aksharas.push(Akshara::new(to_push.clone(), Weight::L));
                            }
                            i += 1;
                            to_push.clear();
                        }
                    } else {
                        // Just push it
                        aksharas.push(Akshara::new(
                            to_push.clone(),
                            match is_dirgha(curr) {
                                true => Weight::G,
                                false => Weight::L,
                            },
                        ));
                        to_push.clear();
                    }
                } else {
                    // If it's a consonant
                    if is_hal(curr) {
                        to_push.push(curr);
                    }
                }
            }

            aksharas
        });

    lines.collect()
}

