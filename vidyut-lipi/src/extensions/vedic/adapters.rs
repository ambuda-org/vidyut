//! Scheme-specific adapters that implement Vedic requirements

use super::*;
use crate::Scheme;
use std::collections::HashMap;

/// WX scheme adapter - ASCII only
pub struct WxAdapter;

impl SchemeAdapter for WxAdapter {
    fn scheme(&self) -> Scheme {
        Scheme::Wx
    }

    fn accent_representation(&self, accent: AccentType) -> Option<String> {
        match accent {
            AccentType::Udatta => Some("=".to_string()),
            AccentType::Anudatta => Some("_".to_string()),
            AccentType::Svarita => Some("^".to_string()),
            AccentType::Pracaya => Some("~".to_string()),
            AccentType::DirghaSvarita => Some("==".to_string()),
            AccentType::Kampa => Some("*".to_string()),
        }
    }

    fn notation_representation(&self, notation: &SpecialNotation) -> Option<String> {
        match notation {
            SpecialNotation::PadaBoundary => Some("#".to_string()),
            SpecialNotation::MusicalPause(1) => Some("|1|".to_string()),
            SpecialNotation::MusicalPause(2) => Some("|2|".to_string()),
            SpecialNotation::MusicalPause(3) => Some("|3|".to_string()),
            SpecialNotation::MusicalPause(_) => Some("|?|".to_string()),
            SpecialNotation::Avagraha => Some(".".to_string()),
            SpecialNotation::Jihvamuliya => Some("H".to_string()),
            SpecialNotation::Upadhmaniya => Some("F".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Standard) => Some("M".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Chandrabindu) => {
                Some("~M".to_string())
            }
            SpecialNotation::AnunasikaType(AnunasikaVariant::Ardhabindu) => Some("'M".to_string()),
            SpecialNotation::Custom(s) => Some(format!("[{}]", s)),
        }
    }

    fn supports_rule(&self, rule: &ContextualRule) -> bool {
        match rule {
            ContextualRule::AccentsOnAllVowels => true,
            ContextualRule::PreservePadaBoundaries => true,
            _ => false,
        }
    }

    fn create_mappings(&self, requirements: &ExtensionRequirements) -> HashMap<String, String> {
        let mut mappings = HashMap::new();

        // Add accent mappings for all vowels
        let vowels = ["a", "A", "i", "I", "u", "U", "q", "Q", "e", "E", "o", "O"];

        for vowel in &vowels {
            for accent in &requirements.accent_marks {
                if let Some(mark) = self.accent_representation(*accent) {
                    // WX uses postfix notation
                    mappings.insert(format!("{}{}", vowel, mark), format!("{}{}", vowel, mark));
                }
            }
        }

        mappings
    }
}

/// Devanagari scheme adapter
pub struct DevanagariAdapter;

impl SchemeAdapter for DevanagariAdapter {
    fn scheme(&self) -> Scheme {
        Scheme::Devanagari
    }

    fn accent_representation(&self, accent: AccentType) -> Option<String> {
        match accent {
            AccentType::Udatta => Some("\u{0951}".to_string()),
            AccentType::Anudatta => Some("\u{0952}".to_string()),
            AccentType::Svarita => Some("\u{0951}\u{0952}".to_string()),
            AccentType::Pracaya => Some("\u{0953}".to_string()),
            AccentType::DirghaSvarita => Some("\u{0954}".to_string()),
            AccentType::Kampa => Some("\u{0971}".to_string()), // Vedic tone mark
        }
    }

    fn notation_representation(&self, notation: &SpecialNotation) -> Option<String> {
        match notation {
            SpecialNotation::PadaBoundary => Some("।".to_string()),
            SpecialNotation::MusicalPause(1) => Some("१".to_string()),
            SpecialNotation::MusicalPause(2) => Some("२".to_string()),
            SpecialNotation::MusicalPause(3) => Some("३".to_string()),
            SpecialNotation::MusicalPause(_) => None,
            SpecialNotation::Avagraha => Some("ऽ".to_string()),
            SpecialNotation::Jihvamuliya => Some("ᳵ".to_string()),
            SpecialNotation::Upadhmaniya => Some("ᳶ".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Standard) => Some("ं".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Chandrabindu) => Some("ँ".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Ardhabindu) => Some("ꣳ".to_string()),
            SpecialNotation::Custom(_) => None,
        }
    }

    fn supports_rule(&self, rule: &ContextualRule) -> bool {
        match rule {
            ContextualRule::AccentsOnAllVowels => true,
            ContextualRule::MusicalPhrasing => true,
            ContextualRule::PreservePadaBoundaries => true,
            ContextualRule::AccentedSandhiRules => true,
            ContextualRule::RegionalVariant(_) => true,
        }
    }

