extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;

#[test]
fn sutra_5_4_3() {
    assert_has_taddhitanta(&prati("sTUla"), T::kan, &["sTUlaka"]);
    assert_has_taddhitanta(&prati("aRu"), T::kan, &["aRuka"]);
    assert_has_taddhitanta(&prati("mAza"), T::kan, &["mAzaka"]);
    assert_has_taddhitanta(&prati("gomUtra"), T::kan, &["gomUtraka"]);
}

#[test]
fn sutra_5_4_3_v1() {
    assert_has_taddhitanta(&prati("caYcut"), T::kan, &["caYcutka"]);
    assert_has_taddhitanta(&prati("bfhat"), T::kan, &["bfhatka"]);
}

#[test]
fn sutra_5_4_17() {
    assert_has_taddhitanta(&prati("paYcan"), T::kftvasuc, &["paYcakftvas"]);
    assert_has_taddhitanta(&prati("saptan"), T::kftvasuc, &["saptakftvas"]);
    // TODO: saNKyAyAH?

    // For these, see 5.4.18.
    assert_has_taddhitanta(&prati("dvi"), T::kftvasuc, &[]);
    assert_has_taddhitanta(&prati("tri"), T::kftvasuc, &[]);
    assert_has_taddhitanta(&prati("catur"), T::kftvasuc, &[]);
}

#[ignore]
#[test]
fn sutra_5_4_18() {
    assert_has_taddhitanta(&prati("dvi"), T::suc, &["dviH"]);
    assert_has_taddhitanta(&prati("tri"), T::suc, &["triH"]);
    assert_has_taddhitanta(&prati("catur"), T::suc, &["catuH"]);
}

#[ignore]
#[test]
fn sutra_5_4_19() {
    assert_has_taddhitanta(&prati("eka"), T::suc, &["sakft"]);
}

#[test]
fn sutra_5_4_21() {
    assert_has_taddhitanta(&prati("anna"), T::mayaw, &["annamaya"]);
    assert_has_taddhitanta(&prati("apUpa"), T::mayaw, &["apUpamaya"]);
    assert_has_taddhitanta(&prati("vawaka"), T::mayaw, &["vawakamaya"]);
}

#[test]
fn sutra_5_4_23() {
    assert_has_taddhitanta(&prati("ananta"), T::Yya, &["Anantya"]);
    assert_has_taddhitanta(&prati("AvasaTa"), T::Yya, &["AvasaTya"]);
    assert_has_taddhitanta(&prati("itiha"), T::Yya, &["Etihya"]);
    assert_has_taddhitanta(&prati("Bezaja"), T::Yya, &["BEzajya"]);
}

#[test]
fn sutra_5_4_26() {
    assert_has_taddhitanta(&prati("atiTi"), T::Yya, &["AtiTya"]);
}

#[test]
fn sutra_5_4_27() {
    assert_has_taddhitanta(&prati("deva"), T::tal, &["devatA"]);
}

#[ignore]
#[test]
fn sutra_5_4_39() {
    assert_has_taddhitanta(&prati("mft"), T::tikan, &["mfttikA"]);
}

#[ignore]
#[test]
fn sutra_5_4_43() {
    assert_has_taddhitanta(&prati("dvi"), T::Sas, &["dviSaH"]);
    assert_has_taddhitanta(&prati("tri"), T::Sas, &["triSaH"]);
    assert_has_taddhitanta(&prati("kArzApaRa"), T::Sas, &["kArzApaRaSaH"]);
    assert_has_taddhitanta(&prati("mAza"), T::Sas, &["mAzaSaH"]);
    assert_has_taddhitanta(&prati("pAda"), T::Sas, &["pAdaSaH"]);
}

#[test]
fn sutra_5_4_44() {
    assert_has_taddhitanta(&prati("vAsudeva"), T::tasi, &["vAsudevatas"]);
    assert_has_taddhitanta(&prati("arjuna"), T::tasi, &["arjunatas"]);

    // Akrtigana
    assert_has_taddhitanta(&prati("Adi"), T::tasi, &["Aditas"]);
    assert_has_taddhitanta(&prati("maDya"), T::tasi, &["maDyatas"]);
    assert_has_taddhitanta(&prati("pArSva"), T::tasi, &["pArSvatas"]);
    assert_has_taddhitanta(&prati("pfzWa"), T::tasi, &["pfzWatas"]);
}

#[test]
fn sutra_5_4_54() {
    assert_has_taddhitanta(&prati("agni"), T::sAti, &["agnisAt"]);
    assert_has_taddhitanta(&prati("udaka"), T::sAti, &["udakasAt"]);
}
