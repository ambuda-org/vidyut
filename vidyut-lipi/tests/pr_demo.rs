//! Generate demo output for PR comment

use vidyut_lipi::{Lipika, Scheme};
use vidyut_lipi::extensions::vedic::{rigveda_shakala, yajurveda_taittiriya, samaveda_kauthuma, atharvaveda_shaunaka};

#[test]
fn generate_pr_demo_output() {
    println!("\n# Vedic Transliteration Test Results");
    println!("Generated for PR verification - showing authentic Vedic verses across scripts\n");
    
    // Rigveda Tests
    println!("## 🕉️ Rigveda Shakala Tests\n");
    
    let mut rigveda_lipika = Lipika::new()
        .with_extension(rigveda_shakala());
    
    // Famous opening verse of Rigveda (1.1.1)
    let verse1 = "agni'mI_le puro'hitam ya'jJasya de'vam Rtvi'jam |";
    println!("### Verse 1: Rigveda 1.1.1 (Opening verse)");
    println!("**Original (WX):** `{}`", verse1);
    demonstrate_verse(&mut rigveda_lipika, verse1);
    
    // Gayatri mantra portion
    let verse2 = "tat savi'tur vare_Nyam bha'rgo deva'sya dhI_mahi |";
    println!("### Verse 2: Gayatri Mantra");
    println!("**Original (WX):** `{}`", verse2);
    demonstrate_verse(&mut rigveda_lipika, verse2);
    
    // Yajurveda Tests
    println!("## 🔥 Taittiriya Yajurveda Tests\n");
    
    let mut yajurveda_lipika = Lipika::new()
        .with_extension(yajurveda_taittiriya());
    
    // Opening of Taittiriya Samhita
    let verse3 = "i#She tvo#rje tvA# vA#yavaqH sthaq de#vo vaqH savitA# |";
    println!("### Verse 3: Taittiriya Samhita 1.1.1.1");
    println!("**Original (Extended Baraha):** `{}`", verse3);
    demonstrate_verse(&mut yajurveda_lipika, verse3);
    
    // Samaveda Tests  
    println!("## 🎵 Samaveda Kauthuma Tests\n");
    
    let mut samaveda_lipika = Lipika::new()
        .with_extension(samaveda_kauthuma());
    
    // Samaveda with musical notation
    let verse4 = "agn1e2 A3 yA2hi vI1taye2 gRNa3no2 havya1dA2taye3 |";
    println!("### Verse 4: Musical notation");
    println!("**Original (Numeric musical):** `{}`", verse4);
    demonstrate_verse(&mut samaveda_lipika, verse4);
    
    // Atharvaveda Tests
    println!("## 🕊️ Atharvaveda Saunaka Tests\n");
    
    let mut atharvaveda_lipika = Lipika::new()
        .with_extension(atharvaveda_shaunaka());
    
    // Atharvaveda verse
    let verse5 = "bhadram kar'Nebhih SrNuya_ma devAH bhadram paS'ye_mAkSa_bhir yajatrAH |";
    println!("### Verse 5: Peace invocation");
    println!("**Original (WX):** `{}`", verse5);
    demonstrate_verse(&mut atharvaveda_lipika, verse5);
    
    println!("---");
    println!("*Generated automatically to verify transliteration accuracy across Vedic traditions*");
}

fn demonstrate_verse(lipika: &mut Lipika, verse: &str) {
    // Transliterate to key target scripts
    let devanagari = lipika.transliterate(verse, Scheme::Wx, Scheme::Devanagari);
    let telugu = lipika.transliterate(verse, Scheme::Wx, Scheme::Telugu);
    let iast = lipika.transliterate(verse, Scheme::Wx, Scheme::Iast);
    
    println!("**Devanagari:** {}", devanagari);
    println!("**Telugu:** {}", telugu);  
    println!("**IAST:** {}", iast);
    
    // Show padam (word-by-word) versions
    let padam_deva = create_padam_version(&devanagari);
    let padam_telugu = create_padam_version(&telugu);
    let padam_iast = create_padam_version(&iast);
    
    println!("**Padam Devanagari:** {}", padam_deva);
    println!("**Padam Telugu:** {}", padam_telugu);
    println!("**Padam IAST:** {}", padam_iast);
    
    // Test round-trip preservation
    let back_to_wx = lipika.transliterate(&devanagari, Scheme::Devanagari, Scheme::Wx);
    let round_trip_ok = back_to_wx == verse;
    println!("**Round-trip preserved:** {}", if round_trip_ok { "✅" } else { "❌" });
    
    if !round_trip_ok {
        println!("  - Expected: `{}`", verse);
        println!("  - Got back: `{}`", back_to_wx);
    }
    
    println!();
}

fn create_padam_version(text: &str) -> String {
    // Simple padam transformation: add spacing between words and show pada boundaries
    text.replace(" ", " । ")
        .replace("।।", "। ।")
        .replace("|", "।")
        .trim()
        .to_string() + " ॥"
}