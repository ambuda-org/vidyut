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
fn sk_314() {
    assert_has_sup_1p("jYAna", Napumsaka, &["jYAnAni"]);
}

#[test]
fn skip_sk_315() {}

#[test]
fn sk_316() {
    assert_has_sup_1s("katara", Napumsaka, &["katarat"]);
    assert_has_sup_1d("katara", Napumsaka, &["katare"]);
    assert_has_sup_1p("katara", Napumsaka, &["katarARi"]);
    assert_has_sup_ss("katara", Napumsaka, &["katarat"]);

    assert_has_sup_1s("katama", Napumsaka, &["katamat"]);
    assert_has_sup_1s("anya", Napumsaka, &["anyat"]);
    assert_has_sup_1s("anyatara", Napumsaka, &["anyatarat"]);
    assert_has_sup_1s("itara", Napumsaka, &["itarat"]);

    assert_has_sup_1s("ekatara", Napumsaka, &["ekataram"]);

    // TODO: ajaras
}

#[test]
fn sk_318() {
    assert_has_sup_1s("SrIpA", Napumsaka, &["SrIpam"]);
    assert_has_sup_4s("SrIpA", Napumsaka, &["SrIpAya"]);
}

#[test]
fn sk_319() {
    assert_has_sup_1s("vAri", Napumsaka, &["vAri"]);
}

#[test]
fn sk_320() {
    assert_has_sup_1d("vAri", Napumsaka, &["vAriRI"]);
    assert_has_sup_1p("vAri", Napumsaka, &["vArIRi"]);
    // TODO: how to justify vAre?
    assert_has_sup_ss("vAri", Napumsaka, &["vAri"]);
    assert_has_sup_3s("vAri", Napumsaka, &["vAriRA"]);
    assert_has_sup_4s("vAri", Napumsaka, &["vAriRe"]);
    assert_has_sup_5s("vAri", Napumsaka, &["vAriRaH"]);
    assert_has_sup_6d("vAri", Napumsaka, &["vAriRoH"]);
    assert_has_sup_6p("vAri", Napumsaka, &["vArIRAm"]);
    assert_has_sup_7s("vAri", Napumsaka, &["vAriRi"]);
    assert_has_sup_7d("vAri", Napumsaka, &["vAriRoH"]);
}

#[test]
fn sk_322() {
    assert_has_sup_3s("daDi", Napumsaka, &["daDnA"]);
}

#[test]
fn sk_323() {
    assert_has_sup_1s("pradyo", Napumsaka, &["pradyu"]);
    assert_has_sup_1d("pradyo", Napumsaka, &["pradyunI"]);
    assert_has_sup_1p("pradyo", Napumsaka, &["pradyUni"]);
    assert_has_sup_3s("pradyo", Napumsaka, &["pradyunA"]);

    let prari = create_avyaya_tatpurusha("prarE", "pra", "rE");
    assert_has_sup_1s(&prari, Napumsaka, &["prari"]);
    assert_has_sup_1d(&prari, Napumsaka, &["prariRI"]);
    assert_has_sup_1p(&prari, Napumsaka, &["prarIRi"]);
    assert_has_sup_3s(&prari, Napumsaka, &["prariRA"]);
    assert_has_sup_3d(&prari, Napumsaka, &["prarAByAm"]);
    assert_has_sup_3p(&prari, Napumsaka, &["prarABiH"]);
    assert_has_sup_6p(&prari, Napumsaka, &["prarIRAm"]);

    assert_has_sup_1s("sunu", Napumsaka, &["sunu"]);
    assert_has_sup_1d("sunu", Napumsaka, &["sununI"]);
    assert_has_sup_1p("sunu", Napumsaka, &["sunUni"]);
    assert_has_sup_3s("sunu", Napumsaka, &["sununA"]);
    assert_has_sup_4s("sunu", Napumsaka, &["sunune"]);
}
