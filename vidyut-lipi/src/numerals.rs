//! Special logic for transliterating non-decimal numerals.

use crate::mapping::Mapping;
use crate::scheme::Scheme;
use rustc_hash::FxHashMap;

type DigitToIntMap = FxHashMap<String, u32>;
type IntToDigitMap = FxHashMap<u32, String>;

/// Transliterates the `numeral` string from Grantha to decimal notation and pushes the result to
/// `buffer`.
fn grantha_to_decimal(buffer: &mut String, numeral: &str, digit_to_int: &DigitToIntMap) {
    /// Grantha expresses numbers in groups that correspond to powers of 1000. In decimal notation,
    /// ecah of these groups requires 3 digits.
    const CHUNK_SIZE: usize = 3;

    // First, convert `numeral` to int data.
    let ints: Vec<u8> = {
        let mut ints: Vec<u8> = numeral
            .chars()
            .rev()
            .flat_map(|digit| {
                let mut temp = [0u8; 4];
                let digit_str = digit.encode_utf8(&mut temp);
                digit_to_int.get(digit_str).map(|i| *i as u8)
            })
            .collect();
        // Pad so that we can iterate by chunks of `CHUNK_SIZE`.
        while ints.len() % CHUNK_SIZE != 0 {
            ints.push(0);
        }
        // Reorder so most significant digit is first.
        ints.iter().rev().copied().collect()
    };

    // Special case for 0.
    if ints.iter().all(|d| *d == 0) {
        buffer.push('௦');
        return;
    }

    // Build our Grantha number in reverse order since doing so is a little simpler.
    let mut glyph_groups = Vec::new();
    for (i_thousands, window) in ints.chunks(CHUNK_SIZE).rev().enumerate() {
        let mut group: u32 = 0;
        for d in window {
            group = 10 * group + (*d as u32);
        }

        if group == 0 {
            // Grantha does not express groups with value 0.
            continue;
        }

        // Thousands, millions, etc.
        glyph_groups.resize(glyph_groups.len() + i_thousands, "௲");
        if i_thousands > 0 && group == 1 {
            continue;
        }

        // Hundreds, tens, and ones
        for offset in &[1, 10, 100] {
            let digit = (group / offset) % 10;
            assert!(digit < 10);

            let text = match digit {
                0 => continue,
                1 => "௧",
                2 => "௨",
                3 => "௩",
                4 => "௪",
                5 => "௫",
                6 => "௬",
                7 => "௭",
                8 => "௮",
                9 => "௯",
                _ => {
                    // This should never occur.
                    return;
                }
            };

            if *offset == 1 {
                glyph_groups.push(text);
            } else if *offset == 10 {
                glyph_groups.push("௰");
                if digit != 1 {
                    glyph_groups.push(text);
                }
            } else if *offset == 100 {
                glyph_groups.push("௱");
                if digit != 1 {
                    glyph_groups.push(text);
                }
            }
        }
    }

    for group in glyph_groups.iter().rev() {
        buffer.push_str(group);
    }
}

