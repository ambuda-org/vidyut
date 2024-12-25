extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::{BaseKrt as Krt, Unadi};

#[test]
fn sk_443() {
    assert_has_sup_3d("ahan", Napumsaka, &["ahoByAm"]);
    assert_has_sup_3p("ahan", Napumsaka, &["ahoBiH"]);

    // TODO: extra ahan forms

    assert_has_sup_1s("daRqin", Napumsaka, &["daRqi"]);
    assert_has_sup_1d("daRqin", Napumsaka, &["daRqinI"]);
    assert_has_sup_1p("daRqin", Napumsaka, &["daRqIni"]);

    assert_has_sup_1s("sragvin", Napumsaka, &["sragvi"]);
    assert_has_sup_1d("sragvin", Napumsaka, &["sragviRI"]);
    assert_has_sup_1p("sragvin", Napumsaka, &["sragvIRi"]);

    assert_has_sup_1s("vAgmin", Napumsaka, &["vAgmi"]);
    assert_has_sup_1d("vAgmin", Napumsaka, &["vAgminI"]);
    assert_has_sup_1p("vAgmin", Napumsaka, &["vAgmIni"]);

    assert_has_sup_1p("bahuvftrahan", Napumsaka, &["bahuvftrahARi"]);
    assert_has_sup_1p("bahupUzan", Napumsaka, &["bahupUzARi"]);
    assert_has_sup_1p("bahvaryaman", Napumsaka, &["bahvaryamARi"]);

    // TODO: others in this section: asfj, etc.

    assert_has_sup_1s("tyad", Napumsaka, &["tyat"]);
    assert_has_sup_1d("tyad", Napumsaka, &["tye"]);
    assert_has_sup_1p("tyad", Napumsaka, &["tyAni"]);

    assert_has_sup_1s("tad", Napumsaka, &["tat"]);
    assert_has_sup_1d("tad", Napumsaka, &["te"]);
    assert_has_sup_1p("tad", Napumsaka, &["tAni"]);

    assert_has_sup_1s("yad", Napumsaka, &["yat"]);
    assert_has_sup_1d("yad", Napumsaka, &["ye"]);
    assert_has_sup_1p("yad", Napumsaka, &["yAni"]);

    // TODO: not sure how to derive enat.

    assert_has_sup_1s("etad", Napumsaka, &["etat"]);
    assert_has_sup_1d("etad", Napumsaka, &["ete"]);
    assert_has_sup_1p("etad", Napumsaka, &["etAni"]);

    let bebhid = create_krdanta("beBid", &[], &yan(&d("Bi\\di~^r", Rudhadi)), Krt::kvip);
    assert_has_sup_1s(&bebhid, Napumsaka, &["beBit"]);
    assert_has_sup_1d(&bebhid, Napumsaka, &["beBidI"]);
    assert_has_sup_1p(&bebhid, Napumsaka, &["beBidi"]);

    let cecchid = create_krdanta("cecCid", &[], &yan(&d("Ci\\di~^r", Rudhadi)), Krt::kvip);
    assert_has_sup_1p(&cecchid, Napumsaka, &["cecCidi"]);

    assert_has_sup_1p("catur", Napumsaka, &["catvAri"]);

    // TODO: go + aYc and permutations

    assert_has_sup_1s("yakft", Napumsaka, &["yakft"]);
    assert_has_sup_1d("yakft", Napumsaka, &["yakftI"]);
    assert_has_sup_1p("yakft", Napumsaka, &["yakfnti"]);
    assert_has_sup_2p("yakft", Napumsaka, &["yakfnti", "yakAni"]);
    assert_has_sup_3s("yakft", Napumsaka, &["yaknA", "yakftA"]);

    assert_has_sup_1s("Sakft", Napumsaka, &["Sakft"]);
    assert_has_sup_1d("Sakft", Napumsaka, &["SakftI"]);
    assert_has_sup_1p("Sakft", Napumsaka, &["Sakfnti"]);
    assert_has_sup_2p("Sakft", Napumsaka, &["Sakfnti", "SakAni"]);
    assert_has_sup_3s("Sakft", Napumsaka, &["SaknA", "SakftA"]);

    let dadat = create_krdanta("dadat", &[], &d("qudA\\Y", Juhotyadi), Krt::Satf);
    assert_has_sup_1s(&dadat, Napumsaka, &["dadat"]);
    assert_has_sup_1d(&dadat, Napumsaka, &["dadatI"]);
}

#[test]
fn sk_444() {
    let dadat = create_krdanta("dadat", &[], &d("qudA\\Y", Juhotyadi), Krt::Satf);
    assert_has_sup_1p(&dadat, Napumsaka, &["dadanti", "dadati"]);

    let tudat = create_krdanta("tudat", &[], &d("tu\\da~^", Tudadi), Krt::Satf);
    assert_has_sup_1s(&tudat, Napumsaka, &["tudat"]);
}

