//! Utilities for detecting the `Scheme` used by some text.

use crate::scheme::Scheme;

type Range = std::ops::RangeInclusive<char>;

// These are ranges of Unicode code points as defined by unicode.org. To see the official spec
// for each scheme, see the comments on `Scheme`.
const DEVANAGARI: Range = '\u{0900}'..='\u{097f}';
const DEVANAGARI_EXTENDED: Range = '\u{a8e0}'..='\u{a8ff}';
const DEVANAGARI_EXTENDED_A: Range = '\u{11b00}'..='\u{11b5f}';
const VEDIC_EXTENSIONS: Range = '\u{1cd0}'..='\u{1cff}';
const BENGALI: Range = '\u{0980}'..='\u{09ff}';
const GURMUKHI: Range = '\u{0a00}'..='\u{0a7f}';
const GUJARATI: Range = '\u{0a80}'..='\u{0aff}';
const ORIYA: Range = '\u{0b00}'..='\u{0b7f}';
const TAMIL: Range = '\u{0b80}'..='\u{0bff}';
const TELUGU: Range = '\u{0c00}'..='\u{0c7f}';
const KANNADA: Range = '\u{0c80}'..='\u{0cff}';
const MALAYALAM: Range = '\u{0d00}'..='\u{0d7f}';
const SINHALA: Range = '\u{0d80}'..='\u{0dff}';
const THAI: Range = '\u{0e00}'..='\u{0e7f}';
const TIBETAN: Range = '\u{0f00}'..='\u{0fff}';
const MYANMAR: Range = '\u{1000}'..='\u{109f}';
const TAI_THAM: Range = '\u{1a20}'..='\u{1aaf}';
const OL_CHIKI: Range = '\u{1c50}'..='\u{1c7f}';
const KHMER: Range = '\u{1780}'..='\u{17ff}';
const LIMBU: Range = '\u{1900}'..='\u{194f}';
const BALINESE: Range = '\u{1b00}'..='\u{1b7f}';
const SAURASHTRA: Range = '\u{a880}'..='\u{a8df}';
const JAVANESE: Range = '\u{a980}'..='\u{a9df}';
const CHAM: Range = '\u{aa00}'..='\u{aa5f}';
const MEETEI_MAYEK: Range = '\u{abc0}'..='\u{abff}';
const KHAROSHTHI: Range = '\u{10a00}'..='\u{10a5f}';
const BRAHMI: Range = '\u{11000}'..='\u{1107f}';
const KAITHI: Range = '\u{11080}'..='\u{110cf}';
const SHARADA: Range = '\u{11180}'..='\u{111df}';
const KHUDAWADI: Range = '\u{112b0}'..='\u{112ff}';
const GRANTHA: Range = '\u{11300}'..='\u{1137f}';
const NEWA: Range = '\u{11400}'..='\u{1147f}';
const TIRHUTA: Range = '\u{11480}'..='\u{114df}';
const SIDDHAM: Range = '\u{11580}'..='\u{115ff}';
const MODI: Range = '\u{11600}'..='\u{1165f}';
const TAKRI: Range = '\u{11680}'..='\u{116cf}';
const _AHOM: Range = '\u{11700}'..='\u{1174f}';
const DOGRA: Range = '\u{11800}'..='\u{1184f}';
const NANDINAGARI: Range = '\u{119a0}'..='\u{119ff}';
const ZANABAZAR_SQUARE: Range = '\u{11a00}'..='\u{11a4f}';
const SOYOMBO: Range = '\u{11a50}'..='\u{11aaf}';
const BHAIKSUKI: Range = '\u{11c00}'..='\u{11c6f}';
const MASARAM_GONDI: Range = '\u{11d00}'..='\u{11d5f}';
const GUNJALA_GONDI: Range = '\u{11d60}'..='\u{11daf}';

// Wraps all of the ranges above.
const INDIC: Range = *DEVANAGARI.start()..=*GUNJALA_GONDI.end();

