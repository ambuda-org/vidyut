extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

#[test]
fn sutra_1_1() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::uR, &["kAru"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), Krt::uR, &["pAyu"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), Krt::uR, &["vAyu"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::uR, &["jAyu"]);
    assert_has_krdanta(&[], &d("qumi\\Y", Svadi), Krt::uR, &["mAyu"]);
    assert_has_krdanta(&[], &d("zvada~\\", Bhvadi), Krt::uR, &["svAdu"]);
    assert_has_krdanta(&[], &d("sA\\Da~", Svadi), Krt::uR, &["sADu"]);
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), Krt::uR, &["ASu"]);
}

#[test]
fn sutra_1_3() {
    assert_has_krdanta(&[], &d("dF", Kryadi), Krt::YuR, &["dAru"]);
    assert_has_krdanta(&[], &d("zaRa~", Bhvadi), Krt::YuR, &["sAnu"]);
    assert_has_krdanta(&[], &d("zaRu~^", Tanadi), Krt::YuR, &["sAnu"]);
    assert_has_krdanta(&[], &d("janI~\\", Divadi), Krt::YuR, &["jAnu"]);
    assert_has_krdanta(&[], &d("jana~", Juhotyadi), Krt::YuR, &["jAnu"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Krt::YuR, &["cAru"]);
    // TODO: ignoring Curadi gana tests as Nic pratyaya is getting added
    // and is considered as dhatu due to sanAdyantA dhAtavaH
    // assert_has_krdanta(&[], &d("cawa~", Curadi), Krt::YuR, &["cAwu"]);
    // assert_has_krdanta(&[], &d("raha~", Curadi), Krt::YuR, &["rAhu"]);
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
