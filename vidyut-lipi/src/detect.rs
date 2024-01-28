//! Utilities for detecting the `Scheme` used by some text.

use crate::scheme::Scheme;

/// Detcts the scheme used by the given text.
///
/// `detect` is ideal for interfaces where the user would otherwise need to manually choose which
/// input encoding to use. `detect` removes some of this user friction by making a reasonable guess
/// about the user's query.
///
/// `detect` is best used on text that is mostly or entirely in one `Scheme`. For text that uses
/// multiple schemes, we recommend splitting the text into smaller chunks and running `detect` on
/// these chunks individually. For greater accuracy, we recommend using a more sophisticated
/// approach than this crate provides.
///
///
/// ### Usage
///
/// ```
/// use vidyut_lipi::{detect, Scheme};
///
/// assert_eq!(detect("देवनागरी"), Some(Scheme::Devanagari));
/// assert_eq!(detect("ಕನ್ನಡ"), Some(Scheme::Kannada));
/// assert_eq!(detect("pARqava"), Some(Scheme::Slp1));
/// ```
///
///
/// ### Implementation
///
/// `detect` analyzes the input string by applying various heuristic tests. For non-ASCII scripts,
/// `detect` checks whether characters are in a specific unicode range. For ASCII scripts, `detect`
/// checks for bigrams and trigrams associated with specific encodings. (For example, `R^i` is
/// indicative of ITRANS.) Currently, `detect` returns the first match found and does not do any
/// kind of scoring, ranking, statistical modeling, etc.
///
/// Our goal is to provide an implementation that is fast, small, and good enough. In the future,
/// we might explore more sophisticated solutions that fit within these bounds.
pub fn detect(input: impl AsRef<str>) -> Option<Scheme> {
    detect_inner(input.as_ref())
}

