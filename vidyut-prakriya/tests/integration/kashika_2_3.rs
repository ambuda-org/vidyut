extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;

#[test]
fn sutra_2_3_28() {
    assert_has_sup_5p("vfka", Pum, &["vfkeByaH"]);
    assert_has_sup_5s("aDyayana", Napumsaka, &["aDyayanAt"]);
    // lyap-lopa
    assert_has_sup_5s("prAsAda", Napumsaka, &["prAsAdAt"]);
    // adhikarana
    assert_has_sup_5s("Asana", Napumsaka, &["AsanAt"]);
    assert_has_sup_5s("Sayana", Napumsaka, &["SayanAt"]);
    // prashna
    assert_has_sup_5s("pAwaliputra", Napumsaka, &["pAwaliputrAt"]);
    // TODO: tas-pratyaya
}

#[test]
fn sutra_2_3_30() {
    assert_has_sup_5s("grAma", Pum, &["grAmAt"]);
}

#[test]
fn sutra_2_3_47() {
    assert_has_sup_ss("devadatta", Pum, &["devadatta"]);
    assert_has_sup_sd("devadatta", Pum, &["devadattO"]);
    assert_has_sup_sp("devadatta", Pum, &["devadattAH"]);
}

#[test]
fn skip_sutra_2_3_48() {}

#[test]
fn sutra_2_3_49() {
    assert_has_sup_ss("pawu", Pum, &["pawo"]);
    assert_has_sup_ss("devadatta", Pum, &["devadatta"]);
}

#[test]
fn sutra_2_3_50() {
    assert_has_sup_6s("rAjan", Pum, &["rAjYaH"]);
    assert_has_sup_6s("paSu", Pum, &["paSoH"]);
    assert_has_sup_6s("pitf", Pum, &["pituH"]);
}
