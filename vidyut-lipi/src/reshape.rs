//! Utilities for reshaping text before and after transliteration.
//!
//! When transliterating simple schemes, we can process the input string in a single left-to-right
//! pass. Backtracking, if needed at all, is minimal and limited to single characters, such as the
//! virama.
//!
//! Some schemes, however, are complex enough that they are challenging to process in a single
//! left-to-right pass. Such schemes typically store characters in a byte order that differs from
//! their phonetic order. For example, the Tamil superscript sequence "கோ⁴" ("gho") stores its
//! numeric superscript (which describes the consonant) after the vowel sign.
//!
//! This module provides utilities for reshaping a piece of text both before transliteration (so
//! that the text's phonetic and byte orders match) and after transliteration (so that the final
//! string matches the conventions of the output scheme).
use crate::mapping::Mapping;
use crate::scheme::Scheme;
use std::borrow::Cow;

/// "ra" consonant
const KHMER_LETTER_RO: char = '\u{179a}';

/// Special diacritic for repha. It follows (in byte order) the consonant it precedes (in
/// pronunciation order).
const KHMER_SIGN_ROBAT: char = '\u{17cc}';

/// Virama
const KHMER_SIGN_VIRIAM: char = '\u{17d1}';

/// Like virama, but indicates that next char should be subscripted.
const KHMER_SIGN_COENG: char = '\u{17d2}';

/// Used instead of space (' ') in tibetan
const TIBETAN_MARK_INTERSYLLABLIC_TSHEG: char = '\u{0f0b}';

/// Tibetan "ba"
const TIBETAN_LETTER_BA: char = '\u{0f56}';

/// Tibetan "va" (archaic)
const TIBETAN_LETTER_WA: char = '\u{0f5d}';

/// Unsure how to use this. For now, used during transliteration and removed afterward.
const TIBETAN_MARK_HALANTA: char = '\u{0f84}';

/// The second component of a voiced aspirate stop consonant (gha, jha, ...).
const TIBETAN_SUBJOINED_LETTER_HA: char = '\u{0fb7}';

fn is_khmer_consonant(c: char) -> bool {
    // Range is defined in Khmer unicode spec.
    ('\u{1780}'..='\u{17a2}').contains(&c)
}

fn is_tamil_superscript(c: char) -> bool {
    ['²', '³', '⁴'].contains(&c)
}

/// Returns whether `c` denotes a Tamil marker that must precede the superscript sign.
fn is_tamil_preceding_mark(c: char) -> bool {
    const TA_VOWEL_AA: char = '\u{0bbe}';
    const TA_VIRAMA: char = '\u{0bcd}';
    (TA_VOWEL_AA..=TA_VIRAMA).contains(&c)
}

/// Returns whether `c` denotes a Thai vowel sign that must precede the consonant it modifies.
fn is_thai_preceding_vowel(c: char) -> bool {
    // Range is defined in Thai unicode spec.
    ('\u{0e40}'..='\u{0e44}').contains(&c)
}

fn is_thai_consonant(c: char) -> bool {
    // Range is defined in Thai unicode spec. Ignore THAI CHARACTER O ANG, which is used for
    // independent vowels.
    const THAI_CHARACTER_O_ANG: char = '\u{0e2d}';
    ('\u{0e01}'..='\u{0e2e}').contains(&c) && c != THAI_CHARACTER_O_ANG
}

fn is_tibetan_r_l_vowel_mark(x: char, y: char) -> bool {
    // (x is subjoined r OR l) AND (y is vowel sign aa OR i OR ii)
    //
    // The vowel sign condition is complex because the "original" code points are decomposed in
    // practice.
    matches!(x, '\u{0fb2}' | '\u{0fb3}') && matches!(y, '\u{0f71}' | '\u{0f80}' | '\u{0f81}')
}

fn is_tibetan_standard_consonant(c: char) -> bool {
    // Range is defined in the Tibetan unicode spec.
    ('\u{0f40}'..='\u{0f6a}').contains(&c)
}

/// Converts a Tibetan subjoined consonant to a standard consonant, or `None` if the input is not a
/// subjoined consonant.
fn to_tibetan_standard_consonant(c: char) -> Option<char> {
    if is_tibetan_subjoined_consonant(c) {
        let code = c as u32;
        char::from_u32(code - 0x0050)
    } else {
        None
    }
}

