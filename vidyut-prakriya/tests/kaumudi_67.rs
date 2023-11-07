// Test cases from the Siddhantakaumudi commentary on the Unadipatha.

extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

#[test]
fn unadi_1_1() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Unadi::uR, &["kAru"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), Unadi::uR, &["pAyu"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), Unadi::uR, &["vAyu"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Unadi::uR, &["jAyu"]);
    assert_has_krdanta(&[], &d("qumi\\Y", Svadi), Unadi::uR, &["mAyu"]);
    assert_has_krdanta(&[], &d("zvada~\\", Bhvadi), Unadi::uR, &["svAdu"]);
    assert_has_krdanta(&[], &d("sA\\Da~", Svadi), Unadi::uR, &["sADu"]);
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), Unadi::uR, &["ASu"]);
}

#[test]
fn unadi_1_2() {
    let t = Tester::with_chaandasa();
    t.assert_has_krt(&[], &d("i\\R", Adadi), Unadi::uR, &["Ayu"]);
}

#[ignore]
#[test]
fn unadi_1_5() {
    // TODO: not sure where to apply r --> l
    assert_has_krdanta(&[], &d("tF", Bhvadi), Unadi::YuR, &["tAlu"]);
}

#[test]
fn unadi_1_45() {
    assert_has_krdanta(&[], &d("ava~", Bhvadi), Unadi::wizac, &["aviza"]);
    // TODO: right mah?
    assert_has_krdanta(&[], &d("maha~", Bhvadi), Unadi::wizac, &["mahiza"]);
}

#[test]
fn unadi_1_69() {
    use Unadi::tun;
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
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Unadi::katu, &["kratu"]);
}

#[test]
fn unadi_2_2() {
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), Unadi::kTan, &["haTa"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), Unadi::kTan, &["kuzWa"]);
    assert_has_krdanta(&[], &d("RI\\Y", Tanadi), Unadi::kTan, &["nITa"]);
    assert_has_krdanta(&[], &d("kASf~", Bhvadi), Unadi::kTan, &["kAzWa"]);
}

#[test]
fn unadi_2_108() {
    assert_has_krdanta(&[], &d("arca~", Bhvadi), Unadi::isi, &["arcis"]);
    assert_has_krdanta(&[], &d("I~Suci~^r", Divadi), Unadi::isi, &["Socis"]);
    assert_has_krdanta(&[], &d("hu\\", Juhotyadi), Unadi::isi, &["havis"]);
    assert_has_krdanta(&[], &d("sf\\px~", Bhvadi), Unadi::isi, &["sarpis"]);
    assert_has_krdanta(&[], &d("Cada~", Curadi), Unadi::isi, &["Cadis"]);
    assert_has_krdanta(&[], &d("Carda~", Curadi), Unadi::isi, &["Cardis"]);
    // TODO: id-anta?
}

#[test]
fn unadi_2_115() {
    assert_has_krdanta(&[], &d("janI~\\", Divadi), Unadi::usi, &["janus"]);
}

#[test]
fn unadi_2_117() {
    use Unadi::usi;
    assert_has_krdanta(&[], &d("f\\", Bhvadi), usi, &["arus"]);
    assert_has_krdanta(&[], &d("pF", Kryadi), usi, &["parus"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), usi, &["vapus"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), usi, &["yajus"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), usi, &["tanus"]);
    assert_has_krdanta(&[], &d("Dana~", Juhotyadi), usi, &["Danus"]);
    assert_has_krdanta(&[], &d("ta\\pa~", Bhvadi), usi, &["tapus"]);
    // TODO: tanuzI, tanUMzi
}

#[test]
fn unadi_2_118() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), Unadi::usi, &["Ayus"]);
    // TODO: AyuzI
}

#[test]
fn unadi_2_119() {
    assert_has_krdanta(&[], &d("ca\\kzi~\\N", Adadi), Unadi::usi, &["cakzus"]);
}

#[test]
fn unadi_2_120() {
    assert_has_krdanta(&[], &d("mu\\ha~", Divadi), Unadi::usi, &["muhus"]);
}

