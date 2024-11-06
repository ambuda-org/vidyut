use crate::mapping::Mapping;
use crate::numerals;
use crate::reshape::{reshape_after, reshape_before};
use crate::scheme::Scheme;

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
    transliterate_inner(input.as_ref(), mapping)
}

/// Transliterates the input string with the provided `Mapping`.
///
///
/// ### Data flow
///
/// This function has three stages:
///
/// 1. *Pre-processing.* Certain schemes are difficult to transliterate in their raw form. To make
///    transliteration easier, pre-process the input string by rearranging or substituting certain
///    code points. For details, see `reshape_before`.
///
/// 2. *Remapping.* Here, we transliterate the output of (1) in a single pass by greedily matching
///    against the entries in `mapping`. For many transliteration pairs, this stage is the only one
///    required.
///
/// 3. *Post-processing.* Certain schemes use a code point order that is difficult to create just
///    by remapping. So, we reshape the output of (2) to create our final output. For details, see
///    `reshape_after`.
///
/// Each of these stages makes at most one pass over the input string. Several scheme pairs will be
/// able to avoid doing work in stages (1) and (3) and thus process the input text in just one
/// pass. For more complex scheme pairs (such as `Tibetan` to `Khmer`), this code will make three
/// passes total.
fn transliterate_inner(input: &str, mapping: &Mapping) -> String {
    let input = reshape_before(input, mapping.from());

    let is_to_alphabet = mapping.to.is_alphabet();
    let is_from_abugida = mapping.from.is_abugida();
    let is_to_abugida = mapping.to.is_abugida();
    let is_from_itrans = mapping.from == Scheme::Itrans;

    let uses_non_decimal =
        mapping.from.has_non_decimal_numerals() || mapping.to.has_non_decimal_numerals();

    let mut output = String::new();
    let mut i = 0;
    let mut had_virama = false;
    while i < input.len() {
        // Special case: Numerals that don't use decimal place notation are transliterated
        // separately.
        if uses_non_decimal {
            let next_i = find_end_of_numeral_span(mapping, input.as_ref(), i);
            if let Some(next_i) = next_i {
                debug_assert!(next_i > i, "next_i = {next_i}, i = {i}");
                numerals::transliterate_numeral(&mut output, &input[i..next_i], mapping);
                i = next_i;
                continue;
            }
        }

        // 1. Find the largest prefix of `input[i..]` that is defined in `mapping`.
        //
        // We must check for the *largest* match to distinguish between `b` and `bh`, `R` and `RR`,
        // etc.
        let mut token = None;
        let mut key: &str = "";
        let mut next_i = i;
        for len_key in (1..=mapping.len_longest_key).rev() {
            // `nth` is 0-indexed.
            let j = input[i..].char_indices().nth(len_key).map(|(i, _)| i);
            next_i = if let Some(j) = j { i + j } else { input.len() };

            key = &input[i..next_i];
            token = mapping.get(key);
            if token.is_some() {
                break;
            }
        }
        debug_assert!(next_i > i, "next_i = {next_i}, i = {i}");

        // 2. Append the mapped result, if it exists.
        if let Some(token) = token {
            // `letter_a` is "a" for Latin scripts but takes other values for non-Latin scripts
            // like Ol Chiki.

            // Abugidas and alphabets have distinct logic here, so keep their code neatly separate.
            if is_from_abugida {
                let a = &mapping.to_map.letter_a;
                if output.ends_with(a)
                    && (mapping.marks.contains_key(key) || key == mapping.from_map.virama)
                {
                    // `key` maps to a token that blocks the default "a" vowel, so pop the "a" that
                    // we added in the previous iteration.
                    output.pop();
                }

                output += &token.text();

                if is_to_alphabet && token.is_consonant() {
                    // Add an implicit "a" vowel.
                    //
                    // (The next loop iteration might pop this "a" off of `output`.)
                    output += a;
                }
            } else {
                // Transliterate from alphabet
                if had_virama && key == mapping.from_map.letter_a {
                    // `key` is the default "a" vowel, so pop the virama that we added in the
                    // previous iteration.
                    output.pop();
                    had_virama = false;
                } else {
                    let mut text = token.text();
                    if had_virama {
                        if let Some(mark) = mapping.marks.get(key) {
                            output.pop();
                            text = mark;
                        }
                    }

                    output += text;

                    if token.is_consonant() && is_to_abugida {
                        // We have not seen a vowel mark yet, so push a virama for now.
                        //
                        // (The next loop iteration might pop this virama off of `output`.)
                        output += &mapping.to_map.virama;
                        had_virama = true;
                    }
                }
            }
        } else {
            // ITRANS: `\` skips the next character.
            if is_from_itrans {
                let mut chars = input[i..].chars();
                if chars.next() == Some('\\') {
                    i += 1;
                    if let Some(c) = chars.next() {
                        output.push(c);
                        i += c.len_utf8();
                    }
                    continue;
                }
            }

            // Use the original character as-is.
            output.push_str(key);
            had_virama = false;
        }

        // Prepare for next loop.
        i = next_i;
    }

    reshape_after(output, mapping.to())
}

/// Finds the end byte of a sequence of numerals starting at byte offset `i`.
///
/// Returns one of:
/// - `Some(j)` if `input[i..]` starts with a numeral.
/// - `None` if `input[i..]` does not start wtih a numeral.
fn find_end_of_numeral_span(mapping: &Mapping, input: &str, i: usize) -> Option<usize> {
    let i_next_non_numeric = input[i..]
        .char_indices()
        .find(|(_, c)| {
            let mut temp = [0u8; 4];
            let digit_str = c.encode_utf8(&mut temp);
            !mapping.numeral_to_int.contains_key(digit_str)
        })
        .map(|(i, _)| i);

    if let Some(j) = i_next_non_numeric {
        if j == 0 {
            // input[i..] does not start with a numeral.
            None
        } else {
            // input[i..] starts with a numeral.
            Some(i + j)
        }
    } else {
        // input[i..] is numeric until the end of the string.
        Some(input.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Scheme::*;

    #[test]
    fn test_find_end_of_numeral_span() {
        let m = Mapping::new(HarvardKyoto, Devanagari);

        // Basic cases
        assert_eq!(find_end_of_numeral_span(&m, "1a", 0), Some(1));
        assert_eq!(find_end_of_numeral_span(&m, "a1a", 1), Some(2));

        // Followed by non-numeric
        assert_eq!(find_end_of_numeral_span(&m, "1a", 0), Some(1));

        // Non-numeric
        assert_eq!(find_end_of_numeral_span(&m, "a1", 0), None);

        // Numeric until end of string
        assert_eq!(find_end_of_numeral_span(&m, "1", 0), Some(1));
        assert_eq!(find_end_of_numeral_span(&m, "10", 0), Some(2));
    }

    /// For more detailed tests, see our integration test file.
    #[test]
    fn test_transliterate() {
        let mapping = Mapping::new(HarvardKyoto, Devanagari);
        let t = |s| transliterate(s, &mapping);
        assert_eq!(t("namaskRtya"), "नमस्कृत्य");
    }
}
