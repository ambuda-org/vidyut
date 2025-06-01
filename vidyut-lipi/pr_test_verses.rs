//! Test verses for PR demonstration - showing Vedic transliteration across scripts
//! This file generates sample output for manual verification in PR comments

use vidyut_lipi::{Lipika, Scheme};
use vidyut_lipi::extensions::vedic::{VedicExtension, RigvedaShakala, TaittiriyaYajurveda, SamavedaKauthuma, AtharvavedaSaunaka};

fn main() {
    println!("# Vedic Transliteration Test Results");
    println!("Generated for PR verification - showing authentic Vedic verses across scripts\n");
    
    // Test verses for each sakha
    test_rigveda_verses();
    test_yajurveda_verses();
    test_samaveda_verses();
    test_atharvaveda_verses();
}

fn test_rigveda_verses() {
    println!("## Rigveda Shakala Tests\n");
    
    let mut lipika = Lipika::new()
        .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));
    
    // Famous opening verse of Rigveda (1.1.1)
    let verse1 = "agni'mI_le puro'hitam ya'jJasya de'vam Rtvi'jam |";
    println!("### Verse 1: Rigveda 1.1.1 (Opening)");
    println!("**Original (WX):** `{}`", verse1);
    demonstrate_verse(&mut lipika, verse1);
    
    // Gayatri mantra portion
    let verse2 = "tat savi'tur vare_Nyam bha'rgo deva'sya dhI_mahi |";
    println!("### Verse 2: Gayatri Mantra");
    println!("**Original (WX):** `{}`", verse2);
    demonstrate_verse(&mut lipika, verse2);
    
    // Verse with complex accents
    let verse3 = "ho'tAram ratna'dhAtamam a'gnim I_le puru'priyam |";
    println!("### Verse 3: Complex Accent Pattern");
    println!("**Original (WX):** `{}`", verse3);
    demonstrate_verse(&mut lipika, verse3);
}

fn test_yajurveda_verses() {
    println!("## Taittiriya Yajurveda Tests\n");
    
    let mut lipika = Lipika::new()
        .with_extension(Box::new(VedicExtension::new(TaittiriyaYajurveda)));
    
    // Opening of Taittiriya Samhita
    let verse1 = "i#She tvo#rje tvA# vA#yavaqH sthaq de#vo vaqH savitA# |";
    println!("### Verse 1: Taittiriya Samhita 1.1.1.1");
    println!("**Original (Extended Baraha):** `{}`", verse1);
    demonstrate_verse(&mut lipika, verse1);
    
    // Verse with phonetic annotations
    let verse2 = "sa#tyam bRha#d Rtaqm ugra#m dI#kSA# tapaqH karma ca# |";
    println!("### Verse 2: Philosophical verse");
    println!("**Original (Extended Baraha):** `{}`", verse2);
    demonstrate_verse(&mut lipika, verse2);
    
    // Verse with special phonetic features
    let verse3 = "agne# (gm)a#gnayeq tvA#Zva# uttama#m havi#H |";
    println!("### Verse 3: With phonetic annotations");
    println!("**Original (Extended Baraha):** `{}`", verse3);
    demonstrate_verse(&mut lipika, verse3);
}

fn test_samaveda_verses() {
    println!("## Samaveda Kauthuma Tests\n");
    
    let mut lipika = Lipika::new()
        .with_extension(Box::new(VedicExtension::new(SamavedaKauthuma)));
    
    // Samaveda with musical notation
    let verse1 = "agn1e2 A3 yA2hi vI1taye2 gRNa3no2 havya1dA2taye3 |";
    println!("### Verse 1: Musical notation");
    println!("**Original (Numeric musical):** `{}`", verse1);
    demonstrate_verse(&mut lipika, verse1);
    
    // Complex musical pattern
    let verse2 = "ni1 hotA3 satsi2 barhiSi2 priyeq ho1tar3 A2sa3de2 |";
    println!("### Verse 2: Complex musical pattern");
    println!("**Original (Numeric musical):** `{}`", verse2);
    demonstrate_verse(&mut lipika, verse2);
}

fn test_atharvaveda_verses() {
    println!("## Atharvaveda Saunaka Tests\n");
    
    let mut lipika = Lipika::new()
        .with_extension(Box::new(VedicExtension::new(AtharvavedaSaunaka)));
    
    // Atharvaveda verse
    let verse1 = "bhadram kar'Nebhih SrNuya_ma devAH bhadram paS'ye_mAkSa_bhir yajatrAH |";
    println!("### Verse 1: Peace invocation");
    println!("**Original (WX):** `{}`", verse1);
    demonstrate_verse(&mut lipika, verse1);
    
    // Protective mantra
    let verse2 = "svasti' na_ indro' vRddha'SravAH svasti' naH pU_SA viS'vavedAH |";
    println!("### Verse 2: Protective mantra");
    println!("**Original (WX):** `{}`", verse2);
    demonstrate_verse(&mut lipika, verse2);
}

fn demonstrate_verse(lipika: &mut Lipika, verse: &str) {
    // Transliterate to key target scripts
    let devanagari = lipika.transliterate(verse, Scheme::Wx, Scheme::Devanagari);
    let telugu = lipika.transliterate(verse, Scheme::Wx, Scheme::Telugu);
    let iast = lipika.transliterate(verse, Scheme::Wx, Scheme::Iast);
    let slp1 = lipika.transliterate(verse, Scheme::Wx, Scheme::Slp1);
    
    println!("**Devanagari:** {}", devanagari);
    println!("**Telugu:** {}", telugu);
    println!("**IAST:** {}", iast);
    println!("**SLP1:** {}", slp1);
    
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