#[ignore]
#[test]
fn unadi_3_29() {
    assert_has_krdanta(&[], &d("stana", Curadi), Unadi::itnuc, &["stanayitnu"]);
    assert_has_krdanta(
        &[],
        &nic(&d("hfza~", Divadi)),
        Unadi::itnuc,
        &["harzayitnu"],
    );
    // TODO: popayitnu?
    assert_has_krdanta(&[], &d("gada", Curadi), Unadi::itnuc, &["gadayitnu"]);
    assert_has_krdanta(&[], &d("mada~", Curadi), Unadi::itnuc, &["madayitnu"]);
}

#[test]
fn unadi_3_43() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), Unadi::kan, &["eka"]);
    assert_has_krdanta(&[], &d("YiBI\\", Juhotyadi), Unadi::kan, &["Beka"]);
    assert_has_krdanta(&[], &d("kE\\", Tanadi), Unadi::kan, &["kAka"]);
    assert_has_krdanta(&[], &d("pA\\", Adadi), Unadi::kan, &["pAka"]);
    assert_has_krdanta(&[], &d("Sala~", Bhvadi), Unadi::kan, &["Salka"]);
    assert_has_krdanta(&[], &d("ata~", Bhvadi), Unadi::kan, &["atka"]);
    assert_has_krdanta(&[], &d("marca~", Bhvadi), Unadi::kan, &["marka", "markka"]);
}

#[test]
fn unadi_3_62() {
    // TODO: why not varza and tarza?
    // assert_has_krdanta(&[], &d("vF", Kryadi), Krt::sa, &["varsa"]);
    // assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::sa, &["tarsa"]);
    assert_has_krdanta(&[], &d("vada~", Bhvadi), Unadi::sa, &["vatsa"]);
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), Unadi::sa, &["haMsa"]);
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), Unadi::sa, &["kaMsa"]);
    assert_has_krdanta(&[], &d("kaza~", Bhvadi), Unadi::sa, &["kakza"]);
}

#[test]
fn unadi_3_70() {
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), Unadi::sara, &["akzara"]);
}

#[test]
fn unadi_3_86() {
    assert_has_krdanta(&[], &d("hase~", Bhvadi), Unadi::tan, &["hasta"]);
    assert_has_krdanta(&[], &d("mf\\N", Tanadi), Unadi::tan, &["marta"]);
    assert_has_krdanta(&[], &d("gF", Tudadi), Unadi::tan, &["garta"]);
    assert_has_krdanta(&[], &d("i\\R", Adadi), Unadi::tan, &["eta"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), Unadi::tan, &["vAta"]);
    assert_has_krdanta(&[], &d("ama~", Bhvadi), Unadi::tan, &["anta"]);
    assert_has_krdanta(&[], &d("damu~", Divadi), Unadi::tan, &["danta"]);
    assert_has_krdanta(&[], &d("lUY", Kryadi), Unadi::tan, &["lota"]);
    assert_has_krdanta(&[], &d("pUY", Kryadi), Unadi::tan, &["pota"]);
    // TODO: enable this after fixing cchvoh
    // assert_has_krdanta(&[], &d("DurvI~", Bhvadi), Krt::tan, &["DUrta"]);
}

#[test]
fn unadi_3_96() {
    assert_has_krdanta(&[], &d("Sru\\", Bhvadi), Unadi::Ayya, &["SravAyya"]);
    assert_has_krdanta(&[], &d("dakza~\\", Bhvadi), Unadi::Ayya, &["dakzAyya"]);
    assert_has_krdanta(&[], &d("spfha", Curadi), Unadi::Ayya, &["spfhayAyya"]);
    assert_has_krdanta(&[], &d("gfha", Curadi), Unadi::Ayya, &["gfhayAyya"]);
}

#[test]
fn unadi_3_126() {
    assert_has_krdanta(&[], &d("jF", Kryadi), Unadi::Jac, &["jaranta"]);
    assert_has_krdanta(&[], &d("vi\\Sa~", Tudadi), Unadi::Jac, &["veSanta"]);
}

#[test]
fn unadi_3_127() {
    assert_has_krdanta(&[], &d("ru\\ha~", Bhvadi), Unadi::Jac, &["rohanta"]);
    assert_has_krdanta(&[], &d("wunadi~", Bhvadi), Unadi::Jac, &["nandanta"]);
    assert_has_krdanta(&[], &d("jIva~", Tudadi), Unadi::Jac, &["jIvanta"]);
    // TODO: zit
}

