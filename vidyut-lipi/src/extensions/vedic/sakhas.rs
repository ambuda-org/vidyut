//! Sakha-specific requirement specifications

use super::*;

/// Rigveda Shakala sakha requirements
pub struct RigvedaShakala;

impl VedicSakha for RigvedaShakala {
    fn requirements(&self) -> ExtensionRequirements {
        ExtensionRequirements {
            name: "Rigveda Shakala".to_string(),
            accent_marks: vec![
                AccentType::Udatta,
                AccentType::Anudatta,
                AccentType::Svarita,
            ],
            special_notations: vec![
                SpecialNotation::Avagraha,
            ],
            contextual_rules: vec![
                ContextualRule::AccentsOnAllVowels,
                ContextualRule::AccentedSandhiRules,
            ],
            minimum_required: RequirementLevel::Basic,
        }
    }
    
    fn is_supported_by(&self, adapter: &dyn SchemeAdapter) -> SupportLevel {
        let reqs = self.requirements();
        
        // Check if all basic accents are supported
        let has_basic_accents = reqs.accent_marks.iter()
            .all(|&accent| adapter.accent_representation(accent).is_some());
        
        if !has_basic_accents {
            return SupportLevel::None;
        }
        
        // Check for avagraha support
        let has_avagraha = adapter.notation_representation(&SpecialNotation::Avagraha).is_some();
        
        if has_avagraha {
            SupportLevel::Full
        } else {
            SupportLevel::Partial { missing: "avagraha" }
        }
    }
}

/// Samaveda Kauthuma sakha requirements
pub struct SamavedaKauthuma;

impl VedicSakha for SamavedaKauthuma {
    fn requirements(&self) -> ExtensionRequirements {
        ExtensionRequirements {
            name: "Samaveda Kauthuma".to_string(),
            accent_marks: vec![
                AccentType::Udatta,
                AccentType::Anudatta,
                AccentType::Svarita,
                AccentType::Kampa,
            ],
            special_notations: vec![
                SpecialNotation::MusicalPause(1),
                SpecialNotation::MusicalPause(2),
                SpecialNotation::MusicalPause(3),
                SpecialNotation::Avagraha,
            ],
            contextual_rules: vec![
                ContextualRule::AccentsOnAllVowels,
                ContextualRule::MusicalPhrasing,
            ],
            minimum_required: RequirementLevel::Partial,
        }
    }
    
    fn is_supported_by(&self, adapter: &dyn SchemeAdapter) -> SupportLevel {
        let _reqs = self.requirements();
        
        // Must have at least basic accents
        let basic_accents = [AccentType::Udatta, AccentType::Anudatta];
        let has_basic = basic_accents.iter()
            .all(|&accent| adapter.accent_representation(accent).is_some());
        
        if !has_basic {
            return SupportLevel::None;
        }
        
        // Check for musical features
        let has_kampa = adapter.accent_representation(AccentType::Kampa).is_some();
        let has_pauses = (1..=3).any(|i| {
            adapter.notation_representation(&SpecialNotation::MusicalPause(i)).is_some()
        });
        
        match (has_kampa, has_pauses) {
            (true, true) => SupportLevel::Full,
            (true, false) => SupportLevel::Partial { missing: "musical pauses" },
            (false, true) => SupportLevel::Partial { missing: "kampa" },
            (false, false) => SupportLevel::Partial { missing: "musical notation" },
        }
    }
}

/// Krishna-Yajurveda Taittiriya sakha requirements
pub struct YajurvedaTaittiriya;

