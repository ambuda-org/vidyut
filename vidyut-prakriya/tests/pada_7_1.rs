extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_7_1_1() {
    let nand = Dhatu::new("wunadi~", Gana::Bhvadi);
    assert_has_krdanta(&[], &nand, Krt::lyu, &["nandana"]);

    let ram = Dhatu::new("ra\\ma~\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &ram, Krt::lyu, &["ramaRa"]);

    let kf = Dhatu::new("qukf\\Y", Gana::Tanadi);
    assert_has_krdanta(&[], &kf, Krt::Rvul, &["kAraka"]);

    let hf = Dhatu::new("hf\\Y", Gana::Bhvadi);
    assert_has_krdanta(&[], &hf, Krt::Rvul, &["hAraka"]);
}
