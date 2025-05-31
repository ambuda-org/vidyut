//! Atharvaveda Saunaka sakha transliteration support.

use super::{VedicAccent, VedicSakha};

/// Atharvaveda Saunaka sakha extension.
#[derive(Clone, Copy)]
pub struct AtharvavedaSaunaka;

impl VedicSakha for AtharvavedaSaunaka {
    fn name(&self) -> &str {
        "Atharvaveda Saunaka"
    }
    
    fn accent_mappings(&self) -> Vec<(&'static str, &'static str, VedicAccent)> {
        vec![
            // Atharvaveda typically uses simplified accent notation
            ("'", "\u{0951}", VedicAccent::Udatta),
            ("_", "\u{0952}", VedicAccent::Anudatta),
        ]
    }
}