impl VedicSakha for YajurvedaTaittiriya {
    fn requirements(&self) -> ExtensionRequirements {
        ExtensionRequirements {
            name: "Krishna-Yajurveda Taittiriya".to_string(),
            accent_marks: vec![
                AccentType::Udatta,
                AccentType::Anudatta,
                AccentType::Svarita,
                AccentType::DirghaSvarita,
            ],
            special_notations: vec![
                SpecialNotation::PadaBoundary,
                SpecialNotation::Avagraha,
                SpecialNotation::AnunasikaType(AnunasikaVariant::Chandrabindu),
            ],
            contextual_rules: vec![
                ContextualRule::AccentsOnAllVowels,
                ContextualRule::PreservePadaBoundaries,
                ContextualRule::RegionalVariant("Andhra".to_string()),
                ContextualRule::RegionalVariant("Dravida".to_string()),
            ],
            minimum_required: RequirementLevel::Partial,
        }
    }
    
    fn is_supported_by(&self, adapter: &dyn SchemeAdapter) -> SupportLevel {
        let _reqs = self.requirements();
        
        // Must support basic accents
        let has_basic = [AccentType::Udatta, AccentType::Anudatta]
            .iter()
            .all(|&accent| adapter.accent_representation(accent).is_some());
        
        if !has_basic {
            return SupportLevel::None;
        }
        
        // Check for pada boundary support (critical for Yajurveda)
        let has_pada = adapter.notation_representation(&SpecialNotation::PadaBoundary).is_some();
        
        if !has_pada {
            return SupportLevel::Partial { missing: "pada boundaries" };
        }
        
        // Check for other features
        let has_dirgha = adapter.accent_representation(AccentType::DirghaSvarita).is_some();
        let has_chandrabindu = adapter.notation_representation(
            &SpecialNotation::AnunasikaType(AnunasikaVariant::Chandrabindu)
        ).is_some();
        
        match (has_dirgha, has_chandrabindu) {
            (true, true) => SupportLevel::Full,
            (true, false) => SupportLevel::Partial { missing: "chandrabindu" },
            (false, true) => SupportLevel::Partial { missing: "dirgha svarita" },
            (false, false) => SupportLevel::Partial { missing: "advanced features" },
        }
    }
    
    fn preprocess(&self, text: &str) -> String {
        // Handle BarahaSouth encoding if present
        text.replace("q", "̱")   // Convert special accent markers
            .to_string()
    }
}

/// Atharvaveda Shaunaka sakha requirements
pub struct AtharvavedaShaunaka;

impl VedicSakha for AtharvavedaShaunaka {
    fn requirements(&self) -> ExtensionRequirements {
        ExtensionRequirements {
            name: "Atharvaveda Shaunaka".to_string(),
            accent_marks: vec![
                AccentType::Udatta,
                AccentType::Anudatta,
                AccentType::Svarita,
                AccentType::Pracaya,
            ],
            special_notations: vec![
                SpecialNotation::Avagraha,
                SpecialNotation::Jihvamuliya,
                SpecialNotation::Upadhmaniya,
            ],
            contextual_rules: vec![
                ContextualRule::AccentsOnAllVowels,
                ContextualRule::AccentedSandhiRules,
            ],
            minimum_required: RequirementLevel::Basic,
        }
    }
    
    fn is_supported_by(&self, adapter: &dyn SchemeAdapter) -> SupportLevel {
        let _reqs = self.requirements();
        
        // Check basic accent support
        let has_basic = [AccentType::Udatta, AccentType::Anudatta, AccentType::Svarita]
            .iter()
            .all(|&accent| adapter.accent_representation(accent).is_some());
        
        if !has_basic {
            return SupportLevel::None;
        }
        
        // Check for special fricatives
        let has_fricatives = [
            SpecialNotation::Jihvamuliya,
            SpecialNotation::Upadhmaniya,
        ].iter().any(|notation| adapter.notation_representation(notation).is_some());
        
        let has_pracaya = adapter.accent_representation(AccentType::Pracaya).is_some();
        
        match (has_fricatives, has_pracaya) {
            (true, true) => SupportLevel::Full,
            (true, false) => SupportLevel::Partial { missing: "pracaya" },
            (false, true) => SupportLevel::Partial { missing: "special fricatives" },
            (false, false) => SupportLevel::Partial { missing: "advanced features" },
        }
    }
}