#[test]
fn unadi_3_128() {
    assert_has_krdanta(&[], &d("tF", Bhvadi), Unadi::Jac, &["taranta"]);
    assert_has_krdanta(&[], &d("BU", Bhvadi), Unadi::Jac, &["Bavanta"]);
    assert_has_krdanta(&[], &d("va\\ha~^", Bhvadi), Unadi::Jac, &["vahanta"]);
    assert_has_krdanta(&[], &d("va\\sa~", Bhvadi), Unadi::Jac, &["vasanta"]);
    assert_has_krdanta(&[], &d("BAsf~\\", Bhvadi), Unadi::Jac, &["BAsanta"]);
    assert_has_krdanta(&[], &d("sA\\Da~", Svadi), Unadi::Jac, &["sADanta"]);
    assert_has_krdanta(&[], &nic(&d("gaqi~", Bhvadi)), Unadi::Jac, &["gaRqayanta"]);
    assert_has_krdanta(
        &[],
        &nic(&d("maqi~\\", Tudadi)),
        Unadi::Jac,
        &["maRqayanta"],
    );
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Unadi::Jac, &["jayanta"]);
    // nandayanta?
}

#[test]
fn unadi_3_155() {
    assert_has_krdanta(&[], &d("pluza~", Kryadi), Unadi::ksi, &["plukzi"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), Unadi::ksi, &["kukzi"]);
    assert_has_krdanta(&[], &d("Su\\za~", Divadi), Unadi::ksi, &["Sukzi"]);
}

#[test]
fn unadi_3_156() {
    assert_has_krdanta(&[], &d("aSU~", Svadi), Unadi::ksi, &["akzi"]);
}

#[test]
fn unadi_3_157() {
    assert_has_krdanta(&[], &d("izu~", Tudadi), Unadi::ksu, &["ikzu"]);
}

#[test]
fn unadi_4_2() {
    assert_has_krdanta(&[], &d("f\\", Juhotyadi), Unadi::katnic, &["ratni"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), Unadi::yatuc, &["tanyatu"]);
    assert_has_krdanta(&[], &d("anjU~", Rudhadi), Unadi::alic, &["aYjali"]);
    assert_has_krdanta(&[], &d("vana~", Bhvadi), Unadi::izWuc, &["vanizWu"]);
    assert_has_krdanta(&[], &d("anjU~", Rudhadi), Unadi::izWac, &["aYjizWa"]);
    assert_has_krdanta(&[], &nic(&d("f\\", Juhotyadi)), Unadi::isan, &["arpisa"]);
    // TODO: why is this aniw?
    // assert_has_krdanta(&[], &d("madI~", Divadi), Krt::syan, &["matsya"]);
    assert_has_krdanta(&[], &d("ata~", Bhvadi), Unadi::iTin, &["atiTi"]);
    assert_has_krdanta(&[], &d("anga", Curadi), Unadi::uli, &["aNguli"]);
    assert_has_krdanta(&[], &d("ku\\", Adadi), Unadi::asa, &["kavasa"]);
    assert_has_krdanta(&[], &d("yu", Adadi), Unadi::Asa, &["yavAsa"]);
    assert_has_krdanta(&[], &d("kfSa~", Divadi), Unadi::Anuk, &["kfSAnu"]);
}

#[test]
fn unadi_4_6() {
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), Unadi::ini, &["gamin"]);
}

#[test]
fn unadi_4_7() {
    assert_has_krdanta(&["AN"], &d("ga\\mx~", Bhvadi), Unadi::ini, &["AgAmin"]);
}

#[test]
fn unadi_4_8() {
    assert_has_krdanta(&[], &d("BU", Bhvadi), Unadi::ini, &["BAvin"]);
}

#[test]
fn unadi_4_9() {
    assert_has_krdanta(&["pra"], &d("zWA\\", Bhvadi), Unadi::ini, &["prasTAyin"]);
}

#[ignore]
#[test]
fn unadi_4_10() {
    assert_has_krdanta(
        &["parama"],
        &d("zWA\\", Bhvadi),
        Unadi::ini,
        &["paramezWin"],
    );
}

#[ignore]
#[test]
fn unadi_4_11() {
    assert_has_krdanta(&[], &d("maTi~", Bhvadi), Unadi::ini, &["maTin"]);
}

