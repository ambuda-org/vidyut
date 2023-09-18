extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;

#[test]
fn sutra_5_1_2() {
    assert_has_taddhitanta(&prati("SaNku"), T::yat, &["SaNkavya"]);
    assert_has_taddhitanta(&prati("picU"), T::yat, &["picavya"]);
    assert_has_taddhitanta(&prati("kamaRqalu"), T::yat, &["kamaRqalavya"]);
    assert_has_taddhitanta(&prati("go"), T::yat, &["gavya"]);
    // TODO: others
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
