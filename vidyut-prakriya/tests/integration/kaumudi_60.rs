extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2679() {
    let as_ = d("Asa~\\", Adadi);
    assert_has_ta(&[], &as_, Lat, &["Aste"]);

    let shi = d("SIN", Adadi);
    assert_has_ta(&[], &shi, Lat, &["Sete"]);

    // TODO: are these supposed to be karmani here?
    let bhu = &d("BU", Bhvadi);
    assert_has_ta_k(&[], bhu, Lit, &["baBUve"]);
    assert_has_ta_k(&["anu"], bhu, Lit, &["anubaBUve"]);
}

#[test]
fn sk_2683() {
    let vish = d("vi\\Sa~", Tudadi);
    assert_has_lat(&["ni"], &vish, &["niviSate"]);
}

#[test]
fn sk_2684() {
    let kri = d("qukrI\\Y", Kryadi);
    assert_has_lat(&["pari"], &kri, &["parikrIRIte"]);
    assert_has_lat(&["vi"], &kri, &["vikrIRIte"]);
    assert_has_lat(&["ava"], &kri, &["avakrIRIte"]);
}

#[test]
fn sk_2685() {
    let ji = d("ji\\", Bhvadi);
    assert_has_lat(&["vi"], &ji, &["vijayate"]);
    assert_has_lat(&["parA"], &ji, &["parAjayate"]);
}

#[test]
fn sk_2696_to_sk_2697() {
    let han = d("ha\\na~", Adadi);
    assert_has_ta(&["AN"], &han, Lun, &["Ahata", "AvaDizwa"]);
    assert_has_aataam(&["AN"], &han, Lun, &["AhasAtAm", "AvaDizAtAm"]);
    assert_has_jha(&["AN"], &han, Lun, &["Ahasata", "AvaDizata"]);
}

#[test]
fn sk_2699() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_ta(&["sam"], &gam, Lat, &["saNgacCate"]);
}

#[test]
fn sk_2703() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_ta(&["ni"], &hve, Lat, &["nihvayate"]);
}
