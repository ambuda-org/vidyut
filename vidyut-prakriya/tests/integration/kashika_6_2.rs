extern crate test_utils;
use std::sync::OnceLock;

use test_utils::*;

static S: OnceLock<Tester> = OnceLock::new();

fn get_tester() -> &'static Tester {
    S.get_or_init(|| Tester::with_svara_rules())
}

#[test]
fn sutra_6_2_27() {
    let s = get_tester();
    s.assert_has_karmadharaya("kumAra", "pratyenas", &["ku/mArapratyenas"]);
}

#[ignore]
#[test]
fn sutra_6_2_179() {
    let s = get_tester();
    s.assert_has_bahuvrihi("antar", "vana", &["antarvaRa/"]);
}

#[ignore]
#[test]
fn sutra_6_2_180() {
    let s = get_tester();
    s.assert_has_bahuvrihi("pra", "antar", &["prAnta/r"]);
}

#[test]
fn sutra_6_2_185() {
    let s = get_tester();
    s.assert_has_bahuvrihi("aBi", "muKa", &["aBimuKa/"]);
}

#[test]
fn sutra_6_2_186() {
    let s = get_tester();
    s.assert_has_bahuvrihi("apa", "muKa", &["apamuKa/"]);
}

#[test]
fn sutra_6_2_187() {
    let s = get_tester();
    s.assert_has_bahuvrihi("apa", "sPiga", &["apasPiga/"]);
    s.assert_has_bahuvrihi("apa", "pUta", &["apapUta/"]);
    s.assert_has_bahuvrihi("apa", "vIRA", &["apavIRA/"]);
    s.assert_has_bahuvrihi("apa", "aYjas", &["apAYja/s"]);
    s.assert_has_bahuvrihi("apa", "aDvan", &["apADva/n"]);
    s.assert_has_bahuvrihi("apa", "kukzi", &["apakukzi/"]);
    s.assert_has_bahuvrihi("apa", "sIra", &["apasIra/"]);
    s.assert_has_bahuvrihi("apa", "nAman", &["apanAma/n"]);

    // TODO: apahala, etc.
}
