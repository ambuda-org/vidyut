use test_utils::{
    assert_has_sup_2d, assert_has_sup_2p, assert_has_sup_2s, assert_has_sup_4d, assert_has_sup_4p,
    assert_has_sup_4s, assert_has_sup_6d, assert_has_sup_6p, assert_has_sup_6s,
};
use vidyut_prakriya::args::Linga::Pum;

// Enclitic variants for asmad/yuzmad
#[test]
fn sutra_8_2_20() {
    assert_has_sup_2d("asmad", Pum, &["AvAm", "nO"]);
    assert_has_sup_2d("yuzmad", Pum, &["yuvAm", "vAm"]);
    assert_has_sup_4d("asmad", Pum, &["AvAByAm", "nO"]);
    assert_has_sup_4d("yuzmad", Pum, &["yuvAByAm", "vAm"]);
    assert_has_sup_6d("asmad", Pum, &["AvayoH", "nO"]);
    assert_has_sup_6d("yuzmad", Pum, &["yuvayoH", "vAm"]);
}
#[test]
fn sutra_8_2_21() {
    assert_has_sup_2p("asmad", Pum, &["asmAn", "naH"]);
    assert_has_sup_2p("yuzmad", Pum, &["yuzmAn", "vaH"]);
    assert_has_sup_4p("asmad", Pum, &["asmaByam", "naH"]);
    assert_has_sup_4p("yuzmad", Pum, &["yuzmaByam", "vaH"]);
    assert_has_sup_6p("asmad", Pum, &["asmAkam", "naH"]);
    assert_has_sup_6p("yuzmad", Pum, &["yuzmAkam", "vaH"]);
}

#[test]
fn sutra_8_2_22() {
    assert_has_sup_4s("asmad", Pum, &["mahyam", "me"]);
    assert_has_sup_4s("yuzmad", Pum, &["tuByam", "te"]);
    assert_has_sup_6s("asmad", Pum, &["mama", "me"]);
    assert_has_sup_6s("yuzmad", Pum, &["tava", "te"]);
}

#[test]
fn sutra_8_2_23() {
    assert_has_sup_2s("asmad", Pum, &["mAm", "mA"]);
    assert_has_sup_2s("yuzmad", Pum, &["tvAm", "tvA"]);
}
