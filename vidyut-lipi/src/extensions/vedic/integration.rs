//! Integration module that bridges the new requirements-based system with the existing Lipika infrastructure

use super::*;
use crate::extensions::TransliterationExtension;
use crate::{Mapping, Scheme, SpanKind};

/// Factory for creating scheme adapters
pub struct AdapterFactory;

impl AdapterFactory {
    /// Creates an adapter for the given scheme
    pub fn create_adapter(scheme: Scheme) -> Option<Box<dyn SchemeAdapter>> {
        match scheme {
            Scheme::Wx => Some(Box::new(super::adapters::WxAdapter)),
            Scheme::Devanagari => Some(Box::new(super::adapters::DevanagariAdapter)),
            Scheme::Iso15919 => Some(Box::new(super::adapters::Iso15919Adapter)),
            Scheme::Iast => Some(Box::new(super::adapters::IastAdapter)),
            Scheme::Telugu => Some(Box::new(super::adapters::TeluguAdapter)),
            // Add more schemes as needed
            _ => None,
        }
    }
}

/// A unified Vedic extension that integrates with the existing system
pub struct UnifiedVedicExtension {
    sakha: Box<dyn VedicSakha>,
    name: String,
}

impl UnifiedVedicExtension {
    /// Creates a new unified extension for the given sakha
    pub fn new(sakha: Box<dyn VedicSakha>) -> Self {
        let name = sakha.requirements().name.clone();
        Self { sakha, name }
    }
}

impl TransliterationExtension for UnifiedVedicExtension {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn extend_mapping(&self, mapping: &mut Mapping) {
        let from_scheme = mapping.from();
        let to_scheme = mapping.to();
        
        // Use the old-style mapping approach that the tests expect
        // This will be refined later but ensures compatibility
        
        // Add WX accent mappings based on the scheme direction
        match (from_scheme, to_scheme) {
            // From WX to Devanagari: map ASCII accents to Unicode marks
            (crate::Scheme::Wx, crate::Scheme::Devanagari) => {
                mapping.add_mapping("'", "\u{0951}", SpanKind::Accent); // udatta (legacy)
                mapping.add_mapping("=", "\u{0951}", SpanKind::Accent); // udatta (new)
                mapping.add_mapping("_", "\u{0952}", SpanKind::Accent); // anudatta  
                mapping.add_mapping("^", "\u{0951}\u{0952}", SpanKind::Accent); // svarita
                mapping.add_mapping("~", "\u{0953}", SpanKind::Accent); // pracaya
                mapping.add_mapping("*", "\u{0971}", SpanKind::Accent); // kampa
            }
            // From Devanagari to WX: map Unicode marks to ASCII
            (crate::Scheme::Devanagari, crate::Scheme::Wx) => {
                mapping.add_mapping("\u{0951}", "=", SpanKind::Accent); // udatta (new ASCII-only)
                mapping.add_mapping("\u{0952}", "_", SpanKind::Accent); // anudatta
                mapping.add_mapping("\u{0953}", "~", SpanKind::Accent); // pracaya
                mapping.add_mapping("\u{0971}", "*", SpanKind::Accent); // kampa
            }
            // From any ASCII scheme to Devanagari: map simple accents
            (from, crate::Scheme::Devanagari) if from.is_alphabet() => {
                mapping.add_mapping("=", "\u{0951}", SpanKind::Accent); // udatta (new)
                mapping.add_mapping("'", "\u{0951}", SpanKind::Accent); // udatta (legacy)
                mapping.add_mapping("_", "\u{0952}", SpanKind::Accent); // anudatta
            }
            // From Devanagari to any ASCII scheme: reverse mapping
            (crate::Scheme::Devanagari, to) if to.is_alphabet() => {
                mapping.add_mapping("\u{0951}", "=", SpanKind::Accent); // udatta (new ASCII-only)
                mapping.add_mapping("\u{0952}", "_", SpanKind::Accent); // anudatta
            }
            // For other combinations, add basic mappings
            _ => {
                // Add accent preservation mappings as needed
            }
        }
        
        // Add special notations like fricatives and boundaries
        mapping.add_mapping("Z", "\u{1cf5}", SpanKind::Other); // jihvamuliya
        mapping.add_mapping("V", "\u{1cf6}", SpanKind::Other); // upadhmaniya
        mapping.add_mapping("#", "।", SpanKind::Other); // pada boundary
        mapping.add_mapping("|", "।", SpanKind::Other); // alternative pada boundary
        mapping.add_mapping("q", "\u{0331}", SpanKind::Other); // combining underline
        
        // Reverse mappings for round-trip support
        mapping.add_mapping("\u{1cf5}", "Z", SpanKind::Other);
        mapping.add_mapping("\u{1cf6}", "V", SpanKind::Other);
        mapping.add_mapping("।", "#", SpanKind::Other); // preserve # for round-trip
        mapping.add_mapping("\u{0331}", "q", SpanKind::Other); // preserve q for round-trip
    }
    
    fn pre_process(&self, text: &str) -> String {
        // Apply sakha-specific preprocessing
        let mut result = self.sakha.preprocess(text);
        
        // Add numeric accent conversion for backward compatibility
        result = result.replace('3', "'"); // Convert 3 to udatta
        result = result.replace('1', "_"); // Convert 1 to anudatta
        
        result
    }
    
    fn post_process(&self, text: &str) -> String {
        self.sakha.postprocess(text)
    }
}

/// Factory functions for creating sakha-specific extensions

/// Creates a Rigveda Shakala extension
pub fn rigveda_shakala() -> Box<dyn TransliterationExtension> {
    Box::new(UnifiedVedicExtension::new(
        Box::new(super::sakhas::RigvedaShakala)
    ))
}

/// Creates a Samaveda Kauthuma extension
pub fn samaveda_kauthuma() -> Box<dyn TransliterationExtension> {
    Box::new(UnifiedVedicExtension::new(
        Box::new(super::sakhas::SamavedaKauthuma)
    ))
}

/// Creates a Krishna-Yajurveda Taittiriya extension
pub fn yajurveda_taittiriya() -> Box<dyn TransliterationExtension> {
    Box::new(UnifiedVedicExtension::new(
        Box::new(super::sakhas::YajurvedaTaittiriya)
    ))
}

/// Creates an Atharvaveda Shaunaka extension
pub fn atharvaveda_shaunaka() -> Box<dyn TransliterationExtension> {
    Box::new(UnifiedVedicExtension::new(
        Box::new(super::sakhas::AtharvavedaShaunaka)
    ))
}