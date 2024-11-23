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
//!
//!
//! ### Approach
//!
//! The transformations here are mostly based on the similar logic performed by Aksharamukha.
//!
//! Many of these transformations are easy to express as regular expressions. But Rust's regex
//! crates greatly increase the binary size of `vidyu-lipi`, which has serving implications for
//! WASM. So instead of using regexes, we've rolled our own logic for these transformations.

use crate::scheme::Scheme;
use crate::unicode_norm;
use core::str::Chars;

const BENGALI_LETTER_YA: char = '\u{09af}';

const BENGALI_LETTER_YYA: &str = "\u{09af}\u{09bc}";

const BENGALI_LETTER_TA: char = '\u{09a4}';

const BENGALI_LETTER_KHANDA_TA: char = '\u{09ce}';

const BENGALI_VIRAMA: char = '\u{09cd}';

/// Used instead of space (' ') in Bhaiksuki.
const BHAIKSUKI_WORD_SEPARATOR: char = '\u{11c43}';

/// Javanese virama.
const JAVANESE_PANGKON: char = '\u{a9c0}';

/// "ra" consonant
const KHMER_LETTER_RO: char = '\u{179a}';

/// Special diacritic for repha. It follows (in byte order) the consonant it precedes (in
/// pronunciation order).
const KHMER_SIGN_ROBAT: char = '\u{17cc}';

/// Virama
const KHMER_SIGN_VIRIAM: char = '\u{17d1}';

/// Like virama, but indicates that next char should be subscripted.
const KHMER_SIGN_COENG: char = '\u{17d2}';

/// Limbu virama.
const LIMBU_SIGN_SA_I: char = '\u{193b}';

// Generic ra
const MASARAM_GONDI_LETTER_RA: char = '\u{11d26}';

const MASARAM_GONDI_HALANTA: char = '\u{11d44}';

const MASARAM_GONDI_VIRAMA: char = '\u{11d45}';

/// Cluster-initial ra
const MASARAM_GONDI_REPHA: char = '\u{11d46}';

/// Cluster-final ra
const MASARAM_GONDI_RA_KARA: char = '\u{11d47}';

const MEETEI_MAYEK_APUN_IYEK: char = '\u{abed}';

const MYANMAR_SIGN_VIRAMA: char = '\u{1039}';

const MYANMAR_SIGN_ASAT: char = '\u{103a}';

// Tai Tham virama.
const TAI_THAM_SIGN_RA_HAAM: char = '\u{1a7a}';

// Tai Tham combiner.
const TAI_THAM_SIGN_SAKOT: char = '\u{1a60}';

/// Used instead of space (' ') in Tibetan
const TIBETAN_MARK_INTERSYLLABLIC_TSHEG: char = '\u{0f0b}';

/// Tibetan "ba"
const TIBETAN_LETTER_BA: char = '\u{0f56}';

/// Tibetan "va" (archaic)
const TIBETAN_LETTER_WA: char = '\u{0f5d}';

/// Unsure how to use this. For now, used during transliteration and removed afterward.
const TIBETAN_MARK_HALANTA: char = '\u{0f84}';

/// The second component of a voiced aspirate stop consonant (gha, jha, ...).
const TIBETAN_SUBJOINED_LETTER_HA: char = '\u{0fb7}';

const ZANABAZAR_SQUARE_SIGN_VIRAMA: char = '\u{11a34}';

const ZANABAZAR_SQUARE_SUBJOINER: char = '\u{11a47}';

fn is_svara(c: char) -> bool {
    matches!(c, '\u{0951}' | '\u{0952}' | '\u{1cda}')
}

fn is_grantha_svara(c: char) -> bool {
    matches!(c, '\u{1cf4}' | '\u{0951}' | '\u{0952}')
}

fn is_bengali_ayogavaha(c: char) -> bool {
    ('\u{0981}'..='\u{0983}').contains(&c)
}

fn is_devanagari_ayogavaha(c: char) -> bool {
    ('\u{0901}'..='\u{0903}').contains(&c)
}

fn is_grantha_ayogavaha(c: char) -> bool {
    ('\u{11300}'..='\u{11303}').contains(&c)
}

fn is_gujarati_ayogavaha(c: char) -> bool {
    ('\u{0a81}'..='\u{0a83}').contains(&c)
}

fn is_kannada_ayogavaha(c: char) -> bool {
    ('\u{0c80}'..='\u{0c83}').contains(&c)
}

fn is_malayalam_ayogavaha(c: char) -> bool {
    ('\u{0d00}'..='\u{0d04}').contains(&c)
}

fn is_oriya_ayogavaha(c: char) -> bool {
    ('\u{0b01}'..='\u{0b03}').contains(&c)
}

