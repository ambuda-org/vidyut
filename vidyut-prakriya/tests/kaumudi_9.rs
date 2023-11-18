extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;

#[test]
fn sk_287() {
    let rama = create_stryanta("ramA", "rama");
    assert_has_sup_1d(&rama, Stri, &["rame"]);
    assert_has_sup_1p(&rama, Stri, &["ramAH"]);
}

#[test]
fn sk_288() {
    let rama = create_stryanta("ramA", "rama");
    assert_has_sup_ss(&rama, Stri, &["rame"]);
    assert_has_sup_sd(&rama, Stri, &["rame"]);
    assert_has_sup_sp(&rama, Stri, &["ramAH"]);
    assert_has_sup_2s(&rama, Stri, &["ramAm"]);
    assert_has_sup_2d(&rama, Stri, &["rame"]);
    assert_has_sup_2p(&rama, Stri, &["ramAH"]);
}

#[test]
fn sk_289() {
    let rama = create_stryanta("ramA", "rama");
    assert_has_sup_3s(&rama, Stri, &["ramayA"]);
    assert_has_sup_3d(&rama, Stri, &["ramAByAm"]);
    assert_has_sup_3p(&rama, Stri, &["ramABiH"]);
}

#[test]
fn sk_290() {
    let rama = create_stryanta("ramA", "rama");
    assert_has_sup_4s(&rama, Stri, &["ramAyE"]);
    assert_has_sup_6s(&rama, Stri, &["ramAyAH"]);
    assert_has_sup_6d(&rama, Stri, &["ramayoH"]);
    assert_has_sup_6p(&rama, Stri, &["ramARAm"]);
    assert_has_sup_7s(&rama, Stri, &["ramAyAm"]);
    assert_has_sup_7d(&rama, Stri, &["ramayoH"]);
    assert_has_sup_7p(&rama, Stri, &["ramAsu"]);
}

#[test]
fn sk_291() {
    assert_has_sup_4s("sarva", Stri, &["sarvasyE"]);
    assert_has_sup_5s("sarva", Stri, &["sarvasyAH"]);
    assert_has_sup_7s("sarva", Stri, &["sarvasyAm"]);
    assert_has_sup_7d("sarva", Stri, &["sarvayoH"]);
    assert_has_sup_7p("sarva", Stri, &["sarvAsu"]);
}

#[test]
fn sk_296() {
    assert_has_sup_4s("mati", Stri, &["matyE", "mataye"]);
    assert_has_sup_5s("mati", Stri, &["matyAH", "mateH"]);
    assert_has_sup_6s("mati", Stri, &["matyAH", "mateH"]);
}

#[test]
fn skip_sk_298() {}

#[test]
fn sk_299() {
    assert_has_sup_1p("tri", Stri, &["tisraH"]);
    assert_has_sup_2p("tri", Stri, &["tisraH"]);
}

#[ignore]
#[test]
fn sk_300() {
    assert_has_sup_6p("tri", Stri, &["tisfRAm"]);
    assert_has_sup_7p("tri", Stri, &["tisfzu"]);
    // TODO: others
}

#[test]
fn sk_301() {
    assert_has_sup_1d("strI", Stri, &["striyO"]);
    assert_has_sup_1p("strI", Stri, &["striyaH"]);
}

#[test]
fn sk_303() {
    let shri = create_krdanta("SrI", &[], &d("SriY", Bhvadi), Krt::kvip);
    assert_has_sup_ss(&shri, Stri, &["SrIH"]);
    assert_has_sup_4s(&shri, Stri, &["Sriye", "SriyE"]);
    assert_has_sup_5s(&shri, Stri, &["SriyAH", "SriyaH"]);
}

#[test]
fn skip_sk_305() {}

#[ignore]
#[test]
fn sk_306() {
    assert_has_sup_1s("krozwu", Stri, &["krozwrI"]);
    assert_has_sup_1d("krozwu", Stri, &["krozwryO"]);
    assert_has_sup_1p("krozwu", Stri, &["krozwryaH"]);

    assert_has_sup_1s("vaDU", Stri, &["vaDUH"]);
    assert_has_sup_1s("BU", Stri, &["BUH"]);
    assert_has_sup_ss("suBrU", Stri, &["suBrUH"]);
}