/// Transliterates the `grantha` string to decimal notation and pushes the result to `buffer`.
fn decimal_to_grantha(buffer: &mut String, grantha: &str, int_to_digit: &IntToDigitMap) {
    fn try_parse_digit(chars: &[char], i: usize) -> Option<u32> {
        let ret = match chars.get(i)? {
            '௦' => 0,
            '௧' => 1,
            '௨' => 2,
            '௩' => 3,
            '௪' => 4,
            '௫' => 5,
            '௬' => 6,
            '௭' => 7,
            '௮' => 8,
            '௯' => 9,
            _ => return None,
        };
        Some(ret)
    }

    fn try_parse_place(chars: &[char], i: usize) -> Option<u32> {
        let ret = match chars.get(i)? {
            '௰' => 10,
            '௱' => 100,
            _ => return None,
        };
        Some(ret)
    }

    let chars: Vec<char> = grantha.chars().collect();

    let mut int_groups: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        // Search within the current "thousands" group.
        let i_group_end = match chars[i..].iter().position(|c| *c == '௲') {
            Some(j) => i + j,
            None => chars.len(),
        };

        let mut group: u32 = 0;
        while i < i_group_end {
            let digit = try_parse_digit(&chars, i);
            if let Some(digit) = digit {
                if let Some(place) = try_parse_place(&chars, i + 1) {
                    group += digit * place;
                    i += 2;
                } else {
                    group += digit;
                    i += 1;
                }
            } else if let Some(place) = try_parse_place(&chars, i) {
                group += place;
                i += 1;
            }
        }

        let mut power = 0;
        while chars.get(i) == Some(&'௲') {
            power += 1;
            i += 1;
        }

        while int_groups.len() < power + 1 {
            int_groups.push(0);
        }

        if group != 0 {
            int_groups[power] = group;
        } else if power > 0 {
            int_groups[power] = 1;
        }
    }

    // Special case for 0.
    if int_groups.iter().all(|c| *c == 0) {
        if let Some(d) = int_to_digit.get(&0_u32) {
            buffer.push_str(d);
        }
        return;
    }

    let mut added = false;
    for group in int_groups.iter().rev() {
        for place in &[100, 10, 1] {
            let i = (group / place) % 10;
            if i != 0 || added {
                if let Some(glyph) = int_to_digit.get(&i) {
                    buffer.push_str(glyph);
                    added = true;
                }
            }
        }
    }
}

