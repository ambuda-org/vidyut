//! Tests for the extension system.

use vidyut_lipi::{Lipika, Scheme};
use vidyut_lipi::extensions::vedic::{rigveda_shakala, yajurveda_taittiriya};

#[test]
fn test_basic_transliteration_without_extension() {
    let mut lipika = Lipika::new();
    let result = lipika.transliterate("namaste", Scheme::HarvardKyoto, Scheme::Devanagari);
    assert_eq!(result, "नमस्ते");
}

#[test]
fn test_rigveda_extension() {
    let mut lipika = Lipika::new()
        .with_extension(rigveda_shakala());
    
    // Test basic text
    let result = lipika.transliterate("namaste", Scheme::HarvardKyoto, Scheme::Devanagari);
    assert_eq!(result, "नमस्ते");
    
    // Test with Vedic accents
    let result = lipika.transliterate("agniM", Scheme::Wx, Scheme::Devanagari);
    assert_eq!(result, "अग्निं");
    
    // Test with udatta accent
    let result = lipika.transliterate("agni'", Scheme::Wx, Scheme::Devanagari);
    assert_eq!(result, "अग्नि\u{0951}");  // अग्नि with udatta mark
    
    // Test with anudatta accent
    let result = lipika.transliterate("soma_", Scheme::Wx, Scheme::Devanagari);
    assert_eq!(result, "सोम\u{0952}");  // सोम with anudatta mark
}

#[test]
fn test_rigveda_pre_processing() {
    let mut lipika = Lipika::new()
        .with_extension(rigveda_shakala());
    
    // Test numeric accent conversion
    let result = lipika.transliterate("soma3", Scheme::Wx, Scheme::Devanagari);
    assert_eq!(result, "सोम\u{0951}");  // Numeric 3 converted to udatta
    
    let result = lipika.transliterate("inxra1", Scheme::Wx, Scheme::Devanagari);
    assert_eq!(result, "इन्द्र\u{0952}");  // Numeric 1 converted to anudatta
}

#[test]
fn test_extension_caching() {
    let mut lipika = Lipika::new()
        .with_extension(rigveda_shakala());
    
    // Multiple transliterations should use cached mapping
    for _ in 0..5 {
        let result = lipika.transliterate("agni'", Scheme::Wx, Scheme::Devanagari);
        assert_eq!(result, "अग्नि\u{0951}");
    }
}

#[test]
fn test_extension_with_different_schemes() {
    let mut lipika = Lipika::new()
        .with_extension(rigveda_shakala());
    
    // Test WX to IAST
    let result = lipika.transliterate("agni'", Scheme::Wx, Scheme::Iast);
    assert!(result.contains("agni"));  // Should preserve the base text
    
    // Test HK to Devanagari with accents
    let result = lipika.transliterate("agnI'", Scheme::HarvardKyoto, Scheme::Devanagari);
    assert!(result.contains("अग्नी"));
}

#[test]
fn test_round_trip_accent_preservation() {
    // Test cases with various accent patterns (updated to new ASCII-only WX)
    let test_cases = vec![
        "a=gni_m I=le puro=hitam",  // Multiple accents (lowercase L)
        "ya=jasya de=vam Rtvi=jam",  // Accents on different syllables
        "ho=tAram ratna=dhAtamam",   // Mixed case with accents
        "agni= agni_",               // Adjacent words with different accents
        "=indra_sya",                // Accent at word beginning
        "soma=",                     // Accent at word end
    ];
    
    // Test round-trip for each scheme pair
    let scheme_pairs = vec![
        (Scheme::HarvardKyoto, Scheme::Devanagari),
        (Scheme::HarvardKyoto, Scheme::Iast),
        (Scheme::Slp1, Scheme::Devanagari),
        (Scheme::Velthuis, Scheme::Devanagari),
    ];
    
    for (from_scheme, to_scheme) in scheme_pairs {
        for test_case in &test_cases {
            let mut forward_lipika = Lipika::new()
                .with_extension(rigveda_shakala());
            let mut reverse_lipika = Lipika::new()
                .with_extension(rigveda_shakala());
            
            // Forward transliteration
            let intermediate = forward_lipika.transliterate(test_case, from_scheme, to_scheme);
            
            // Reverse transliteration
            let round_trip = reverse_lipika.transliterate(&intermediate, to_scheme, from_scheme);
            
            assert_eq!(
                round_trip, *test_case,
                "Round-trip failed for '{}' via {:?} -> {:?} -> {:?}",
                test_case, from_scheme, to_scheme, from_scheme
            );
        }
    }
}

