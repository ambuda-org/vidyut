extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_2_4_36() {
    let ad = Dhatu::new("a\\da~", Gana::Adadi);
    assert_has_krdanta(&["pra"], &ad, Krt::ktvA, &["prajagDya"]);
    assert_has_krdanta(&["vi"], &ad, Krt::ktvA, &["vijagDya"]);
    assert_has_krdanta(&[], &ad, Krt::kta, &["jagDa", "jagdDa"]);
    assert_has_krdanta(&[], &ad, Krt::ktavatu, &["jagDavat", "jagdDavat"]);

    assert_has_lat_karmani(&[], &ad, &["adyate"]);
    assert_has_krdanta(&[], &ad, Krt::tavya, &["attavya"]);
}
