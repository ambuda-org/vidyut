use crate::autogen_schemes;
use wasm_bindgen::prelude::wasm_bindgen;

type Pair = (&'static str, &'static str);

/// Models the coverage of a given scheme.
///
/// This code is not part of any APIs or used internally. We use it only to record the strength of
/// different schemes.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(crate) enum Coverage {
    /// Supports all Indic sounds.
    Complete,

    /// Supports all sounds used in Classical Sanskrit.
    Classical,

    /// Missing one or more classical Sanskrit sounds.
    Partial,

    /// Not classified.
    Unknown,
}

/// A method of encoding text.
///
/// Schemes vary on various dimensions, including:
///
/// - writing system (alphabet vs. abugida)
/// - text encoding (ASCII vs. Unicode)
/// - support for Sanskrit (complete vs. partial)
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[wasm_bindgen]
pub enum Scheme {
    /// Assamese script.
    ///
    /// The Assamese script uses the same characters as the Bengali script, with a few minor
    /// differences.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0980.pdf>
    Assamese,

    /// Balinese script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1B00.pdf>
    Balinese,

    /// Bengali script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0980.pdf>
    Bengali,

    /// Bhaiksuki script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11C00.pdf>
    Bhaiksuki,

    /// Brahmi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11000.pdf>
    Brahmi,

    /// Burmese script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1000.pdf>
    Burmese,

    /// Cham script.
    ///
    /// <https://unicode.org/charts/PDF/UAA00.pdf>
    Cham,

    /// Devanagari script.
    ///
    /// Docs:
    /// - <https://unicode.org/charts/PDF/U0900.pdf>
    /// - <https://unicode.org/charts/PDF/UA8E0.pdf> (Devanagari Extended)
    /// - <https://unicode.org/charts/PDF/U11B00.pdf> (Devanagari Extended-A)
    /// - <https://unicode.org/charts/PDF/U1CD0.pdf> (Vedic Extensions)
    Devanagari,

    /// Dogra script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11800.pdf>
    Dogra,

    /// Grantha script.
    ///
    /// Docs:
    /// - <http://www.unicode.org/charts/PDF/U11300.pdf>
    /// - <https://unicode.org/L2/L2009/09372-grantha.pdf>
    Grantha,

    /// Gujarati script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0A80.pdf>
    Gujarati,

    /// Gunjala Gondi script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11D60.pdf>
    GunjalaGondi,

    /// Gurmukhi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0A00.pdf>
    Gurmukhi,

    /// Javanese script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/UA980.pdf>
    Javanese,

    /// Kaithi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11080.pdf>
    Kaithi,

    /// Kannada script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0C80.pdf>
    Kannada,

    /// Kharoshthi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U10A00.pdf>
    Kharoshthi,

    /// Khmer script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1780.pdf>
    Khmer,

    /// Khudawadi script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U112B0.pdf>
    Khudawadi,

    /// Lepcha script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1C00.pdf>
    // Lepcha,

    /// Limbu script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U1900.pdf>
    Limbu,

    /// Mahajani script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11150.pdf>
    /// Mahajani,

    /// Malayalam script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0D00.pdf>
    Malayalam,

    /// Meetei script, known as Meetei Mayek.
    ///
    /// Docs: <https://unicode.org/charts/PDF/UABC0.pdf>
    MeeteiMayek,

    /// Masaram Gondi script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11D00.pdf>
    MasaramGondi,

    /// Modi script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11600.pdf>
    Modi,

    /// Mon script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1000.pdf>
    Mon,

    /// Lao script.
    ///
    /// Docs:
    /// - <https://unicode.org/charts/PDF/U0E80.pdf>
    /// - <https://www.unicode.org/wg2/docs/n4861-17106r-lao-for-pali.pdf>
    // Lao,

    /// Nandinagari script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U119A0.pdf>
    Nandinagari,

    /// Newa script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11400.pdf>
    Newa,

    /// Odia script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0B00.pdf>
    Odia,

