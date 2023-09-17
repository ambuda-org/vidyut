extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

#[test]
fn sutra_6_3_43() {
    assert_has_taddhitanta(&prati("brAhmaRI"), T::tarap, &["brAhmaRitara"]);
    assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRitama"]);
    assert_has_taddhitanta(&prati("brAhmaRI"), T::rUpap, &["brAhmaRirUpa"]);
    assert_has_taddhitanta(&prati("brAhmaRI"), T::kalpap, &["brAhmaRikalpa"]);

    // TODO: others
    // assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRibruva"]);
    // assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRigotra"]);
    // assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRimata"]);
    // assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRihata"]);
}

#[test]
fn sutra_6_3_111() {
    assert_has_krdanta(&[], &d("li\\ha~^", Adadi), Krt::kta, &["lIQa"]);
    assert_has_krdanta(&[], &d("mi\\ha~", Bhvadi), Krt::kta, &["mIQa"]);
    assert_has_krdanta(&["upa"], &d("guhU~^", Bhvadi), Krt::kta, &["upagUQa"]);
    assert_has_krdanta(&[], &d("mu\\ha~", Divadi), Krt::kta, &["mUQa", "mugDa"]);
    // TODO: ra
}

#[test]
fn sutra_6_3_112() {
    let sah = d("zaha~\\", Bhvadi);
    assert_has_krdanta(&[], &sah, Krt::tfc, &["soQf", "sahitf"]);
    assert_has_krdanta(&[], &sah, Krt::tumun, &["soQum", "sahitum"]);
    assert_has_krdanta(&[], &sah, Krt::tavya, &["soQavya", "sahitavya"]);

    let vah = d("va\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::tfc, &["voQf"]);
    assert_has_krdanta(&[], &vah, Krt::tumun, &["voQum"]);
    assert_has_krdanta(&[], &vah, Krt::tavya, &["voQavya"]);

    assert_has_krdanta(&[], &vah, Krt::kta, &["UQa"]);
    assert_has_krdanta(&[], &vah, Krt::ktavatu, &["UQavat"]);

    assert_has_tas(&["ud"], &vah, Lun, &["udavoQAm"]);
    assert_has_thas(&["ud"], &vah, Lun, &["udavoQam"]);
}
