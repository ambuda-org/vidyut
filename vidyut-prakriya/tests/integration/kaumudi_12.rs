extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;

#[test]
fn sk_440() {
    let upanah = create_krdanta("upAnah", &["upa", "AN"], &d("Ra\\ha~^", Divadi), Krt::kvip);
    assert_has_sup_1s(&upanah, Stri, &["upAnat"]);
    assert_has_sup_1d(&upanah, Stri, &["upAnahO"]);
    assert_has_sup_1p(&upanah, Stri, &["upAnahaH"]);
    assert_has_sup_3d(&upanah, Stri, &["upAnadByAm"]);
    assert_has_sup_7p(&upanah, Stri, &["upAnatsu"]);

    let ushnih = create_krdanta("uzRih", &["ud"], &d("zRi\\ha~", Divadi), Krt::kvin);
    assert_has_sup_1s(&ushnih, Stri, &["uzRik"]);
    assert_has_sup_1d(&ushnih, Stri, &["uzRihO"]);
    assert_has_sup_1p(&ushnih, Stri, &["uzRihaH"]);
    assert_has_sup_3d(&ushnih, Stri, &["uzRigByAm"]);
    assert_has_sup_7p(&ushnih, Stri, &["uzRikzu"]);

    assert_has_sup_1s("div", Stri, &["dyOH"]);
    assert_has_sup_1d("div", Stri, &["divO"]);
    assert_has_sup_1p("div", Stri, &["divaH"]);
    assert_has_sup_7p("div", Stri, &["dyuzu"]);

    let gir = create_krdanta("gir", &[], &d("gF", Kryadi), Krt::kvip);
    assert_has_sup_1s(&gir, Stri, &["gIH"]);
    assert_has_sup_1d(&gir, Stri, &["girO"]);
    assert_has_sup_1p(&gir, Stri, &["giraH"]);

    let pur = create_krdanta("pur", &[], &d("pF", Kryadi), Krt::kvip);
    assert_has_sup_1s(&pur, Stri, &["pUH"]);

    assert_has_sup_1p("catur", Stri, &["catasraH"]);
    assert_has_sup_6p("catur", Stri, &["catasfRAm"]);

    assert_has_sup_1s("kim", Stri, &["kA"]);
    assert_has_sup_1d("kim", Stri, &["ke"]);
    assert_has_sup_1p("kim", Stri, &["kAH"]);
}

#[test]
fn sk_441() {
    assert_has_sup_1s("idam", Stri, &["iyam"]);
    assert_has_sup_1d("idam", Stri, &["ime"]);
    assert_has_sup_1p("idam", Stri, &["imAH"]);
    assert_has_sup_2s("idam", Stri, &["imAm"]);
    assert_has_sup_2d("idam", Stri, &["ime"]);
    assert_has_sup_2p("idam", Stri, &["imAH"]);
    assert_has_sup_3s("idam", Stri, &["anayA"]);
    assert_has_sup_3d("idam", Stri, &["AByAm"]);
    assert_has_sup_3p("idam", Stri, &["ABiH"]);
    assert_has_sup_4s("idam", Stri, &["asyE"]);
    assert_has_sup_5s("idam", Stri, &["asyAH"]);
    assert_has_sup_6d("idam", Stri, &["anayoH"]);
    assert_has_sup_6p("idam", Stri, &["AsAm"]);
    assert_has_sup_7s("idam", Stri, &["asyAm"]);
    assert_has_sup_7p("idam", Stri, &["Asu"]);

    assert_has_sup_1s("sraj", Stri, &["srak"]);
    assert_has_sup_1d("sraj", Stri, &["srajO"]);
    assert_has_sup_1p("sraj", Stri, &["srajaH"]);
    assert_has_sup_3d("sraj", Stri, &["sragByAm"]);
    assert_has_sup_7p("sraj", Stri, &["srakzu"]);

    assert_has_sup_1s("tyad", Stri, &["syA"]);
    assert_has_sup_1d("tyad", Stri, &["tye"]);
    assert_has_sup_1p("tyad", Stri, &["tyAH"]);

    // "evaM tad yad etad"
    assert_has_sup_1s("tad", Stri, &["sA"]);
    assert_has_sup_1d("tad", Stri, &["te"]);
    assert_has_sup_1p("tad", Stri, &["tAH"]);

    assert_has_sup_1s("yad", Stri, &["yA"]);
    assert_has_sup_1d("yad", Stri, &["ye"]);
    assert_has_sup_1p("yad", Stri, &["yAH"]);

    assert_has_sup_1s("etad", Stri, &["ezA"]);
    assert_has_sup_1d("etad", Stri, &["ete"]);
    assert_has_sup_1p("etad", Stri, &["etAH"]);

    assert_has_sup_1s("vAc", Stri, &["vAk"]);
    assert_has_sup_1d("vAc", Stri, &["vAcO"]);
    assert_has_sup_1p("vAc", Stri, &["vAcaH"]);
    assert_has_sup_3d("vAc", Stri, &["vAgByAm"]);
    assert_has_sup_7p("vAc", Stri, &["vAkzu"]);

    assert_has_sup_1p("ap", Stri, &["ApaH"]);
    assert_has_sup_2p("ap", Stri, &["apaH"]);
}

