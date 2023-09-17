extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

#[test]
fn sutra_4_4_20() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::ktri, &["paktrima"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), Krt::ktri, &["uptrima"]);
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::ktri, &["kftrima"]);
}

#[test]
fn sutra_4_4_52() {
    assert_has_taddhitanta(&prati("lavaRa"), T::WaY, &["lAvaRika"]);
}

#[test]
fn sutra_4_4_53() {
    // TODO: strI-linga
    assert_has_taddhitanta(&prati("kiSara"), T::zWan, &["kiSarika"]);
    assert_has_taddhitanta(&prati("narada"), T::zWan, &["naradika"]);
}
