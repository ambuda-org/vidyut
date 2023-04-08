extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

#[ignore]
#[test]
fn sutra_1_5() {
    // TODO: not sure where to apply r --> l
    assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::YuR, &["tAlu"]);
}

#[test]
fn sutra_1_77() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::katu, &["kratu"]);
}

#[test]
fn sutra_4_54() {
    assert_has_krdanta(&[], &d("jF", Kryadi), Krt::kvinUnadi, &["jIrvi"]);
    assert_has_krdanta(&[], &d("SFY", Kryadi), Krt::kvinUnadi, &["SIrvi"]);
    assert_has_krdanta(&[], &d("stFY", Kryadi), Krt::kvinUnadi, &["stIrvi"]);
    assert_has_krdanta(&[], &d("jAgf", Adadi), Krt::kvinUnadi, &["jAgfvi"]);
}
