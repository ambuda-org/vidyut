//! Unified Vedic Transliteration Scheme (UVTS) support.
//!
//! UVTS is an ASCII-safe encoding scheme that can represent all Vedic śākhās' accent systems,
//! regional pronunciation variants, and manuscript traditions using only ASCII characters (32-126).
//!
//! This module provides special handling for UVTS features that go beyond simple character mapping:
//! - Post-syllable accent notation (udātta \, anudātta /, svarita =)
//! - Musical notations for Sāmaveda
//! - Regional variant markers
//! - Manuscript tradition markers

use crate::scheme::Scheme;
use crate::transliterate;
use crate::Mapping;
use vidyut_chandas::Akshara;

/// Vedic accent types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VedicAccent {
    /// Udātta (high tone) - marked with \ after syllable in UVTS
    Udatta,
    /// Anudātta (low tone) - marked with / after syllable in UVTS  
    Anudatta,
    /// Svarita (falling tone) - marked with = after syllable in UVTS
    Svarita,
}

/// Information about a syllable and its accent
#[derive(Debug, Clone)]
pub struct AccentedSyllable {
    /// The syllable text
    pub text: String,
    /// The accent type, if any
    pub accent: Option<VedicAccent>,
}

/// Musical notation markers for Sāmaveda
#[derive(Debug, Clone, PartialEq)]
pub enum MusicalNotation {
    /// Elongation by 1 mātrā (+)
    Elongation1,
    /// Elongation by 2 mātrās (++)
    Elongation2,
    /// Elongation by 3 mātrās (+++)
    Elongation3,
    /// Short pause (^)
    PauseShort,
    /// Long pause (^^)
    PauseLong,
    /// Glide up (~)
    GlideUp,
    /// Glide down (`)
    GlideDown,
    /// Stobha non-semantic sound (%)
    Stobha,
    /// Pluta protracted vowel (@)
    Pluta,
    /// Kampana vibration (|)
    Kampana,
}

impl MusicalNotation {
    /// Converts the notation to its UVTS string representation
    pub fn as_str(&self) -> &str {
        match self {
            Self::Elongation1 => "+",
            Self::Elongation2 => "++",
            Self::Elongation3 => "+++",
            Self::PauseShort => "^",
            Self::PauseLong => "^^",
            Self::GlideUp => "~",
            Self::GlideDown => "`",
            Self::Stobha => "%",
            Self::Pluta => "@",
            Self::Kampana => "|",
        }
    }
    
    /// Parses a string to find musical notations
    pub fn parse_from_str(s: &str) -> Option<(Self, usize)> {
        if s.starts_with("+++") {
            Some((Self::Elongation3, 3))
        } else if s.starts_with("++") {
            Some((Self::Elongation2, 2))
        } else if s.starts_with("+") {
            Some((Self::Elongation1, 1))
        } else if s.starts_with("^^") {
            Some((Self::PauseLong, 2))
        } else if s.starts_with("^") {
            Some((Self::PauseShort, 1))
        } else if s.starts_with("~") {
            Some((Self::GlideUp, 1))
        } else if s.starts_with("`") {
            Some((Self::GlideDown, 1))
        } else if s.starts_with("%") {
            Some((Self::Stobha, 1))
        } else if s.starts_with("@") {
            Some((Self::Pluta, 1))
        } else if s.starts_with("|") {
            Some((Self::Kampana, 1))
        } else {
            None
        }
    }
}

/// Regional variant markers
#[derive(Debug, Clone)]
pub enum RegionalVariant {
    /// Andhra Pradesh pronunciation variant
    Andhra,
    /// Dravida pronunciation variant  
    Dravida,
    /// Maharashtra pronunciation variant
    Maharashtra,
    /// Gujarat pronunciation variant
    Gujarat,
    /// Kashmir Śāradā tradition
    Kashmir,
    /// Kerala pronunciation variant
    Kerala,
    /// Karnataka pronunciation variant
    Karnataka,
    /// Bengal pronunciation variant
    Bengal,
}

impl RegionalVariant {
    fn as_str(&self) -> &str {
        match self {
            Self::Andhra => "[andhra]",
            Self::Dravida => "[dravida]",
            Self::Maharashtra => "[maharastra]",
            Self::Gujarat => "[gujarat]",
            Self::Kashmir => "[kashmir]",
            Self::Kerala => "[kerala]",
            Self::Karnataka => "[karnataka]",
            Self::Bengal => "[bengal]",
        }
    }
}

