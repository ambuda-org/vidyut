use crate::aksharas::{Akshara, Weight};
use lazy_static::lazy_static;

pub fn to_aksharas(text: impl AsRef<str>, seperator: Option<&str>) -> Vec<Vec<Akshara>> { 
    todo!("
        Need to do all these 3 things in one pass....

        1. Break the text down into lines.
        2. In each line break it down into aksharas.
        3. While finding the akshara, also find whether it is guru or laghu.
        4. Keep pushing into result
    ")
}

pub fn get_scheme(raw: &str) -> Vec<Weight> {
    let mut scheme = Vec::new();
    for i in 0..raw.len() {
        let curr: char = raw.chars().nth(i).unwrap();
        //Other than from second last char
        if i <= raw.len() - 3 {
            let next: char = raw.chars().nth(i + 1).unwrap();
            let next_next: char = raw.chars().nth(i + 2).unwrap();
            if is_dirgha(curr) {
                scheme.push(Weight::G);
            } else if is_hrasva(curr) && is_hal(next) && is_hal(next_next) {
                scheme.push(Weight::G);
            } else if is_hrasva(curr) && is_special(next) {
                scheme.push(Weight::G);
            } else if is_hrasva(curr) {
                scheme.push(Weight::L);
            }
        } else if i == raw.len() - 2 {
            let next: char = raw.chars().nth(i + 1).unwrap();
            //From second last character to last character it is Laghu only if
            //Dirgha vowel and followed by anusvara/visarga.
            if is_dirgha(curr) {
                scheme.push(Weight::G);
            } else if is_hrasva(curr) && is_special(next) {
                scheme.push(Weight::G);
            } else if is_hrasva(curr) {
                scheme.push(Weight::L);
            }
        } else {
            if is_dirgha(curr) {
                scheme.push(Weight::G);
            } else if is_hrasva(curr) {
                scheme.push(Weight::L);
            }
        }
    }
    
    scheme
}

pub fn clean(raw: &str) -> String {
    let mut cleaned = String::new();

    for i in raw.chars() {
        if is_sanskrit(i) {
            cleaned.push(i);
        }
    }

    cleaned
}


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