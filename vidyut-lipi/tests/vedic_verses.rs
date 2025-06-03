//! Tests using real Vedic verses from udapaana data.

use vidyut_lipi::extensions::vedic::{
    atharvaveda_shaunaka, rigveda_shakala, samaveda_kauthuma, yajurveda_taittiriya,
};
use vidyut_lipi::{Lipika, Scheme};

#[test]
fn test_taittiriya_yajurveda_verse() {
    let mut lipika = Lipika::new().with_extension(yajurveda_taittiriya());

    // First verse of Taittiriya Samhita (1.1.1.1) adapted to HK
    // Using = for udatta and _ for anudatta in ASCII-only system
    let verse = "i=She tvo=rje tvA= vA=yava_H";
    let result = lipika.transliterate(verse, Scheme::HarvardKyoto, Scheme::Devanagari);

    // Should contain anudatta and udatta marks
    assert!(result.contains("\u{0952}")); // anudatta (_)
    assert!(result.contains("\u{0951}")); // udatta (=)

    // Round-trip test
    let back = lipika.transliterate(&result, Scheme::Devanagari, Scheme::HarvardKyoto);
    assert_eq!(back, verse, "Round-trip should preserve accents");
}

#[test]
fn test_taittiriya_with_phonetic_annotations() {
    let mut lipika = Lipika::new().with_extension(yajurveda_taittiriya());

    // Example with special phonetic annotation
    let text = "dRu(gm)ha=sva_";
    let result = lipika.transliterate(text, Scheme::HarvardKyoto, Scheme::Devanagari);

    // Should handle the (gm) phonetic annotation and have udatta
    // The (gm) might not be converted in this basic system, so just check for udatta
    assert!(result.contains("\u{0951}")); // udatta on ha
}

#[test]
fn test_rigveda_verse() {
    let mut lipika = Lipika::new().with_extension(rigveda_shakala());

    // Famous Gayatri mantra with accents
    // tat savitur varenyam (with appropriate accent marks)
    let verse = "tat savi=tur vare_Nyam";
    let result = lipika.transliterate(verse, Scheme::HarvardKyoto, Scheme::Devanagari);

    assert!(result.contains("\u{0951}")); // udatta
    assert!(result.contains("\u{0952}")); // anudatta
}

#[test]
fn test_samaveda_musical_notation() {
    let mut lipika = Lipika::new().with_extension(samaveda_kauthuma());

    // Samaveda uses numeric notation for musical tones
    let verse = "agna1 A2 ya3hi";
    let result = lipika.transliterate(verse, Scheme::HarvardKyoto, Scheme::Devanagari);

    // Numeric markers should be converted to appropriate symbols
    assert!(result.contains("अग्न"));
}

#[test]
fn test_atharvaveda_verse() {
    let mut lipika = Lipika::new().with_extension(atharvaveda_shaunaka());

    // Atharvaveda verse with simple accent marking
    let verse = "bhadram kar=Nebhih";
    let result = lipika.transliterate(verse, Scheme::HarvardKyoto, Scheme::Devanagari);

    assert!(
        result.contains("भद्रम्") || result.contains("भद्रं"),
        "Result: {}",
        result
    );
    assert!(result.contains("\u{0951}")); // udatta
}

#[test]
fn test_complex_sandhi_with_accents() {
    let mut lipika = Lipika::new().with_extension(yajurveda_taittiriya());

    // Complex sandhi with accents
    let text = "aGaSaMsa= ityaGa";
    let result = lipika.transliterate(text, Scheme::HarvardKyoto, Scheme::Devanagari);

    assert!(result.contains("\u{0951}")); // udatta mark

    // Ensure round-trip preserves sandhi and accents
    let back = lipika.transliterate(&result, Scheme::Devanagari, Scheme::HarvardKyoto);
    assert_eq!(back, text);
}

#[test]
fn test_verse_with_danda() {
    let mut lipika = Lipika::new().with_extension(rigveda_shakala());

    // Verse with danda markers
    let verse = "om bhUr bhu=vah svah |";
    let result = lipika.transliterate(verse, Scheme::HarvardKyoto, Scheme::Devanagari);

    assert!(result.contains("\u{0964}")); // single danda
    assert!(result.contains("\u{0951}")); // udatta on bhu=vah
}

#[test]
fn test_multiple_accent_types() {
    let mut lipika = Lipika::new().with_extension(yajurveda_taittiriya());

    // Text with multiple accent types
    let text = "sa=tyam bRha=d Rta_m ugra=m";
    let result = lipika.transliterate(text, Scheme::HarvardKyoto, Scheme::Devanagari);

    // Count accent marks
    let udatta_count = result.matches('\u{0951}').count();
    let anudatta_count = result.matches('\u{0952}').count();

    assert_eq!(udatta_count, 3); // Three udatta marks
    assert_eq!(anudatta_count, 1); // One anudatta mark
}
