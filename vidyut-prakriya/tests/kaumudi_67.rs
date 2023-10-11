// Test cases from the Siddhantakaumudi commentary on the Unadipatha.

extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

#[test]
fn unadi_1_1() {
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
fn unadi_1_5() {
    // TODO: not sure where to apply r --> l
    assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::YuR, &["tAlu"]);
}

#[test]
fn unadi_1_45() {
    assert_has_krdanta(&[], &d("ava~", Bhvadi), Krt::wizac, &["aviza"]);
    // TODO: right mah?
    assert_has_krdanta(&[], &d("maha~", Bhvadi), Krt::wizac, &["mahiza"]);
}

#[test]
fn unadi_1_69() {
    use Krt::tun;
    assert_has_krdanta(&[], &d("zi\\Y", Svadi), tun, &["setu"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), tun, &["tantu"]);
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), tun, &["gantu"]);
    assert_has_krdanta(&[], &d("masI~", Divadi), tun, &["mastu"]);
    assert_has_krdanta(&[], &d("zaca~\\", Bhvadi), tun, &["saktu"]);
    // TODO: return to this later
    // assert_has_krdanta(&[], &d("ava~", Bhvadi), Krt::tun, &["otu"]);
    assert_has_krdanta(&[], &d("quDA\\Y", Juhotyadi), tun, &["DAtu"]);
    assert_has_krdanta(&[], &d("kru\\Sa~", Bhvadi), tun, &["krozwu"]);
}

#[test]
fn unadi_1_77() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::katu, &["kratu"]);
}

#[test]
fn unadi_1_144() {
    use Krt::manin;
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), manin, &["karman"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), manin, &["carman"]);
    assert_has_krdanta(&[], &d("Basa~", Juhotyadi), manin, &["Basman"]);
    assert_has_krdanta(&[], &d("janI~\\", Divadi), manin, &["janman"]);
    assert_has_krdanta(&[], &d("SF", Kryadi), manin, &["Sarman"]);
    assert_has_krdanta(&[], &d("Cada~", Curadi), manin, &["Cadman"]);
    // TODO: sutrAman
}

#[test]
fn unadi_2_2() {
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), Krt::kTan, &["haTa"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), Krt::kTan, &["kuzWa"]);
    assert_has_krdanta(&[], &d("RI\\Y", Tanadi), Krt::kTan, &["nITa"]);
    assert_has_krdanta(&[], &d("kASf~", Bhvadi), Krt::kTan, &["kAzWa"]);
}

#[test]
fn unadi_2_108() {
    assert_has_krdanta(&[], &d("arca~", Bhvadi), Krt::isi, &["arcis"]);
    assert_has_krdanta(&[], &d("I~Suci~^r", Divadi), Krt::isi, &["Socis"]);
    assert_has_krdanta(&[], &d("hu\\", Juhotyadi), Krt::isi, &["havis"]);
    assert_has_krdanta(&[], &d("sf\\px~", Bhvadi), Krt::isi, &["sarpis"]);
    assert_has_krdanta(&[], &d("Cada~", Curadi), Krt::isi, &["Cadis"]);
    assert_has_krdanta(&[], &d("Carda~", Curadi), Krt::isi, &["Cardis"]);
    // TODO: id-anta?
}

#[ignore]
#[test]
fn unadi_3_29() {
    assert_has_krdanta(&[], &d("stana", Curadi), Krt::itnuc, &["stanayitnu"]);
    assert_has_krdanta(&[], &nic(&d("hfza~", Divadi)), Krt::itnuc, &["harzayitnu"]);
    // TODO: popayitnu?
    assert_has_krdanta(&[], &d("gada", Curadi), Krt::itnuc, &["gadayitnu"]);
    assert_has_krdanta(&[], &d("mada~", Curadi), Krt::itnuc, &["madayitnu"]);
}

#[test]
fn unadi_3_43() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), Krt::kan, &["eka"]);
    assert_has_krdanta(&[], &d("YiBI\\", Juhotyadi), Krt::kan, &["Beka"]);
    assert_has_krdanta(&[], &d("kE\\", Tanadi), Krt::kan, &["kAka"]);
    assert_has_krdanta(&[], &d("pA\\", Adadi), Krt::kan, &["pAka"]);
    assert_has_krdanta(&[], &d("Sala~", Bhvadi), Krt::kan, &["Salka"]);
    assert_has_krdanta(&[], &d("ata~", Bhvadi), Krt::kan, &["atka"]);
    assert_has_krdanta(&[], &d("marca~", Bhvadi), Krt::kan, &["marka", "markka"]);
}

#[test]
fn unadi_3_62() {
    // TODO: why not varza and tarza?
    // assert_has_krdanta(&[], &d("vF", Kryadi), Krt::sa, &["varsa"]);
    // assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::sa, &["tarsa"]);
    assert_has_krdanta(&[], &d("vada~", Bhvadi), Krt::sa, &["vatsa"]);
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), Krt::sa, &["haMsa"]);
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), Krt::sa, &["kaMsa"]);
    assert_has_krdanta(&[], &d("kaza~", Bhvadi), Krt::sa, &["kakza"]);
}

#[test]
fn unadi_3_70() {
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), Krt::sara, &["akzara"]);
}