fn is_telugu_ayogavaha(c: char) -> bool {
    ('\u{0c00}'..='\u{0c04}').contains(&c)
}

fn is_ayogavaha(c: char) -> bool {
    is_bengali_ayogavaha(c)
        || is_devanagari_ayogavaha(c)
        || is_grantha_ayogavaha(c)
        || is_gujarati_ayogavaha(c)
        || is_kannada_ayogavaha(c)
        || is_malayalam_ayogavaha(c)
        || is_oriya_ayogavaha(c)
        || is_telugu_ayogavaha(c)
}

fn is_cham_yrlv(c: char) -> bool {
    to_cham_subjoined_yrlv(c).is_some()
}

fn to_cham_subjoined_yrlv(c: char) -> Option<char> {
    if ('\u{aa22}'..='\u{aa25}').contains(&c) {
        let code = c as u32;
        char::from_u32(code + 0x0011)
    } else {
        None
    }
}

fn has_cham_final_consonant(c: char) -> bool {
    to_cham_final_consonant(c).is_some()
}

fn to_cham_final_consonant(c: char) -> Option<char> {
    let ret = match c {
        // Skip SIGN_FINAL_NG -- not sure how to use it.
        '\u{aa06}' => '\u{aa40}', // KA
        '\u{aa07}' => '\u{aa40}', // KHA --> KA
        '\u{aa08}' => '\u{aa41}', // GA
        '\u{aa09}' => '\u{aa41}', // GHA --> GA
        '\u{aa0b}' => '\u{aa42}', // NGA
        '\u{aa0c}' => '\u{aa44}', // CHA
        '\u{aa0d}' => '\u{aa44}', // CHHA --> CHA
        '\u{aa0e}' => '\u{aa44}', // JA --> CHA
        '\u{aa0f}' => '\u{aa44}', // JHA --> CHA
        '\u{aa13}' => '\u{aa45}', // TA
        '\u{aa14}' => '\u{aa45}', // THA --> TA
        '\u{aa15}' => '\u{aa45}', // DA --> TA
        '\u{aa16}' => '\u{aa45}', // DHA --> TA
        '\u{aa18}' => '\u{aa46}', // NA
        '\u{aa1a}' => '\u{aa47}', // PA
        '\u{aa1c}' => '\u{aa47}', // PHA --> PA
        '\u{aa1d}' => '\u{aa47}', // BA --> PA
        '\u{aa1e}' => '\u{aa47}', // BHA --> PA
        '\u{aa20}' => '\u{aa4c}', // MA
        '\u{aa22}' => '\u{aa48}', // YA
        '\u{aa23}' => '\u{aa49}', // RA
        '\u{aa24}' => '\u{aa4a}', // LA
        '\u{aa26}' => '\u{aa4b}', // SSA
        '\u{aa27}' => '\u{aa4b}', // SA --> SSA
        '\u{aa28}' => '\u{aa4d}', // HA
        _ => return None,
    };
    Some(ret)
}

fn is_gunjala_gondi_consonant(c: char) -> bool {
    ('\u{11d6c}'..='\u{11d89}').contains(&c)
}

fn from_javanese_medial(c: char) -> Option<char> {
    let ret = match c {
        '\u{a9be}' => '\u{a9aa}',
        '\u{a9bf}' => '\u{a9ab}',
        _ => return None,
    };
    Some(ret)
}

fn to_javanese_medial(c: char) -> Option<char> {
    let ret = match c {
        '\u{a9aa}' => '\u{a9be}',
        '\u{a9ab}' => '\u{a9bf}',
        _ => return None,
    };
    Some(ret)
}

fn is_khmer_consonant(c: char) -> bool {
    ('\u{1780}'..='\u{17a2}').contains(&c)
}

fn is_limbu_standard_yrv(c: char) -> bool {
    to_limbu_subjoined_yrv(c).is_some()
}

fn is_limbu_subjoined_yrv(c: char) -> bool {
    to_limbu_standard_yrv(c).is_some()
}

fn to_limbu_standard_yrv(c: char) -> Option<char> {
    let ret = match c {
        '\u{1929}' => '\u{1915}',
        '\u{192a}' => '\u{1916}',
        '\u{192b}' => '\u{1918}',
        _ => return None,
    };
    Some(ret)
}

fn to_limbu_subjoined_yrv(c: char) -> Option<char> {
    let ret = match c {
        '\u{1915}' => '\u{1929}',
        '\u{1916}' => '\u{192a}',
        '\u{1918}' => '\u{192b}',
        _ => return None,
    };
    Some(ret)
}

#[allow(unused)]
fn has_limbu_final_consonant(c: char) -> bool {
    to_limbu_final_consonant(c).is_some()
}

