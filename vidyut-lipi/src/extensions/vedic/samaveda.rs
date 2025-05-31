//! Samaveda Kauthuma sakha transliteration support.

use super::{VedicAccent, VedicSakha};

/// Samaveda Kauthuma sakha extension.
#[derive(Clone, Copy)]
pub struct SamavedaKauthuma;

impl VedicSakha for SamavedaKauthuma {
    fn name(&self) -> &str {
        "Samaveda Kauthuma"
    }
    
    fn accent_mappings(&self) -> Vec<(&'static str, &'static str, VedicAccent)> {
        vec![
            // Samaveda uses numeric notation for its complex musical system
            ("1", "\u{0967}", VedicAccent::Udatta),
            ("2", "\u{0968}", VedicAccent::Anudatta),
            ("3", "\u{0969}", VedicAccent::Svarita),
        ]
    }
}