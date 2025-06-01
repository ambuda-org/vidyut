//! Test that WX scheme produces ASCII-only output

use vidyut_lipi::{Lipika, Scheme};
use vidyut_lipi::extensions::vedic::rigveda_shakala;

#[test]
fn test_wx_ascii_only_output() {
    let mut lipika = Lipika::new()
        .with_extension(rigveda_shakala());
    
    // Test Devanagari with udatta mark to WX
    let input_with_udatta = "अ॑ग्नि"; // agni with udatta ॑
    let wx_result = lipika.transliterate(input_with_udatta, Scheme::Devanagari, Scheme::Wx);
    
    println!("Input: {} → WX: {}", input_with_udatta, wx_result);
    
    // Verify the result is ASCII-only
    assert!(wx_result.is_ascii(), "WX output should be ASCII-only, got: {}", wx_result);
    
    // Should contain = for udatta, not Unicode ॑
    assert!(wx_result.contains('='), "WX should use = for udatta, got: {}", wx_result);
    assert!(!wx_result.contains('\u{0951}'), "WX should not contain Unicode udatta mark ॑");
    
    // Test anudatta as well
    let input_with_anudatta = "सोम॒"; // soma with anudatta ॒
    let wx_result2 = lipika.transliterate(input_with_anudatta, Scheme::Devanagari, Scheme::Wx);
    
    println!("Input: {} → WX: {}", input_with_anudatta, wx_result2);
    
    assert!(wx_result2.is_ascii(), "WX output should be ASCII-only, got: {}", wx_result2);
    assert!(wx_result2.contains('_'), "WX should use _ for anudatta, got: {}", wx_result2);
    assert!(!wx_result2.contains('\u{0952}'), "WX should not contain Unicode anudatta mark ॒");
}