#[allow(unused)]
fn to_limbu_final_consonant(c: char) -> Option<char> {
    let ret = match c {
        '\u{1901}' => '\u{1930}',
        '\u{1905}' => '\u{1931}',
        '\u{190b}' => '\u{1933}',
        '\u{190f}' => '\u{1934}',
        '\u{1910}' => '\u{1935}',
        '\u{1914}' => '\u{1936}',
        '\u{1916}' => '\u{1937}',
        '\u{1917}' => '\u{1938}',
        _ => return None,
    };
    Some(ret)
}

fn is_masaram_gondi_consonant(c: char) -> bool {
    ('\u{11d0c}'..='\u{11d30}').contains(&c)
        || c == MASARAM_GONDI_REPHA
        || c == MASARAM_GONDI_RA_KARA
}

fn to_meetei_mayek_final_consonant(c: char) -> Option<char> {
    let ret = match c {
        '\u{abc0}' => '\u{abdb}', // k (kok)
        '\u{abc2}' => '\u{abdc}', // l (lai)
        '\u{abc3}' => '\u{abdd}', // m (mit)
        '\u{abc4}' => '\u{abde}', // p (pa)
        '\u{abc5}' => '\u{abdf}', // n (na)
        '\u{abc7}' => '\u{abe0}', // t (til)
        '\u{abc9}' => '\u{abe1}', // ng (ngou)
        '\u{abcf}' => '\u{abe2}', // i (i)
        _ => return None,
    };
    Some(ret)
}

fn is_myanmar_consonant(c: char) -> bool {
    ('\u{1000}'..='\u{1020}').contains(&c) || matches!(c, '\u{1050}' | '\u{1051}')
}

fn to_myanmar_subjoined_yrvh(c: char) -> Option<char> {
    let ret = match c {
        '\u{101a}' => '\u{103b}',
        '\u{101b}' => '\u{103c}',
        '\u{101d}' => '\u{103d}',
        '\u{101f}' => '\u{103e}',
        _ => return None,
    };
    Some(ret)
}

fn to_myanmar_standard_yrvh(c: char) -> Option<char> {
    let ret = match c {
        '\u{103b}' => '\u{101a}',
        '\u{103c}' => '\u{101b}',
        '\u{103d}' => '\u{101d}',
        '\u{103e}' => '\u{101f}',
        _ => return None,
    };
    Some(ret)
}

fn is_tamil_superscript(c: char) -> bool {
    ['²', '³', '⁴'].contains(&c)
}

fn is_tai_tham_consonant(c: char) -> bool {
    const TAI_THAM_LETTER_RUE: char = '\u{1a42}';
    const TAI_THAM_LETTER_LUE: char = '\u{1a44}';

    // Ignore RUE and LUE, which are used for vocalic R/L.
    ('\u{1a20}'..='\u{1a4c}').contains(&c)
        && !matches!(c, TAI_THAM_LETTER_RUE | TAI_THAM_LETTER_LUE)
}

/// Returns whether `c` denotes a Tamil marker that must precede the superscript sign.
fn is_tamil_preceding_mark(c: char) -> bool {
    const TA_VOWEL_AA: char = '\u{0bbe}';
    const TA_VIRAMA: char = '\u{0bcd}';
    (TA_VOWEL_AA..=TA_VIRAMA).contains(&c)
}

/// Returns whether `c` denotes a Thai vowel sign that must precede the consonant it modifies.
fn is_thai_preceding_vowel(c: char) -> bool {
    ('\u{0e40}'..='\u{0e44}').contains(&c)
}

fn is_thai_consonant(c: char) -> bool {
    // Ignore THAI CHARACTER O ANG, which is used for independent vowels.
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
    ('\u{0f40}'..='\u{0f6a}').contains(&c)
}