fn detect_inner(input: &str) -> Option<Scheme> {
    use Scheme::*;

    type Range = std::ops::RangeInclusive<u32>;

    // These are Latin supplements for IAST, ISO-15919, etc.
    //
    // - https://unicode.org/charts/PDF/U0080.pdf
    // - https://unicode.org/charts/PDF/U0100.pdf
    // - https://unicode.org/charts/PDF/U1E00.pdf
    const LATIN_1_SUPPLEMENT: Range = 0x0080..=0x00ff;
    const LATIN_EXTENDED_A: Range = 0x0100..=0x017f;
    const LATIN_EXTENDED: Range = 0x01e00..=0x01eff;

    // These are ranges of Unicode code points as defined by unicode.org. To see the official spec
    // for each scheme, see the comments on `Scheme`.
    const DEVANAGARI: Range = 0x0900..=0x097f;
    const DEVANAGARI_EXTENDED: Range = 0xa8e0..=0xa8ff;
    const DEVANAGARI_EXTENDED_A: Range = 0x11b00..=0x11b5f;
    const VEDIC_EXTENSIONS: Range = 0x1cd0..=0x1cff;
    const BENGALI: Range = 0x0980..=0x09ff;
    const GURMUKHI: Range = 0x0a00..=0x0a7f;
    const GUJARATI: Range = 0x0a80..=0x0aff;
    const ORIYA: Range = 0x0b00..=0x0b7f;
    const TAMIL: Range = 0x0b80..=0x0bff;
    const TELUGU: Range = 0x0c00..=0x0c7f;
    const KANNADA: Range = 0x0c80..=0x0cff;
    const MALAYALAM: Range = 0x0d00..=0x0d7f;
    const SINHALA: Range = 0x0d80..=0x0dff;
    const THAI: Range = 0x0e00..=0x0e7f;
    const TIBETAN: Range = 0x0f00..=0x0fff;
    const BURMESE: Range = 0x1000..=0x109f;
    const KHMER: Range = 0x1780..=0x17ff;
    const BALINESE: Range = 0x1b00..=0x1b7f;
    const SAURASHTRA: Range = 0xa880..=0xa8df;
    const JAVANESE: Range = 0xa980..=0xa9df;
    const BRAHMI: Range = 0x11000..=0x1107f;
    const SHARADA: Range = 0x11180..=0x111df;
    const GRANTHA: Range = 0x11300..=0x1137f;
    const SIDDHAM: Range = 0x11580..=0x115ff;
    const NEWA: Range = 0x11400..=0x1147f;
    const TIRHUTA: Range = 0x11480..=0x114df;
    const MODI: Range = 0x11600..=0x1165f;

    // Wraps all of the ranges above.
    const INDIC: Range = *DEVANAGARI.start()..=*MODI.end();
    const ASCII: Range = 0..=0xff;

    for (i, c) in input.char_indices() {
        let code = c as u32;

        // Rust supports [range matching][1], but only if the range is "inlined" and not in a
        // const. But having a bunch of inlined hex ranges (as opposed to our consts above) seems
        // unreadable, so just use an if-else chain.
        //
        // [1]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
        if LATIN_1_SUPPLEMENT.contains(&code)
            || LATIN_EXTENDED_A.contains(&code)
            || LATIN_EXTENDED.contains(&code)
        {
            // TODO: add Kolkata scheme and detection
            if "āīūṛṝḷḹēōṃḥṅñṭḍṇśṣḻĀĪŪṚṜḶḸĒŌṂḤṄÑṬḌṆŚṢḺ".contains(c)
            {
                return Some(Iast);
            }
        } else if INDIC.contains(&code) {
            println!("Checking code {code:x} from char {c} at index {i}");
            let maybe = if DEVANAGARI.contains(&code)
                || DEVANAGARI_EXTENDED.contains(&code)
                || DEVANAGARI_EXTENDED_A.contains(&code)
                || VEDIC_EXTENSIONS.contains(&code)
            {
                Some(Devanagari)
            } else if BENGALI.contains(&code) {
                Some(Bengali)
            } else if GURMUKHI.contains(&code) {
                Some(Gurmukhi)
            } else if GUJARATI.contains(&code) {
                Some(Gujarati)
            } else if ORIYA.contains(&code) {
                Some(Odia)
            } else if TAMIL.contains(&code) {
                Some(Tamil)
            } else if TELUGU.contains(&code) {
                Some(Telugu)
            } else if KANNADA.contains(&code) {
                Some(Kannada)
            } else if MALAYALAM.contains(&code) {
                Some(Malayalam)
            } else if SINHALA.contains(&code) {
                Some(Sinhala)
            } else if THAI.contains(&code) {
                Some(Thai)
            } else if TIBETAN.contains(&code) {
                Some(Tibetan)
            } else if BURMESE.contains(&code) {
                Some(Burmese)
            } else if KHMER.contains(&code) {
                Some(Khmer)
            } else if BALINESE.contains(&code) {
                Some(Balinese)
            } else if SAURASHTRA.contains(&code) {
                Some(Saurashtra)
            } else if JAVANESE.contains(&code) {
                Some(Javanese)
            } else if BRAHMI.contains(&code) {
                Some(Brahmi)
            } else if SHARADA.contains(&code) {
                Some(Sharada)
            } else if GRANTHA.contains(&code) {
                Some(Grantha)
            } else if SIDDHAM.contains(&code) {
                Some(Siddham)
            } else if NEWA.contains(&code) {
                Some(Newa)
            } else if TIRHUTA.contains(&code) {
                Some(Tirhuta)
            } else if MODI.contains(&code) {
                Some(Modi)
            } else {
                None
            };
            if maybe.is_some() {
                return maybe;
            }
        } else if ASCII.contains(&code) {
            if i + 3 <= input.len() {
                const ITRANS_TRIGRAMS: &[&[u8]] = &[b"chh", b"RRi", b"RRI", b"LLi", b"LLI"];

                let trigram = &input.as_bytes()[i..i + 3];
                debug_assert!(trigram.len() == 3);

                if ITRANS_TRIGRAMS.contains(&trigram) {
                    return Some(Itrans);
                }
            }
            if i + 2 <= input.len() {
                const ITRANS_ONLY_BIGRAMS: &[&[u8]] = &[
                    b"ee", b"oo", b"^i", b"^I", b"Ch", b"JN", b"sh", b"Sh", b"~N", b".a", b"N^",
                ];
                const SLP1_ONLY_BIGRAMS: &[&[u8]] = &[
                    b"kz", b"Nk", b"Ng", b"tT", b"dD", b"Sc", b"Sn", b"Gy", b"Gr", b"aR", b"AR",
                    b"iR", b"IR", b"uR", b"UR", b"eR", b"oR",
                ];
                const VELTHUIS_ONLY_BIGRAMS: &[&[u8]] = &[b"\"n", b"~s"];

                let bigram = &input.as_bytes()[i..i + 2];
                debug_assert!(bigram.len() == 2);

                if ITRANS_ONLY_BIGRAMS.contains(&bigram) {
                    return Some(Itrans);
                } else if SLP1_ONLY_BIGRAMS.contains(&bigram) {
                    return Some(Slp1);
                } else if VELTHUIS_ONLY_BIGRAMS.contains(&bigram)
                    || bigram[0] == b'.' && b"mhnrltds".contains(&bigram[1])
                {
                    return Some(Velthuis);
                }
            }

            if "fFxXEOCYwWqQPB".contains(c) {
                return Some(Slp1);
            }
        }
    }

    for (i, _) in input.char_indices() {
        if i + 2 <= input.len() {
            let bigram = &input.as_bytes()[i..i + 2];
            const ITRANS_OR_VELTHUIS_BIGRAMS: &[&[u8]] = &[b"aa", b"ii", b"uu", b"~n"];
            if ITRANS_OR_VELTHUIS_BIGRAMS.contains(&bigram) {
                return Some(Itrans);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use Scheme::*;

    const TEST_CASES: &[(&str, Scheme)] = &[
        // Indic
        // -----
        ("ᬅᬕ᭄ᬦᬶᬫ᭄", Balinese),
        ("অগ্নিম্", Bengali),
        ("𑀅𑀕𑁆𑀦𑀺𑀫𑁆", Brahmi),
        ("အဂ်နိမ်", Burmese),
        ("अग्निम्", Devanagari),
        ("𑌅𑌗𑍍𑌨𑌿𑌮𑍍", Grantha),
        ("ਅਗ੍ਨਿਮ੍", Gurmukhi),
        ("અગ્નિમ્", Gujarati),
        ("ꦄꦒ꧀ꦤꦶꦩ꧀", Javanese),
        ("ಅಗ್ನಿಮ್", Kannada),
        ("អគ្និម៑", Khmer),
        ("അഗ്നിമ്", Malayalam),
        ("𑘀𑘐𑘿𑘡𑘱𑘦𑘿", Modi),
        ("𑐀𑐐𑑂𑐣𑐶𑐩𑑂", Newa),
        ("ଅଗ୍ନିମ୍", Odia),
        ("ꢂꢔ꣄ꢥꢶꢪ꣄", Saurashtra),
        ("𑆃𑆓𑇀𑆤𑆴𑆩𑇀", Sharada),
        ("𑖀𑖐𑖿𑖡𑖰𑖦𑖿", Siddham),
        ("අග්නිම්", Sinhala),
        ("அக்³நிம்", Tamil),
        ("అగ్నిమ్", Telugu),
        ("ཨགིམ", Tibetan),
        ("𑒁𑒑𑓂𑒢𑒱𑒧𑓂", Tirhuta),
        ("อคฺนิมฺ", Thai),
        // IAST
        // ----
        ("rāga", Iast),
        ("nadī", Iast),
        ("vadhū", Iast),
        ("kṛta", Iast),
        ("pitṝn", Iast),
        ("kḷpta", Iast),
        ("ḹ", Iast),
        ("tejasvī", Iast),
        ("gomayaḥ", Iast),
        ("haṃsa", Iast),
        ("naraḥ", Iast),
        ("aṅga", Iast),
        ("añjana", Iast),
        ("kuṭumba", Iast),
        ("kaṭhora", Iast),
        ("ḍamaru", Iast),
        ("soḍhā", Iast),
        ("aruṇa", Iast),
        ("śveta", Iast),
        ("ṣaṣ", Iast),
        ("ḻa", Iast),
        ("pāṇḍava", Iast),
        ("śṛṇoti", Iast),
        ("jñāna", Iast),
        // ITRANS
        // ------
        ("raaga", Itrans),
        ("nadii", Itrans),
        ("nadee", Itrans),
        ("vadhuu", Itrans),
        ("vadhoo", Itrans),
        ("kRRita", Itrans),
        ("kR^ita", Itrans),
        ("pitRRIn", Itrans),
        ("pitR^In", Itrans),
        ("kLLipta", Itrans),
        ("kL^ipta", Itrans),
        ("LLI", Itrans),
        ("L^i", Itrans),
        ("a~Nga", Itrans),
        ("aN^ga", Itrans),
        ("ChAyA", Itrans),
        ("chhAyA", Itrans),
        ("a~njana", Itrans),
        ("aJNjana", Itrans),
        ("shveta", Itrans),
        ("ShaSh", Itrans),
        ("shhashh", Itrans),
        (".akarot", Itrans),
        ("shRRiNoti", Itrans),
        ("j~nAna", Itrans),
        // SLP1
        // ----
        ("kfta", Slp1),
        ("pitFn", Slp1),
        ("xfpta", Slp1),
        ("XkAra", Slp1),
        ("kEvalya", Slp1),
        ("kOsalya", Slp1),
        ("arGya", Slp1),
        ("aNka", Slp1),
        ("aNga", Slp1),
        ("CAyA", Slp1),
        ("jYAna", Slp1),
        ("kuwumba", Slp1),
        ("kaWora", Slp1),
        ("qamaru", Slp1),
        ("soQA", Slp1),
        ("pARqava", Slp1),
        ("Pala", Slp1),
        ("Bara", Slp1),
        ("gacCati", Slp1),
        ("zaRmAsa", Slp1),
        ("SfRoti", Slp1),
        ("aSvatTAman", Slp1),
        ("yudDa", Slp1),
        // Velthuis
        // --------
        ("k.rta", Velthuis),
        ("pit.rrn", Velthuis),
        ("k.lipta", Velthuis),
        (".ll", Velthuis),
        ("sa.myoga", Velthuis),
        ("gomaya.h", Velthuis),
        ("a\"nga", Velthuis),
        ("ku.tumba", Velthuis),
        ("ka.thora", Velthuis),
        (".damaru", Velthuis),
        ("so.dhaa", Velthuis),
        ("aru.na", Velthuis),
        ("~sveta", Velthuis),
        (".sa.s", Velthuis),
    ];

    #[test]
    fn detect_basic() {
        for (input, expected) in TEST_CASES {
            assert_eq!(
                detect(input),
                Some(*expected),
                "\"{input}\" should be {expected:?}"
            );
        }
    }
}
