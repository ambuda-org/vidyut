extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn assert_has_lun_3d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(
        prefixes,
        dhatu,
        Lakara::Lun,
        Purusha::Prathama,
        Vacana::Dvi,
        expected,
    );
}
fn assert_has_lun_2d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(
        prefixes,
        dhatu,
        Lakara::Lun,
        Purusha::Madhyama,
        Vacana::Dvi,
        expected,
    );
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

    assert_has_lun_3d(&["ud"], &vah, &["udavoQAm"]);
    assert_has_lun_2d(&["ud"], &vah, &["udavoQam"]);
}
