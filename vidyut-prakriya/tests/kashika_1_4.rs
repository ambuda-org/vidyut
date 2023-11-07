extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

fn dhatu_prati(text: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(text)
        .is_dhatu(true)
        .build()
        .unwrap()
}

#[test]
fn sutra_1_4_3() {
    // IkArAnta
    assert_has_sup_1s(&nyap("kumArI"), Stri, &["kumArI"]);
    assert_has_sup_1s(&nyap("gOrI"), Stri, &["gOrI"]);
    assert_has_sup_1s("lakzmI", Stri, &["lakzmIH"]);
    assert_has_sup_1s(&nyap("SArNgaravI"), Stri, &["SArNgaravI"]);
    // UkArAnta
    assert_has_sup_1s("brahmabanDU", Stri, &["brahmabanDUH"]);
    assert_has_sup_1s("yavagU", Stri, &["yavagUH"]);
    // yU?
    assert_has_sup_4s("mAtf", Stri, &["mAtre"]);
    assert_has_sup_4s("duhitf", Stri, &["duhitre"]);
    // stryAKyO
    assert_has_sup_1s("grAmaRI", Pum, &["grAmaRIH"]);
    assert_has_sup_1s("senAnI", Pum, &["senAnIH"]);
    assert_has_sup_1s("KalapU", Pum, &["KalapUH"]);
}

#[test]
fn sutra_1_4_4() {
    let shri = Pratipadika::builder()
        .text("SrI")
        .is_dhatu(true)
        .build()
        .unwrap();
    assert_has_sup_ss(&shri, Stri, &["SrIH"]);
    assert_has_sup_ss("BrU", Stri, &["BrUH"]);
    // astrI
    assert_has_sup_ss("strI", Stri, &["stri"]);
}

#[test]
fn sutra_1_4_5() {
    let shri = dhatu_prati("SrI");
    assert_has_sup_6p(&shri, Stri, &["SriyAm", "SrIRAm"]);
    assert_has_sup_6p("BrU", Stri, &["BruvAm", "BrURAm"]);
    // astrI
    assert_has_sup_6p("strI", Stri, &["strIRAm"]);
}

#[test]
fn sutra_1_4_6() {
    assert_has_sup_4s("kfti", Stri, &["kftyE", "kftaye"]);
    assert_has_sup_4s("Denu", Stri, &["DenvE", "Denave"]);
    assert_has_sup_4s(&dhatu_prati("SrI"), Stri, &["SriyE", "Sriye"]);
    assert_has_sup_4s("BrU", Stri, &["BruvE", "Bruve"]);
    // astrI
    assert_has_sup_4s("strI", Stri, &["striyE"]);
    // stryAKyO
    assert_has_sup_4s("agni", Pum, &["agnaye"]);
    assert_has_sup_4s("vAyu", Pum, &["vAyave"]);
    assert_has_sup_4s("Banu", Pum, &["Banave"]);
}

#[test]
fn sutra_1_4_7() {
    assert_has_sup_4s("agni", Pum, &["agnaye"]);
    assert_has_sup_4s("vAyu", Pum, &["vAyave"]);
    assert_has_sup_4s("kfti", Stri, &["kftyE", "kftaye"]);
    assert_has_sup_4s("Denu", Stri, &["DenvE", "Denave"]);
    // asaKi
    assert_has_sup_3s("saKi", Pum, &["saKyA"]);
    assert_has_sup_4s("saKi", Pum, &["saKye"]);
    assert_has_sup_5s("saKi", Pum, &["saKyuH"]);
    assert_has_sup_7s("saKi", Pum, &["saKyO"]);
}

#[test]
fn sutra_1_4_8() {
    assert_has_sup_3s("pati", Pum, &["patyA"]);
    assert_has_sup_4s("pati", Pum, &["patye"]);
    assert_has_sup_5s("pati", Pum, &["patyuH"]);
    assert_has_sup_6s("pati", Pum, &["patyuH"]);
    assert_has_sup_7s("pati", Pum, &["patyO"]);
    // TODO: test when in samasa.
}

#[test]
fn sutra_1_4_10() {
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &d("Ci\\di~^r", Rudhadi), Krt::tfc, &["Cettf"]);
    assert_has_tip(&[], &nic(&d("qukf\\Y", Tanadi)), Lun, &["acIkarat"]);
    assert_has_tip(&[], &nic(&d("hf\\Y", Bhvadi)), Lun, &["ajIharat"]);
}

#[test]
fn sutra_1_4_11() {
    assert_has_krdanta(&[], &d("kuqi~\\", Bhvadi), Krt::a, &["kuRqA"]);
    assert_has_krdanta(&[], &d("huqi~\\", Bhvadi), Krt::a, &["huRqA"]);
    assert_has_krdanta(&[], &d("Sikza~\\", Bhvadi), Krt::a, &["SikzA"]);
    assert_has_krdanta(&[], &d("Bikza~\\", Bhvadi), Krt::a, &["BikzA"]);
}

