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
    /// Balinese script.
    ///
    /// https://unicode.org/charts/PDF/U1B00.pdf
    Balinese,

    /// Bengali script.
    ///
    /// https://unicode.org/charts/PDF/U0980.pdf
    Bengali,

    /// Burmese script.
    ///
    /// https://unicode.org/charts/PDF/U1000.pdf
    Burmese,

    /// Brahmi script.
    ///
    /// https://unicode.org/charts/PDF/U11000.pdf
    Brahmi,

    /// Devanagari script.
    ///
    /// https://unicode.org/charts/PDF/U0900.pdf
    Devanagari,

    /// Gujarati script.
    ///
    /// https://unicode.org/charts/PDF/U0A80.pdf
    Gujarati,

    /// Gurmukhi script.
    ///
    /// https://unicode.org/charts/PDF/U0A00.pdf
    Gurmukhi,

    /// Grantha script.
    ///
    /// http://www.unicode.org/charts/PDF/U11300.pdf
    Grantha,

    /// Javanese script.
    ///
    /// https://unicode.org/charts/PDF/UA980.pdf
    Javanese,

    /// Kannada script.
    ///
    /// https://unicode.org/charts/PDF/U0C80.pdf
    Kannada,

    /// Malayalam script.
    ///
    /// https://unicode.org/charts/PDF/U0D00.pdf
    Malayalam,

    /// Odia script.
    ///
    /// https://unicode.org/charts/PDF/U0B00.pdf
    Odia,

    /// Sharada script.
    ///
    /// https://unicode.org/charts/PDF/U11180.pdf
    Sharada,

    /// Sinhala script.
    ///
    /// https://unicode.org/charts/PDF/U0D80.pdf
    Sinhala,

    /// Tamil script.
    ///
    /// https://unicode.org/charts/PDF/U0B80.pdf
    Tamil,

    /// Tibetan script.
    ///
    /// https://unicode.org/charts/PDF/U0F00.pdf
    // Tibetan,

    /// Telugu script.
    ///
    /// https://unicode.org/charts/PDF/U0C00.pdf
    Telugu,

    /// Harvard-Kyoto transliteration.
    ///
    /// TODO: find documentation link for HK.
    HarvardKyoto,

    /// IAST transliteration.
    ///
    /// TODO: find documentation link for IAST.
    Iast,

    /// ISO 19519 transliteration.
    ///
    /// TODO: find a free documentation link for ISO 19519.
    Iso19519,

    /// ITRANS transliteration.
    ///
    /// https://www.aczoom.com/itrans/online/itrans6/itrans-tables-unicode.pdf
    Itrans,

    /// SLP1 transliteration.
    ///
    /// https://www.sanskritlibrary.org/pub/SLP1LiesAppendixB.pdf
    Slp1,

    /// Velthuis transliteration.
    ///
    /// https://mirrors.mit.edu/CTAN/language/devanagari/velthuis/doc/manual.pdf
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
            Balinese,
            Bengali,
            Brahmi,
            Burmese,
            Devanagari,
            Grantha,
            Gujarati,
            Gurmukhi,
            HarvardKyoto,
            Iast,
            Itrans,
            Javanese,
            Kannada,
            Malayalam,
            Odia,
            Sharada,
            Sinhala,
            Slp1,
            Tamil,
            Telugu,
            Velthuis,
            Wx,
        ];
        SCHEMES.iter()
    }

    pub(crate) fn token_pairs(&self) -> &[Pair] {
        use autogen_schemes as auto;

        match self {
            Scheme::Balinese => auto::BALINESE,
            Scheme::Bengali => auto::BENGALI,
            Scheme::Brahmi => auto::BRAHMI,
            Scheme::Burmese => auto::BURMESE,
            Scheme::Devanagari => auto::DEVANAGARI,
            Scheme::Gujarati => auto::GUJARATI,
            Scheme::Gurmukhi => auto::GURMUKHI,
            Scheme::Grantha => auto::GRANTHA,
            Scheme::Javanese => auto::JAVANESE,
            Scheme::Kannada => auto::KANNADA,
            Scheme::Malayalam => auto::MALAYALAM,
            Scheme::Odia => auto::ORIYA,
            Scheme::Sharada => auto::SHARADA,
            Scheme::Sinhala => auto::SINHALA,
            Scheme::Tamil => auto::TAMIL,
            Scheme::Telugu => auto::TELUGU,
            // Scheme::Tibetan => auto::TIBETAN,
            Scheme::Slp1 => auto::SLP1,
            Scheme::HarvardKyoto => auto::HK,
            Scheme::Iast => auto::IAST,
            Scheme::Iso19519 => auto::ISO,
            Scheme::Itrans => auto::ITRANS,
            Scheme::Velthuis => auto::VELTHUIS,
            Scheme::Wx => auto::WX,
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
            Balinese | Bengali | Brahmi | Burmese | Devanagari | Gujarati | Gurmukhi | Grantha
            | Javanese | Kannada | Malayalam | Odia | Sharada | Sinhala | Tamil | Telugu => true,

            // Alphabets are all `false`.
            HarvardKyoto | Iso19519 | Itrans | Iast | Slp1 | Velthuis | Wx => false,
        }
    }

    /// Returns whether this scheme represents an alphabet.
    ///
    /// A writing system is an *alphabet* if
    pub(crate) fn is_alphabet(&self) -> bool {
        !self.is_abugida()
    }

    /// Returns how well this scheme support Sanskrit.
    #[allow(unused)]
    pub(crate) fn coverage(&self) -> Coverage {
        use Coverage::*;
        use Scheme::*;

        match self {
            Balinese => Classical,
            Bengali | Tamil => Partial,
            Brahmi => Classical,
            Burmese => Classical,
            Devanagari => Complete,
            Grantha => Classical,
            Gujarati => Classical,
            Gurmukhi => Classical,
            Javanese => Classical,
            Kannada => Classical,
            Malayalam => Classical,
            Odia => Classical,
            Sharada => Classical,
            Sinhala => Classical,
            Telugu => Classical,
            // Tibetan => Classical,
            _ => Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                Devanagari | Balinese | Bengali | Tamil | Brahmi | Burmese | Grantha | Gujarati
                | Gurmukhi | Javanese | Odia | Sharada | Kannada | Malayalam | Sinhala | Telugu
                | Itrans | HarvardKyoto | Slp1 | Velthuis | Iast | Wx | Iso19519 => {
                    expected.push(*s);
                }
            }
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn token_pairs() {
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

    #[test]
    fn is_abugida_or_alphabet() {
        use Scheme::*;
        let schemes = &[Devanagari, Kannada, Iast, Itrans];
        for s in schemes {
            assert!(s.is_abugida() != s.is_alphabet());
        }
    }

    #[test]
    fn coverage() {
        use Scheme::*;
        assert_eq!(Devanagari.coverage(), Coverage::Complete);
        assert_eq!(Kannada.coverage(), Coverage::Classical);
        assert_eq!(Bengali.coverage(), Coverage::Partial);
    }
}
