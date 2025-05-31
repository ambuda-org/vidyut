//! Example demonstrating Vedic transliteration extensions.

use vidyut_lipi::{Lipika, Scheme};
use vidyut_lipi::extensions::vedic::{RigvedaShakala, TaittiriyaYajurveda, VedicExtension};

fn main() {
    println!("=== Vedic Transliteration Extension Example ===\n");
    
    // Standard transliteration without extensions
    println!("1. Standard transliteration (no extensions):");
    let mut standard_lipika = Lipika::new();
    let text = "agnimILe purohitam";
    let result = standard_lipika.transliterate(text, Scheme::HarvardKyoto, Scheme::Devanagari);
    println!("   Input:  {}", text);
    println!("   Output: {}\n", result);
    
    // Rigveda Shakala extension
    println!("2. With Rigveda Shakala extension:");
    let mut rigveda_lipika = Lipika::new()
        .with_extension(Box::new(VedicExtension::new(RigvedaShakala)));
    
    let text_with_accents = "agni'mILe puro_hitam";
    let result = rigveda_lipika.transliterate(text_with_accents, Scheme::HarvardKyoto, Scheme::Devanagari);
    println!("   Input:  {} (note: ' = udatta, _ = anudatta)", text_with_accents);
    println!("   Output: {}", result);
    println!("   (The udatta ॑ and anudatta ॒ marks are visible in Devanagari)\n");
    
    // Taittiriya Yajurveda extension with special notation
    println!("3. With Taittiriya Yajurveda extension:");
    let mut taittiriya_lipika = Lipika::new()
        .with_extension(Box::new(VedicExtension::new(TaittiriyaYajurveda)));
    
    let ty_text = "i#She tvo#rje tvA# vA#yavaqH";
    let result = taittiriya_lipika.transliterate(ty_text, Scheme::HarvardKyoto, Scheme::Devanagari);
    println!("   Input:  {} (# = udatta, q = anudatta)", ty_text);
    println!("   Output: {}\n", result);
    
    // Multiple scripts
    println!("4. Transliterating to different scripts with accents:");
    let verse = "tat savi'tur vare_Nyam";
    
    let deva_result = rigveda_lipika.transliterate(verse, Scheme::HarvardKyoto, Scheme::Devanagari);
    let iast_result = rigveda_lipika.transliterate(verse, Scheme::HarvardKyoto, Scheme::Iast);
    
    println!("   Input:       {}", verse);
    println!("   Devanagari:  {}", deva_result);
    println!("   IAST:        {}", iast_result);
    
    println!("\n=== Key Features ===");
    println!("- Extensions preserve Vedic accent marks during transliteration");
    println!("- Each sakha (branch) can have its own accent notation system");
    println!("- Extensions can handle special phonetic annotations");
    println!("- The same Lipika instance can be reused for multiple transliterations");
}