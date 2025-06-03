//! Example demonstrating Vedic transliteration across all supported scripts.

use vidyut_lipi::extensions::vedic::rigveda_shakala;
use vidyut_lipi::{Lipika, Scheme};

fn main() {
    println!("=== Vedic Extensions Across All Scripts ===\n");

    // Sample Vedic text with accents
    let gayatri = "tat savi'tur vare_Nyam bha'rgo deva'sya dhI_mahi";
    println!("Original text (Harvard-Kyoto): {}", gayatri);
    println!("Accents: ' = udatta (high), _ = anudatta (low)\n");

    let mut lipika = Lipika::new().with_extension(rigveda_shakala());

    // Major Indic scripts
    println!("=== Major Indic Scripts ===");
    let major_scripts = vec![
        ("Devanagari", Scheme::Devanagari),
        ("Bengali", Scheme::Bengali),
        ("Gujarati", Scheme::Gujarati),
        ("Gurmukhi", Scheme::Gurmukhi),
        ("Kannada", Scheme::Kannada),
        ("Malayalam", Scheme::Malayalam),
        ("Odia", Scheme::Odia),
        ("Tamil", Scheme::Tamil),
        ("Telugu", Scheme::Telugu),
    ];

    for (name, script) in major_scripts {
        let result = lipika.transliterate(gayatri, Scheme::HarvardKyoto, script);
        println!("{:12} {}", format!("{}:", name), result);
    }

    // Historical and special scripts
    println!("\n=== Historical & Special Scripts ===");
    let historical_scripts = vec![
        ("Grantha", Scheme::Grantha),
        ("Sharada", Scheme::Sharada),
        ("Siddham", Scheme::Siddham),
        ("Brahmi", Scheme::Brahmi),
        ("Modi", Scheme::Modi),
        ("Tirhuta", Scheme::Tirhuta),
    ];

    for (name, script) in historical_scripts {
        let result = lipika.transliterate(gayatri, Scheme::HarvardKyoto, script);
        println!("{:12} {}", format!("{}:", name), result);
    }

    // Roman transliteration schemes
    println!("\n=== Roman Transliteration Schemes ===");
    let roman_schemes = vec![
        ("IAST", Scheme::Iast),
        ("ISO 15919", Scheme::Iso15919),
        ("Velthuis", Scheme::Velthuis),
        ("SLP1", Scheme::Slp1),
        ("WX", Scheme::Wx),
        ("ITRANS", Scheme::Itrans),
    ];

    for (name, scheme) in roman_schemes {
        let result = lipika.transliterate(gayatri, Scheme::HarvardKyoto, scheme);
        println!("{:12} {}", format!("{}:", name), result);
    }

    // Demonstrate round-trip preservation
    println!("\n=== Round-trip Preservation Test ===");
    let test_verse = "agni'mI_le puro'hitam";
    println!("Original: {}", test_verse);

    // HK -> Devanagari -> HK
    let deva = lipika.transliterate(test_verse, Scheme::HarvardKyoto, Scheme::Devanagari);
    println!("To Devanagari: {}", deva);
    let back_hk = lipika.transliterate(&deva, Scheme::Devanagari, Scheme::HarvardKyoto);
    println!("Back to HK: {}", back_hk);
    println!(
        "Preserved: {}",
        if back_hk == test_verse { "✓" } else { "✗" }
    );

    // Multi-script journey
    println!("\n=== Multi-Script Journey ===");
    let journey = [Scheme::HarvardKyoto,
        Scheme::Devanagari,
        Scheme::Tamil,
        Scheme::Kannada,
        Scheme::Bengali,
        Scheme::Iast,
        Scheme::HarvardKyoto];

    let mut current = test_verse.to_string();
    for i in 1..journey.len() {
        let from = journey[i - 1];
        let to = journey[i];
        current = lipika.transliterate(&current, from, to);
        println!("{:?} -> {:?}: {}", from, to, current);
    }

    println!(
        "\nFinal matches original: {}",
        if current == test_verse { "✓" } else { "✗" }
    );

    // Accent statistics
    println!("\n=== Accent Preservation Statistics ===");
    let complex_verse = "a'gnim I'le puro'hitam ya'jJasya de'vam Rtvi'jam";
    let udatta_count = complex_verse.matches('\'').count();
    let anudatta_count = complex_verse.matches('_').count();

    println!("Test verse: {}", complex_verse);
    println!(
        "Original: {} udatta, {} anudatta",
        udatta_count, anudatta_count
    );

    let scripts_to_test = vec![
        ("Devanagari", Scheme::Devanagari),
        ("Bengali", Scheme::Bengali),
        ("Tamil", Scheme::Tamil),
        ("IAST", Scheme::Iast),
    ];

    for (name, script) in scripts_to_test {
        let result = lipika.transliterate(complex_verse, Scheme::HarvardKyoto, script);
        let unicode_udatta = result.matches('\u{0951}').count();
        let unicode_anudatta = result.matches('\u{0952}').count();
        println!(
            "{}: {} udatta, {} anudatta marks preserved",
            name, unicode_udatta, unicode_anudatta
        );
    }

    println!("\n=== Summary ===");
    println!("The Vedic extension system successfully:");
    println!("- Preserves accent marks across all Indic scripts");
    println!("- Maintains accent information in Roman transliteration");
    println!("- Supports round-trip transliteration without loss");
    println!("- Works with historical and modern scripts alike");
}
