use crate::autogen_schemes;
use crate::errors::LipiError;
use wasm_bindgen::prelude::wasm_bindgen;

type Pair = (&'static str, &'static str);

/// Models the coverage of a given scheme.
///
/// This code is not part of any APIs or used internally. We use it only to record the strength of
/// different schemes.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
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
    /// Docs:
    /// - <https://mirrors.mit.edu/CTAN/language/devanagari/velthuis/doc/manual.pdf> (Devanagari)
    /// - <https://ctan.math.illinois.edu/language/bengali/pandey/doc/bengdoc.pdf> (Bengali)
    Velthuis,

    /// WX transliteration.
    ///
    /// TODO: find documentation link for WX.
    Wx,
}

impl Scheme {
    /// Converts the given scheme to its ISO 15924 four-letter code, if one exists.
    ///
    /// ### Usage
    ///
    /// ```
    /// use vidyut_lipi::Scheme;
    ///
    /// assert_eq!(Scheme::Devanagari.iso_15924_code(), "Deva");
    /// ```
    pub fn iso_15924_code(&self) -> &str {
        use Scheme::*;
        match self {
            Assamese => "Beng",
            Balinese => "Bali",
            BarahaSouth => "Latn",
            Bengali => "Beng",
            Bhaiksuki => "Bhks",
            Brahmi => "Brah",
            Burmese => "Mymr",
            Cham => "Cham",
            Devanagari => "Deva",
            Dogra => "Dogr",
            Grantha => "Gran",
            Gujarati => "Gujr",
            GunjalaGondi => "Gong",
            Gurmukhi => "Guru",
            HarvardKyoto => "Latn",
            Iast => "Latn",
            Iso15919 => "Latn",
            Itrans => "Latn",
            Javanese => "Java",
            Kaithi => "Kthi",
            Kannada => "Knda",
            Kharoshthi => "Khar",
            Khmer => "Khmr",
            Khudawadi => "Sind",
            Limbu => "Limb",
            Malayalam => "Mlym",
            MasaramGondi => "Gonm",
            MeeteiMayek => "Mtei",
            Modi => "Modi",
            Mon => "Mymr",
            Nandinagari => "Nand",
            Newa => "Newa",
            Odia => "Orya",
            OlChiki => "Olck",
            Saurashtra => "Saur",
            Sharada => "Shrd",
            Siddham => "Sidd",
            Sinhala => "Sinh",
            Slp1 => "Latn",
            Soyombo => "Soyo",
            TaiTham => "Lana",
            Takri => "Takr",
            Tamil => "Taml",
            Telugu => "Telu",
            Thai => "Thai",
            Tibetan => "Tibt",
            Tirhuta => "Tirh",
            Velthuis => "Latn",
            Wx => "Latn",
            ZanabazarSquare => "Zanb",
        }
    }

    /// Converts the given scheme to its ISO 15924 numeric code, if one exists.
    ///
    /// ### Usage
    ///
    /// ```
    /// use vidyut_lipi::Scheme;
    ///
    /// assert_eq!(Scheme::Devanagari.iso_15924_numeric_code(), 315);
    /// ```
    pub fn iso_15924_numeric_code(&self) -> u16 {
        use Scheme::*;
        match self {
            Assamese => 325,
            Balinese => 360,
            BarahaSouth => 215,
            Bengali => 325,
            Bhaiksuki => 334,
            Brahmi => 300,
            Burmese => 350,
            Cham => 358,
            Devanagari => 315,
            Dogra => 328,
            Grantha => 343,
            Gujarati => 320,
            GunjalaGondi => 312,
            Gurmukhi => 310,
            HarvardKyoto => 215,
            Iast => 215,
            Iso15919 => 215,
            Itrans => 215,
            Javanese => 361,
            Kaithi => 317,
            Kannada => 345,
            Kharoshthi => 305,
            Khmer => 355,
            Khudawadi => 318,
            Limbu => 336,
            Malayalam => 347,
            MasaramGondi => 313,
            MeeteiMayek => 337,
            Modi => 324,
            Mon => 350,
            Nandinagari => 311,
            Newa => 333,
            Odia => 327,
            OlChiki => 261,
            Saurashtra => 344,
            Sharada => 319,
            Siddham => 302,
            Sinhala => 348,
            Slp1 => 215,
            Soyombo => 329,
            TaiTham => 351,
            Takri => 321,
            Tamil => 346,
            Telugu => 340,
            Thai => 352,
            Tibetan => 330,
            Tirhuta => 326,
            Velthuis => 215,
            Wx => 215,
            ZanabazarSquare => 339,
        }
    }

