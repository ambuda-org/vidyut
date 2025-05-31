//! Rigveda Shakala sakha transliteration support.

use super::{VedicAccent, VedicSakha};

/// Rigveda Shakala sakha extension.
///
/// This implements the accent notation system used in the Shakala recension
/// of the Rigveda, which is the most commonly studied version.
#[derive(Clone, Copy)]
pub struct RigvedaShakala;

impl VedicSakha for RigvedaShakala {
    fn name(&self) -> &str {
        "Rigveda Shakala"
    }
    
    fn accent_mappings(&self) -> Vec<(&'static str, &'static str, VedicAccent)> {
        vec![
            // Basic accents in WX notation -> Devanagari Unicode
            ("'", "\u{0951}", VedicAccent::Udatta),        // Udatta mark
            ("_", "\u{0952}", VedicAccent::Anudatta),      // Anudatta mark
            ("=", "\u{0951}\u{0952}", VedicAccent::Svarita), // Svarita (combination)
        ]
    }
    
    fn phonetic_mappings(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // Rigvedic-specific phonetic notations
            ("L", "\u{0962}"),  // Vedic vowel L
            ("LL", "\u{0963}"), // Vedic vowel LL
            
            // Anusvara variations
            ("M~", "\u{0901}"), // Candrabindu for nasalization
            
            // Special markers
            ("|", "\u{0964}"),  // Danda (verse separator)
            ("||", "\u{0965}"), // Double danda (section separator)
        ]
    }
    
    fn pre_process(&self, text: &str) -> String {
        // Handle common Rigvedic conventions
        text.replace("a3", "a'")  // Convert numeric accent notation to WX
            .replace("a1", "a_")
            .replace("a2", "a=")
    }
    
    fn post_process(&self, text: &str) -> String {
        // Clean up any double accent marks that might have been created
        text.replace("\u{0951}\u{0951}", "\u{0951}")
            .replace("\u{0952}\u{0952}", "\u{0952}")
    }
}