/// Unicode Vedic accent markers
const UDATTA_MARK: char = '\u{0951}';
const ANUDATTA_MARK: char = '\u{0952}';

/// Transliterates text to UVTS with proper accent marking
pub fn to_uvts_with_accents(
    input: &str,
    from: Scheme,
    syllable_accents: &[AccentedSyllable],
) -> String {
    // First, do basic transliteration to UVTS
    let mapping = Mapping::new(from, Scheme::Uvts);
    let basic_uvts = transliterate(input, &mapping);
    
    // If no accent information provided, return basic transliteration
    if syllable_accents.is_empty() {
        return basic_uvts;
    }
    
    // Apply post-syllable accent markers
    apply_accent_markers(&basic_uvts, syllable_accents)
}

/// Transliterates Devanagari text with Unicode accent marks to UVTS with proper syllable-based accents
pub fn devanagari_to_uvts_with_vedic_accents(devanagari_text: &str) -> String {
    // Step 1: Convert to SLP1 for syllabification
    let mapping_to_slp1 = Mapping::new(Scheme::Devanagari, Scheme::Slp1);
    let slp1_text = transliterate(devanagari_text, &mapping_to_slp1);
    
    // Step 2: Extract accent information from Unicode marks
    let accent_positions = extract_accent_positions(devanagari_text);
    
    // Step 3: Syllabify the SLP1 text
    let aksharas = vidyut_chandas::akshara::scan_line(&slp1_text);
    
    // Step 4: Map accents to syllables
    let accented_syllables = map_accents_to_syllables(&aksharas, &accent_positions, &slp1_text);
    
    // Step 5: Build UVTS with proper accent placement
    build_uvts_from_syllables(&accented_syllables)
}

/// Extracts positions of accent marks in the text
fn extract_accent_positions(devanagari_text: &str) -> Vec<(usize, VedicAccent)> {
    let mut positions = Vec::new();
    let mut char_count = 0;
    
    for ch in devanagari_text.chars() {
        match ch {
            UDATTA_MARK => {
                positions.push((char_count, VedicAccent::Udatta));
            }
            ANUDATTA_MARK => {
                positions.push((char_count, VedicAccent::Anudatta));
            }
            // SVARITA_MARK handled separately if different
            _ => {}
        }
        
        // Don't increment for combining marks
        if !is_combining_mark(ch) {
            char_count += 1;
        }
    }
    
    positions
}

/// Check if a character is a combining mark
fn is_combining_mark(ch: char) -> bool {
    matches!(ch, '\u{0951}'..='\u{0954}' | '\u{0300}'..='\u{036F}')
}

/// Maps accent positions to syllables
fn map_accents_to_syllables(
    aksharas: &[Akshara],
    accent_positions: &[(usize, VedicAccent)],
    slp1_text: &str,
) -> Vec<AccentedSyllable> {
    let mut result = Vec::new();
    let mut char_offset = 0;
    
    // Build a map of character positions to syllable indices
    let mut pos_to_syllable = vec![0; slp1_text.len()];
    for (idx, akshara) in aksharas.iter().enumerate() {
        let syllable_len = akshara.text().len();
        for i in 0..syllable_len {
            if char_offset + i < pos_to_syllable.len() {
                pos_to_syllable[char_offset + i] = idx;
            }
        }
        char_offset += syllable_len;
    }
    
    // Create syllables with accent information
    for (idx, akshara) in aksharas.iter().enumerate() {
        let uvts_text = slp1_to_uvts_syllable(akshara.text());
        
        // Check if this syllable has an accent
        let mut syllable_accent = None;
        for &(pos, accent) in accent_positions {
            if pos < pos_to_syllable.len() && pos_to_syllable[pos] == idx {
                syllable_accent = Some(accent);
                break;
            }
        }
        
        result.push(AccentedSyllable {
            text: uvts_text,
            accent: syllable_accent,
        });
    }
    
    result
}

/// Builds UVTS text from accented syllables
fn build_uvts_from_syllables(syllables: &[AccentedSyllable]) -> String {
    let mut result = String::new();
    
    for syllable in syllables {
        result.push_str(&syllable.text);
        
        // Add accent marker after the syllable
        if let Some(accent) = syllable.accent {
            match accent {
                VedicAccent::Udatta => result.push('\\'),
                VedicAccent::Anudatta => result.push('/'),
                VedicAccent::Svarita => result.push('='),
            }
        }
    }
    
    result
}