    /// Ol Chiki (Santali) script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U1C50.pdf>
    OlChiki,

    /// `Phags-pa script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/UA840.pdf>
    // PhagsPa,

    /// Saurashtra script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/UA880.pdf>
    Saurashtra,

    /// Sharada script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11180.pdf>
    Sharada,

    /// Siddham script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11580.pdf>
    Siddham,

    /// Sinhala script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0D80.pdf>
    Sinhala,

    /// Soyombo script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U11A50.pdf>
    Soyombo,

    /// Tai Tham script (Lanna)
    ///
    /// Docs: <https://unicode.org/charts/PDF/U1A20.pdf>
    TaiTham,

    /// Takri script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11680.pdf>
    Takri,

    /// Tamil script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0B80.pdf>
    Tamil,

    /// Telugu script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0C00.pdf>
    Telugu,

    /// Thai script.
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0E00.pdf>
    Thai,

    /// Tibetan script.
    ///
    /// **Status: buggy and partial.**
    ///
    /// Docs: <https://unicode.org/charts/PDF/U0F00.pdf>
    Tibetan,

    /// Tirhuta script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11480.pdf>
    Tirhuta,

    /// Zanabazar Square script.
    ///
    /// Docs: <https://www.unicode.org/charts/PDF/U11A00.pdf>
    ZanabazarSquare,

    /// Baraha transliteration.
    ///
    /// Docs:
    /// - <https://baraha.com/help//Keyboards/dev-phonetic.htm> (Baraha North)
    /// - <https://baraha.com/help/special-symbols.htm>
    BarahaSouth,

    /// Harvard-Kyoto transliteration.
    ///
    /// TODO: find documentation link for HK.
    HarvardKyoto,

    /// IAST transliteration.
    ///
    /// TODO: find documentation link for IAST.
    Iast,

    /// ISO 15919 transliteration.
    ///
    /// TODO: find a free documentation link for ISO 15919.
    Iso15919,

    /// ITRANS 5.3 transliteration.
    ///
    /// Docs:
    /// - <https://www.aczoom.com/itrans/> (official ITRANS site for version 5.3)
    /// - <https://www.aczoom.com/itrans/html/dvng/node3.html> (DEVNAG table)
    /// - <http://www.sanskritweb.net/itrans/itmanual2003.pdf> (Itranslator 2003 manual)
    ///
    /// ITRANS appears in various versions, some of which conflict with each other. Version 5.3
    /// seems to be the most widely used, and it is supported by software like Itranslator 2003.
    Itrans,

    /// SLP1 transliteration.
    ///
    /// Docs: <https://www.sanskritlibrary.org/pub/SLP1LiesAppendixB.pdf>
    Slp1,

    /// Velthuis transliteration.
    ///
    /// Docs: <https://mirrors.mit.edu/CTAN/language/devanagari/velthuis/doc/manual.pdf>
    Velthuis,

    /// WX transliteration.
    ///
    /// TODO: find documentation link for WX.
    Wx,
}

