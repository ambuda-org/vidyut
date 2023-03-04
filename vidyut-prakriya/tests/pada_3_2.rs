extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

#[test]
fn sutra_3_2_102() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::kta, &["kfta"]);
    assert_has_krdanta(&[], &kf, Krt::ktavatu, &["kftavat"]);
    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_krdanta(&[], &bhuj, Krt::kta, &["Bukta"]);
    assert_has_krdanta(&[], &bhuj, Krt::ktavatu, &["Buktavat"]);
}

#[test]
fn sutra_3_2_103() {
    assert_has_krdanta(&[], &d("zu\\Y", Svadi), Krt::Nvanip, &["sutvan"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::Nvanip, &["yajvan"]);
}

#[test]
fn sutra_3_2_104() {
    let jf = d("jFz", Divadi);
    assert_has_krdanta(&[], &jf, Krt::atfn, &["jarat"]);
    assert_has_krdanta(&[], &jf, Krt::kta, &["jIrRa"]);
    assert_has_krdanta(&[], &jf, Krt::ktavatu, &["jIrRavat"]);
}

#[test]
fn sutra_3_2_123() {
    assert_has_lat_p(&[], &d("qupa\\ca~^z", Bhvadi), &["pacati"]);
    assert_has_lat_p(&[], &d("paWa~", Bhvadi), &["paWati"]);
}

#[test]
fn sutra_3_2_187() {
    let d = Dhatu::new;
    assert_has_krdanta(&[], &d("YimidA~\\", Bhvadi), Krt::kta, &["minna"]);
    assert_has_krdanta(&[], &d("YikzvidA~", Divadi), Krt::kta, &["kzviRRa"]);
    assert_has_krdanta(&[], &d("YiDfzA~", Svadi), Krt::kta, &["Dfzwa"]);
}
