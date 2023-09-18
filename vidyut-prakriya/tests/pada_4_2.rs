extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;

#[ignore]
#[test]
fn sutra_4_2_114() {
    assert_has_taddhitanta(&prati("gArgya"), T::Ca, &["gArgIya"]);
    assert_has_taddhitanta(&prati("vAtsa"), T::Ca, &["vAtsIya"]);
    assert_has_taddhitanta(&prati("SAla"), T::Ca, &["SAlIya"]);
    assert_has_taddhitanta(&prati("mAla"), T::Ca, &["mAlIya"]);
}

#[ignore]
#[test]
fn sutra_4_2_115() {
    assert_has_taddhitanta(&prati("Bavat"), T::Wak, &["BAvatka"]);
    assert_has_taddhitanta(&prati("Bavat"), T::Cas, &["BavatIya"]);
    assert_has_taddhitanta(&prati("Bavat"), T::Ca, &[]);
}
