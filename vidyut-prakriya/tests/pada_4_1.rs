extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

fn prati(text: &str) -> Pratipadika {
    Pratipadika::builder().text(text).build().unwrap()
}

fn assert_has_pum(prati: &str, expected: &[&str]) {
    assert_has_subantas(prati, Pum, Prathama, Eka, expected);
}

fn assert_has_stri(prati: &str, expected: &[&str]) {
    assert_has_subantas(prati, Stri, Prathama, Eka, expected);
}

#[test]
fn sutra_4_1_4() {
    assert_has_stri("aja", &["ajA"]);
    assert_has_stri("devadatta", &["devadattA"]);
    // striyAm
    assert_has_pum("aja", &["ajaH"]);
    assert_has_pum("devadatta", &["devadattaH"]);

    // ajAdi -- jAti
    assert_has_stri("aja", &["ajA"]);
    assert_has_stri("eqaka", &["eqakA"]);
    assert_has_stri("kokila", &["kokilA"]);
    assert_has_stri("cawaka", &["cawakA"]);
    assert_has_stri("aSva", &["aSvA"]);
    assert_has_stri("mUzika", &["mUzikA"]);
    // ajAdi -- vayaH
    assert_has_stri("bAla", &["bAlA"]);
    assert_has_stri("hoQa", &["hoQA"]);
    assert_has_stri("pAka", &["pAkA"]);
    assert_has_stri("vatsa", &["vatsA"]);
    assert_has_stri("manda", &["mandA"]);
    assert_has_stri("vilAta", &["vilAtA"]);
    // ajAdi -- Pala
    assert_has_stri("samPala", &["samPalA"]);
    assert_has_stri("BastraPala", &["BastraPalA"]);
    assert_has_stri("ajinaPala", &["ajinaPalA"]);
    assert_has_stri("SaRaPala", &["SaRaPalA"]);
    assert_has_stri("piRqaPala", &["piRqaPalA"]);
    assert_has_stri("triPala", &["triPalA"]);
    // ajAdi -- puzpa
    assert_has_stri("satpuzpa", &["satpuzpA"]);
    assert_has_stri("prAkpuzpa", &["prAkpuzpA"]);
    assert_has_stri("kARqapuzpa", &["kARqapuzpA"]);
    assert_has_stri("prAntapuzpa", &["prAntapuzpA"]);
    assert_has_stri("Satapuzpa", &["SatapuzpA"]);
    assert_has_stri("ekapuzpa", &["ekapuzpA"]);

    // TODO: others
}

#[test]
fn sutra_4_1_5() {
    assert_has_stri("kartf", &["kartrI"]);
    assert_has_stri("hartf", &["hartrI"]);
    assert_has_stri("daRqin", &["daRqinI"]);
    assert_has_stri("Catrin", &["CatriRI"]);
}

#[test]
fn sutra_4_1_10() {
    assert_has_stri("svasf", &["svasA"]);
    assert_has_stri("duhitf", &["duhitA"]);
    assert_has_stri("nanAndf", &["nanAndA"]);
    assert_has_stri("yAtf", &["yAtA"]);
    assert_has_stri("mAtf", &["mAtA"]);
    // TODO: others
}

#[test]
fn sutra_4_1_45() {
    assert_has_stri("bahu", &["bahvI", "bahuH"]);
}

#[test]
fn sutra_4_1_49() {
    assert_has_stri("indra", &["indrARI"]);
    assert_has_stri("varuRa", &["varuRAnI"]);
    assert_has_stri("Bava", &["BavAnI"]);
    assert_has_stri("Sarva", &["SarvARI"]);
    assert_has_stri("rudra", &["rudrARI"]);
    assert_has_stri("mfqa", &["mfqAnI"]);
    assert_has_stri("hima", &["himAnI"]);
    assert_has_stri("araRya", &["araRyAnI"]);
    assert_has_stri("yava", &["yavAnI"]);
    assert_has_stri("yavana", &["yavanAnI"]);
    assert_has_stri("mAtula", &["mAtulAnI"]);

    // TODO: others;
}

#[test]
fn sutra_4_1_146() {
    assert_has_taddhitanta(&prati("revatI"), Taddhita::Wak, &["rEvatika"]);
    assert_has_taddhitanta(&prati("aSvapAlI"), Taddhita::Wak, &["ASvapAlika"]);
}
