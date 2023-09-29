extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Artha;
use vidyut_prakriya::args::Artha::*;
use vidyut_prakriya::args::Taddhita as T;

fn assert_blocked(text: &str, artha: Artha, t: T) {
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
    assert_has_taddhitanta(&prati("SaNku"), T::yat, &["SaNkavya"]);
    assert_has_taddhitanta(&prati("picU"), T::yat, &["picavya"]);
    assert_has_taddhitanta(&prati("kamaRqalu"), T::yat, &["kamaRqalavya"]);
    assert_has_taddhitanta(&prati("go"), T::yat, &["gavya"]);
    // TODO: others
}

#[test]
fn sutra_5_1_3() {
    assert_has_taddhitanta(&prati("kambala"), T::yat, &["kambalya"]);
    assert_has_artha_taddhita("kambala", TasmaiHitam, T::Ca, &["kambalIya"]);
}

#[ignore]
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

#[ignore]
#[test]
fn sutra_5_1_22() {
    assert_has_taddhitanta(&prati("paYca"), T::kan, &["paYcaka"]);
    assert_has_taddhitanta(&prati("bahu"), T::kan, &["bahuka"]);
    assert_has_taddhitanta(&prati("gaRa"), T::kan, &["gaRaka"]);
    // a-ti-Sat
    assert_has_taddhitanta(&prati("saptati"), T::WaY, &["sAptatika"]);
    assert_has_taddhitanta(&prati("catvAriMSat"), T::WaY, &["cAtvAriMSatka"]);
}

#[test]
fn sutra_5_1_105() {
    assert_has_taddhitanta(&prati("ftu"), T::aR, &["Artava"]);
}

#[test]
fn sutra_5_1_106() {
    assert_has_taddhitanta(&prati("ftu"), T::Gas, &["ftviya"]);
}

#[test]
fn sutra_5_1_119() {
    assert_has_taddhitanta(&prati("aSva"), T::tva, &["aSvatva"]);
    assert_has_taddhitanta(&prati("aSva"), T::tal, &["aSvatA"]);
    assert_has_taddhitanta(&prati("go"), T::tva, &["gotva"]);
    assert_has_taddhitanta(&prati("go"), T::tal, &["gotA"]);
}

#[test]
fn sutra_5_1_122() {
    assert_has_taddhitanta(&prati("pfTu"), T::imanic, &["praTiman"]);
    assert_has_taddhitanta(&prati("pfTu"), T::aR, &["pArTava"]);
    assert_has_taddhitanta(&prati("mfdu"), T::imanic, &["mradiman"]);
    assert_has_taddhitanta(&prati("mfdu"), T::aR, &["mArdava"]);
}
