extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn test_7_3_52() {
    let d = Dhatu::new;

    let pac = d("qupa\\ca~^z", Gana::Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Gana::Bhvadi), Krt::GaY, &["tyAga"]);
    assert_has_krdanta(
        &[],
        &d("ra\\nja~^", Gana::Bhvadi),
        Krt::GaY,
        &["rAga", "raNga"],
    );

    assert_has_krdanta(&[], &pac, Krt::Ryat, &["pAkya"]);
    assert_has_krdanta(&[], &d("va\\ca~", Gana::Bhvadi), Krt::Ryat, &["vAkya"]);
    assert_has_krdanta(&[], &d("ri\\ci~^r", Gana::Bhvadi), Krt::Ryat, &["rekya"]);
}