#[test]
fn sk_445() {
    let tudat = create_krdanta("tudat", &[], &d("tu\\da~^", Tudadi), Krt::Satf);
    assert_has_sup_1d(&tudat, Napumsaka, &["tudatI", "tudantI"]);
    assert_has_sup_1p(&tudat, Napumsaka, &["tudanti"]);

    let bhaat = create_krdanta("BAt", &[], &d("BA\\", Adadi), Krt::Satf);
    assert_has_sup_1s(&bhaat, Napumsaka, &["BAt"]);
    assert_has_sup_1d(&bhaat, Napumsaka, &["BAntI", "BAtI"]);
    assert_has_sup_1p(&bhaat, Napumsaka, &["BAnti"]);

    let pacat = create_krdanta("pacat", &[], &d("qupa\\ca~^z", Bhvadi), Krt::Satf);
    assert_has_sup_1s(&pacat, Napumsaka, &["pacat"]);
}

#[test]
fn sk_446() {
    let pacat = create_krdanta("pacat", &[], &d("qupa\\ca~^z", Bhvadi), Krt::Satf);
    assert_has_sup_1d(&pacat, Napumsaka, &["pacantI"]);
    assert_has_sup_1p(&pacat, Napumsaka, &["pacanti"]);

    let divyat = create_krdanta("dIvyat", &[], &d("divu~", Divadi), Krt::Satf);
    assert_has_sup_1s(&divyat, Napumsaka, &["dIvyat"]);
    assert_has_sup_1d(&divyat, Napumsaka, &["dIvyantI"]);
    assert_has_sup_1p(&divyat, Napumsaka, &["dIvyanti"]);

    assert_has_sup_1s("svap", Napumsaka, &["svap"]);
    assert_has_sup_1d("svap", Napumsaka, &["svapI"]);
    assert_has_sup_1p("svap", Napumsaka, &["svAmpi", "svampi"]);
    assert_has_sup_3s("svap", Napumsaka, &["svapA"]);
    assert_has_sup_3d("svap", Napumsaka, &["svadByAm"]);
    assert_has_sup_3p("svap", Napumsaka, &["svadBiH"]);

    let dhanus = create_krdanta("Danus", &[], &d("Dana~", Juhotyadi), Unadi::usi);
    assert_has_sup_1s(&dhanus, Napumsaka, &["DanuH"]);
    assert_has_sup_1d(&dhanus, Napumsaka, &["DanuzI"]);
    assert_has_sup_1p(&dhanus, Napumsaka, &["DanUMzi"]);
    assert_has_sup_3s(&dhanus, Napumsaka, &["DanuzA"]);
    assert_has_sup_3d(&dhanus, Napumsaka, &["DanurByAm"]);

    // evaM cakzurhavirAdayaH
    let caksus = create_krdanta("cakzus", &[], &d("ca\\kzi~\\N", Adadi), Unadi::usi);
    assert_has_sup_1s(&caksus, Napumsaka, &["cakzuH"]);
    assert_has_sup_1d(&caksus, Napumsaka, &["cakzuzI"]);
    assert_has_sup_1p(&caksus, Napumsaka, &["cakzUMzi"]);
    assert_has_sup_3s(&caksus, Napumsaka, &["cakzuzA"]);
    assert_has_sup_3d(&caksus, Napumsaka, &["cakzurByAm"]);

    let havis = create_krdanta("havis", &[], &d("hu\\", Juhotyadi), Unadi::isi);
    assert_has_sup_1s(&havis, Napumsaka, &["haviH"]);
    assert_has_sup_1d(&havis, Napumsaka, &["havizI"]);
    assert_has_sup_1p(&havis, Napumsaka, &["havIMzi"]);
    assert_has_sup_3s(&havis, Napumsaka, &["havizA"]);
    assert_has_sup_3d(&havis, Napumsaka, &["havirByAm"]);

    let pipathis = krdanta(&[], &san(&d("paWa~", Bhvadi)), Krt::kvip);
    assert_has_sup_1s(&pipathis, Napumsaka, &["pipaWIH"]);
    assert_has_sup_1d(&pipathis, Napumsaka, &["pipaWizI"]);
    assert_has_sup_1p(&pipathis, Napumsaka, &["pipaWizi"]);
    assert_has_sup_3d(&pipathis, Napumsaka, &["pipaWIrByAm"]);

    assert_has_sup_1s("payas", Napumsaka, &["payaH"]);
    assert_has_sup_1d("payas", Napumsaka, &["payasI"]);
    assert_has_sup_1p("payas", Napumsaka, &["payAMsi"]);
    assert_has_sup_3s("payas", Napumsaka, &["payasA"]);
    assert_has_sup_3d("payas", Napumsaka, &["payoByAm"]);

    let supums = bahuvrihi("su", "pums");
    assert_has_sup_1s(&supums, Napumsaka, &["supum"]);
    assert_has_sup_1d(&supums, Napumsaka, &["supuMsI"]);
    assert_has_sup_1p(&supums, Napumsaka, &["supumAMsi"]);

    assert_has_sup_1s("adas", Napumsaka, &["adaH"]);
    assert_has_sup_1d("adas", Napumsaka, &["amU"]);
    assert_has_sup_1p("adas", Napumsaka, &["amUni"]);
}
