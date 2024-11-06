//! Bug fixes that aren't captured in other tests.
//!
//! The tests here are responses to bug fixes that users have filed. Each should show that the bug
//! has been fixed and help ensure that the bug does not reappear.
extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;

#[test]
fn ambibat() {
    assert_has_tip(&[], &nic(&d("abi~", Bhvadi)), Lun, &["Ambibat"]);
}

#[test]
fn aucicchat() {
    assert_has_tip(&[], &nic(&d("uCI~", Tudadi)), Lun, &["OcicCat"]);
}

#[test]
fn urjijiayizati() {
    // Given as Urj in sk.
    assert_has_tip(&[], &san(&d("Urja~", Curadi)), Lat, &["Urjijayizati"]);
}

#[test]
fn titikzizati() {
    assert_has_ta(&[], &san(&d("tija~\\", Bhvadi)), Lat, &["titikzizate"]);
}

#[test]
fn praipsat() {
    // Not a real regression, just want to make sure this works consistently.
    let aap = d("A\\px~", Svadi);
    assert_has_tip(&["pra"], &san(&aap), Lan, &["prEpsat"]);
}

#[test]
fn irshy_san_lan() {
    assert_has_tip(
        &[],
        &san(&d("Irzya~", Bhvadi)),
        Lan,
        &["Erzyiyizat", "Erzyizizat"],
    );
}

// Fixes https://github.com/ambuda-org/vidyut/issues/120
#[test]
fn ayiyat() {
    assert_has_tip(&[], &nic(&d("I\\N", Divadi)), Lun, &["Ayiyat"]);
}

// Fixes https://github.com/ambuda-org/vidyut/issues/121
#[test]
fn adhyajigapat_adhyapipat() {
    assert_has_tip(
        &["aDi"],
        &nic(&d("i\\N", Adadi)),
        Lun,
        &["aDyajIgapat", "aDyApipat"],
    );
}

// Verifies that Sap-pratyaya is added when Satf follows.
#[test]
fn shap_shatr() {
    let bhavat = krdanta(&[], &d("BU", Bhvadi), BaseKrt::Satf);
    assert_has_sup_1s(&bhavat, Pum, &["Bavan"]);
    assert_has_sup_1d(&bhavat, Pum, &["BavantO"]);
    assert_has_sup_1p(&bhavat, Pum, &["BavantaH"]);
}
