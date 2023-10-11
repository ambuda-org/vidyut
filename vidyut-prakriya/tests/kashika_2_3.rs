extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;

#[test]
fn sutra_2_3_28() {
    assert_has_subantas("vfka", Pum, Panchami, Bahu, &["vfkeByaH"]);
    assert_has_subantas("aDyayana", Napumsaka, Panchami, Eka, &["aDyayanAt"]);
    // lyap-lopa
    assert_has_subantas("prAsAda", Napumsaka, Panchami, Eka, &["prAsAdAt"]);
    // adhikarana
    assert_has_subantas("Asana", Napumsaka, Panchami, Eka, &["AsanAt"]);
    assert_has_subantas("Sayana", Napumsaka, Panchami, Eka, &["SayanAt"]);
    // prashna
    assert_has_subantas("pAwaliputra", Napumsaka, Panchami, Eka, &["pAwaliputrAt"]);
    // TODO: tas-pratyaya
}

#[test]
fn sutra_2_3_30() {
    assert_has_subantas("grAma", Pum, Panchami, Eka, &["grAmAt"]);
}

#[test]
fn sutra_2_3_47() {
    assert_has_subantas("devadatta", Pum, Sambodhana, Eka, &["devadatta"]);
    assert_has_subantas("devadatta", Pum, Sambodhana, Dvi, &["devadattO"]);
    assert_has_subantas("devadatta", Pum, Sambodhana, Bahu, &["devadattAH"]);
}

#[test]
fn sutra_2_3_49() {
    assert_has_subantas("pawu", Pum, Sambodhana, Eka, &["pawo"]);
    assert_has_subantas("devadatta", Pum, Sambodhana, Eka, &["devadatta"]);
}

#[test]
fn sutra_2_3_50() {
    assert_has_subantas("rAjan", Pum, Sasthi, Eka, &["rAjYaH"]);
    assert_has_subantas("paSu", Pum, Sasthi, Eka, &["paSoH"]);
    assert_has_subantas("pitf", Pum, Sasthi, Eka, &["pituH"]);
}