impl Scheme {
    /// Returns an iterator over all available `Scheme`s.
    ///
    /// We guarantee that all pre-defined `Scheme`s will be present exactly once. However, we make
    /// no guarantees on iteration order.
    pub fn iter() -> impl Iterator<Item = &'static Scheme> {
        use Scheme::*;
        const SCHEMES: &[Scheme] = &[
            Assamese,
            Balinese,
            BarahaSouth,
            Bengali,
            Bhaiksuki,
            Brahmi,
            Burmese,
            Cham,
            Devanagari,
            Dogra,
            Grantha,
            Gujarati,
            GunjalaGondi,
            Gurmukhi,
            HarvardKyoto,
            Iast,
            Iso15919,
            Itrans,
            Javanese,
            Kaithi,
            Kannada,
            Kharoshthi,
            Khmer,
            Khudawadi,
            Limbu,
            Malayalam,
            MasaramGondi,
            MeeteiMayek,
            Modi,
            Mon,
            Nandinagari,
            Newa,
            Odia,
            OlChiki,
            Saurashtra,
            Sharada,
            Siddham,
            Sinhala,
            Slp1,
            Soyombo,
            TaiTham,
            Takri,
            Tamil,
            Telugu,
            Thai,
            Tibetan,
            Tirhuta,
            Velthuis,
            Wx,
            ZanabazarSquare,
        ];
        SCHEMES.iter()
    }

    pub(crate) fn token_pairs(&self) -> &[Pair] {
        use autogen_schemes as auto;

        match self {
            // Abugidas
            Scheme::Assamese => auto::ASSAMESE,
            Scheme::Balinese => auto::BALINESE,
            Scheme::Bengali => auto::BENGALI,
            Scheme::Bhaiksuki => auto::BHAIKSUKI,
            Scheme::Brahmi => auto::BRAHMI,
            Scheme::Burmese => auto::BURMESE,
            Scheme::Cham => auto::CHAM,
            Scheme::Devanagari => auto::DEVANAGARI,
            Scheme::Dogra => auto::DOGRA,
            Scheme::Grantha => auto::GRANTHA,
            Scheme::Gujarati => auto::GUJARATI,
            Scheme::GunjalaGondi => auto::GUNJALA_GONDI,
            Scheme::Gurmukhi => auto::GURMUKHI,
            Scheme::Javanese => auto::JAVANESE,
            Scheme::Kaithi => auto::KAITHI,
            Scheme::Kannada => auto::KANNADA,
            Scheme::Kharoshthi => auto::KHAROSHTHI,
            Scheme::Khmer => auto::KHMER,
            Scheme::Khudawadi => auto::KHUDAWADI,
            // Scheme::Lao => auto::LAO,
            // Scheme::Lepcha => auto::LEPCHA,
            Scheme::Limbu => auto::LIMBU,
            Scheme::Malayalam => auto::MALAYALAM,
            Scheme::MeeteiMayek => auto::MEETEI_MAYEK,
            Scheme::MasaramGondi => auto::MASARAM_GONDI,
            Scheme::Modi => auto::MODI,
            Scheme::Mon => auto::MON,
            Scheme::Nandinagari => auto::NANDINAGARI,
            Scheme::Newa => auto::NEWA,
            Scheme::Odia => auto::ORIYA,
            Scheme::OlChiki => auto::OL_CHIKI,
            Scheme::Saurashtra => auto::SAURASHTRA,
            Scheme::Sharada => auto::SHARADA,
            Scheme::Siddham => auto::SIDDHAM,
            Scheme::Sinhala => auto::SINHALA,
            Scheme::Soyombo => auto::SOYOMBO,
            Scheme::TaiTham => auto::TAI_THAM,
            Scheme::Takri => auto::TAKRI,
            Scheme::Tamil => auto::TAMIL,
            Scheme::Telugu => auto::TELUGU,
            Scheme::Thai => auto::THAI,
            Scheme::Tibetan => auto::TIBETAN,
            Scheme::Tirhuta => auto::TIRHUTA,
            Scheme::ZanabazarSquare => auto::ZANABAZAR_SQUARE,

            // Alphabets
            Scheme::BarahaSouth => auto::BARAHA,
            Scheme::HarvardKyoto => auto::HK,
            Scheme::Iast => auto::IAST,
            Scheme::Iso15919 => auto::ISO_15919,
            Scheme::Itrans => auto::ITRANS,
            Scheme::Slp1 => auto::SLP1,
            Scheme::Velthuis => auto::VELTHUIS,
            Scheme::Wx => auto::WX,
        }
    }

    /// Returns a map from tokens to their NFD forms.
    ///
    /// (NFD = Unicode normal form canonical decomposition)
    pub(crate) fn unicode_nfd_pairs(&self) -> &[Pair] {
        use crate::unicode_norm as u;
        use Scheme::*;

        match self {
            Balinese => u::BALINESE_NFD,
            Bengali => u::BENGALI_NFD,
            Burmese => u::MYANMAR_NFD,
            Devanagari => u::DEVANAGARI_NFD,
            Grantha => u::GRANTHA_NFD,
            Gurmukhi => u::GURMUKHI_NFD,
            Kaithi => u::KAITHI_NFD,
            Kannada => u::KANNADA_NFD,
            Malayalam => u::MALAYALAM_NFD,
            Odia => u::ORIYA_NFD,
            Siddham => u::SIDDHAM_NFD,
            Sinhala => u::SINHALA_NFD,
            Tamil => u::TAMIL_NFD,
            Telugu => u::TELUGU_NFD,
            Tirhuta => u::TIRHUTA_NFD,
            Iast | Iso15919 => u::LATIN_NFD,
            _ => &[],
        }
    }

    pub(crate) fn unicode_composition_exclusions(&self) -> &[&str] {
        use crate::unicode_norm as u;
        use Scheme::*;

        match self {
            Devanagari => u::DEVANAGARI_COMPOSITION_EXCLUSIONS,
            Bengali => u::BENGALI_COMPOSITION_EXCLUSIONS,
            Gurmukhi => u::GURMUKHI_COMPOSITION_EXCLUSIONS,
            Odia => u::ORIYA_COMPOSITION_EXCLUSIONS,
            _ => &[],
        }
    }

    /// Returns whether this scheme represents an abugida.
    ///
    /// A writing system is an *abugida* if consonants have an inherent vowel sound associated with
    /// them by default. For example, the Devanagari symbol *क* represents both the consonant "k"
    /// and the vowel "a." To represent just the consonate "k", we must add a separate symbol
    /// called the *virama* after the consonant, as in *क्*.
    ///
    /// As of all now, all of the abugidas we use are descended from the Brahmi script.
    pub(crate) fn is_abugida(&self) -> bool {
        use Scheme::*;

        // Use an exhaustive match (no `_`) so that we explicitly account for all schemes.
        match self {
            // Abugidas are all `true`.
            Assamese | Balinese | Bengali | Bhaiksuki | Brahmi | Burmese | Cham | Devanagari
            | Dogra | Grantha | Gujarati | GunjalaGondi | Gurmukhi | Javanese | Kaithi
            | Kannada | Khmer | Kharoshthi | Khudawadi | Limbu | Malayalam | MasaramGondi
            | MeeteiMayek | Modi | Mon | Nandinagari | Newa | Odia | Saurashtra | Sharada
            | Siddham | Sinhala | Soyombo | TaiTham | Takri | Tamil | Telugu | Thai | Tibetan
            | Tirhuta | ZanabazarSquare => true,

            // Alphabets are all `false`.
            BarahaSouth | HarvardKyoto | Iso15919 | Itrans | Iast | OlChiki | Slp1 | Velthuis
            | Wx => false,
        }
    }

    /// Returns whether this scheme represents an alphabet.
    ///
    /// A writing system is an *alphabet* if
    pub(crate) fn is_alphabet(&self) -> bool {
        !self.is_abugida()
    }

    /// Returns whether the scheme uses non-decimal numerals.
    ///
    /// Currently, our only scheme of this kind is `Grantha`.
    pub(crate) fn has_non_decimal_numerals(&self) -> bool {
        matches!(self, Scheme::Grantha)
    }

    /// Returns how well this scheme support Sanskrit.
    #[allow(unused)]
    pub(crate) fn coverage(&self) -> Coverage {
        use Coverage::*;
        use Scheme::*;

        match self {
            Balinese => Classical,
            Brahmi => Classical,
            Burmese => Classical,
            Devanagari => Complete,
            Grantha => Classical,
            Gujarati => Classical,
            Gurmukhi => Classical,
            Javanese => Classical,
            Kannada => Classical,
            Malayalam => Classical,
            Newa => Classical,
            Odia => Classical,
            Saurashtra => Classical,
            Sharada => Classical,
            Sinhala => Classical,
            Telugu => Classical,

            Bengali | Tamil => Partial,
            Siddham => Partial,

            _ => Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_normalization::UnicodeNormalization;

    #[test]
    fn iter_contains_all_defined_schemes() {
        use Scheme::*;

        let actual: Vec<_> = Scheme::iter().collect();
        let mut expected = Vec::new();
        for s in &actual {
            // Use an explicit `match` so that we are forced to account for all `Scheme`s.
            //
            // Don't use `_`, as that would defeat the point of this test.
            match s {
                Assamese | Balinese | BarahaSouth | Bengali | Bhaiksuki | Brahmi | Burmese
                | Cham | Devanagari | Dogra | GunjalaGondi | Grantha | Gujarati | Gurmukhi
                | HarvardKyoto | Iast | Iso15919 | Itrans | Javanese | Kaithi | Kannada
                | Kharoshthi | Khmer | Khudawadi | Limbu | Malayalam | MasaramGondi
                | MeeteiMayek | Nandinagari | Modi | Mon | Newa | Odia | OlChiki | Saurashtra
                | Sharada | Siddham | Sinhala | Slp1 | Soyombo | TaiTham | Takri | Tamil
                | Telugu | Thai | Tibetan | Tirhuta | Velthuis | Wx | ZanabazarSquare => {
                    expected.push(*s);
                }
            }
        }
        assert_eq!(expected, actual);
    }

    /// Checks basic token pairs.
    #[test]
    fn token_pairs_basic() {
        let mark_aa = "\u{093e}";

        let slp1 = Scheme::Slp1.token_pairs();
        assert!(slp1.contains(&("आ", "A")));
        assert!(slp1.contains(&(mark_aa, "A")));

        let hk = Scheme::HarvardKyoto.token_pairs();
        assert!(hk.contains(&("आ", "A")));
        assert!(hk.contains(&(mark_aa, "A")));

        let deva = Scheme::Devanagari.token_pairs();
        assert!(deva.contains(&("आ", "आ")));
        assert!(deva.contains(&(mark_aa, mark_aa)));
    }

    /// Checks that all tokens are encoded in NFC.
    ///
    /// This is just a sanity check to ensure that our default schemes are somewhat well-formed.
    #[test]
    fn token_pairs_are_all_nfc() {
        for scheme in Scheme::iter() {
            for (key, value) in scheme.token_pairs() {
                assert!(
                    !value.contains('\u{25cc}'),
                    "{value} contains the dreaded 25cc"
                );

                let key_nfc: String = key.nfc().collect();
                let value_nfc: String = value.nfc().collect();
                assert_eq!(&key_nfc, key);
                assert_eq!(&value_nfc, value);
            }
        }
    }

    /// Checks that token pairs don't contain repeated pairs.
    ///
    /// This is just a sanity check to ensure that our default schemes are somewhat well-formed.
    #[test]
    fn token_pairs_have_no_repeated_pairs() {
        for scheme in Scheme::iter() {
            let mut seen = std::collections::HashSet::new();
            for pair in scheme.token_pairs() {
                let key_codes: Vec<_> = pair.0.chars().map(|c| c as u32).collect();
                let value_codes: Vec<_> = pair.1.chars().map(|c| c as u32).collect();
                assert!(
                    !seen.contains(pair),
                    "Token pair {pair:?} ({key_codes:x?} --> {value_codes:x?}) already exists in scheme {scheme:?}"
                );
                seen.insert(pair);
            }
        }
    }

    #[test]
    fn is_abugida_or_alphabet() {
        for s in Scheme::iter() {
            assert!(s.is_abugida() != s.is_alphabet());
        }
    }

    /// Not used anywhere yet.
    #[test]
    fn coverage() {
        use Scheme::*;
        assert_eq!(Devanagari.coverage(), Coverage::Complete);
        assert_eq!(Kannada.coverage(), Coverage::Classical);
        assert_eq!(Bengali.coverage(), Coverage::Partial);
    }
}
