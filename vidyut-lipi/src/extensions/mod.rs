//! Extensions for customizing transliteration behavior.

/// Vedic transliteration extensions for sakha-specific schemes.
pub mod vedic;

use crate::mapping::Mapping;

/// A trait for extending transliteration behavior.
///
/// Extensions can modify the mapping table and apply pre/post-processing
/// to handle special cases like Vedic accents or sakha-specific conventions.
pub trait TransliterationExtension: Send + Sync {
    /// Returns the name of this extension.
    fn name(&self) -> &str;
    
    /// Extends the mapping with additional character mappings.
    ///
    /// This is called during `Mapping` construction to add extension-specific mappings.
    fn extend_mapping(&self, mapping: &mut Mapping);
    
    /// Pre-processes input text before transliteration.
    ///
    /// This can be used to normalize or prepare text for transliteration.
    fn pre_process(&self, text: &str) -> String {
        text.to_string()
    }
    
    /// Post-processes output text after transliteration.
    ///
    /// This can be used to apply final transformations or fix edge cases.
    fn post_process(&self, text: &str) -> String {
        text.to_string()
    }
}