#[test]
fn sutra_1_4_13() {
    let kf = d("qukf\\Y", Tanadi);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &hf, Krt::tfc, &["hartf"]);
    assert_has_tip(&[], &kf, Lrt, &["karizyati"]);
    assert_has_tip(&[], &hf, Lrt, &["harizyati"]);
    assert_has_tip(&[], &kf, Lrn, &["akarizyat"]);
    assert_has_taddhitanta(&prati("upagu"), T::aR, &["Opagava"]);
    assert_has_taddhitanta(&prati("kapawu"), T::aR, &["kApawava"]);

    // pratyayagrahana
    assert_has_lan(&["ni"], &d("vi\\Sa~", Tudadi), &["nyaviSata"]);
    assert_has_lan(&["vi"], &d("qukrI\\Y", Kryadi), &["vyakrIRIta"]);

    assert_has_vas(&[], &kf, Lrt, &["karizyAvaH"]);
    assert_has_mas(&[], &kf, Lrt, &["karizyAmaH"]);
    assert_has_sup_1p("kuRqa", Napumsaka, &["kuRqAni"]);
    // TODO: others;
}

#[test]
fn sutra_1_4_14() {
    assert_has_sup_1p("brAhmaRa", Pum, &["brAhmaRAH"]);
}

#[test]
fn sutra_1_4_16() {
    assert_has_taddhitanta(&prati("Bavat"), T::Cas, &["BavadIya"]);
    assert_has_taddhitanta(&prati("UrRA"), T::yus, &["UrRAyu"]);
    assert_has_taddhitanta(&prati("ftu"), T::Gas, &["ftviya"]);
}

#[test]
fn sutra_1_4_19() {
    assert_has_taddhitanta(&prati("udaSvit"), T::matup, &["udaSvitvat"]);
    assert_has_taddhitanta(&prati("vidyut"), T::matup, &["vidyutvat"]);
    assert_has_taddhitanta(&prati("payas"), T::vini, &["payasvin"]);
    assert_has_taddhitanta(&prati("yaSas"), T::vini, &["yaSasvin"]);
    // tasau
    assert_has_taddhitanta(&prati("takzan"), T::matup, &["takzavat"]);
}

#[test]
fn sutra_1_4_21() {
    assert_has_sup_1p("brAhmaRa", Pum, &["brAhmaRAH"]);
    assert_has_jhi(&[], &d("paWa~", Bhvadi), Lat, &["paWanti"]);
}

#[test]
fn sutra_1_4_22() {
    let path = d("paWa~", Bhvadi);
    assert_has_sup_1d("brAhmaRa", Pum, &["brAhmaRO"]);
    assert_has_tas(&[], &path, Lat, &["paWataH"]);

    assert_has_sup_1s("brAhmaRa", Pum, &["brAhmaRaH"]);
    assert_has_tip(&[], &path, Lat, &["paWati"]);
}

#[test]
fn sutra_1_4_59() {
    let ni = d("RI\\Y", Bhvadi);
    assert_has_tip(&["pra"], &ni, Lat, &["praRayati"]);
    assert_has_tip(&["pari"], &ni, Lat, &["pariRayati"]);
    assert_has_krdanta(&["pra"], &ni, Krt::Rvul, &["praRAyaka"]);
    assert_has_krdanta(&["pari"], &ni, Krt::Rvul, &["pariRAyaka"]);
}

#[test]
fn sutra_1_4_80() {
    assert_has_lat(&["vi"], &d("liKa~", Tudadi), &["viliKati"]);
    assert_has_tip(&["tiras"], &d("qukf\\Y", Tanadi), Lat, &["tiraskaroti"]);
}

#[test]
fn sutra_1_4_105() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_sip(&[], &pac, Lat, &["pacasi"]);
    assert_has_thas(&[], &pac, Lat, &["pacaTaH"]);
    assert_has_tha(&[], &pac, Lat, &["pacaTa"]);
}

#[test]
fn sutra_1_4_107() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_mip(&[], &pac, Lat, &["pacAmi"]);
    assert_has_vas(&[], &pac, Lat, &["pacAvaH"]);
    assert_has_mas(&[], &pac, Lat, &["pacAmaH"]);
}

#[test]
fn sutra_1_4_108() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lat, &["pacati"]);
    assert_has_tas(&[], &pac, Lat, &["pacataH"]);
    assert_has_jhi(&[], &pac, Lat, &["pacanti"]);
}

#[test]
fn sutra_1_4_109() {
    assert_has_sandhi("daDi", "atra", &["daDyatra"]);
    assert_has_sandhi("maDu", "atra", &["maDvatra"]);
}