fn is_tibetan_subjoined_consonant(c: char) -> bool {
    ('\u{0f90}'..='\u{0fb9}').contains(&c)
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

fn is_zanabazar_square_consonant(c: char) -> bool {
    ('\u{11a0b}'..='\u{11a32}').contains(&c)
}

/// A utility struct for building an output buffer by matching character sequences.
///
/// The changes we apply in this module are easily modeled as regular expressions (regexes). We
/// have avoided regexes here for two reasons:
///
/// - Rust's regex crates substantially increases the size of our final binary, which causes
///   difficulty in WASM environments. This holds even when using `regex-lite`.
///
/// - Most of our changes are simple enough that we don't need the full power of a standard regex
///   engine. A simpler solution works just as well while providing better readability.
struct Matcher {
    /// The input text to reshap.
    text: String,
    /// Byte offset within `text`. This points to the next character to consider.
    i: usize,
    /// Byte offset when using `push_next`. We use this so that we can push spans.
    prev: usize,
    /// The output buffer.
    buf: String,
}

impl Matcher {
    /// Creates a new `Matcher`.
    fn new(text: String) -> Self {
        Self {
            text,
            i: 0,
            prev: 0,
            buf: String::new(),
        }
    }

    /// Returns the final output buffer.
    fn finish(mut self) -> String {
        if !self.buf.is_empty() {
            self.flush();
            self.buf
        } else {
            // Nothing matched, so return the original text.
            self.text
        }
    }

    /// Returns whether there is still content remaining in the input string.
    fn not_empty(&self) -> bool {
        self.i < self.text.len()
    }

    /// Returns the active slice.
    fn slice(&self) -> &str {
        &self.text[self.i..]
    }

    /// Returns the active slice.
    fn chars(&self) -> Chars<'_> {
        self.text[self.i..].chars()
    }

    /// Returns whether the next character matches the given `filter`.
    fn match_1(&mut self, filter: impl Fn(char) -> bool) -> bool {
        let mut chars = self.slice().chars();
        chars.next().map_or(false, filter)
    }

    /// Returns whether the next 2 characters match the given `filter`.
    fn match_2(&mut self, filter: impl Fn(char, char) -> bool) -> bool {
        let mut chars = self.slice().chars();
        if let (Some(x), Some(y)) = (chars.next(), chars.next()) {
            filter(x, y)
        } else {
            false
        }
    }

    /// Returns whether the next 3 characters match the given `filter`.
    fn match_3(&mut self, filter: impl Fn(char, char, char) -> bool) -> bool {
        let mut chars = self.slice().chars();
        if let (Some(x), Some(y), Some(z)) = (chars.next(), chars.next(), chars.next()) {
            filter(x, y, z)
        } else {
            false
        }
    }

    /// Consumes the next character then uses `func` to append to the buffer.
    fn take_1(&mut self, func: impl Fn(&mut String, char)) {
        self.flush();
        let mut chars = self.slice().chars();
        if let Some(x) = chars.next() {
            func(&mut self.buf, x);
            self.i += x.len_utf8();
            self.prev = self.i;
        }
    }

    /// Consumes the next 2 characters then uses `func` to append to the buffer.
    fn take_2(&mut self, func: impl Fn(&mut String, char, char)) {
        self.flush();
        let mut chars = self.slice().chars();
        if let (Some(x), Some(y)) = (chars.next(), chars.next()) {
            func(&mut self.buf, x, y);
            self.i += x.len_utf8() + y.len_utf8();
            self.prev = self.i;
        }
    }

    /// Consumes the next 3 characters then uses `func` to append to the buffer.
    fn take_3(&mut self, func: impl Fn(&mut String, char, char, char)) {
        self.flush();
        let mut chars = self.slice().chars();
        if let (Some(x), Some(y), Some(z)) = (chars.next(), chars.next(), chars.next()) {
            func(&mut self.buf, x, y, z);
            self.i += x.len_utf8() + y.len_utf8() + z.len_utf8();
            self.prev = self.i;
        }
    }

    fn flush(&mut self) {
        if self.prev < self.i {
            self.buf += &self.text[self.prev..self.i];
            self.prev = self.i;
        }
    }

    /// Consumes the next char and adds it immediately to the buffer.
    fn push_next(&mut self) {
        if let Some(c) = self.slice().chars().next() {
            self.i += c.len_utf8();
        }
    }

    fn swap(&mut self, old: &[char], new: &[char]) {
        self.flush();
        self.buf.extend(new);
        self.i += old.iter().map(|c| c.len_utf8()).sum::<usize>();
        self.prev = self.i;
    }

    fn add(&mut self, c: char) {
        self.i += c.len_utf8();
    }
}