#[test]
fn test_complex_round_trip_preservation() {
    // Test with different sakhas
    let rigveda_tests = vec![
        "agni=mI_le puro=hitam",
        "ya=jJasya de_vam",
        "=agnA_ye",
    ];
    
    let taittiriya_tests = vec![
        "i#She tvO#rje tvAq",
        "vA#yavaqH stha",
        "de#vo vaqH savitA#",
    ];
    
    // Test Rigveda
    for test_case in rigveda_tests {
        let mut forward = Lipika::new()
            .with_extension(rigveda_shakala());
        let mut reverse = Lipika::new()
            .with_extension(rigveda_shakala());
        
        // HK -> Devanagari -> HK
        let deva = forward.transliterate(test_case, Scheme::HarvardKyoto, Scheme::Devanagari);
        let back_to_hk = reverse.transliterate(&deva, Scheme::Devanagari, Scheme::HarvardKyoto);
        assert_eq!(back_to_hk, test_case, 
            "Failed Rigveda round-trip HK->Deva->HK for '{}'", test_case);
    }
    
    // Test Taittiriya
    for test_case in taittiriya_tests {
        let mut forward = Lipika::new()
            .with_extension(yajurveda_taittiriya());
        let mut reverse = Lipika::new()
            .with_extension(yajurveda_taittiriya());
        
        // HK -> Devanagari -> HK
        let deva = forward.transliterate(test_case, Scheme::HarvardKyoto, Scheme::Devanagari);
        let back_to_hk = reverse.transliterate(&deva, Scheme::Devanagari, Scheme::HarvardKyoto);
        assert_eq!(back_to_hk, test_case,
            "Failed Taittiriya round-trip HK->Deva->HK for '{}'", test_case);
    }
}

#[test]
fn test_accent_count_preservation() {
    let mut lipika = Lipika::new()
        .with_extension(rigveda_shakala());
    
    let text = "a=gni_m I=Le_ puro=hita_m";
    
    // Count accents in original (using new ASCII-only notation)
    let udatta_count = text.matches('=').count();
    let anudatta_count = text.matches('_').count();
    
    // Transliterate to Devanagari
    let deva = lipika.transliterate(text, Scheme::HarvardKyoto, Scheme::Devanagari);
    
    // Count Unicode accent marks
    let deva_udatta = deva.matches('\u{0951}').count();
    let deva_anudatta = deva.matches('\u{0952}').count();
    
    assert_eq!(deva_udatta, udatta_count, "Udatta count mismatch");
    assert_eq!(deva_anudatta, anudatta_count, "Anudatta count mismatch");
}

#[test]
fn test_vedic_fricatives() {
    let mut lipika = Lipika::new()
        .with_extension(rigveda_shakala());
    
    // Test upadhmaniya and jihvamuliya (updated to new ASCII-only WX)
    let text_with_fricatives = "ka=Z ka=V"; // Z = jihvamuliya, V = upadhmaniya
    let result = lipika.transliterate(text_with_fricatives, Scheme::Slp1, Scheme::Devanagari);
    
    // Should contain the Vedic fricatives
    assert!(result.contains("\u{1cf5}"), "Should contain upadhmaniya (ᳵ). Result: {}", result);
    assert!(result.contains("\u{1cf6}"), "Should contain jihvamuliya (ᳶ). Result: {}", result);
    assert!(result.contains("\u{0951}"), "Should contain udatta marks. Result: {}", result);
    
    // Round-trip test
    let back = lipika.transliterate(&result, Scheme::Devanagari, Scheme::Slp1);
    assert_eq!(back, text_with_fricatives, "Round-trip should preserve fricatives and accents");
}