/// Applies accent markers to UVTS text based on syllable information
fn apply_accent_markers(_uvts_text: &str, syllable_accents: &[AccentedSyllable]) -> String {
    let mut result = String::new();
    
    // This is a simplified implementation - a full version would need proper
    // syllable boundary detection in UVTS text
    for syllable in syllable_accents {
        result.push_str(&syllable.text);
        
        // Add accent marker after syllable
        if let Some(accent) = &syllable.accent {
            match accent {
                VedicAccent::Udatta => result.push('\\'),
                VedicAccent::Anudatta => result.push('/'),
                VedicAccent::Svarita => result.push('='),
            }
        }
    }
    
    result
}

/// Converts SLP1 syllables to UVTS syllables
pub fn slp1_to_uvts_syllable(slp1: &str) -> String {
    // Map SLP1 characters to UVTS equivalents
    slp1.chars()
        .map(|c| match c {
            // Vowels
            'f' => 'R', // ṛ
            'F' => 'q', // ṝ  
            'x' => 'L', // ḷ
            'X' => 'Q', // ḹ
            
            // Consonants
            'N' => 'f', // ṅ
            'Y' => 'F', // ñ
            'w' => 'w', // ṭ
            'W' => 'W', // ṭh
            'q' => 'x', // ḍ
            'Q' => 'X', // ḍh
            'R' => 'N', // ṇ
            'S' => 'S', // ś
            'z' => 'z', // ṣ
            
            // Others remain the same
            _ => c,
        })
        .collect()
}

/// Parses UVTS text with accent markers back into syllables
pub fn parse_uvts_accents(uvts_text: &str) -> Vec<AccentedSyllable> {
    let mut syllables = Vec::new();
    let mut current_syllable = String::new();
    let mut chars = uvts_text.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            // Accent markers
            '\\' => {
                if !current_syllable.is_empty() {
                    syllables.push(AccentedSyllable {
                        text: current_syllable.clone(),
                        accent: Some(VedicAccent::Udatta),
                    });
                    current_syllable.clear();
                }
            }
            '/' => {
                if !current_syllable.is_empty() {
                    syllables.push(AccentedSyllable {
                        text: current_syllable.clone(),
                        accent: Some(VedicAccent::Anudatta),
                    });
                    current_syllable.clear();
                }
            }
            '=' => {
                if !current_syllable.is_empty() {
                    syllables.push(AccentedSyllable {
                        text: current_syllable.clone(),
                        accent: Some(VedicAccent::Svarita),
                    });
                    current_syllable.clear();
                }
            }
            // Regular characters
            _ => {
                current_syllable.push(c);
            }
        }
    }
    
    // Add final syllable if any
    if !current_syllable.is_empty() {
        syllables.push(AccentedSyllable {
            text: current_syllable,
            accent: None,
        });
    }
    
    syllables
}

/// Adds a regional variant marker to UVTS text
pub fn add_regional_variant(uvts_text: &str, variant: RegionalVariant) -> String {
    format!("{} {}", uvts_text, variant.as_str())
}

/// Adds a manuscript tradition marker to UVTS text  
pub fn add_manuscript_marker(uvts_text: &str, tradition: &str) -> String {
    format!("{} [ms:{}]", uvts_text, tradition)
}

/// Parses UVTS text with musical notations (Sāmaveda)
pub fn parse_musical_notations(uvts_text: &str) -> Vec<(String, Option<MusicalNotation>)> {
    let mut result = Vec::new();
    let mut chars = uvts_text.chars().peekable();
    let mut current_text = String::new();
    
    while let Some(ch) = chars.next() {
        // Check if this could be the start of a musical notation
        let remaining: String = std::iter::once(ch).chain(chars.clone()).collect();
        
        if let Some((notation, consumed)) = MusicalNotation::parse_from_str(&remaining) {
            // Push the text accumulated so far
            if !current_text.is_empty() {
                result.push((current_text.clone(), None));
                current_text.clear();
            }
            
            // Push the notation
            result.push((notation.as_str().to_string(), Some(notation)));
            
            // Skip the consumed characters (minus 1 since we already consumed one)
            for _ in 0..(consumed - 1) {
                chars.next();
            }
        } else {
            current_text.push(ch);
        }
    }
    
    // Push any remaining text
    if !current_text.is_empty() {
        result.push((current_text, None));
    }
    
    result
}

