extern crate test_utils;
use lazy_static::lazy_static;
use test_utils::*;

lazy_static! {
    static ref S: Tester = Tester::with_svara_rules();
}

#[test]
fn sutra_6_2_27() {
    S.assert_has_karmadharaya("kumAra", "pratyenas", &["ku/mArapratyenas"]);
}

#[ignore]
#[test]
fn sutra_6_2_179() {
    S.assert_has_bahuvrihi("antar", "vana", &["antarvaRa/"]);
}

#[ignore]
#[test]
fn sutra_6_2_180() {
    S.assert_has_bahuvrihi("pra", "antar", &["prAnta/r"]);
}

#[test]
fn sutra_6_2_185() {
    S.assert_has_bahuvrihi("aBi", "muKa", &["aBimuKa/"]);
}

#[test]
fn sutra_6_2_186() {
    S.assert_has_bahuvrihi("apa", "muKa", &["apamuKa/"]);
}

#[test]
fn sutra_6_2_187() {
    S.assert_has_bahuvrihi("apa", "sPiga", &["apasPiga/"]);
    S.assert_has_bahuvrihi("apa", "pUta", &["apapUta/"]);
    S.assert_has_bahuvrihi("apa", "vIRA", &["apavIRA/"]);
    S.assert_has_bahuvrihi("apa", "aYjas", &["apAYja/s"]);
    S.assert_has_bahuvrihi("apa", "aDvan", &["apADva/n"]);
    S.assert_has_bahuvrihi("apa", "kukzi", &["apakukzi/"]);
    S.assert_has_bahuvrihi("apa", "sIra", &["apasIra/"]);
    S.assert_has_bahuvrihi("apa", "nAman", &["apanAma/n"]);

    // TODO: apahala, etc.
}