#[test]
fn unadi_4_12() {
    assert_has_krdanta(&[], &d("patx~", Bhvadi), Unadi::ini, &["paTin"]);
}

#[test]
fn unadi_4_54() {
    assert_has_krdanta(&[], &d("jF", Kryadi), Unadi::kvin, &["jIrvi"]);
    assert_has_krdanta(&[], &d("SFY", Kryadi), Unadi::kvin, &["SIrvi"]);
    assert_has_krdanta(&[], &d("stFY", Kryadi), Unadi::kvin, &["stIrvi"]);
    assert_has_krdanta(&[], &d("jAgf", Adadi), Unadi::kvin, &["jAgfvi"]);
}

#[ignore]
#[test]
fn unadi_4_117() {
    use Unadi::in_;
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), in_, &["paci"]);
    assert_has_krdanta(&[], &d("tuqa~", Tudadi), in_, &["tuqi"]);
    assert_has_krdanta(&[], &d("tuqi~\\", Bhvadi), in_, &["tuRqi"]);
    assert_has_krdanta(&[], &d("vala~\\", Bhvadi), in_, &["vali"]);
    assert_has_krdanta(&[], &d("vawa~", Bhvadi), in_, &["vawi"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), in_, &["yaji"]);
    assert_has_upapada_krdanta("deva", &[], &d("ya\\ja~^", Bhvadi), in_, &["devayaji"]);
    assert_has_krdanta(&[], &d("kASf~\\", Bhvadi), in_, &["kASi"]);
    assert_has_krdanta(&[], &d("yatI~\\", Bhvadi), in_, &["yati"]);
    assert_has_krdanta(&[], &d("malla~\\", Bhvadi), in_, &["malli"]);
    assert_has_krdanta(&[], &d("kila~", Tudadi), in_, &["keli"]);
    assert_has_krdanta(&[], &d("masI~", Divadi), in_, &["masi"]);
    assert_has_krdanta(&[], &d("kuwa~", Tudadi), in_, &["kowi"]);
    assert_has_krdanta(&[], &d("hila~", Tudadi), in_, &["heli"]);
    assert_has_krdanta(&[], &d("buDa~", Bhvadi), in_, &["boDi"]);
    assert_has_krdanta(&[], &d("wunadi~", Bhvadi), in_, &["nandi"]);
    assert_has_krdanta(&[], &d("kala~\\", Bhvadi), in_, &["kali"]);
}

#[test]
fn unadi_4_144() {
    use Unadi::manin;
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), manin, &["karman"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), manin, &["carman"]);
    assert_has_krdanta(&[], &d("Basa~", Juhotyadi), manin, &["Basman"]);
    assert_has_krdanta(&[], &d("janI~\\", Divadi), manin, &["janman"]);
    assert_has_krdanta(&[], &d("SF", Kryadi), manin, &["Sarman"]);
    assert_has_krdanta(&[], &d("Cada~", Curadi), manin, &["Cadman"]);
    // TODO: sutrAman
}

#[test]
fn unadi_4_158() {
    assert_has_krdanta(&[], &d("vasa~\\", Adadi), Unadi::zwran, &["vastra"]);
    assert_has_krdanta(&[], &d("asu~", Divadi), Unadi::zwran, &["astra"]);
    assert_has_krdanta(&[], &d("Sasu~", Bhvadi), Unadi::zwran, &["Sastra"]);
}

#[test]
fn unadi_4_188() {
    use Unadi::asun;
    assert_has_krdanta(&[], &d("citI~", Bhvadi), asun, &["cetas"]);
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), asun, &["saras"]);
    assert_has_krdanta(&[], &d("pI\\N", Divadi), asun, &["payas"]);
    assert_has_krdanta(&[], &d("za\\dx~", Bhvadi), asun, &["sadas"]);
}

#[test]
fn unadi_4_189() {
    assert_has_krdanta(&[], &d("rapa~", Bhvadi), Unadi::asun, &["repas"]);
}

#[test]
fn unadi_5_68() {
    assert_has_krdanta(&[], &d("praTa~\\", Bhvadi), Unadi::amac, &["praTama"]);
}

#[test]
fn unadi_5_69() {
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Unadi::amac, &["carama"]);
}

#[test]
fn unadi_5_70() {
    assert_has_krdanta(&[], &d("magi~", Bhvadi), Unadi::alac, &["maNgala"]);
}