/// Detects the scheme used by the given text.
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

    // The Latin blocks below are used by IAST, ISO-15919, etc.
    //
    // Docs:
    // - <https://unicode.org/charts/PDF/U0000.pdf>
    // - <https://unicode.org/charts/PDF/U0080.pdf>
    // - <https://unicode.org/charts/PDF/U0100.pdf>
    // - <https://unicode.org/charts/PDF/U1E00.pdf>
    const _LATIN_1_SUPPLEMENT: Range = '\u{0080}'..='\u{00ff}';
    const _LATIN_EXTENDED_A: Range = '\u{0100}'..='\u{017f}';
    const LATIN_EXTENDED: Range = '\u{01e00}'..='\u{01eff}';

    let input = crate::unicode_norm::to_nfc(input);

    let is_latin_text = !input
        .chars()
        .any(|c| INDIC.contains(&c) && !LATIN_EXTENDED.contains(&c));

    if is_latin_text {
        detect_latin(&input)
    } else {
        for c in input.chars() {
            // Rust supports [range matching][1], but only if the range is "inlined" and not in a
            // const. But having a bunch of inlined hex ranges (as opposed to our consts above) seems
            // unreadable, so just use an if-else chain.
            //
            // [1]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html

            let maybe = if DEVANAGARI.contains(&c)
                || DEVANAGARI_EXTENDED.contains(&c)
                || DEVANAGARI_EXTENDED_A.contains(&c)
                || VEDIC_EXTENSIONS.contains(&c)
            {
                Some(Devanagari)
            } else if BENGALI.contains(&c) {
                Some(Bengali)
            } else if GURMUKHI.contains(&c) {
                Some(Gurmukhi)
            } else if GUJARATI.contains(&c) {
                Some(Gujarati)
            } else if ORIYA.contains(&c) {
                Some(Odia)
            } else if TAMIL.contains(&c) {
                Some(Tamil)
            } else if TELUGU.contains(&c) {
                Some(Telugu)
            } else if KANNADA.contains(&c) {
                Some(Kannada)
            } else if MALAYALAM.contains(&c) {
                Some(Malayalam)
            } else if SINHALA.contains(&c) {
                Some(Sinhala)
            } else if THAI.contains(&c) {
                Some(Thai)
            } else if TIBETAN.contains(&c) {
                Some(Tibetan)
            } else if MYANMAR.contains(&c) {
                Some(Burmese)
            } else if TAI_THAM.contains(&c) {
                Some(TaiTham)
            } else if OL_CHIKI.contains(&c) {
                Some(OlChiki)
            } else if KHMER.contains(&c) {
                Some(Khmer)
            } else if LIMBU.contains(&c) {
                Some(Limbu)
            } else if BALINESE.contains(&c) {
                Some(Balinese)
            } else if SAURASHTRA.contains(&c) {
                Some(Saurashtra)
            } else if JAVANESE.contains(&c) {
                Some(Javanese)
            } else if CHAM.contains(&c) {
                Some(Cham)
            } else if MEETEI_MAYEK.contains(&c) {
                Some(MeeteiMayek)
            } else if KHAROSHTHI.contains(&c) {
                Some(Kharoshthi)
            } else if BRAHMI.contains(&c) {
                Some(Brahmi)
            } else if KAITHI.contains(&c) {
                Some(Kaithi)
            } else if SHARADA.contains(&c) {
                Some(Sharada)
            } else if KHUDAWADI.contains(&c) {
                Some(Khudawadi)
            } else if GRANTHA.contains(&c) {
                Some(Grantha)
            } else if NEWA.contains(&c) {
                Some(Newa)
            } else if TIRHUTA.contains(&c) {
                Some(Tirhuta)
            } else if SIDDHAM.contains(&c) {
                Some(Siddham)
            } else if MODI.contains(&c) {
                Some(Modi)
            } else if TAKRI.contains(&c) {
                Some(Takri)
            } else if DOGRA.contains(&c) {
                Some(Dogra)
            } else if NANDINAGARI.contains(&c) {
                Some(Nandinagari)
            } else if ZANABAZAR_SQUARE.contains(&c) {
                Some(ZanabazarSquare)
            } else if SOYOMBO.contains(&c) {
                Some(Soyombo)
            } else if BHAIKSUKI.contains(&c) {
                Some(Bhaiksuki)
            } else if MASARAM_GONDI.contains(&c) {
                Some(MasaramGondi)
            } else if GUNJALA_GONDI.contains(&c) {
                Some(GunjalaGondi)
            } else {
                None
            };
            if maybe.is_some() {
                return maybe;
            }
        }
        None
    }
}

