extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::Unadi;

fn assert_blocked(text: &str, artha: TaddhitaArtha, t: T) {
    assert_has_artha_taddhita(text, artha, t, &[]);
}

#[test]
fn sutra_5_1_1() {
    assert_has_artha_taddhita("vatsa", TasmaiHitam, T::Ca, &["vatsIya"]);
    assert_has_artha_taddhita("karaBa", TasmaiHitam, T::Ca, &["karaBIya"]);
    assert_has_artha_taddhita("akaraBa", TasmaiHitam, T::Ca, &["akaraBIya"]);
    assert_has_artha_taddhita("avatsa", TasmaiHitam, T::Ca, &["avatsIya"]);
}

#[test]
fn sutra_5_1_2() {
    assert_has_taddhita("SaNku", T::yat, &["SaNkavya"]);
    assert_has_taddhita("picU", T::yat, &["picavya"]);
    assert_has_taddhita("kamaRqalu", T::yat, &["kamaRqalavya"]);
    assert_has_taddhita("go", T::yat, &["gavya"]);
    // TODO: others
}

#[test]
fn sutra_5_1_3() {
    assert_has_taddhita("kambala", T::yat, &["kambalya"]);
    assert_has_artha_taddhita("kambala", TasmaiHitam, T::Ca, &["kambalIya"]);
}

#[ignore]
#[test]
fn sutra_5_1_5() {
    assert_has_artha_taddhita("vatsa", TasmaiHitam, T::Ca, &["vatsIya"]);
    assert_has_artha_taddhita("avatsa", TasmaiHitam, T::Ca, &["avatsIya"]);
    assert_has_artha_taddhita("pawu", TasmaiHitam, T::yat, &["pawavya"]);
    assert_has_artha_taddhita("go", TasmaiHitam, T::yat, &["gavya"]);
    assert_has_artha_taddhita("havis", TasmaiHitam, T::yat, &["havizya"]);
    assert_has_artha_taddhita("apUpa", TasmaiHitam, T::yat, &["apUpya"]);
    assert_has_artha_taddhita("apUpa", TasmaiHitam, T::Ca, &["apUpIya"]);
}

