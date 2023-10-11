extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;

#[test]
fn sk_324() {
    assert_has_sup_1s("lih", Pum, &["liw"]);
    assert_has_sup_1d("lih", Pum, &["lihO"]);
    assert_has_sup_1p("lih", Pum, &["lihaH"]);
    assert_has_sup_2s("lih", Pum, &["liham"]);
    assert_has_sup_2d("lih", Pum, &["lihO"]);
    assert_has_sup_2p("lih", Pum, &["lihaH"]);
    assert_has_sup_3s("lih", Pum, &["lihA"]);
    assert_has_sup_3d("lih", Pum, &["liqByAm"]);
    assert_has_sup_7p("lih", Pum, &["liwtsu", "liwsu"]);
}

#[ignore]
#[test]
fn sk_325() {
    assert_has_sup_1s("gardaB", Pum, &["gardap"]);

    assert_has_sup_1s("duh", Pum, &["Duk"]);
    assert_has_sup_1d("duh", Pum, &["duhO"]);
    assert_has_sup_1p("duh", Pum, &["duhaH"]);
    assert_has_sup_7p("duh", Pum, &["Dukzu"]);
}

#[test]
fn skip_sk_328_to_sk_329() {}

#[test]
fn sk_342() {
    assert_has_sup_1s("kim", Pum, &["kaH"]);
    assert_has_sup_1d("kim", Pum, &["kO"]);
    assert_has_sup_1p("kim", Pum, &["ke"]);
    assert_has_sup_2s("kim", Pum, &["kam"]);
    assert_has_sup_2d("kim", Pum, &["kO"]);
    assert_has_sup_2p("kim", Pum, &["kAn"]);
}

#[test]
fn skip_sk_343() {}

#[test]
fn sk_344() {
    assert_has_sup_1s("idam", Pum, &["ayam"]);
}

#[test]
fn sk_345() {
    assert_has_sup_1d("idam", Pum, &["imO"]);
    assert_has_sup_1p("idam", Pum, &["ime"]);
}

#[test]
fn sk_346() {
    assert_has_sup_3s("idam", Pum, &["anena"]);
}

#[test]
fn skip_sk_347() {}

#[test]
fn sk_348() {
    assert_has_sup_3d("idam", Pum, &["AByAm"]);
}

#[test]
fn sk_349() {
    assert_has_sup_3p("idam", Pum, &["eBiH"]);
    assert_has_sup_4s("idam", Pum, &["asmE"]);
    assert_has_sup_4d("idam", Pum, &["AByAm"]);
    assert_has_sup_4p("idam", Pum, &["eByaH"]);
    assert_has_sup_5s("idam", Pum, &["asmAt"]);
    assert_has_sup_5d("idam", Pum, &["AByAm"]);
    assert_has_sup_5p("idam", Pum, &["eByaH"]);
    assert_has_sup_6s("idam", Pum, &["asya"]);
    assert_has_sup_6d("idam", Pum, &["anayoH"]);
    assert_has_sup_6p("idam", Pum, &["ezAm"]);
    assert_has_sup_7s("idam", Pum, &["asmin"]);
    assert_has_sup_7d("idam", Pum, &["anayoH"]);
    assert_has_sup_7p("idam", Pum, &["ezu"]);

    // TODO: ayaka
}

#[test]
fn sk_354() {
    assert_has_sup_1s("yajvan", Pum, &["yajvA"]);
    assert_has_sup_1d("yajvan", Pum, &["yajvAnO"]);
    assert_has_sup_1p("yajvan", Pum, &["yajvAnaH"]);
}

#[test]
fn sk_355() {
    assert_has_sup_2p("yajvan", Pum, &["yajvanaH"]);
    assert_has_sup_3s("yajvan", Pum, &["yajvanA"]);
    assert_has_sup_3d("yajvan", Pum, &["yajvaByAm"]);

    assert_has_sup_2p("brahman", Pum, &["brahmaRaH"]);
    assert_has_sup_3s("brahman", Pum, &["brahmaRA"]);
    assert_has_sup_3d("brahman", Pum, &["brahmaByAm"]);
}

#[test]
fn skip_sk_356() {}

#[test]
fn skip_sk_358() {}

#[test]
fn skip_sk_371() {}

#[test]
fn sk_390() {
    assert_has_sup_2s("yuzmad", Pum, &["tvAm"]);
    assert_has_sup_2s("asmad", Pum, &["mAm"]);
    assert_has_sup_2d("yuzmad", Pum, &["yuvAm"]);
    assert_has_sup_2d("asmad", Pum, &["AvAm"]);
}

#[test]
fn sk_391() {
    assert_has_sup_2p("yuzmad", Pum, &["yuzmAn"]);
    assert_has_sup_2p("asmad", Pum, &["asmAn"]);
}

#[test]
fn sk_392() {
    assert_has_sup_3s("yuzmad", Pum, &["tvayA"]);
    assert_has_sup_3s("asmad", Pum, &["mayA"]);
}

#[test]
fn sk_393() {
    assert_has_sup_3d("yuzmad", Pum, &["yuvAByAm"]);
    assert_has_sup_3d("asmad", Pum, &["AvAByAm"]);
    assert_has_sup_3p("yuzmad", Pum, &["yuzmABiH"]);
    assert_has_sup_3p("asmad", Pum, &["asmABiH"]);
}

#[ignore]
#[test]
fn sk_394() {
    assert_has_sup_4s("yuzmad", Pum, &["tuByam"]);
    assert_has_sup_4d("asmad", Pum, &["mahyam"]);
}

#[test]
fn sk_395() {
    assert_has_sup_4p("yuzmad", Pum, &["yuzmaByam"]);
    assert_has_sup_4p("asmad", Pum, &["asmaByam"]);
}

#[test]
fn sk_396() {
    assert_has_sup_5s("yuzmad", Pum, &["tvat"]);
    assert_has_sup_5s("asmad", Pum, &["mat"]);
    assert_has_sup_5d("yuzmad", Pum, &["yuvAByAm"]);
    assert_has_sup_5d("asmad", Pum, &["AvAByAm"]);
}

#[ignore]
#[test]
fn sk_397() {
    assert_has_sup_5p("yuzmad", Pum, &["yuzmat"]);
    assert_has_sup_5p("asmad", Pum, &["asmat"]);
}

#[test]
fn skip_sk_398() {}

#[test]
fn sk_399() {
    assert_has_sup_6s("yuzmad", Pum, &["tava"]);
    assert_has_sup_6s("asmad", Pum, &["mama"]);
    assert_has_sup_6d("yuzmad", Pum, &["yuvayoH"]);
    assert_has_sup_6d("asmad", Pum, &["AvayoH"]);
}

#[test]
fn skip_sk_401_to_sk_406() {}