/// Adds musical notation to UVTS text at the specified position
pub fn add_musical_notation(uvts_text: &str, position: usize, notation: MusicalNotation) -> String {
    let mut result = String::new();
    let chars: Vec<char> = uvts_text.chars().collect();
    
    for (i, ch) in chars.iter().enumerate() {
        result.push(*ch);
        if i == position {
            result.push_str(notation.as_str());
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_slp1_to_uvts_syllable() {
        assert_eq!(slp1_to_uvts_syllable("kf"), "kR");
        assert_eq!(slp1_to_uvts_syllable("jYa"), "jFa");
        assert_eq!(slp1_to_uvts_syllable("aNga"), "afga");
    }
    
    #[test]
    fn test_parse_uvts_accents() {
        let uvts = "a/gni\\m";
        let syllables = parse_uvts_accents(uvts);
        
        assert_eq!(syllables.len(), 3);  // "a", "gni", "m"
        assert_eq!(syllables[0].text, "a");
        assert_eq!(syllables[0].accent, Some(VedicAccent::Anudatta));
        assert_eq!(syllables[1].text, "gni");
        assert_eq!(syllables[1].accent, Some(VedicAccent::Udatta));
        assert_eq!(syllables[2].text, "m");
        assert_eq!(syllables[2].accent, None);
    }
    
    #[test]
    fn test_regional_variant() {
        let uvts = "saMskRtam";
        let with_variant = add_regional_variant(uvts, RegionalVariant::Andhra);
        assert_eq!(with_variant, "saMskRtam [andhra]");
    }
    
    #[test]
    fn test_syllabification_with_accents() {
        // Test with vidyut-chandas syllabification
        let slp1_text = "agnim";
        let aksharas = vidyut_chandas::akshara::scan_line(slp1_text);
        
        assert_eq!(aksharas.len(), 2);
        assert_eq!(aksharas[0].text(), "a");
        assert_eq!(aksharas[1].text(), "gnim");
    }
    
    #[test]
    fn test_build_uvts_from_syllables() {
        let syllables = vec![
            AccentedSyllable {
                text: "a".to_string(),
                accent: Some(VedicAccent::Anudatta),
            },
            AccentedSyllable {
                text: "gni".to_string(),
                accent: Some(VedicAccent::Udatta),
            },
            AccentedSyllable {
                text: "m".to_string(),
                accent: None,
            },
        ];
        
        let uvts = build_uvts_from_syllables(&syllables);
        assert_eq!(uvts, "a/gni\\m");
    }
    
    #[test]
    fn test_musical_notation_parsing() {
        let uvts = "agni+m I++Le";
        let parsed = parse_musical_notations(uvts);
        
        assert_eq!(parsed.len(), 5);
        assert_eq!(parsed[0], ("agni".to_string(), None));
        assert_eq!(parsed[1], ("+".to_string(), Some(MusicalNotation::Elongation1)));
        assert_eq!(parsed[2], ("m I".to_string(), None));
        assert_eq!(parsed[3], ("++".to_string(), Some(MusicalNotation::Elongation2)));
        assert_eq!(parsed[4], ("Le".to_string(), None));
    }
    
    #[test]
    fn test_add_musical_notation() {
        let uvts = "agnim";
        let with_notation = add_musical_notation(uvts, 3, MusicalNotation::GlideUp);
        assert_eq!(with_notation, "agni~m");
    }
    
    #[test]
    fn test_samaveda_example() {
        // Example from UVTS spec: agni\m+ I\Le++ puro\hita\M^
        let uvts = "agni\\m+ I\\Le++ puro\\hita\\M^";
        let parsed = parse_musical_notations(uvts);
        
        // Should have text segments with accents and musical notations
        let has_elongation1 = parsed.iter().any(|(_, n)| n.as_ref() == Some(&MusicalNotation::Elongation1));
        let has_elongation2 = parsed.iter().any(|(_, n)| n.as_ref() == Some(&MusicalNotation::Elongation2));
        let has_pause = parsed.iter().any(|(_, n)| n.as_ref() == Some(&MusicalNotation::PauseShort));
        
        assert!(has_elongation1);
        assert!(has_elongation2);
        assert!(has_pause);
    }
}