fn detect_latin(input: &str) -> Option<Scheme> {
    fn is_iso_15919_only(c: char) -> bool {
        const COMBINING_RING: char = '\u{0325}';
        matches!(c, 'ē' | 'ō' | COMBINING_RING | 'ṁ' | 'ẖ' | 'ḫ')
    }

    use Scheme::*;
    if input.chars().all(|x| x.is_ascii()) {
        const ITRANS_ONLY_TRIGRAMS: &[&[u8]] = &[b"chh", b"RRi", b"RRI", b"LLi", b"LLI"];
        const ITRANS_ONLY_BIGRAMS: &[&[u8]] = &[
            b"ee", b"oo", b"^i", b"^I", b"Ch", b"JN", b"sh", b"Sh", b"~N", b".a", b"N^",
        ];
        const SLP1_ONLY_BIGRAMS: &[&[u8]] = &[
            b"kz", b"Nk", b"Ng", b"tT", b"dD", b"Sc", b"Sn", b"Gy", b"Gr", b"aR", b"AR", b"iR",
            b"IR", b"uR", b"UR", b"eR", b"oR",
        ];
        const VELTHUIS_ONLY_BIGRAMS: &[&[u8]] = &[b"\"n", b"~s"];

        for (i, c) in input.char_indices() {
            // Trigrams
            if i + 3 <= input.len() {
                let trigram = &input.as_bytes()[i..i + 3];
                debug_assert!(trigram.len() == 3, "All ASCII");

                if ITRANS_ONLY_TRIGRAMS.contains(&trigram) {
                    return Some(Itrans);
                } else if matches!(trigram, b"kRu" | b"~lu" | b"~lU") {
                    return Some(BarahaSouth);
                }
            }

            // Bigrams
            if i + 2 <= input.len() {
                let bigram = &input.as_bytes()[i..i + 2];
                debug_assert!(bigram.len() == 2, "All ASCII");

                if ITRANS_ONLY_BIGRAMS.contains(&bigram) {
                    return Some(Itrans);
                } else if SLP1_ONLY_BIGRAMS.contains(&bigram) {
                    return Some(Slp1);
                } else if VELTHUIS_ONLY_BIGRAMS.contains(&bigram)
                    || bigram[0] == b'.' && b"mhnrltds".contains(&bigram[1])
                {
                    return Some(Velthuis);
                } else if matches!(bigram[0], b'x' | b'X')
                    && matches!(
                        bigram[1],
                        b'a' | b'A'
                            | b'i'
                            | b'I'
                            | b'u'
                            | b'U'
                            | b'q'
                            | b'Q'
                            | b'e'
                            | b'E'
                            | b'o'
                            | b'O'
                    )
                {
                    return Some(Wx);
                }
            }

            if "fFxXEOCYwWqQPB".contains(c) {
                return Some(Slp1);
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

        Some(HarvardKyoto)
    } else if input.chars().any(is_iso_15919_only) {
        Some(Iso15919)
    } else {
        Some(Iast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lipika;
    use Scheme::*;

    const TEST_CASES: &[(Scheme, &[&str])] = &[
        (BarahaSouth, &["namaskRutya", "k~lupta", "~lUkAra"]),
        (
            Iast,
            &[
                "rāga",
                "nadī",
                "vadhū",
                "kṛta",
                "pitṝn",
                "kḷpta",
                "ḹ",
                "tejasvī",
                "gomayaḥ",
                "haṃsa",
                "naraḥ",
                "aṅga",
                "añjana",
                "kuṭumba",
                "kaṭhora",
                "ḍamaru",
                "soḍhā",
                "aruṇa",
                "śveta",
                "ṣaṣ",
                "ḻa",
                "pāṇḍava",
                "śṛṇoti",
                "jñāna",
            ],
        ),
        (
            Iso15919,
            &[
                "śr̥ṇōti",
                "pitr̥̄n",
                "kl̥pta",
                "l̥̄",
                "dēva",
                "yōga",
                "aṁ",
                "aḫ",
                "aẖ",
            ],
        ),
        (
            Itrans,
            &[
                "raaga",
                "nadii",
                "nadee",
                "vadhuu",
                "vadhoo",
                "kRRita",
                "kR^ita",
                "pitRRIn",
                "pitR^In",
                "kLLipta",
                "kL^ipta",
                "LLI",
                "L^i",
                "a~Nga",
                "aN^ga",
                "ChAyA",
                "chhAyA",
                "a~njana",
                "aJNjana",
                "shveta",
                "ShaSh",
                "shhashh",
                ".akarot",
                "shRRiNoti",
                "j~nAna",
            ],
        ),
        (
            Slp1,
            &[
                "kfta",
                "pitFn",
                "xfpta",
                "XkAra",
                "kEvalya",
                "kOsalya",
                "arGya",
                "aNka",
                "aNga",
                "CAyA",
                "jYAna",
                "kuwumba",
                "kaWora",
                "qamaru",
                "soQA",
                "pARqava",
                "Pala",
                "Bara",
                "gacCati",
                "zaRmAsa",
                "SfRoti",
                "aSvatTAman",
                "yudDa",
            ],
        ),
        (Wx, &["xeva", "Xarma"]),
        (
            Velthuis,
            &[
                "k.rta", "pit.rrn", "k.lipta", ".ll", "sa.myoga", "gomaya.h", "a\"nga", "ku.tumba",
                "ka.thora", ".damaru", "so.dhaa", "aru.na", "~sveta", ".sa.s",
            ],
        ),
    ];

    #[test]
    fn detect_basic() {
        for (scheme, inputs) in TEST_CASES {
            for input in *inputs {
                assert_eq!(
                    detect(input),
                    Some(*scheme),
                    "\"{input}\" should be {scheme:?}"
                );
            }
        }
    }

    #[test]
    fn detect_abugidas() {
        use Scheme::*;

        let schemes: Vec<Scheme> = Scheme::iter()
            .map(|s| *s)
            // Assamese is confused for Bengali, and Mon is confused for Burmese.
            .filter(|s| s.is_abugida() && !matches!(*s, Assamese | Mon))
            .collect();
        assert!(!schemes.is_empty());

        let mut lipika = Lipika::new();
        for scheme in schemes {
            let output = lipika.transliterate("saMskftam", Slp1, scheme);
            assert_eq!(
                detect(&output),
                Some(scheme),
                "\"{output}\" should be {scheme:?}"
            );
        }
    }
}
