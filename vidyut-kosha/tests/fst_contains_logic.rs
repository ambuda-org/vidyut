
use vidyut_kosha::Builder;
use vidyut_kosha::Kosha;
use vidyut_kosha::packing::PackedEntry;
use vidyut_prakriya::args::{Linga, Vibhakti, Vacana, Pratipadika, Slp1String};
use tempfile::tempdir;

/// Bug 2: `contains_key` returning true for keys that are ONLY prefixes.
#[test]
fn test_repro_bug_2_prefix_is_not_a_word() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let mut builder = Builder::new(dir.path())?;

    let agni_pratipadika = Pratipadika::basic(Slp1String::from("agni")?);
    let agni_pratipadika_e = vidyut_kosha::entries::PratipadikaEntry::try_from(&agni_pratipadika)?;
    
    let padas = vec![
        ("agnim".to_string(), Linga::Pum, Vibhakti::Dvitiya, Vacana::Eka),
        ("agninA".to_string(), Linga::Pum, Vibhakti::Trtiya, Vacana::Eka),
    ];
    let (prefix, packed_prefix) = builder.register_subanta_paradigm(&agni_pratipadika_e, &padas)?;
    
    // Insert "agni" ONLY as a prefix.
    builder.insert_packed(&prefix, &packed_prefix)?;
    builder.finish()?;

    let kosha = Kosha::new(dir.path())?;
    let has_key = kosha.contains_key("agni");
    println!("contains_key('agni') = {}", has_key);
    
    // Bug 2: contains_key("agni") should be false because "agni" is only a prefix.
    assert!(!has_key, "'agni' is only a prefix and should not be a key");

    Ok(())
}

/// Bug 1: `contains_suffix` missing `final_output` when identifying prefixes.
#[test]
fn test_repro_bug_1_prefix_with_final_output() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let mut builder = Builder::new(dir.path())?;

    let agni_pratipadika = Pratipadika::basic(Slp1String::from("agni")?);
    let agni_pratipadika_e = vidyut_kosha::entries::PratipadikaEntry::try_from(&agni_pratipadika)?;
    let padas = vec![
        ("agnim".to_string(), Linga::Pum, Vibhakti::Dvitiya, Vacana::Eka),
        ("agninA".to_string(), Linga::Pum, Vibhakti::Trtiya, Vacana::Eka),
    ];
    let (prefix, packed_prefix) = builder.register_subanta_paradigm(&agni_pratipadika_e, &padas)?;
    assert_eq!(prefix, "agni");

    // Strategy: Force the FST to store a non-zero `final_output` on the prefix node.
    // In an FST, if prefix P has value `v_p` and longer key W has value `v_w`:
    // 1. Accumulate common output `C` on shared edges.
    // 2. `C = min(v_p, v_w)`.
    // 3. `node_at_prefix.final_output = v_p - C`.
    //
    // If we ensure `v_p > v_w`, then `C = v_w`, and `final_output` is `v_p - v_w > 0`.
    // The buggy code incorrectly ignores `final_output` when reconstructing the prefix's 
    // packed value, which causes the subsequent suffix lookup to fail.
    let p_u32 = u32::from_le_bytes(packed_prefix.into_bytes());
    let w_u32 = p_u32 / 2;
    let packed_w = PackedEntry::from_u32(w_u32);

    builder.insert_packed("agni", &packed_prefix)?;
    builder.insert_packed("agnihotf", &packed_w)?;
    builder.finish()?;

    let kosha = Kosha::new(dir.path())?;

    // Bug 1: Without the fix, contains_key("agnim") fails because it misses final_output.
    assert!(kosha.contains_key("agnim"), "'agnim' should be found via its prefix");

    Ok(())
}