#[test]
fn unadi_3_86() {
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
fn unadi_3_96() {
    assert_has_krdanta(&[], &d("Sru\\", Bhvadi), Krt::Ayya, &["SravAyya"]);
    assert_has_krdanta(&[], &d("dakza~\\", Bhvadi), Krt::Ayya, &["dakzAyya"]);
    assert_has_krdanta(&[], &d("spfha", Curadi), Krt::Ayya, &["spfhayAyya"]);
    assert_has_krdanta(&[], &d("gfha", Curadi), Krt::Ayya, &["gfhayAyya"]);
}

#[test]
fn unadi_3_126() {
    assert_has_krdanta(&[], &d("jF", Kryadi), Krt::Jac, &["jaranta"]);
    assert_has_krdanta(&[], &d("vi\\Sa~", Tudadi), Krt::Jac, &["veSanta"]);
}

#[test]
fn unadi_3_127() {
    assert_has_krdanta(&[], &d("ru\\ha~", Bhvadi), Krt::Jac, &["rohanta"]);
    assert_has_krdanta(&[], &d("wunadi~", Bhvadi), Krt::Jac, &["nandanta"]);
    assert_has_krdanta(&[], &d("jIva~", Tudadi), Krt::Jac, &["jIvanta"]);
    // TODO: zit
}

#[test]
fn unadi_3_128() {
    assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::Jac, &["taranta"]);
    assert_has_krdanta(&[], &d("BU", Bhvadi), Krt::Jac, &["Bavanta"]);
    assert_has_krdanta(&[], &d("va\\ha~^", Bhvadi), Krt::Jac, &["vahanta"]);
    assert_has_krdanta(&[], &d("va\\sa~", Bhvadi), Krt::Jac, &["vasanta"]);
    assert_has_krdanta(&[], &d("BAsf~\\", Bhvadi), Krt::Jac, &["BAsanta"]);
    assert_has_krdanta(&[], &d("sA\\Da~", Svadi), Krt::Jac, &["sADanta"]);
    assert_has_krdanta(&[], &nic(&d("gaqi~", Bhvadi)), Krt::Jac, &["gaRqayanta"]);
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Tudadi)), Krt::Jac, &["maRqayanta"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::Jac, &["jayanta"]);
    // nandayanta?
}

#[test]
fn unadi_3_155() {
    assert_has_krdanta(&[], &d("pluza~", Kryadi), Krt::ksi, &["plukzi"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), Krt::ksi, &["kukzi"]);
    assert_has_krdanta(&[], &d("Su\\za~", Divadi), Krt::ksi, &["Sukzi"]);
}

#[test]
fn unadi_3_156() {
    assert_has_krdanta(&[], &d("aSU~", Svadi), Krt::ksi, &["akzi"]);
}

#[test]
fn unadi_3_157() {
    assert_has_krdanta(&[], &d("izu~", Tudadi), Krt::ksu, &["ikzu"]);
}

#[test]
fn unadi_4_2() {
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
fn unadi_4_6() {
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), Krt::ini, &["gamin"]);
}

#[test]
fn unadi_4_7() {
    assert_has_krdanta(&["AN"], &d("ga\\mx~", Bhvadi), Krt::ini, &["AgAmin"]);
}

#[test]
fn unadi_4_8() {
    assert_has_krdanta(&[], &d("BU", Bhvadi), Krt::ini, &["BAvin"]);
}

#[test]
fn unadi_4_9() {
    assert_has_krdanta(&["pra"], &d("zWA\\", Bhvadi), Krt::ini, &["prasTAyin"]);
}

#[ignore]
#[test]
fn unadi_4_10() {
    assert_has_krdanta(&["parama"], &d("zWA\\", Bhvadi), Krt::ini, &["paramezWin"]);
}

#[ignore]
#[test]
fn unadi_4_11() {
    assert_has_krdanta(&[], &d("maTi~", Bhvadi), Krt::ini, &["maTin"]);
}

#[test]
fn unadi_4_12() {
    assert_has_krdanta(&[], &d("patx~", Bhvadi), Krt::ini, &["paTin"]);
}

#[test]
fn unadi_4_54() {
    assert_has_krdanta(&[], &d("jF", Kryadi), Krt::kvinUnadi, &["jIrvi"]);
    assert_has_krdanta(&[], &d("SFY", Kryadi), Krt::kvinUnadi, &["SIrvi"]);
    assert_has_krdanta(&[], &d("stFY", Kryadi), Krt::kvinUnadi, &["stIrvi"]);
    assert_has_krdanta(&[], &d("jAgf", Adadi), Krt::kvinUnadi, &["jAgfvi"]);
}

#[test]
fn unadi_4_158() {
    assert_has_krdanta(&[], &d("vasa~\\", Adadi), Krt::zwran, &["vastra"]);
    assert_has_krdanta(&[], &d("asu~", Divadi), Krt::zwran, &["astra"]);
    assert_has_krdanta(&[], &d("Sasu~", Bhvadi), Krt::zwran, &["Sastra"]);
}

#[test]
fn unadi_5_68() {
    assert_has_krdanta(&[], &d("praTa~\\", Bhvadi), Krt::amac, &["praTama"]);
}

#[test]
fn unadi_5_69() {
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Krt::amac, &["carama"]);
}

#[test]
fn unadi_5_70() {
    assert_has_krdanta(&[], &d("magi~", Bhvadi), Krt::alac, &["maNgala"]);
}
