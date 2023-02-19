extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_6_1_91() {
    let r = Dhatu::new("f\\", Gana::Bhvadi);
    assert_has_lat(&["upa"], &r, &["upArCati", "upArcCati"]);
    assert_has_lat(&["pra"], &r, &["prArCati", "prArcCati"]);

    let rdh = Dhatu::new("fDu~", Gana::Svadi);
    assert_has_lat(&["upa"], &rdh, &["upArDnoti"]);

    // Exception -- KawvA is not an upasarga
    assert_has_lat(&["KawvA"], &r, &["KawvarCati", "KawvarcCati"]);
}

#[test]
fn sutra_6_1_136() {
    let kr = Dhatu::new("qukf\\Y", Gana::Tanadi);
    assert_has_lan_p(&["sam"], &kr, &["samaskarot", "samakarot"]);
    assert_has_lun_p(&["sam"], &kr, &["samaskArzIt", "samakArzIt"]);
    assert_has_lit_p(&["sam"], &kr, &["saYcaskAra", "saYcakAra"]);
    assert_has_lit_p(&["pari"], &kr, &["paricaskAra", "paricakAra"]);
}

#[test]
fn sutra_6_1_137() {
    let kr = Dhatu::new("qukf\\Y", Gana::Tanadi);

    assert_has_lut_p(&["sam"], &kr, &["saMskartA", "saNkartA"]);
    assert_has_krdanta(&["sam"], &kr, Krt::tumun, &["saMskartum", "saNkartum"]);
    assert_has_krdanta(&["sam"], &kr, Krt::tavya, &["saMskartavya", "saNkartavya"]);

    assert_has_lut_p(&["pari"], &kr, &["parizkartA", "parikartA"]);
    assert_has_krdanta(&["pari"], &kr, Krt::tumun, &["parizkartum", "parikartum"]);
    assert_has_krdanta(
        &["pari"],
        &kr,
        Krt::tavya,
        &["parizkartavya", "parikartavya"],
    );

    assert_has_lut_p(&["upa"], &kr, &["upaskartA", "upakartA"]);
    assert_has_krdanta(&["upa"], &kr, Krt::tumun, &["upaskartum", "upakartum"]);
    assert_has_krdanta(&["upa"], &kr, Krt::tavya, &["upaskartavya", "upakartavya"]);
}

#[test]
fn sutra_6_1_140() {
    let kr = Dhatu::new("kF", Gana::Tudadi);
    assert_has_lat(&["upa"], &kr, &["upaskirati", "upakirati"]);
}

#[ignore]
#[test]
fn sutra_6_1_141() {
    let kr = Dhatu::new("kF", Gana::Tudadi);
    assert_has_krdanta(&["prati"], &kr, Krt::kta, &["pratiskIrRa", "pratikIrRa"]);
}