#[test]
fn sk_442() {
    assert_has_sup_3p("ap", Stri, &["adBiH"]);
    assert_has_sup_4p("ap", Stri, &["adByaH"]);
    assert_has_sup_6p("ap", Stri, &["apAm"]);
    assert_has_sup_7p("ap", Stri, &["apsu"]);

    let dish = create_krdanta("diS", &[], &d("di\\Sa~^", Tudadi), Krt::kvin);
    assert_has_sup_1s(&dish, Stri, &["dik"]);
    assert_has_sup_1d(&dish, Stri, &["diSO"]);
    assert_has_sup_1p(&dish, Stri, &["diSaH"]);
    assert_has_sup_3d(&dish, Stri, &["digByAm"]);
    assert_has_sup_7p(&dish, Stri, &["dikzu"]);

    let drsh = create_krdanta("dfS", &[], &d("df\\Si~r", Tudadi), Krt::kvin);
    assert_has_sup_1s(&drsh, Stri, &["dfk"]);
    assert_has_sup_1d(&drsh, Stri, &["dfSO"]);
    assert_has_sup_1p(&drsh, Stri, &["dfSaH"]);

    assert_has_sup_1s("tviz", Stri, &["tviw"]);
    assert_has_sup_1d("tviz", Stri, &["tvizO"]);
    assert_has_sup_1p("tviz", Stri, &["tvizaH"]);
    assert_has_sup_3d("tviz", Stri, &["tviqByAm"]);
    assert_has_sup_7p("tviz", Stri, &["tviwtsu", "tviwsu"]);

    let jush = create_krdanta("juz", &[], &d("juzI~\\", Tudadi), Krt::kvip);
    // HACK: technically "saha", but use "sa" for convenience.
    let sajush = bahuvrihi("sa", jush);
    assert_has_sup_1s(&sajush, Stri, &["sajUH"]);
    assert_has_sup_1d(&sajush, Stri, &["sajuzO"]);
    assert_has_sup_1p(&sajush, Stri, &["sajuzaH"]);
    assert_has_sup_3d(&sajush, Stri, &["sajUrByAm"]);
    assert_has_sup_7p(&sajush, Stri, &["sajUHzu", "sajUzzu"]);

    let ashis = create_krdanta("ASis", &["AN"], &d("SAsu~\\", Adadi), Krt::kvip);
    assert_has_sup_1s(&ashis, Stri, &["ASIH"]);
    assert_has_sup_1d(&ashis, Stri, &["ASizO"]);
    assert_has_sup_1p(&ashis, Stri, &["ASizaH"]);
    assert_has_sup_3d(&ashis, Stri, &["ASIrByAm"]);

    assert_has_sup_1s("adas", Stri, &["asO"]);
    assert_has_sup_1d("adas", Stri, &["amU"]);
    assert_has_sup_1p("adas", Stri, &["amUH"]);
    assert_has_sup_2s("adas", Stri, &["amUm"]);
    assert_has_sup_2d("adas", Stri, &["amU"]);
    assert_has_sup_2p("adas", Stri, &["amUH"]);
    assert_has_sup_3s("adas", Stri, &["amuyA"]);
    assert_has_sup_3d("adas", Stri, &["amUByAm"]);
    assert_has_sup_3p("adas", Stri, &["amUBiH"]);
    assert_has_sup_4s("adas", Stri, &["amuzyE"]);
    assert_has_sup_4d("adas", Stri, &["amUByAm"]);
    assert_has_sup_4p("adas", Stri, &["amUByaH"]);
    assert_has_sup_5s("adas", Stri, &["amuzyAH"]);
    assert_has_sup_6d("adas", Stri, &["amuyoH"]);
    assert_has_sup_6p("adas", Stri, &["amUzAm"]);
    assert_has_sup_7p("adas", Stri, &["amUzu"]);
}
