//! Requirements-based Vedic Extension System
//! 
//! This module provides a flexible extension system where:
//! 1. Sakhas specify WHAT they need (requirements)
//! 2. Schemes specify HOW they implement those requirements
//! 3. The system automatically resolves the appropriate representations

use std::collections::HashMap;

pub mod adapters;
pub mod sakhas;
pub mod integration;

// Re-export key types for convenience
pub use integration::{rigveda_shakala, samaveda_kauthuma, yajurveda_taittiriya, atharvaveda_shaunaka};

/// Types of accents used in Vedic texts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccentType {
    /// उदात्त - High tone
    Udatta,
    /// अनुदात्त - Low tone  
    Anudatta,
    /// स्वरित - Falling tone (combination of udatta + anudatta)
    Svarita,
    /// प्रचय - Accumulated svarita
    Pracaya,
    /// दीर्घ स्वरित - Long svarita
    DirghaSvarita,
    /// काम्प - Tremulous tone (Samaveda)
    Kampa,
}

/// Special notations and marks beyond basic accents
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpecialNotation {
    /// Pada boundary marker (Yajurveda)
    PadaBoundary,
    /// Musical pause of specified duration (Samaveda)
    MusicalPause(u8),
    /// Avagraha (elision marker)
    Avagraha,
    /// Jihvamuliya (velar fricative before ka/kha)
    Jihvamuliya,
    /// Upadhmaniya (bilabial fricative before pa/pha)
    Upadhmaniya,
    /// Anunasika variations
    AnunasikaType(AnunasikaVariant),
    /// Regional or sakha-specific marks
    Custom(String),
}

/// Variants of anunasika (nasal) marks
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnunasikaVariant {
    /// Standard anusvara
    Standard,
    /// Chandrabindu
    Chandrabindu,
    /// Ardhabindu
    Ardhabindu,
}

/// Contextual rules that affect transliteration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextualRule {
    /// Accents can appear on all vowels (not just 'a')
    AccentsOnAllVowels,
    /// Special handling for musical phrasing
    MusicalPhrasing,
    /// Preserve pada boundaries in continuous text
    PreservePadaBoundaries,
    /// Apply sandhi rules differently in accented contexts
    AccentedSandhiRules,
    /// Regional pronunciation variants
    RegionalVariant(String),
}

/// Requirements specification for a Vedic extension
pub struct ExtensionRequirements {
    /// Name of the sakha/tradition
    pub name: String,
    /// Required accent marks
    pub accent_marks: Vec<AccentType>,
    /// Required special notations
    pub special_notations: Vec<SpecialNotation>,
    /// Contextual rules to apply
    pub contextual_rules: Vec<ContextualRule>,
    /// Minimum required for basic support
    pub minimum_required: RequirementLevel,
}

/// Level of support required for a feature
#[derive(Debug, Clone, Copy)]
pub enum RequirementLevel {
    /// Must support all features
    Full,
    /// Can work with subset of features
    Partial,
    /// Only basic accents needed
    Basic,
}

/// Trait for Vedic sakha extensions
pub trait VedicSakha: Send + Sync {
    /// Returns the requirements for this sakha
    fn requirements(&self) -> ExtensionRequirements;
    
    /// Validates if a scheme adapter supports this sakha
    fn is_supported_by(&self, adapter: &dyn SchemeAdapter) -> SupportLevel;
    
    /// Pre-processes text according to sakha conventions
    fn preprocess(&self, text: &str) -> String {
        text.to_string()
    }
    
    /// Post-processes text according to sakha conventions  
    fn postprocess(&self, text: &str) -> String {
        text.to_string()
    }
}

/// Level of support provided by a scheme
#[derive(Debug, Clone, Copy)]
pub enum SupportLevel {
    /// Full support for all features
    Full,
    /// Partial support (some features missing)
    Partial { 
        /// Description of missing features
        missing: &'static str 
    },
    /// Not supported
    None,
}

/// How a specific scheme implements Vedic requirements
pub trait SchemeAdapter: Send + Sync {
    /// Returns the scheme this adapter is for
    fn scheme(&self) -> crate::Scheme;
    
    /// Returns accent mark representations
    fn accent_representation(&self, accent: AccentType) -> Option<String>;
    
    /// Returns special notation representations
    fn notation_representation(&self, notation: &SpecialNotation) -> Option<String>;
    
    /// Checks if a contextual rule is supported
    fn supports_rule(&self, rule: &ContextualRule) -> bool;
    
    /// Apply the adapter's representations to create mappings
    fn create_mappings(&self, requirements: &ExtensionRequirements) -> HashMap<String, String>;
}

/// Resolves requirements to concrete mappings
pub struct ExtensionResolver {
    adapters: HashMap<crate::Scheme, Box<dyn SchemeAdapter>>,
}

impl ExtensionResolver {
    /// Creates a new extension resolver
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }
    
    /// Register a scheme adapter
    pub fn register_adapter(&mut self, adapter: Box<dyn SchemeAdapter>) {
        self.adapters.insert(adapter.scheme(), adapter);
    }
    
    /// Resolve requirements for a specific transliteration direction
    pub fn resolve(
        &self,
        sakha: &dyn VedicSakha,
        from_scheme: crate::Scheme,
        to_scheme: crate::Scheme,
    ) -> Result<HashMap<String, String>, String> {
        let requirements = sakha.requirements();
        
        // Get adapters for both schemes
        let from_adapter = self.adapters.get(&from_scheme)
            .ok_or_else(|| format!("No Vedic adapter for scheme {:?}", from_scheme))?;
        let to_adapter = self.adapters.get(&to_scheme)
            .ok_or_else(|| format!("No Vedic adapter for scheme {:?}", to_scheme))?;
        
        // Check support levels
        match (sakha.is_supported_by(from_adapter.as_ref()), sakha.is_supported_by(to_adapter.as_ref())) {
            (SupportLevel::None, _) | (_, SupportLevel::None) => {
                return Err("Sakha not supported by one or both schemes".to_string());
            }
            (SupportLevel::Partial { missing: m1 }, SupportLevel::Partial { missing: m2 }) => {
                eprintln!("Warning: Partial support - missing: {}, {}", m1, m2);
            }
            _ => {}
        }
        
        // Create bidirectional mappings
        let mut mappings = HashMap::new();
        
        // Map from source representations to target representations
        for accent in &requirements.accent_marks {
            if let (Some(from_repr), Some(to_repr)) = (
                from_adapter.accent_representation(*accent),
                to_adapter.accent_representation(*accent),
            ) {
                mappings.insert(from_repr, to_repr);
            }
        }
        
        for notation in &requirements.special_notations {
            if let (Some(from_repr), Some(to_repr)) = (
                from_adapter.notation_representation(notation),
                to_adapter.notation_representation(notation),
            ) {
                mappings.insert(from_repr, to_repr);
            }
        }
        
        Ok(mappings)
    }
}