    fn create_mappings(&self, _requirements: &ExtensionRequirements) -> HashMap<String, String> {
        // Devanagari handles accents as combining marks, so no pre-mapping needed
        HashMap::new()
    }
}

/// ISO 15919 scheme adapter
pub struct Iso15919Adapter;

impl SchemeAdapter for Iso15919Adapter {
    fn scheme(&self) -> Scheme {
        Scheme::Iso15919
    }

    fn accent_representation(&self, accent: AccentType) -> Option<String> {
        match accent {
            AccentType::Udatta => Some("\u{0301}".to_string()), // Combining acute
            AccentType::Anudatta => Some("\u{0300}".to_string()), // Combining grave
            AccentType::Svarita => Some("\u{0302}".to_string()), // Combining circumflex
            AccentType::Pracaya => Some("\u{0303}".to_string()), // Combining tilde
            AccentType::DirghaSvarita => Some("\u{0304}".to_string()), // Combining macron
            AccentType::Kampa => Some("\u{0330}".to_string()),  // Combining tilde below
        }
    }

    fn notation_representation(&self, notation: &SpecialNotation) -> Option<String> {
        match notation {
            SpecialNotation::PadaBoundary => Some(" | ".to_string()),
            SpecialNotation::MusicalPause(n) => Some(format!(" [{}] ", n)),
            SpecialNotation::Avagraha => Some("'".to_string()),
            SpecialNotation::Jihvamuliya => Some("ḫ".to_string()),
            SpecialNotation::Upadhmaniya => Some("ḟ".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Standard) => Some("ṁ".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Chandrabindu) => Some("m̐".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Ardhabindu) => Some("ṁ̇".to_string()),
            SpecialNotation::Custom(s) => Some(format!("<{}>", s)),
        }
    }

    fn supports_rule(&self, rule: &ContextualRule) -> bool {
        match rule {
            ContextualRule::AccentsOnAllVowels => true,
            ContextualRule::PreservePadaBoundaries => true,
            ContextualRule::AccentedSandhiRules => true,
            _ => false,
        }
    }

    fn create_mappings(&self, _requirements: &ExtensionRequirements) -> HashMap<String, String> {
        // ISO uses combining marks, handled during transliteration
        HashMap::new()
    }
}

/// IAST scheme adapter
pub struct IastAdapter;

impl SchemeAdapter for IastAdapter {
    fn scheme(&self) -> Scheme {
        Scheme::Iast
    }

    fn accent_representation(&self, accent: AccentType) -> Option<String> {
        // IAST traditionally doesn't mark accents, but we can use combining marks
        match accent {
            AccentType::Udatta => Some("\u{0301}".to_string()),
            AccentType::Anudatta => Some("\u{0300}".to_string()),
            AccentType::Svarita => Some("\u{0302}".to_string()),
            _ => None,
        }
    }

    fn notation_representation(&self, notation: &SpecialNotation) -> Option<String> {
        match notation {
            SpecialNotation::Avagraha => Some("'".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Standard) => Some("ṃ".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Chandrabindu) => Some("m̐".to_string()),
            _ => None,
        }
    }

    fn supports_rule(&self, rule: &ContextualRule) -> bool {
        match rule {
            ContextualRule::AccentsOnAllVowels => true,
            _ => false,
        }
    }

    fn create_mappings(&self, _requirements: &ExtensionRequirements) -> HashMap<String, String> {
        HashMap::new()
    }
}

/// Telugu scheme adapter with traditional Vedic marks
pub struct TeluguAdapter;

impl SchemeAdapter for TeluguAdapter {
    fn scheme(&self) -> Scheme {
        Scheme::Telugu
    }

    fn accent_representation(&self, accent: AccentType) -> Option<String> {
        // Telugu uses same Unicode Vedic marks as Devanagari
        match accent {
            AccentType::Udatta => Some("\u{0951}".to_string()),
            AccentType::Anudatta => Some("\u{0952}".to_string()),
            AccentType::Svarita => Some("\u{0951}\u{0952}".to_string()),
            _ => None,
        }
    }

    fn notation_representation(&self, notation: &SpecialNotation) -> Option<String> {
        match notation {
            SpecialNotation::PadaBoundary => Some("।".to_string()),
            SpecialNotation::Avagraha => Some("ఽ".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Standard) => Some("ం".to_string()),
            SpecialNotation::AnunasikaType(AnunasikaVariant::Chandrabindu) => Some("ఁ".to_string()),
            _ => None,
        }
    }

    fn supports_rule(&self, rule: &ContextualRule) -> bool {
        match rule {
            ContextualRule::AccentsOnAllVowels => true,
            ContextualRule::RegionalVariant(s) if s == "Andhra" => true,
            _ => false,
        }
    }

    fn create_mappings(&self, _requirements: &ExtensionRequirements) -> HashMap<String, String> {
        HashMap::new()
    }
}
