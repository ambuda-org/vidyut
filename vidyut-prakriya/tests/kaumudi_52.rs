extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2563() {
    let cur = d("cura~", Curadi);
    assert_has_tip(&[], &cur, Lat, &["corayati"]);
}

#[ignore]
#[test]
fn sk_2564() {
    let cur = d("cura~", Curadi);
    assert_has_ta(&[], &cur, Lat, &["corayate"]);
    assert_has_tip(
        &[],
        &cur,
        Lat,
        &["corayAYcakAra", "corayAmbaBUva", "corayAmAsa"],
    );
    assert_has_tip(&[], &cur, Lut, &["corayitA"]);
    assert_has_tip(&[], &cur, AshirLin, &["coryAt"]);
    assert_has_ta(&[], &cur, AshirLin, &["corayizIzwa"]);
    assert_has_tip(&[], &cur, Lun, &["acUcurat"]);
    assert_has_ta(&[], &cur, Lun, &["acUcurata"]);

    let cint = d("citi~", Curadi);
    assert_has_tip(&[], &cint, Lat, &["cintayati"]);
    assert_has_tip(&[], &cint, Lun, &["acicintat"]);
    assert_has_tip(&[], &cint, AshirLin, &["cintyAt"]);
    assert_has_ta_k(&[], &cint, Lat, &["cintyate"]);

    // TODO: cintati, etc.
}

#[ignore]
#[test]
fn sk_2565() {
    let pid = d("pIqa~", Curadi);
    assert_has_tip(&[], &pid, Lun, &["apipIqat", "apIpiqat"]);

    let badh = d("baDa~", Curadi);
    assert_has_tip(&[], &badh, Lat, &["bADayati"]);

    let pr = d("pF", Curadi);
    assert_has_tip(&[], &pr, Lat, &["pArayati"]);
    // TODO: parati?
}

#[test]
fn sk_2566() {
    let prath = d("praTa~", Curadi);
    assert_has_tip(&[], &prath, Lun, &["apapraTat"]);

    let prth = d("pfTa~", Curadi);
    assert_has_tip(&[], &prth, Lat, &["parTayati"]);
}

#[test]
fn skip_sk_2569() {}

#[test]
fn sk_2574() {
    // TODO: add more

    // ac-Adi dhatus with can-pratyaya and ak-lopa
    // TODO: why does the SK have AndaDat and not AndiDat?
    assert_has_tip(&[], &d("anDa", Curadi), Lun, &["AndaDat"]);
    assert_has_tip(&[], &d("anka", Curadi), Lun, &["AYcakat"]);
    assert_has_tip(&[], &d("anga", Curadi), Lun, &["AYjagat"]);
    assert_has_tip(&[], &d("vyaya", Curadi), Lun, &["avavyayat"]);
}
