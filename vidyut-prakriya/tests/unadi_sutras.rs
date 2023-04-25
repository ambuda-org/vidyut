// Test cases from the Siddhantakaumudi commentary on the Unadipatha.

extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic])
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

#[ignore]
#[test]
fn sutra_1_5() {
    // TODO: not sure where to apply r --> l
    assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::YuR, &["tAlu"]);
}

#[test]
fn sutra_1_69() {
    assert_has_krdanta(&[], &d("zi\\Y", Svadi), Krt::tun, &["setu"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), Krt::tun, &["tantu"]);
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), Krt::tun, &["gantu"]);
    assert_has_krdanta(&[], &d("masI~", Divadi), Krt::tun, &["mastu"]);
    assert_has_krdanta(&[], &d("zaca~\\", Bhvadi), Krt::tun, &["saktu"]);
    // TODO: return to this later
    // assert_has_krdanta(&[], &d("ava~", Bhvadi), Krt::tun, &["otu"]);
    assert_has_krdanta(&[], &d("quDA\\Y", Juhotyadi), Krt::tun, &["DAtu"]);
    assert_has_krdanta(&[], &d("kru\\Sa~", Bhvadi), Krt::tun, &["krozwu"]);
}

#[test]
fn sutra_1_77() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::katu, &["kratu"]);
}

#[test]
fn sutra_2_2() {
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), Krt::kTan, &["haTa"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), Krt::kTan, &["kuzWa"]);
    assert_has_krdanta(&[], &d("RI\\Y", Tanadi), Krt::kTan, &["nITa"]);
    assert_has_krdanta(&[], &d("kASf~", Bhvadi), Krt::kTan, &["kAzWa"]);
}

#[test]
fn sutra_3_43() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), Krt::kan, &["eka"]);
    assert_has_krdanta(&[], &d("YiBI\\", Juhotyadi), Krt::kan, &["Beka"]);
    assert_has_krdanta(&[], &d("kE\\", Tanadi), Krt::kan, &["kAka"]);
    assert_has_krdanta(&[], &d("pA\\", Adadi), Krt::kan, &["pAka"]);
    assert_has_krdanta(&[], &d("Sala~", Bhvadi), Krt::kan, &["Salka"]);
    assert_has_krdanta(&[], &d("ata~", Bhvadi), Krt::kan, &["atka"]);
    assert_has_krdanta(&[], &d("marca~", Bhvadi), Krt::kan, &["marka", "markka"]);
}

#[test]
fn sutra_3_62() {
    // TODO: why not varza and tarza?
    // assert_has_krdanta(&[], &d("vF", Kryadi), Krt::sa, &["varsa"]);
    // assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::sa, &["tarsa"]);
    assert_has_krdanta(&[], &d("vada~", Bhvadi), Krt::sa, &["vatsa"]);
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), Krt::sa, &["haMsa"]);
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), Krt::sa, &["kaMsa"]);
    assert_has_krdanta(&[], &d("kaza~", Bhvadi), Krt::sa, &["kakza"]);
}

#[test]
fn sutra_3_70() {
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), Krt::sara, &["akzara"]);
}

#[test]
fn sutra_3_86() {
    assert_has_krdanta(&[], &d("hase~", Bhvadi), Krt::tan, &["hasta"]);
    assert_has_krdanta(&[], &d("mf\\N", Tanadi), Krt::tan, &["marta"]);
    assert_has_krdanta(&[], &d("gF", Tudadi), Krt::tan, &["garta"]);
    assert_has_krdanta(&[], &d("i\\R", Adadi), Krt::tan, &["eta"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), Krt::tan, &["vAta"]);
    assert_has_krdanta(&[], &d("ama~", Bhvadi), Krt::tan, &["anta"]);
    assert_has_krdanta(&[], &d("damu~", Divadi), Krt::tan, &["danta"]);
    assert_has_krdanta(&[], &d("lUY", Kryadi), Krt::tan, &["lota"]);
    assert_has_krdanta(&[], &d("pUY", Kryadi), Krt::tan, &["pota"]);
    // TODO: enable this after fixing cchvoh
    // assert_has_krdanta(&[], &d("DurvI~", Bhvadi), Krt::tan, &["DUrta"]);
}

#[test]
fn sutra_3_155() {
    assert_has_krdanta(&[], &d("pluza~", Kryadi), Krt::ksi, &["plukzi"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), Krt::ksi, &["kukzi"]);
    assert_has_krdanta(&[], &d("Su\\za~", Divadi), Krt::ksi, &["Sukzi"]);
}

#[test]
fn sutra_3_156() {
    assert_has_krdanta(&[], &d("aSU~", Svadi), Krt::ksi, &["akzi"]);
}

#[test]
fn sutra_3_157() {
    assert_has_krdanta(&[], &d("izu~", Tudadi), Krt::ksu, &["ikzu"]);
}

#[test]
fn sutra_4_2() {
    assert_has_krdanta(&[], &d("f\\", Juhotyadi), Krt::katnic, &["ratni"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), Krt::yatuc, &["tanyatu"]);
    assert_has_krdanta(&[], &d("anjU~", Rudhadi), Krt::alic, &["aYjali"]);
    assert_has_krdanta(&[], &d("vana~", Bhvadi), Krt::izWuc, &["vanizWu"]);
    assert_has_krdanta(&[], &d("anjU~", Rudhadi), Krt::izWac, &["aYjizWa"]);
    assert_has_krdanta(&[], &nic(&d("f\\", Juhotyadi)), Krt::isan, &["arpisa"]);
    // TODO: why is this aniw?
    // assert_has_krdanta(&[], &d("madI~", Divadi), Krt::syan, &["matsya"]);
    assert_has_krdanta(&[], &d("ata~", Bhvadi), Krt::iTin, &["atiTi"]);
    assert_has_krdanta(&[], &d("anga", Curadi), Krt::uli, &["aNguli"]);
    assert_has_krdanta(&[], &d("ku\\", Adadi), Krt::asa, &["kavasa"]);
    assert_has_krdanta(&[], &d("yu", Adadi), Krt::Asa, &["yavAsa"]);
    assert_has_krdanta(&[], &d("kfSa~", Divadi), Krt::Anuk, &["kfSAnu"]);
}

#[test]
fn sutra_4_54() {
    assert_has_krdanta(&[], &d("jF", Kryadi), Krt::kvinUnadi, &["jIrvi"]);
    assert_has_krdanta(&[], &d("SFY", Kryadi), Krt::kvinUnadi, &["SIrvi"]);
    assert_has_krdanta(&[], &d("stFY", Kryadi), Krt::kvinUnadi, &["stIrvi"]);
    assert_has_krdanta(&[], &d("jAgf", Adadi), Krt::kvinUnadi, &["jAgfvi"]);
}

#[test]
fn sutra_4_158() {
    assert_has_krdanta(&[], &d("vasa~\\", Adadi), Krt::zwran, &["vastra"]);
    assert_has_krdanta(&[], &d("asu~", Divadi), Krt::zwran, &["astra"]);
    assert_has_krdanta(&[], &d("Sasu~", Bhvadi), Krt::zwran, &["Sastra"]);
}