fn is_tibetan_subjoined_consonant(c: char) -> bool {
    // Range is defined in the Tibetan unicode spec.
    ('\u{0f90}'..='\u{0fb9}').contains(&c)
}

/// Converts a Tibetan standard consonant to a subjoined consonant, or `None` if the input is not a
/// standard consonant.
fn to_tibetan_subjoined_consonant(c: char) -> Option<char> {
    if is_tibetan_standard_consonant(c) {
        let code = c as u32;
        char::from_u32(code + 0x0050)
    } else {
        None
    }
}

/// Returns the number of bytes taken by the first `num_chars` chars in `text`.
fn chars_to_byte_offset(text: &str, num_chars: usize) -> usize {
    text.chars().take(num_chars).map(|c| c.len_utf8()).sum()
}

pub fn reshape_before<'a>(input: &'a str, mapping: &Mapping) -> Cow<'a, str> {
    match mapping.from() {
        Scheme::Khmer => {
            let mut ret = String::new();
            let mut i = 0;
            let seek = |i, num_chars| i + chars_to_byte_offset(&input[i..], num_chars);

            // TODO: rewrite anusvara per Aksharamukha.
            while i < input.len() {
                let mut chars = input[i..].chars();
                let x = chars.next().expect("text remaining");
                let y = chars.next();

                if x == KHMER_SIGN_COENG {
                    // COENG + (cons) --> VIRIAM + (cons)
                    if y.map_or(false, is_khmer_consonant) {
                        ret.push(KHMER_SIGN_VIRIAM);
                        i = seek(i, 1);
                        continue;
                    }
                } else if is_khmer_consonant(x) {
                    // (cons) + ROBAT --> RO + VIRIAM + (cons)
                    if y == Some(KHMER_SIGN_ROBAT) {
                        ret.extend([KHMER_LETTER_RO, KHMER_SIGN_VIRIAM, x]);
                        i = seek(i, 2);
                        continue;
                    }
                }
                ret.push(x);
                i = seek(i, 1);
            }
            Cow::Owned(ret)
        }
        Scheme::Tamil => {
            let mut ret = String::new();
            let mut chars = input.chars();

            // Move superscripts next to the consonants they modify.
            while let Some(x) = chars.next() {
                if is_tamil_preceding_mark(x) {
                    let y = chars.next();
                    if y.map_or(false, is_tamil_superscript) {
                        y.map(|c| ret.push(c));
                        ret.push(x);
                    } else {
                        ret.push(x);
                        y.map(|c| ret.push(c));
                    }
                } else {
                    ret.push(x)
                }
            }
            Cow::Owned(ret)
        }
        Scheme::Thai => {
            // Move certain Thai vowel signs right by one index.
            //
            // For Thai, a vowel mark that appears visually to the left of a consonant is stored
            // logically before the consonant.
            let mut ret = String::new();
            let mut chars = input.chars();
            while let Some(x) = chars.next() {
                if is_thai_preceding_vowel(x) {
                    let y = chars.next();
                    if let Some(y) = y {
                        if is_thai_consonant(y) {
                            ret.extend(&[y, x]);
                        } else {
                            ret.extend(&[x, y]);
                        }
                    } else {
                        ret.push(x);
                    }
                } else {
                    ret.push(x);
                }
            }
            Cow::Owned(ret)
        }
        Scheme::Tibetan => {
            const TIBETAN_CATURTHA_HALF: &[char] =
                &['\u{0f42}', '\u{0f4c}', '\u{0f51}', '\u{0f56}', '\u{0f5b}'];

            let seek = |i, n| i + chars_to_byte_offset(&input[i..], n);

            let mut ret = String::new();
            let mut i = 0;
            while i < input.len() {
                let mut chars = input[i..].chars();
                let x = chars.next().expect("text remaining");

                if x == TIBETAN_MARK_INTERSYLLABLIC_TSHEG {
                    // tsheg --> space
                    ret.push(' ');
                } else {
                    // (subjoined cons) --> virama + (cons)
                    let x_new = to_tibetan_standard_consonant(x);
                    if let Some(x_new) = x_new {
                        // Unwrap to dummy characters for simpler logic below.
                        let w = ret.chars().last().unwrap_or('_');
                        let y = chars.next().unwrap_or('_');
                        let is_voiced_aspirated_consonant =
                            TIBETAN_CATURTHA_HALF.contains(&w) && x == TIBETAN_SUBJOINED_LETTER_HA;

                        if is_voiced_aspirated_consonant || is_tibetan_r_l_vowel_mark(x, y) {
                            // But, not for voiced aspirated consonants, which we transliterate as
                            // single units.
                            //
                            // Nor for certain dependent vowel marks (SLP f, F, x, X), which we
                            // likewise transliterate as signle units.
                            ret.push(x);
                        } else {
                            ret.extend(&[TIBETAN_MARK_HALANTA, x_new]);
                        }
                    } else {
                        ret.push(x);
                    }
                }

                i = seek(i, 1);
            }
            Cow::Owned(ret)
        }
        _ => Cow::Borrowed(input),
    }
}

