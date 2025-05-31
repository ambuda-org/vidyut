//! Taittiriya Yajurveda sakha transliteration support.

use super::{VedicAccent, VedicSakha};

/// Taittiriya Yajurveda sakha extension.
///
/// Based on the Extended Baraha ASCII format used in udapaana:
/// - `#` = udātta (high tone)
/// - `q` = anudātta (low tone)
/// - Special annotations like `(gm)`, `(gg)` for phonetic variations
#[derive(Clone, Copy)]
pub struct TaittiriyaYajurveda;

impl VedicSakha for TaittiriyaYajurveda {
    fn name(&self) -> &str {
        "Taittiriya Yajurveda"
    }
    
    fn accent_mappings(&self) -> Vec<(&'static str, &'static str, VedicAccent)> {
        vec![
            // Baraha format mappings (primary for Taittiriya)
            ("#", "\u{0951}", VedicAccent::Udatta),      // udātta
            ("q", "\u{0952}", VedicAccent::Anudatta),    // anudātta
            ("=", "\u{0953}", VedicAccent::Svarita),     // svarita
        ]
    }
    
    fn phonetic_mappings(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            // Special phonetic annotations from Taittiriya
            ("(gm)", "ṁ"),      // Guttural + m sound
            ("(gg)", "g̃"),      // Nasalized g
            ("~M", "ṁ"),        // Special nasalization
            
            // Vedic-specific characters
            ("L", "\u{0962}"),  // Vedic L
            ("LL", "\u{0963}"), // Vedic LL
            
            // Word boundary markers (preserve as-is in transliteration)
            // These are semantic markers, not phonetic, so they pass through unchanged
        ]
    }
    
    fn pre_process(&self, text: &str) -> String {
        // Preserve all text including pada markers (| and " - ")
        // These are linguistic markers that should pass through transliteration
        text.to_string()
    }
}