#[test]
fn sutra_5_1_7() {
    assert_has_artha_taddhita("Kala", TasmaiHitam, T::yat, &["Kalya"]);
    assert_has_artha_taddhita("yava", TasmaiHitam, T::yat, &["yavya"]);
    assert_has_artha_taddhita("mAza", TasmaiHitam, T::yat, &["mAzya"]);
    assert_has_artha_taddhita("tila", TasmaiHitam, T::yat, &["tilya"]);
    assert_has_artha_taddhita("vfza", TasmaiHitam, T::yat, &["vfzya"]);
    assert_has_artha_taddhita("brahman", TasmaiHitam, T::yat, &["brahmaRya"]);
    assert_blocked("Kala", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_8() {
    assert_has_artha_taddhita("aja", TasmaiHitam, T::Tyan, &["ajaTya"]);
    assert_has_artha_taddhita("avi", TasmaiHitam, T::Tyan, &["aviTya"]);
    assert_blocked("aja", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_9() {
    assert_has_artha_taddhita("Atman", TasmaiHitam, T::Ka, &["AtmanIna"]);
    assert_has_artha_taddhita("viSvajana", TasmaiHitam, T::Ka, &["viSvajanIna"]);
    assert_has_artha_taddhita("mAtfBoga", TasmaiHitam, T::Ka, &["mAtfBogIRa"]);
    assert_has_artha_taddhita("pitfBoga", TasmaiHitam, T::Ka, &["pitfBogIRa"]);
    assert_blocked("Atman", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_10() {
    assert_has_artha_taddhita("sarva", TasmaiHitam, T::Ra, &["sArva"]);
    assert_has_artha_taddhita("puruza", TasmaiHitam, T::QaY, &["pOruzeya"]);
    assert_blocked("sarva", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_11() {
    assert_has_artha_taddhita("mARava", TasmaiHitam, T::KaY, &["mARavIna"]);
    assert_has_artha_taddhita("caraka", TasmaiHitam, T::KaY, &["cArakIRa"]);
    assert_blocked("mARava", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_12() {
    let artha = TadarthamVikrtehPrakrtau;
    assert_has_artha_taddhita("aNgAra", artha, T::Ca, &["aNgArIya"]);
    assert_has_artha_taddhita("prAkAra", artha, T::Ca, &["prAkArIya"]);
    assert_has_artha_taddhita("Sanku", artha, T::yat, &["SaNkavya"]);
    assert_has_artha_taddhita("picU", artha, T::yat, &["picavya"]);
}

#[ignore]
#[test]
fn sutra_5_1_13() {
    let artha = TadarthamVikrtehPrakrtau;
    let chadis = create_krdanta("Cadis", &[], &d("Cada~", Curadi), Unadi::isi);
    assert_has_artha_taddhita(&chadis, artha, T::QaY, &["CAdizeya"]);
    assert_has_artha_taddhita("upaDi", artha, T::QaY, &["OpaDeya"]);
    assert_has_artha_taddhita("bali", artha, T::QaY, &["bAleya"]);
    assert_blocked("Cadis", artha, T::Ca);
}

#[test]
fn sutra_5_1_14() {
    let artha = TadarthamVikrtehPrakrtau;
    assert_has_artha_taddhita("fzaBa", artha, T::Yya, &["ArzaBya"]);
    assert_has_artha_taddhita("upAnah", artha, T::Yya, &["OpAnahya"]);
}

#[test]
fn sutra_5_1_15() {
    let artha = TadarthamVikrtehPrakrtau;
    assert_has_artha_taddhita("varDrI", artha, T::aY, &["vArDra"]);
    assert_has_artha_taddhita("varatrA", artha, T::aY, &["vAratra"]);
    assert_blocked("varDrI", artha, T::Ca);
}

#[test]
fn sutra_5_1_17() {
    let artha = TadAsyaTadAsminSyat;
    assert_has_artha_taddhita("pariKA", artha, T::QaY, &["pAriKeya"]);
    assert_blocked("pariKA", artha, T::Ca);
}

#[test]
fn sutra_5_1_18() {
    assert_has_artha_taddhita("pArAyaRa", TadVartayati, T::WaY, &["pArAyaRika"]);
    assert_has_artha_taddhita("turAyaRa", TadVartayati, T::WaY, &["tOrAyaRika"]);
    assert_has_artha_taddhita("cAndrAyaRa", TadVartayati, T::WaY, &["cAndrAyaRika"]);
}

#[ignore]
#[test]
fn sutra_5_1_22() {
    assert_has_taddhita("paYca", T::kan, &["paYcaka"]);
    assert_has_taddhita("bahu", T::kan, &["bahuka"]);
    assert_has_taddhita("gaRa", T::kan, &["gaRaka"]);
    // a-ti-Sat
    assert_has_taddhita("saptati", T::WaY, &["sAptatika"]);
    assert_has_taddhita("catvAriMSat", T::WaY, &["cAtvAriMSatka"]);
}

#[test]
fn sutra_5_1_63() {
    assert_has_artha_taddhita("SvetacCatra", TadArhati, T::Wak, &["SvEtacCatrika"]);
    assert_has_artha_taddhita("vastrayugma", TadArhati, T::Wak, &["vAstrayugmika"]);
    assert_has_artha_taddhita("Sata", TadArhati, T::Wan, &["Satika"]);
    assert_has_artha_taddhita("Sata", TadArhati, T::yat, &["Satya"]);
    assert_has_artha_taddhita("sahasra", TadArhati, T::aR, &["sAhasra"]);
}

#[test]
fn sutra_5_1_66() {
    assert_has_artha_taddhita("daRqa", TadArhati, T::yat, &["daRqya"]);
    assert_has_artha_taddhita("musala", TadArhati, T::yat, &["musalya"]);
    assert_blocked("daRqa", TadArhati, T::Wak);
}

#[test]
fn sutra_5_1_67() {
    let t = Tester::with_chaandasa();
    t.assert_has_artha_taddhita("udaka", TadArhati, T::yat, &["udakya"]);
    t.assert_has_artha_taddhita("yUpa", TadArhati, T::yat, &["yUpya"]);
}

#[test]
fn sutra_5_1_68() {
    assert_has_artha_taddhita("pAtra", TadArhati, T::Gan, &["pAtriya"]);
    assert_has_artha_taddhita("pAtra", TadArhati, T::yat, &["pAtrya"]);
    assert_blocked("pAtra", TadArhati, T::Wak);
    assert_blocked("pAtra", TadArhati, T::WaY);
}

#[test]
fn sutra_5_1_69() {
    assert_has_artha_taddhita("kaqaNkara", TadArhati, T::Ca, &["kaqaNkarIya"]);
    assert_has_artha_taddhita("kaqaNkara", TadArhati, T::yat, &["kaqaNkarya"]);
    assert_has_artha_taddhita("dakziRA", TadArhati, T::Ca, &["dakziRIya"]);
    assert_has_artha_taddhita("dakziRA", TadArhati, T::yat, &["dakziRya"]);
    assert_blocked("kaqaNkara", TadArhati, T::Wak);
}

#[test]
fn sutra_5_1_70() {
    assert_has_artha_taddhita("sTAlIbila", TadArhati, T::Ca, &["sTAlIbilIya"]);
    assert_has_artha_taddhita("sTAlIbila", TadArhati, T::yat, &["sTAlIbilya"]);
    assert_blocked("sTAlIbila", TadArhati, T::Wak);
}

#[test]
fn sutra_5_1_71() {
    assert_has_artha_taddhita("yajYa", TadArhati, T::Ga, &["yajYiya"]);
    assert_has_artha_taddhita("ftvij", TadArhati, T::KaY, &["ArtvijIna"]);
    assert_blocked("yajYa", TadArhati, T::Wak);
    assert_blocked("ftvij", TadArhati, T::Wak);
}

#[test]
fn sutra_5_1_72() {
    assert_has_artha_taddhita("pArAyaRa", TadVartayati, T::WaY, &["pArAyaRika"]);
    assert_has_artha_taddhita("turAyaRa", TadVartayati, T::WaY, &["tOrAyaRika"]);
    assert_has_artha_taddhita("cAndrAyaRa", TadVartayati, T::WaY, &["cAndrAyaRika"]);
}

#[test]
fn sutra_5_1_73() {
    assert_has_artha_taddhita("saMSaya", Apanna, T::WaY, &["sAMSayika"]);
}

#[test]
fn sutra_5_1_74() {
    assert_has_artha_taddhita("yojana", Gacchati, T::WaY, &["yOjanika"]);
}

#[test]
fn sutra_5_1_74_v1() {
    let artha = AbhigamanamArhati;
    assert_has_artha_taddhita("kroSaSata", artha, T::WaY, &["krOSaSatika"]);
    assert_has_artha_taddhita("yojanaSata", artha, T::WaY, &["yOjanaSatika"]);
}

#[test]
fn sutra_5_1_75() {
    let pathika = create_artha_taddhita("paTika", "paTin", Gacchati, T::zkan);
    assert_has_sup_1s(&pathika, Pum, &["paTikaH"]);
    assert_has_sup_1s(&pathika, Stri, &["paTikI"]);
}

#[test]
fn sutra_5_1_105() {
    assert_has_taddhita("ftu", T::aR, &["Artava"]);
}

#[test]
fn sutra_5_1_106() {
    assert_has_taddhita("ftu", T::Gas, &["ftviya"]);
}

#[test]
fn sutra_5_1_119() {
    assert_has_taddhita("aSva", T::tva, &["aSvatva"]);
    assert_has_taddhita("aSva", T::tal, &["aSvatA"]);
    assert_has_taddhita("go", T::tva, &["gotva"]);
    assert_has_taddhita("go", T::tal, &["gotA"]);
}

#[test]
fn sutra_5_1_122() {
    assert_has_taddhita("pfTu", T::imanic, &["praTiman"]);
    assert_has_taddhita("pfTu", T::aR, &["pArTava"]);
    assert_has_taddhita("mfdu", T::imanic, &["mradiman"]);
    assert_has_taddhita("mfdu", T::aR, &["mArdava"]);
}