    /// Converts the given scheme to its ICU code, if one exists.
    ///
    /// ICU codes are defined in conformance with the values in the ICU4X `Script` impl [here][1].
    ///
    /// [1]: https://github.com/unicode-org/icu4x/tree/main/components/properties/src/props.rs
    ///
    /// ### Usage
    ///
    /// ```
    /// use vidyut_lipi::Scheme;
    ///
    /// assert_eq!(Scheme::Devanagari.icu_numeric_code(), 10);
    /// ```
    pub fn icu_numeric_code(&self) -> u16 {
        use Scheme::*;
        match self {
            Assamese => 4,
            Balinese => 62,
            BarahaSouth => 25,
            Bengali => 4,
            Bhaiksuki => 168,
            Brahmi => 65,
            Burmese => 28,
            Cham => 66,
            Devanagari => 10,
            Dogra => 178,
            Grantha => 137,
            Gujarati => 15,
            GunjalaGondi => 179,
            Gurmukhi => 16,
            HarvardKyoto => 25,
            Iast => 25,
            Iso15919 => 25,
            Itrans => 25,
            Javanese => 78,
            Kaithi => 120,
            Kannada => 21,
            Kharoshthi => 57,
            Khmer => 23,
            Khudawadi => 145,
            Limbu => 48,
            Malayalam => 26,
            MasaramGondi => 175,
            MeeteiMayek => 115,
            Modi => 163,
            Mon => 28,
            Nandinagari => 187,
            Newa => 170,
            Odia => 31,
            OlChiki => 109,
            Saurashtra => 111,
            Sharada => 151,
            Siddham => 166,
            Sinhala => 33,
            Slp1 => 25,
            Soyombo => 176,
            TaiTham => 106,
            Takri => 153,
            Tamil => 35,
            Telugu => 36,
            Thai => 38,
            Tibetan => 39,
            Tirhuta => 158,
            Velthuis => 25,
            Wx => 25,
            ZanabazarSquare => 177,
        }
    }