/// Transliterates `numeral` according to `mapping` and pushes the result to `buffer`.
///
/// `buffer` is part of the API so that callers can avoid extra allocations on the default path.
/// (Grantha transliteration, however, generally needs extra allocations.)
///
/// Assumptions:
/// - `number` contains only numeric characters as encoded in `mapping.from()`.
///
/// Procedure:
/// - If from `Grantha` or to `Grantha`, use Grantha-specific logic.
/// - Otherwise, transliterate digit by digit.
pub fn transliterate_numeral(buffer: &mut String, numeral: &str, mapping: &Mapping) {
    if mapping.from() == mapping.to() {
        // Leave the number unchanged.
        buffer.push_str(numeral);
    } else if mapping.from() == Scheme::Grantha {
        // Convert to Grantha place notation.
        decimal_to_grantha(buffer, numeral, &mapping.int_to_numeral);
    } else if mapping.to() == Scheme::Grantha {
        // Convert from Grantha place notation.
        grantha_to_decimal(buffer, numeral, &mapping.numeral_to_int)
    } else {
        // For decimal-decimal, transliterate one char at a time.
        for glyph in numeral.chars().flat_map(|c| {
            let mut temp = [0u8; 4];
            let glyph_str = c.encode_utf8(&mut temp);
            mapping.all.get(glyph_str)
        }) {
            buffer.push_str(&glyph.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheme::Scheme::*;
    use crate::transliterate;

    const NOISE: &str = "!@#$%^&*()";

    const DEVANAGARI_TESTS: &[(&str, &str)] = &[
        ("0", "०"),
        ("1", "१"),
        ("2", "२"),
        ("3", "३"),
        ("4", "४"),
        ("5", "५"),
        ("6", "६"),
        ("7", "७"),
        ("8", "८"),
        ("9", "९"),
        ("10", "१०"),
        ("1234567890", "१२३४५६७८९०"),
    ];

    #[test]
    fn roman_to_devanagari() {
        let roman_to_deva = Mapping::new(Iast, Devanagari);
        for (roman, deva) in DEVANAGARI_TESTS {
            let actual = transliterate(&roman, &roman_to_deva);
            assert_eq!(deva, &actual);
        }
    }

    #[test]
    fn roman_to_devanagari_with_noise() {
        let roman_to_deva = Mapping::new(Iast, Devanagari);
        for (roman, deva) in DEVANAGARI_TESTS {
            let roman = String::new() + NOISE + roman + NOISE;
            let expected = String::new() + NOISE + deva + NOISE;
            let actual = transliterate(&roman, &roman_to_deva);
            assert_eq!(&expected, &actual);
        }
    }

    #[test]
    fn devanagari_to_roman() {
        let deva_to_roman = Mapping::new(Devanagari, Iast);
        for (roman, deva) in DEVANAGARI_TESTS {
            let actual = transliterate(&deva, &deva_to_roman);
            assert_eq!(roman, &actual);
        }
    }

    const GRANTHA_TESTS: &[(&str, &str)] = &[
        // Basic digits
        ("0", "௦"),
        ("1", "௧"),
        ("2", "௨"),
        ("3", "௩"),
        ("4", "௪"),
        ("5", "௫"),
        ("6", "௬"),
        ("7", "௭"),
        ("8", "௮"),
        ("9", "௯"),
        // Powers of 10
        ("10", "௰"),
        ("100", "௱"),
        ("1000", "௲"),
        ("10000", "௰௲"),
        ("100000", "௱௲"),
        ("1000000", "௲௲"),
        ("10000000", "௰௲௲"),
        ("100000000", "௱௲௲"),
        ("1000000000", "௲௲௲"),
        // Stacked powers of 10
        ("11", "௰௧"),
        ("111", "௱௰௧"),
        ("1111", "௲௱௰௧"),
        ("11111", "௰௧௲௱௰௧"),
        ("111111", "௱௰௧௲௱௰௧"),
        ("1111111", "௲௲௱௰௧௲௱௰௧"),
        ("11111111", "௰௧௲௲௱௰௧௲௱௰௧"),
        ("111111111", "௱௰௧௲௲௱௰௧௲௱௰௧"),
        // Regular numbers
        ("141", "௱௪௰௧"),
        ("213", "௨௱௰௩"),
        ("971", "௯௱௭௰௧"),
        ("985", "௯௱௮௰௫"),
        ("2013", "௨௲௰௩"),
        ("11000", "௰௧௲"),
        ("20003", "௨௰௲௩"),
        ("20000013", "௨௰௲௲௰௩"),
        ("23650566", "௨௰௩௲௲௬௱௫௰௲௫௱௬௰௬"),
        ("123456789234", "௱௨௰௩௲௲௲௪௱௫௰௬௲௲௭௱௮௰௯௲௨௱௩௰௪"),
    ];

    #[test]
    fn roman_to_grantha() {
        let roman_to_grantha = Mapping::new(Iast, Grantha);
        for (roman, grantha) in GRANTHA_TESTS {
            let actual = transliterate(&roman, &roman_to_grantha);
            assert_eq!(grantha, &actual);
        }
    }

    #[test]
    fn roman_to_grantha_with_noise() {
        let roman_to_grantha = Mapping::new(Iast, Grantha);
        for (roman, grantha) in GRANTHA_TESTS {
            let roman = String::new() + NOISE + roman + NOISE;
            let expected = String::new() + NOISE + grantha + NOISE;
            let actual = transliterate(&roman, &roman_to_grantha);
            assert_eq!(&expected, &actual);
        }
    }

    #[test]
    fn grantha_to_roman() {
        let grantha_to_roman = Mapping::new(Grantha, Iast);
        for (roman, grantha) in GRANTHA_TESTS {
            let actual = transliterate(&grantha, &grantha_to_roman);
            assert_eq!(roman, &actual);
        }
    }

    #[test]
    fn grantha_to_roman_with_noise() {
        let grantha_to_roman = Mapping::new(Grantha, Iast);
        for (roman, grantha) in GRANTHA_TESTS {
            let expected = String::new() + NOISE + roman + NOISE;
            let grantha = String::new() + NOISE + grantha + NOISE;
            let actual = transliterate(&grantha, &grantha_to_roman);
            assert_eq!(&expected, &actual);
        }
    }

    #[test]
    fn grantha_to_grantha() {
        let grantha_to_grantha = Mapping::new(Grantha, Grantha);
        for (_, grantha) in GRANTHA_TESTS {
            let actual = transliterate(&grantha, &grantha_to_grantha);
            assert_eq!(grantha, &actual);
        }
    }

    #[test]
    fn roman_to_roman() {
        let roman_to_grantha = Mapping::new(Iast, HarvardKyoto);
        for (roman, _) in GRANTHA_TESTS {
            let actual = transliterate(&roman, &roman_to_grantha);
            assert_eq!(roman, &actual);
        }
    }
}