pub fn reshape_after(output: String, mapping: &Mapping) -> String {
    match mapping.to() {
        Scheme::Khmer => {
            let mut ret = String::new();
            let mut i = 0;

            let seek = |i, n| i + chars_to_byte_offset(&output[i..], n);

            // TODO: rewrite anusvara per Aksharamukha.
            while i < output.len() {
                let mut chars = output[i..].chars();
                let x = chars.next().expect("text remaining");
                let y = chars.next();

                if x == KHMER_SIGN_VIRIAM {
                    // VIRIAM + (cons) --> COENG + (cons)
                    if y.map_or(false, is_khmer_consonant) {
                        ret.push(KHMER_SIGN_COENG);
                        i = seek(i, 1);
                        continue;
                    };
                } else if x == KHMER_LETTER_RO {
                    // RO + VIRIAM + (cons) --> (cons) + ROBAT
                    let z = chars.next();
                    if y == Some(KHMER_SIGN_VIRIAM) && z.map_or(false, is_khmer_consonant) {
                        let z = z.expect("is consonant");
                        ret.extend(&[z, KHMER_SIGN_ROBAT]);
                        i = seek(i, 3);
                        continue;
                    }
                }
                ret.push(x);
                i = seek(i, 1)
            }
            ret
        }
        Scheme::Tamil => {
            let mut ret = String::new();
            let mut chars = output.chars();

            // Move superscripts after the marks they should follow.
            while let Some(x) = chars.next() {
                if is_tamil_superscript(x) {
                    let y = chars.next();
                    if y.map_or(false, is_tamil_preceding_mark) {
                        y.map(|c| ret.push(c));
                        ret.push(x);
                    } else {
                        ret.push(x);
                        y.map(|c| ret.push(c));
                    }
                } else {
                    ret.push(x)
                }
            }
            ret
        }
        Scheme::Thai => {
            let mut ret = String::new();
            for y in output.chars() {
                if is_thai_preceding_vowel(y) {
                    if let Some(x) = ret.pop() {
                        if is_thai_consonant(x) {
                            ret.extend(&[y, x]);
                        } else {
                            ret.extend(&[x, y]);
                        }
                    } else {
                        ret.push(y);
                    }
                } else {
                    ret.push(y);
                }
            }
            ret
        }
        Scheme::Tibetan => {
            let mut ret = String::new();
            let mut i = 0;

            let seek = |i, n| i + chars_to_byte_offset(&output[i..], n);

            while i < output.len() {
                let mut chars = output[i..].chars();
                let x = chars.next().expect("text remaining");
                if x == ' ' {
                    // space --> tsheg
                    ret.push(TIBETAN_MARK_INTERSYLLABLIC_TSHEG);
                    i = seek(i, 1);
                } else if x == TIBETAN_LETTER_WA {
                    // va --> ba
                    //
                    // This is for consistency with Aksharamukha.
                    ret.push(TIBETAN_LETTER_BA);
                    i = seek(i, 1);
                } else if x == TIBETAN_MARK_HALANTA {
                    // virama + (cons) --> (subjoined cons)
                    let maybe_y = chars.next().and_then(to_tibetan_subjoined_consonant);
                    if let Some(y) = maybe_y {
                        ret.push(y);
                        i = seek(i, 2);
                    } else {
                        // Don't push halanta, per Aksharamukha.
                        i = seek(i, 1);
                    }
                } else {
                    ret.push(x);
                    i = seek(i, 1);
                }
            }
            ret
        }
        _ => output,
    }
}