    pub(crate) fn token_pairs(&self) -> &[Pair] {
        use autogen_schemes as auto;
        use Scheme::*;

        match self {
            Assamese => auto::ASSAMESE,
            Balinese => auto::BALINESE,
            BarahaSouth => auto::BARAHA,
            Bengali => auto::BENGALI,
            Bhaiksuki => auto::BHAIKSUKI,
            Brahmi => auto::BRAHMI,
            Burmese => auto::BURMESE,
            Cham => auto::CHAM,
            Devanagari => auto::DEVANAGARI,
            Dogra => auto::DOGRA,
            Grantha => auto::GRANTHA,
            Gujarati => auto::GUJARATI,
            GunjalaGondi => auto::GUNJALA_GONDI,
            Gurmukhi => auto::GURMUKHI,
            HarvardKyoto => auto::HK,
            Iast => auto::IAST,
            Iso15919 => auto::ISO_15919,
            Itrans => auto::ITRANS,
            Javanese => auto::JAVANESE,
            Kaithi => auto::KAITHI,
            Kannada => auto::KANNADA,
            Kharoshthi => auto::KHAROSHTHI,
            Khmer => auto::KHMER,
            Khudawadi => auto::KHUDAWADI,
            Limbu => auto::LIMBU,
            Malayalam => auto::MALAYALAM,
            MasaramGondi => auto::MASARAM_GONDI,
            MeeteiMayek => auto::MEETEI_MAYEK,
            Modi => auto::MODI,
            Mon => auto::MON,
            Nandinagari => auto::NANDINAGARI,
            Newa => auto::NEWA,
            Odia => auto::ORIYA,
            OlChiki => auto::OL_CHIKI,
            Saurashtra => auto::SAURASHTRA,
            Sharada => auto::SHARADA,
            Siddham => auto::SIDDHAM,
            Sinhala => auto::SINHALA,
            Slp1 => auto::SLP1,
            Soyombo => auto::SOYOMBO,
            TaiTham => auto::TAI_THAM,
            Takri => auto::TAKRI,
            Tamil => auto::TAMIL,
            Telugu => auto::TELUGU,
            Thai => auto::THAI,
            Tibetan => auto::TIBETAN,
            Tirhuta => auto::TIRHUTA,
            Velthuis => auto::VELTHUIS,
            Wx => auto::WX,
            ZanabazarSquare => auto::ZANABAZAR_SQUARE,
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

    #[cfg(target_arch = "wasm32")]
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

/// Defines various Scheme utils without the boilerplate.
macro_rules! scheme_utils {
    [$( $variant:ident ),*] => {
        impl Scheme {
            /// Returns an iterator over all available `Scheme`s.
            ///
            /// We guarantee that all pre-defined `Scheme`s will be present exactly once. However, we make
            /// no guarantees on iteration order.
            ///
            /// ### Usage
            ///
            /// ```rust,ignore
            /// use vidyut_lipi::Scheme;
            ///
            /// for scheme in Scheme::iter() {
            ///     println!("- {scheme}");
            /// }
            /// ```
            pub fn iter() -> impl Iterator<Item = &'static Scheme> {
                use Scheme::*;
                const SCHEMES: &[Scheme] = &[
                    $(
                        $variant,
                    )*
                ];
                SCHEMES.iter()
            }

            /// Returns a string representation of the given `Scheme`.
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        Self::$variant => stringify!($variant),
                    )*
                }
            }
        }

        impl std::str::FromStr for Scheme {
            type Err = LipiError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($variant) => Ok(Self::$variant),
                    )*
                    _ => Err(LipiError::ParseError)
                }
            }
        }
    }
}

scheme_utils![
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
    ZanabazarSquare
];

#[cfg(test)]
mod tests {
    use super::*;
    use codes_iso_15924::ALL_CODES;
    use unicode_normalization::UnicodeNormalization;

    #[test]
    fn as_str_round_trip() {
        for scheme in Scheme::iter() {
            let text = scheme.as_str();
            let from: Scheme = text.parse().expect("defined");
            assert_eq!(*scheme, from);
        }
    }

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
    fn is_abugida_xor_is_alphabet() {
        for s in Scheme::iter() {
            assert!(s.is_abugida() != s.is_alphabet());
        }
    }

    #[test]
    fn iso_15924_codes() {
        for expected in Scheme::iter() {
            let code = expected.iso_15924_code();
            assert!(
                ALL_CODES.iter().find(|c| c.code() == code).is_some(),
                "{code} is not a valid code."
            );
        }
    }

    #[test]
    fn iso_15924_numeric_codes() {
        for expected in Scheme::iter() {
            let num = expected.iso_15924_numeric_code();
            assert!(
                ALL_CODES.iter().find(|c| c.numeric_code() == num).is_some(),
                "{num} is not a valid numeric code."
            );
        }
    }

    // Checks that schemes with identical ISO codes also have identical ICU codes.
    #[test]
    fn icu_numeric_codes_agree_with_iso_numeric_codes() {
        use std::collections::HashMap;
        let mut map: HashMap<u16, u16> = HashMap::new();

        for s in Scheme::iter() {
            let key = s.iso_15924_numeric_code();
            if let Some(seen_icu_numeric_code) = map.get(&key) {
                assert_eq!(*seen_icu_numeric_code, s.icu_numeric_code());
            } else {
                map.insert(key, s.icu_numeric_code());
            }
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