/// Reshapes `input` before we run the main transliteration function.
///
/// Once this function matures, we will consider switching to an iterator-based implementation.
pub fn reshape_before(input: &str, from: Scheme) -> String {
    // Convert to NFC first to avoid certain transliteration errors.
    // (See `iso_15919_bug_no_greedy_match_on_nfd` for an example of what we want to prevent.)
    let input = unicode_norm::to_nfc(input);
    let mut m = Matcher::new(input);

    match from {
        Scheme::Assamese | Scheme::Bengali => {
            while m.not_empty() {
                if m.match_2(|x, y| is_bengali_ayogavaha(x) && is_svara(y)) {
                    m.take_2(|buf, x, y| buf.extend(&[y, x]));
                } else if m.match_1(|x| x == BENGALI_LETTER_KHANDA_TA) {
                    m.take_1(|buf, _| buf.extend(&[BENGALI_LETTER_TA, BENGALI_VIRAMA]))
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Bhaiksuki => {
            while m.not_empty() {
                if m.match_1(|x| x == BHAIKSUKI_WORD_SEPARATOR) {
                    // word separator --> space
                    m.take_1(|buf, _| buf.push(' '));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Burmese | Scheme::Mon => {
            while m.not_empty() {
                if m.match_1(|x| to_myanmar_standard_yrvh(x).is_some()) {
                    // word separator --> space
                    m.take_1(|buf, x| {
                        let x_new = to_myanmar_standard_yrvh(x).expect("yrvh");
                        buf.extend(&[MYANMAR_SIGN_ASAT, x_new]);
                    });
                } else if m.match_1(|x| x == MYANMAR_SIGN_VIRAMA) {
                    m.take_1(|buf, _| buf.push(MYANMAR_SIGN_ASAT));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Devanagari | Scheme::Gujarati | Scheme::Kannada | Scheme::Odia | Scheme::Telugu => {
            // TODO: efficient but not as readable as match/take. Is there a way to make this more
            // concise?
            while m.not_empty() {
                let mut it = m.chars();
                if let Some(x) = it.next() {
                    if is_ayogavaha(x) {
                        if let Some(y) = it.next().filter(|c| is_svara(*c)) {
                            m.swap(&[x, y], &[y, x]);
                            continue;
                        }
                    }
                    m.add(x);
                }
            }
            m.finish()
        }
        Scheme::Grantha => {
            while m.not_empty() {
                if m.match_2(|x, y| is_grantha_ayogavaha(x) && is_grantha_svara(y)) {
                    m.take_2(|buf, x, y| buf.extend(&[y, x]));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Javanese => {
            while m.not_empty() {
                if m.match_1(|x| from_javanese_medial(x).is_some()) {
                    // word separator --> space
                    m.take_1(|buf, x| {
                        let x_new = from_javanese_medial(x).expect("medial");
                        buf.extend(&[JAVANESE_PANGKON, x_new]);
                    });
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Khmer => {
            // TODO: rewrite anusvara per Aksharamukha.
            while m.not_empty() {
                if m.match_2(|x, y| x == KHMER_SIGN_COENG && is_khmer_consonant(y)) {
                    // COENG + (cons) --> VIRIAM + (cons)
                    m.take_2(|buf, _, y| buf.extend(&[KHMER_SIGN_VIRIAM, y]));
                } else if m.match_2(|x, y| is_khmer_consonant(x) && y == KHMER_SIGN_ROBAT) {
                    // (cons) + ROBAT --> RO + VIRIAM + (cons)
                    m.take_2(|buf, x, _| buf.extend(&[KHMER_LETTER_RO, KHMER_SIGN_VIRIAM, x]));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Limbu => {
            while m.not_empty() {
                if m.match_1(is_limbu_subjoined_yrv) {
                    m.take_1(|buf, x| {
                        buf.extend(&[LIMBU_SIGN_SA_I, to_limbu_standard_yrv(x).expect("yrv")])
                    });
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Malayalam => {
            const MALAYALAM_SIGN_VIRAMA: char = '\u{0d4d}';

            fn from_malayalam_chillu(c: char) -> Option<char> {
                let ret = match c {
                    '\u{0d7a}' => '\u{0d23}', // nna
                    '\u{0d7b}' => '\u{0d28}', // na
                    '\u{0d7c}' => '\u{0d30}', // rra
                    '\u{0d7d}' => '\u{0d32}', // la
                    '\u{0d7e}' => '\u{0d33}', // lla
                    '\u{0d7f}' => '\u{0d15}', // ka
                    _ => return None,
                };
                Some(ret)
            }

            while m.not_empty() {
                if m.match_2(|x, y| is_malayalam_ayogavaha(x) && is_svara(y)) {
                    m.take_2(|buf, x, y| buf.extend(&[y, x]));
                } else if m.match_1(|x| from_malayalam_chillu(x).is_some()) {
                    m.take_1(|buf, x| {
                        let x_new = from_malayalam_chillu(x).expect("chillu");
                        buf.extend(&[x_new, MALAYALAM_SIGN_VIRAMA])
                    })
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }

        Scheme::MasaramGondi => {
            while m.not_empty() {
                if m.match_2(|x, y| is_masaram_gondi_consonant(x) && y == MASARAM_GONDI_RA_KARA) {
                    // (cons) + ra-kara --> (cons) + virama + ra
                    m.take_2(|buf, x, _| {
                        buf.extend(&[x, MASARAM_GONDI_VIRAMA, MASARAM_GONDI_LETTER_RA])
                    });
                } else if m
                    .match_2(|x, y| x == MASARAM_GONDI_REPHA && is_masaram_gondi_consonant(y))
                {
                    // repha + (cons) --> ra + virama + (cons)
                    m.take_2(|buf, _, y| {
                        buf.extend(&[MASARAM_GONDI_LETTER_RA, MASARAM_GONDI_VIRAMA, y])
                    });
                } else if m.match_1(|x| x == MASARAM_GONDI_HALANTA) {
                    // Use virama instead of halanta for main transliteration loop.
                    m.take_1(|buf, _| buf.push(MASARAM_GONDI_VIRAMA));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::TaiTham => {
            while m.not_empty() {
                if m.match_1(|x| x == TAI_THAM_SIGN_SAKOT) {
                    // combiner --> virama
                    m.take_1(|buf, _| buf.push(TAI_THAM_SIGN_RA_HAAM));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Tamil => {
            while m.not_empty() {
                // (mark) + (superscript) --> (superscript) + (mark)
                if m.match_2(|x, y| is_tamil_preceding_mark(x) && is_tamil_superscript(y)) {
                    m.take_2(|buf, x, y| buf.extend(&[y, x]));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Thai => {
            while m.not_empty() {
                if m.match_2(|x, y| is_thai_preceding_vowel(x) && is_thai_consonant(y)) {
                    // Move certain Thai vowel signs right by one index.
                    //
                    // For Thai, a vowel mark that appears visually to the left of a consonant is stored
                    // logically before the consonant.
                    m.take_2(|buf, x, y| buf.extend(&[y, x]));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Tibetan => {
            const TIBETAN_CATURTHA_HALF: &[char] =
                &['\u{0f42}', '\u{0f4c}', '\u{0f51}', '\u{0f56}', '\u{0f5b}'];

            while m.not_empty() {
                if m.match_1(|x| x == TIBETAN_MARK_INTERSYLLABLIC_TSHEG) {
                    // tsheg --> space
                    m.take_1(|buf, _| buf.push(' '))
                } else if m.match_1(is_tibetan_subjoined_consonant) {
                    m.flush();
                    // Unwrap to dummy characters for simpler logic below.
                    let w = m.buf.chars().last().unwrap_or('_');
                    let y = m.slice().chars().nth(1).unwrap_or('_');

                    // (subjoined cons) --> virama + (cons)
                    m.take_1(|buf, x| {
                        if let Some(x_new) = to_tibetan_standard_consonant(x) {
                            let is_voiced_aspirated_consonant = TIBETAN_CATURTHA_HALF.contains(&w)
                                && x == TIBETAN_SUBJOINED_LETTER_HA;

                            if is_voiced_aspirated_consonant || is_tibetan_r_l_vowel_mark(x, y) {
                                // But, not for voiced aspirated consonants, which we transliterate as
                                // single units.
                                //
                                // Nor for certain dependent vowel marks (SLP f, F, x, X), which we
                                // likewise transliterate as signle units.
                                buf.push(x);
                            } else {
                                buf.extend(&[TIBETAN_MARK_HALANTA, x_new]);
                            }
                        }
                    });
                } else {
                    m.push_next();
                }
            }

            m.finish()
        }
        Scheme::ZanabazarSquare => {
            const ZANABAZAR_SQUARE_MARK_TSHEG: char = '\u{11a41}';

            while m.not_empty() {
                if m.match_1(|x| x == ZANABAZAR_SQUARE_MARK_TSHEG) {
                    // tsheg --> space
                    m.take_1(|buf, _| buf.push(' '));
                } else if m.match_1(|x| x == ZANABAZAR_SQUARE_SUBJOINER) {
                    // subjoiner --> virama
                    m.take_1(|buf, _| buf.push(ZANABAZAR_SQUARE_SIGN_VIRAMA));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        _ => m.finish(),
    }
}

fn is_bengali_sound(c: char) -> bool {
    match c {
        // Signs, vowels, consonants
        '\u{0981}'..='\u{09bc}' => true,
        // Dependent vowels
        '\u{09be}'..='\u{09cc}' => true,
        // Other consonants and signs
        '\u{09ce}'..='\u{09e3}' => true,
        // Assamese
        '\u{09f0}'..='\u{09f1}' => true,
        _ => false,
    }
}

/// Reshapes `output` after we run the main transliteration function.
pub fn reshape_after(output: String, to: Scheme) -> String {
    let mut m = Matcher::new(output);

    match to {
        Scheme::Assamese | Scheme::Bengali => {
            while m.not_empty() {
                if m.match_2(|x, y| is_svara(x) && is_bengali_ayogavaha(y)) {
                    m.take_2(|buf, x, y| buf.extend(&[y, x]));
                } else if m.match_2(|x, y| is_bengali_sound(x) && y == BENGALI_LETTER_YA) {
                    m.take_2(|buf, x, _| {
                        buf.push(x);
                        buf.push_str(BENGALI_LETTER_YYA);
                    })
                } else if m.match_2(|x, y| x == BENGALI_LETTER_TA && y == BENGALI_VIRAMA) {
                    let mut chars = m.slice().chars();
                    let z = chars.nth(2);

                    // virama + ta + (end) --> khanda ta + (end)
                    // virama + ta + (non-sound) --> khanda ta + (non-sound)
                    if z.map_or(true, |c| !is_bengali_sound(c)) {
                        m.take_2(|buf, _, _| buf.push(BENGALI_LETTER_KHANDA_TA));
                    } else {
                        m.push_next();
                    }
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Bhaiksuki => {
            while m.not_empty() {
                if m.match_1(|x| x == ' ') {
                    // space --> word separator
                    m.take_1(|buf, _| buf.push(BHAIKSUKI_WORD_SEPARATOR));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Burmese | Scheme::Mon => {
            while m.not_empty() {
                if m.match_2(|x, y| x == MYANMAR_SIGN_ASAT && is_myanmar_consonant(y)) {
                    m.take_2(|buf, _, y| {
                        if let Some(y_new) = to_myanmar_subjoined_yrvh(y) {
                            buf.extend(&[y_new]);
                        } else {
                            buf.extend(&[MYANMAR_SIGN_VIRAMA, y]);
                        }
                    });
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Cham => {
            const FAKE_VIRAMA: char = '\u{02be}';
            while m.not_empty() {
                if m.match_2(|x, y| x == FAKE_VIRAMA && is_cham_yrlv(y)) {
                    m.take_2(|buf, _, y| {
                        let y_new = to_cham_subjoined_yrlv(y).expect("yrlv");
                        buf.push(y_new)
                    })
                } else {
                    m.push_next();
                }
            }

            // Substitution above blocks substitution here, so split into two Matchers.
            let first = m.finish();
            let mut m = Matcher::new(first);
            while m.not_empty() {
                if m.match_2(|x, y| has_cham_final_consonant(x) && y == FAKE_VIRAMA) {
                    m.take_2(|buf, x, _| {
                        let x_new = to_cham_final_consonant(x).expect("has final");
                        buf.push(x_new)
                    });
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Devanagari
        | Scheme::Gujarati
        | Scheme::Kannada
        | Scheme::Malayalam
        | Scheme::Odia
        | Scheme::Telugu => {
            // TODO: efficient but not as readable as match/take. Is there a way to make this more
            // concise?
            while m.not_empty() {
                let mut it = m.chars();
                if let Some(x) = it.next() {
                    if is_svara(x) {
                        if let Some(y) = it.next().filter(|c| is_ayogavaha(*c)) {
                            m.swap(&[x, y], &[y, x]);
                            continue;
                        }
                    }
                    m.add(x);
                }
            }
            m.finish()
        }
        Scheme::Grantha => {
            while m.not_empty() {
                if m.match_2(|x, y| is_grantha_svara(x) && is_grantha_ayogavaha(y)) {
                    m.take_2(|buf, x, y| buf.extend(&[y, x]));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }

        Scheme::Javanese => {
            while m.not_empty() {
                if m.match_2(|x, y| x == JAVANESE_PANGKON && to_javanese_medial(y).is_some()) {
                    m.take_2(|buf, _, y| {
                        let y_new = to_javanese_medial(y).expect("medial");
                        buf.extend(&[y_new]);
                    });
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::GunjalaGondi => {
            const GUNJALA_GONDI_VIRAMA: char = '\u{11d97}';
            while m.not_empty() {
                if m.match_2(|x, y| x == GUNJALA_GONDI_VIRAMA && !is_gunjala_gondi_consonant(y)) {
                    // virama + (!cons) --> (!cons)
                    m.take_2(|buf, _, y| buf.push(y));
                } else {
                    let mut chars = m.slice().chars();
                    if chars.next() == Some(GUNJALA_GONDI_VIRAMA) && chars.next().is_none() {
                        // virama + (end of utterance) --> ""
                        m.take_1(|_, _| {});
                    } else {
                        m.push_next();
                    }
                }
            }
            m.finish()
        }
        Scheme::Khmer => {
            // TODO: rewrite anusvara per Aksharamukha.
            while m.not_empty() {
                if m.match_2(|x, y| x == KHMER_SIGN_VIRIAM && is_khmer_consonant(y)) {
                    // VIRIAM + (cons) --> COENG + (cons)
                    //
                    // Note that we don't `take_2` because we might need to rewrite the consonant
                    // with a rule below.
                    m.take_1(|buf, _| buf.push(KHMER_SIGN_COENG));
                } else if m.match_3(|x, y, z| {
                    x == KHMER_LETTER_RO && y == KHMER_SIGN_VIRIAM && is_khmer_consonant(z)
                }) {
                    // RO + VIRIAM + (cons) --> (cons) + ROBAT
                    m.take_3(|buf, _, _, z| buf.extend(&[z, KHMER_SIGN_ROBAT]));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Limbu => {
            while m.not_empty() {
                if m.match_2(|x, y| x == LIMBU_SIGN_SA_I && is_limbu_standard_yrv(y)) {
                    m.take_2(|buf, _, y| buf.push(to_limbu_subjoined_yrv(y).expect("y r v")));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::MasaramGondi => {
            while m.not_empty() {
                if m.match_3(|x, y, z| {
                    is_masaram_gondi_consonant(x)
                        && y == MASARAM_GONDI_VIRAMA
                        && z == MASARAM_GONDI_LETTER_RA
                }) {
                    // (cons) + virama + ra --> (cons) + ra-kara
                    m.take_3(|buf, x, _, _| buf.extend(&[x, MASARAM_GONDI_RA_KARA]));
                } else if m.match_3(|x, y, z| {
                    x == MASARAM_GONDI_LETTER_RA
                        && y == MASARAM_GONDI_VIRAMA
                        && is_masaram_gondi_consonant(z)
                }) {
                    // ra + virama + (cons) --> repha + (cons)
                    m.take_3(|buf, _, _, z| buf.extend(&[MASARAM_GONDI_REPHA, z]));
                } else if m
                    .match_2(|x, y| x == MASARAM_GONDI_VIRAMA && !is_masaram_gondi_consonant(y))
                {
                    m.take_1(|buf, _| buf.push(MASARAM_GONDI_HALANTA));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::MeeteiMayek => {
            while m.not_empty() {
                if m.match_2(|_, y| y == MEETEI_MAYEK_APUN_IYEK) {
                    let x = m.slice().chars().next().expect("present");
                    if let Some(x_new) = to_meetei_mayek_final_consonant(x) {
                        m.take_2(|buf, _, _| buf.push(x_new));
                    } else {
                        m.push_next();
                    }
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::TaiTham => {
            const RA: char = '\u{1a41}';
            const LA: char = '\u{1a43}';
            const MEDIAL_RA: char = '\u{1a55}';
            const MEDIAL_LA: char = '\u{1a56}';

            while m.not_empty() {
                if m.match_2(|x, y| x == TAI_THAM_SIGN_RA_HAAM && matches!(y, RA | LA)) {
                    m.take_2(|buf, _, y| {
                        if y == RA {
                            buf.push(MEDIAL_RA);
                        } else {
                            buf.push(MEDIAL_LA);
                        }
                    });
                } else if m.match_2(|x, y| x == TAI_THAM_SIGN_RA_HAAM && is_tai_tham_consonant(y)) {
                    m.take_2(|buf, _, y| {
                        buf.extend(&[TAI_THAM_SIGN_SAKOT, y]);
                    });
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Tamil => {
            while m.not_empty() {
                if m.match_2(|x, y| is_tamil_superscript(x) && is_tamil_preceding_mark(y)) {
                    // (superscript) + (mark) --> (mark) + (superscript)
                    m.take_2(|buf, x, y| buf.extend(&[y, x]));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Thai => {
            while m.not_empty() {
                if m.match_2(|x, y| is_thai_consonant(x) && is_thai_preceding_vowel(y)) {
                    m.take_2(|buf, x, y| buf.extend(&[y, x]));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::Tibetan => {
            while m.not_empty() {
                if m.match_1(|x| x == ' ') {
                    // space --> tsheg
                    m.take_1(|buf, _| buf.push(TIBETAN_MARK_INTERSYLLABLIC_TSHEG));
                } else if m.match_1(|x| x == TIBETAN_LETTER_WA) {
                    // va --> ba
                    //
                    // This is for consistency with Aksharamukha.
                    m.take_1(|buf, _| buf.push(TIBETAN_LETTER_BA));
                } else if m
                    .match_2(|x, y| x == TIBETAN_MARK_HALANTA && is_tibetan_standard_consonant(y))
                {
                    // virama + (cons) --> (subjoined cons)
                    m.take_2(|buf, _, y| {
                        if let Some(y_sub) = to_tibetan_subjoined_consonant(y) {
                            buf.push(y_sub)
                        }
                    })
                } else if m.match_1(|x| x == TIBETAN_MARK_HALANTA) {
                    // Don't push virama, per Aksharamukha.
                    m.take_1(|_, _| {});
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        Scheme::ZanabazarSquare => {
            while m.not_empty() {
                if m.match_2(|x, y| {
                    x == ZANABAZAR_SQUARE_SIGN_VIRAMA && is_zanabazar_square_consonant(y)
                }) {
                    m.take_2(|buf, _, y| buf.extend(&[ZANABAZAR_SQUARE_SUBJOINER, y]));
                } else {
                    m.push_next();
                }
            }
            m.finish()
        }
        _ => m.finish(),
    }
}
