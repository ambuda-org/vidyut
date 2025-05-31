//! Vedic transliteration extensions for sakha-specific schemes.
//!
//! This module provides support for Vedic accents and sakha-specific transliteration rules.
//! Each sakha (branch) of the Vedas has its own accent notation system and phonetic conventions.

mod rigveda;
mod yajurveda;
mod samaveda;
mod atharvaveda;

pub use rigveda::RigvedaShakala;
pub use yajurveda::TaittiriyaYajurveda;
pub use samaveda::SamavedaKauthuma;
pub use atharvaveda::AtharvavedaSaunaka;

use crate::{Mapping, SpanKind};
use crate::extensions::TransliterationExtension;

/// Common Vedic accent types used across sakhas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VedicAccent {
    /// Udatta (high tone)
    Udatta,
    /// Anudatta (low tone)
    Anudatta,
    /// Svarita (falling tone)
    Svarita,
    /// Pracaya (accumulated tone)
    Pracaya,
    /// Dirgha Svarita (long falling tone)
    DirghaSvarita,
}

/// Trait for sakha-specific Vedic extensions.
pub trait VedicSakha: Send + Sync {
    /// Returns the name of this sakha.
    fn name(&self) -> &str;
    
    /// Returns the accent mappings for this sakha.
    ///
    /// Each mapping is a tuple of (WX notation, Unicode character for target script).
    fn accent_mappings(&self) -> Vec<(&'static str, &'static str, VedicAccent)>;
    
    /// Returns additional phonetic mappings specific to this sakha.
    fn phonetic_mappings(&self) -> Vec<(&'static str, &'static str)> {
        Vec::new()
    }
    
    /// Pre-processes text before transliteration.
    fn pre_process(&self, text: &str) -> String {
        text.to_string()
    }
    
    /// Post-processes text after transliteration.
    fn post_process(&self, text: &str) -> String {
        text.to_string()
    }
}

/// A generic Vedic extension that wraps a sakha-specific implementation.
#[derive(Clone)]
pub struct VedicExtension<S: VedicSakha + Clone> {
    sakha: S,
}

impl<S: VedicSakha + Clone> VedicExtension<S> {
    /// Creates a new Vedic extension for the given sakha.
    pub fn new(sakha: S) -> Self {
        Self { sakha }
    }
}

impl<S: VedicSakha + Clone + 'static> TransliterationExtension for VedicExtension<S> {
    fn name(&self) -> &str {
        self.sakha.name()
    }
    
    fn extend_mapping(&self, mapping: &mut Mapping) {
        use crate::Scheme;
        
        // Add accent mappings for the specific direction of transliteration
        for (wx, unicode, _accent_type) in self.sakha.accent_mappings() {
            match (mapping.from(), mapping.to()) {
                // From any ASCII scheme to any Indic script
                (from, to) if from.is_alphabet() && to.is_abugida() => {
                    mapping.add_mapping(wx, unicode, SpanKind::Accent);
                }
                
                // From any Indic script to any ASCII scheme
                (from, to) if from.is_abugida() && to.is_alphabet() => {
                    mapping.add_mapping(unicode, wx, SpanKind::Accent);
                }
                
                // From IAST/ISO to ASCII schemes: convert Unicode marks back
                (Scheme::Iast | Scheme::Iso15919,
                 Scheme::HarvardKyoto | Scheme::Velthuis | Scheme::Slp1 | Scheme::Wx) => {
                    mapping.add_mapping(unicode, wx, SpanKind::Accent);
                }
                
                // From ASCII to IAST/ISO: convert to Unicode marks
                (Scheme::HarvardKyoto | Scheme::Velthuis | Scheme::Slp1 | Scheme::Wx,
                 Scheme::Iast | Scheme::Iso15919) => {
                    mapping.add_mapping(wx, unicode, SpanKind::Accent);
                }
                
                // Indic to Indic: preserve Unicode marks
                (from, to) if from.is_abugida() && to.is_abugida() => {
                    mapping.add_mapping(unicode, unicode, SpanKind::Accent);
                }
                
                // For same-script, preserve as-is
                _ => {
                    if mapping.from() == mapping.to() {
                        mapping.add_mapping(wx, wx, SpanKind::Accent);
                        mapping.add_mapping(unicode, unicode, SpanKind::Accent);
                    }
                }
            }
        }
        
        // Add phonetic mappings
        for (wx, unicode) in self.sakha.phonetic_mappings() {
            let kind = if wx.chars().any(|c| "aAiIuUeEoO".contains(c)) {
                SpanKind::VowelMark
            } else {
                SpanKind::Other
            };
            mapping.add_mapping(wx, unicode, kind);
        }
    }
    
    fn pre_process(&self, text: &str) -> String {
        self.sakha.pre_process(text)
    }
    
    fn post_process(&self, text: &str) -> String {
        self.sakha.post_process(text)
    }
}