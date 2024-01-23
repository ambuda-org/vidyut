use crate::mapping::Mapping;
use crate::numerals;
use crate::scheme::Scheme;

/// Transliterates from an abugida.
fn transliterate_from_abugida(input: &str, mapping: &Mapping) -> String {
    let chars: Vec<char> = input.chars().collect();
    let is_to_alpha = mapping.to.is_alphabet();
    let uses_non_decimal =
        mapping.from.has_non_decimal_numerals() || mapping.to.has_non_decimal_numerals();

    let mut output = String::new();
    let mut i = 0;
    let mut key = String::new();
    let mut had_consonant = false;
    while i < chars.len() {
        if uses_non_decimal {
            let num_numerals = chars[i..]
                .iter()
                .take_while(|c| {
                    let mut temp = [0u8; 4];
                    let digit_str = c.encode_utf8(&mut temp);
                    mapping.numeral_to_int.contains_key(digit_str)
                })
                .count();
            if num_numerals > 0 {
                numerals::transliterate_numeral(&mut output, &chars[i..i + num_numerals], mapping);
                i += num_numerals;
                continue;
            }
        }

        let mut o: Option<&String> = None;
        let mut key_len_in_chars = 0;
        for len_key in (1..=mapping.len_longest_key).rev() {
            let j = std::cmp::min(i + len_key, chars.len());
            key.clear();
            key.extend(&chars[i..j]);
            key_len_in_chars = j - i;

            o = mapping.get(&key);
            if o.is_some() {
                break;
            }
        }

        match o {
            Some(s) => {
                if had_consonant
                    && (mapping.marks.contains_key(&key) || key == mapping.input_virama)
                {
                    // Pop implicit "a" vowel.
                    output.pop();
                }

                output += s;

                if is_to_alpha && mapping.consonants.contains_key(&key) {
                    // Add implicit "a" vowel.
                    output += "a";
                    had_consonant = true;
                }
            }
            None => {
                output.push_str(&key);
            }
        }

        // Add length in *chars*, not in *bytes*. Otherwise we get weird output.
        debug_assert!(key_len_in_chars > 0);
        i += key_len_in_chars;
    }

    output
}

/// Transliterates from an alphabet.
fn transliterate_from_alphabet(input: &str, mapping: &Mapping) -> String {
    let chars: Vec<char> = input.chars().collect();
    let is_to_abugida = mapping.to.is_abugida();
    let is_itrans = mapping.from == Scheme::Itrans;

    let mut output = String::new();
    let mut i = 0;
    let mut key = String::new();
    let mut had_consonant = false;
    while i < chars.len() {
        if mapping.to().has_non_decimal_numerals() {
            let num_numerals = chars[i..]
                .iter()
                .take_while(|c| {
                    let mut temp = [0u8; 4];
                    let digit_str = c.encode_utf8(&mut temp);
                    mapping.numeral_to_int.contains_key(digit_str)
                })
                .count();
            if num_numerals > 0 {
                numerals::transliterate_numeral(&mut output, &chars[i..i + num_numerals], mapping);
                i += num_numerals;
                continue;
            }
        }

        let mut o: Option<&String> = None;
        let mut key_len_in_chars = 0;
        for len_key in (1..=mapping.len_longest_key).rev() {
            let j = std::cmp::min(i + len_key, chars.len());
            key.clear();
            key.extend(&chars[i..j]);
            key_len_in_chars = j - i;

            o = mapping.get(&key);
            if o.is_some() {
                break;
            }
        }

        match o {
            Some(o) => {
                if had_consonant {
                    if let Some(mark) = mapping.marks.get(&key) {
                        if is_to_abugida {
                            output.pop();
                        }
                        output += mark;
                    } else if key == "a" && is_to_abugida {
                        output.pop();
                    } else {
                        output += o;
                    }
                } else {
                    output += o;
                }

                had_consonant = mapping.consonants.contains_key(&key);
                if had_consonant && is_to_abugida {
                    output += &mapping.output_virama;
                }
            }
            None => {
                // ITRANS: `\` skips the next character.
                if is_itrans && chars[i] == '\\' {
                    if let Some(c) = chars.get(i + 1) {
                        output.push(*c);
                    }
                    i += 2;
                    continue;
                }

                // Use the original character as-is.
                output.push_str(&key);
                had_consonant = false;
            }
        }

        // Add length in *chars*, not in *bytes*. Otherwise we get weird output.
        debug_assert!(key_len_in_chars > 0);
        i += key_len_in_chars;
    }
    output
}

/// Transliterates the input string with the provided `Mapping`.
///
/// For most use cases, we recommend using the API on `Lipika` instead.
///
/// ### Usage
///
/// ```
/// use vidyut_lipi::{transliterate, Mapping, Scheme};
///
/// let mapping = Mapping::new(Scheme::HarvardKyoto, Scheme::Devanagari);
/// let result = transliterate("saMskRtam", &mapping);
/// assert_eq!(result, "संस्कृतम्");
/// ```
pub fn transliterate(input: impl AsRef<str>, mapping: &Mapping) -> String {
    if mapping.from.is_abugida() {
        transliterate_from_abugida(input.as_ref(), mapping)
    } else {
        transliterate_from_alphabet(input.as_ref(), mapping)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transliterate() {
        let mapping = Mapping::new(Scheme::HarvardKyoto, Scheme::Devanagari);
        let t = |s| transliterate(s, &mapping);
        assert_eq!(t("namaskRtya"), "नमस्कृत्य");
    }
}
