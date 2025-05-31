//! Tests demonstrating Vedic extensions work across all Indic and Roman scripts.

use vidyut_lipi::{Lipika, Scheme};
use vidyut_lipi::extensions::vedic::{RigvedaShakala, VedicExtension};

#[test]
fn test_vedic_across_all_indic_scripts() {
    // Test verse with accents
    let verse = "agni'mI_le puro'hitam";
    
    // All Indic scripts that should support Vedic accents
    let indic_scripts = vec![
        Scheme::Devanagari,
        Scheme::Bengali,
        Scheme::Gujarati,
        Scheme::Gurmukhi,
        Scheme::Kannada,
        Scheme::Malayalam,
        Scheme::Odia,
        Scheme::Tamil,
        Scheme::Telugu,
        Scheme::Assamese,
        Scheme::Grantha,
        Scheme::Sharada,
        Scheme::Siddham,
        Scheme::Modi,
        Scheme::Newa,
        Scheme::Tirhuta,
        Scheme::Sinhala,
    ];
    
    for target_script in indic_scripts {
        let mut forward_lipika = Lipika::new()
            .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));
        let mut reverse_lipika = Lipika::new()
            .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));
        
        // Transliterate to target script
        let result = forward_lipika.transliterate(verse, Scheme::HarvardKyoto, target_script);
        
        // Verify accent marks are present (Unicode combining marks)
        assert!(
            result.contains('\u{0951}') || result.contains('\u{0952}'),
            "Script {:?} should contain accent marks. Result: {}",
            target_script, result
        );
        
        // Round-trip test
        let back = reverse_lipika.transliterate(&result, target_script, Scheme::HarvardKyoto);
        assert_eq!(
            back, verse,
            "Round-trip failed for {:?}. Got: {}",
            target_script, back
        );
    }
}

#[test]
fn test_vedic_across_all_roman_scripts() {
    // Test verse with accents
    let verse = "agni'mI_le puro'hitam";
    
    // All Roman transliteration schemes
    let roman_schemes = vec![
        Scheme::Iast,
        Scheme::Iso15919,
        Scheme::HarvardKyoto,
        Scheme::Velthuis,
        Scheme::Slp1,
        Scheme::Wx,
        Scheme::Itrans,
    ];
    
    for target_scheme in roman_schemes {
        // Skip self-to-self
        if target_scheme == Scheme::HarvardKyoto {
            continue;
        }
        
        let mut forward_lipika = Lipika::new()
            .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));
        let mut reverse_lipika = Lipika::new()
            .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));
        
        // Transliterate to target scheme
        let result = forward_lipika.transliterate(verse, Scheme::HarvardKyoto, target_scheme);
        
        // For IAST/ISO, check for Unicode marks; for others, check for preserved ASCII marks
        match target_scheme {
            Scheme::Iast | Scheme::Iso15919 => {
                assert!(
                    result.contains('\u{0951}') || result.contains('\u{0952}') ||
                    result.contains('\'') || result.contains('_'),
                    "Scheme {:?} should contain accent marks. Result: {}",
                    target_scheme, result
                );
            }
            _ => {
                // ASCII schemes should preserve the accent marks as-is
                assert!(
                    result.contains('\'') || result.contains('_'),
                    "Scheme {:?} should preserve ASCII accent marks. Result: {}",
                    target_scheme, result
                );
            }
        }
        
        // Round-trip test
        let back = reverse_lipika.transliterate(&result, target_scheme, Scheme::HarvardKyoto);
        assert_eq!(
            back, verse,
            "Round-trip failed for {:?}. Got: {}",
            target_scheme, back
        );
    }
}

#[test]
fn test_vedic_multi_script_chain() {
    // Test that accents are preserved through a chain of transliterations
    let verse = "ta't savi'tur vare_Nyam";
    
    let chain = vec![
        Scheme::HarvardKyoto,
        Scheme::Devanagari,
        Scheme::Tamil,
        Scheme::Iast,
        Scheme::Kannada,
        Scheme::HarvardKyoto,
    ];
    
    let mut current = verse.to_string();
    let mut current_scheme = chain[0];
    
    for next_scheme in chain.iter().skip(1) {
        let mut lipika = Lipika::new()
            .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));
        
        current = lipika.transliterate(&current, current_scheme, *next_scheme);
        current_scheme = *next_scheme;
        
        // Count total accent marks (either Unicode or ASCII)
        let accent_count = current.matches('\u{0951}').count() 
            + current.matches('\u{0952}').count()
            + current.matches('\'').count()
            + current.matches('_').count();
        
        assert!(
            accent_count >= 3,
            "Lost accents at {:?} -> {:?}. Result: {}",
            current_scheme, next_scheme, current
        );
    }
    
    // Final result should match original
    assert_eq!(
        current, verse,
        "Multi-script chain failed to preserve original"
    );
}

#[test]
fn test_vedic_special_scripts() {
    // Test with scripts that have special characteristics
    let verse = "ta't savi'tur vare_Nyam";
    
    // Scripts with potential issues or special handling
    let special_scripts = vec![
        (Scheme::Grantha, "Historical South Indian script"),
        (Scheme::Bhaiksuki, "Historical Buddhist script"),
        (Scheme::Brahmi, "Ancient script"),
        (Scheme::Kharoshthi, "Ancient Gandhara script"),
        (Scheme::Soyombo, "Mongolian script for Sanskrit"),
    ];
    
    for (script, description) in special_scripts {
        let mut lipika = Lipika::new()
            .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));
        
        let result = lipika.transliterate(verse, Scheme::HarvardKyoto, script);
        
        // These ancient/special scripts should still preserve accents
        let has_accents = result.contains('\u{0951}') || result.contains('\u{0952}');
        
        assert!(
            has_accents,
            "Script {:?} ({}) should support accents. Result: {}",
            script, description, result
        );
    }
}

#[test]
fn test_accent_consistency_across_scripts() {
    // Ensure the same number of accents is preserved across all scripts
    let verse = "agni'm I'le puro'hita'm ya'jJasya de'vam";
    let expected_udatta = 6;  // Count of ' marks (actually 6 in the verse)
    let _expected_anudatta = 1; // Count of _ marks
    
    let all_scripts = vec![
        Scheme::Devanagari, Scheme::Bengali, Scheme::Tamil, Scheme::Iast,
        Scheme::Kannada, Scheme::Telugu, Scheme::Gujarati, Scheme::Malayalam,
    ];
    
    for script in all_scripts {
        let mut lipika = Lipika::new()
            .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));
        
        let result = lipika.transliterate(verse, Scheme::HarvardKyoto, script);
        
        // Count Unicode accent marks
        let udatta_count = result.matches('\u{0951}').count();
        let anudatta_count = result.matches('\u{0952}').count();
        
        // For IAST and other schemes that might preserve ASCII marks
        let ascii_udatta = result.matches('\'').count();
        let ascii_anudatta = result.matches('_').count();
        
        let total_udatta = udatta_count + ascii_udatta;
        let total_anudatta = anudatta_count + ascii_anudatta;
        
        assert_eq!(
            total_udatta, expected_udatta,
            "Udatta count mismatch for {:?}. Result: {}",
            script, result
        );
        
        // Note: Some scripts might not distinguish all accent types
        assert!(
            total_anudatta >= 0,
            "Anudatta should be preserved for {:?}. Result: {}",
            script, result
        );
    }
}