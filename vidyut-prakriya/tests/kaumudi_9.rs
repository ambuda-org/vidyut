extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;

#[test]
fn sk_309() {
    assert_has_sup_1s("jYAna", Napumsaka, &["jYAnam"]);
    assert_has_sup_ss("jYAna", Napumsaka, &["jYAna"]);
}

#[test]
fn skip_sk_310() {}

#[test]
fn sk_311() {
    assert_has_sup_1d("jYAna", Napumsaka, &["jYAne"]);
}

#[test]
fn skip_sk_312_to_sk_313() {}

#[test]
fn sk_315() {
    assert_has_sup_1p("jYAna", Napumsaka, &["jYAnAni"]);
}

#[test]
fn skip_sk_316() {}

#[test]
fn sk_319() {
    assert_has_sup_1s("vAri", Napumsaka, &["vAri"]);
}

#[ignore]
#[test]
fn sk_320() {
    assert_has_sup_1d("vAri", Napumsaka, &["vAriRI"]);
    assert_has_sup_1p("vAri", Napumsaka, &["vArIRi"]);
    assert_has_sup_ss("vAri", Napumsaka, &["vAri", "vAre"]);
    assert_has_sup_3s("vAri", Napumsaka, &["vAriRA"]);
    assert_has_sup_4s("vAri", Napumsaka, &["vAriRe"]);
    assert_has_sup_5s("vAri", Napumsaka, &["vAriRaH"]);
    assert_has_sup_6d("vAri", Napumsaka, &["vAriRoH"]);
    assert_has_sup_6p("vAri", Napumsaka, &["vArIRAm"]);
    assert_has_sup_7s("vAri", Napumsaka, &["vArIRi"]);
    assert_has_sup_7d("vAri", Napumsaka, &["vArIRoH"]);
}
