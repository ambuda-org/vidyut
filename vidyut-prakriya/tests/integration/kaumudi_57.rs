extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::{Dhatu, Pratipadika, Sanadi};

fn p(text: &str) -> Pratipadika {
    phit(text)
}

fn kyac(pratipadika: Pratipadika) -> Dhatu {
    Dhatu::nama(pratipadika, Some(Sanadi::kyac))
}

fn kamyac(pratipadika: Pratipadika) -> Dhatu {
    Dhatu::nama(pratipadika, Some(Sanadi::kAmyac))
}

fn nama(pratipadika: Pratipadika) -> Dhatu {
    Dhatu::nama(pratipadika, None)
}

#[test]
fn skip_sk_2657() {}

#[test]
fn sk_2658() {
    assert_has_tip(&[], &kyac(p("putra")), Lat, &["putrIyati"]);
    assert_has_tip(&[], &kyac(p("go")), Lat, &["gavyati"]);
    assert_has_tip(&[], &kyac(p("nO")), Lat, &["nAvyati"]);
}

#[ignore]
#[test]
fn sk_2659() {
    let gavya = kyac(p("go"));
    assert_has_tip(
        &[],
        &gavya,
        Lit,
        &["gavyAYcakAra", "gavyAmAsa", "gavyAmbaBUva"],
    );
    assert_has_tip(&[], &gavya, Lut, &["gavyitA"]);

    let navya = kyac(p("nO"));
    assert_has_tip(
        &[],
        &navya,
        Lit,
        &["nAvyAYcakAra", "nAvyAmAsa", "nAvyAmbaBUva"],
    );
    assert_has_tip(&[], &navya, Lut, &["nAvyitA"]);

    assert_has_tip(&[], &kyac(p("rAjan")), Lat, &["rAjIyati"]);
    assert_has_tip(&[], &kyac(p("tvad")), Lat, &["tvadyati"]);
    assert_has_tip(&[], &kyac(p("mad")), Lat, &["madyati"]);
    assert_has_tip(&[], &kyac(p("yuzmad")), Lat, &["yuzmadyati"]);
    assert_has_tip(&[], &kyac(p("asmad")), Lat, &["asmadyati"]);

    assert_has_tip(&[], &kyac(p("gir")), Lat, &["gIryati"]);
    assert_has_tip(&[], &kyac(p("pur")), Lat, &["pUryati"]);
    assert_has_tip(&[], &kyac(p("div")), Lat, &["divyati"]);
    assert_has_tip(&[], &kyac(p("adas")), Lat, &["adasyati"]);
    assert_has_tip(&[], &kyac(p("kartf")), Lat, &["kartrIyati"]);
    // assert_has_tip(&[], &kyac(p("kartf")), Lat, &["gargIyati"]);
    assert_has_tip(&[], &kyac(p("vAc")), Lat, &["vAcyati"]);
    assert_has_tip(&[], &kyac(p("kavi")), Lat, &["kavIyati"]);
    assert_has_tip(&[], &kyac(p("samiD")), Lat, &["samiDyati"]);
}

#[test]
fn sk_2660() {
    let samidhya = &kyac(p("samiD"));
    assert_has_tip(&[], &samidhya, Lut, &["samiDitA", "samiDyitA"]);
}

#[test]
fn sk_2663() {
    assert_has_tip(&[], &kamyac(p("putra")), Lat, &["putrakAmyati"]);
    assert_has_tip(&[], &kamyac(p("yaSas")), Lat, &["yaSaskAmyati"]);
    assert_has_tip(&[], &kamyac(p("sarpis")), Lat, &["sarpizkAmyati"]);
    assert_has_tip(&[], &kamyac(p("kim")), Lat, &["kiNkAmyati"]);
    assert_has_tip(&[], &kamyac(p("svar")), Lat, &["svaHkAmyati"]);
}

#[test]
fn sk_2664() {
    assert_has_tip(&[], &kyac(p("putra")), Lat, &["putrIyati"]);
    assert_has_tip(&[], &kyac(p("vizRu")), Lat, &["vizRUyati"]);
    assert_has_tip(&[], &kyac(p("prAsAda")), Lat, &["prAsAdIyati"]);
    assert_has_tip(&[], &kyac(p("kuwI")), Lat, &["kuwIyati"]);
}

#[ignore]
#[test]
fn sk_2665() {
    assert_has_ta(&[], &kyan(p("kfzRa")), Lat, &["kfzRAyate"]);
    assert_has_ta(&[], &kyan(p("ojas")), Lat, &["ojAyate"]);
    assert_has_ta(&[], &kyan(p("apsaras")), Lat, &["apsarAyate"]);
    assert_has_ta(&[], &kyan(p("yaSas")), Lat, &["yaSAyate", "yaSasyate"]);
    assert_has_ta(&[], &kyan(p("vidvas")), Lat, &["vidvAyate", "vidvasyate"]);
    assert_has_ta(&[], &kyan(p("tvad")), Lat, &["tvadyate"]);
    assert_has_ta(&[], &kyan(p("mad")), Lat, &["madyate"]);
    assert_has_ta(&[], &kyan(p("yuzmad")), Lat, &["yuzmadyate"]);
    assert_has_ta(&[], &kyan(p("asmad")), Lat, &["asmadyate"]);

    // TODO: many, many more
}

#[test]
fn skip_sk_2668() {}

#[test]
fn sk_2670() {
    assert_has_ta(&[], &nama(p("kazwa")), Lat, &["kazwAyate"]);
    assert_has_ta(&[], &nama(p("satra")), Lat, &["satrAyate"]);
    assert_has_ta(&[], &nama(p("kakza")), Lat, &["kakzAyate"]);
}

#[test]
fn sk_2671() {
    assert_has_ta(&[], &nama(p("romanTa")), Lat, &["romanTAyate"]);
    assert_has_tip(&[], &nama(p("tapas")), Lat, &["tapasyati"]);
}

#[test]
fn sk_2672() {
    assert_has_ta(&[], &nama(p("bAzpa")), Lat, &["bAzpAyate"]);
    assert_has_ta(&[], &nama(p("uzma")), Lat, &["uzmAyate"]);
    assert_has_ta(&[], &nama(p("Pena")), Lat, &["PenAyate"]);
}

#[test]
fn sk_2673() {
    assert_has_ta(&[], &nama(p("Sabda")), Lat, &["SabdAyate"]);
    assert_has_ta(&[], &nama(p("sudina")), Lat, &["sudinAyate"]);
}

#[test]
fn skip_sk_2674() {}

#[test]
fn sk_2675() {
    assert_has_tip(&[], &nama(p("namas")), Lat, &["namasyati"]);
    assert_has_tip(&[], &nama(p("varivas")), Lat, &["varivasyati"]);
    assert_has_ta(&[], &nama(p("citra")), Lat, &["citrIyate"]);
}

#[test]
fn sk_2676() {
    let nama = |prati| Dhatu::nama(p(prati), None);
    assert_has_ta(&["ud"], &nama("pucCa"), Lat, &["utpucCayate"]);
    assert_has_ta(&["vi"], &nama("pucCa"), Lat, &["vipucCayate"]);
    assert_has_ta(&["pari"], &nama("pucCa"), Lat, &["paripucCayate"]);
    assert_has_ta(&["sam"], &nama("BARqa"), Lat, &["samBARqayate"]);
    assert_has_ta(&["sam"], &nama("cIvara"), Lat, &["saYcIvarayate"]);
}
