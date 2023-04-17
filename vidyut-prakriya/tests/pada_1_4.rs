extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn prati(text: &str) -> Pratipadika {
    Pratipadika::builder().text(text).build().unwrap()
}

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
    assert_has_subantas_p(&stri("kumArI"), Stri, Prathama, Eka, &["kumArI"]);
    assert_has_subantas_p(&stri("gOrI"), Stri, Prathama, Eka, &["gOrI"]);
    assert_has_subantas("lakzmI", Stri, Prathama, Eka, &["lakzmIH"]);
    assert_has_subantas_p(&stri("SArNgaravI"), Stri, Prathama, Eka, &["SArNgaravI"]);
    // UkArAnta
    assert_has_subantas("brahmabanDU", Stri, Prathama, Eka, &["brahmabanDUH"]);
    assert_has_subantas("yavagU", Stri, Prathama, Eka, &["yavagUH"]);
    // yU?
    assert_has_subantas("mAtf", Stri, Caturthi, Eka, &["mAtre"]);
    assert_has_subantas("duhitf", Stri, Caturthi, Eka, &["duhitre"]);
    // stryAKyO
    assert_has_subantas("grAmaRI", Pum, Prathama, Eka, &["grAmaRIH"]);
    assert_has_subantas("senAnI", Pum, Prathama, Eka, &["senAnIH"]);
    assert_has_subantas("KalapU", Pum, Prathama, Eka, &["KalapUH"]);
}

#[ignore]
#[test]
fn sutra_1_4_6() {
    assert_has_subantas("kfti", Stri, Caturthi, Eka, &["kftyE", "kftaye"]);
    assert_has_subantas("Denu", Stri, Caturthi, Eka, &["DenvE", "Denave"]);
    assert_has_subantas_p(
        &dhatu_prati("SrI"),
        Stri,
        Caturthi,
        Eka,
        &["SriyE", "Sriye"],
    );
    assert_has_subantas("BrU", Stri, Caturthi, Eka, &["BruvE", "Bruve"]);
    // astrI
    assert_has_subantas("strI", Stri, Caturthi, Eka, &["striyE"]);
    // stryAKyO
    assert_has_subantas("agni", Pum, Caturthi, Eka, &["agnaye"]);
    assert_has_subantas("vAyu", Pum, Caturthi, Eka, &["vAyave"]);
    assert_has_subantas("BAnave", Pum, Caturthi, Eka, &["BAnave"]);
}

#[test]
fn sutra_1_4_7() {
    assert_has_subantas("agni", Pum, Caturthi, Eka, &["agnaye"]);
    assert_has_subantas("vAyu", Pum, Caturthi, Eka, &["vAyave"]);
    assert_has_subantas("kfti", Stri, Caturthi, Eka, &["kftyE", "kftaye"]);
    assert_has_subantas("Denu", Stri, Caturthi, Eka, &["DenvE", "Denave"]);
    // asaKi
    assert_has_subantas("saKi", Pum, Trtiya, Eka, &["saKyA"]);
    assert_has_subantas("saKi", Pum, Caturthi, Eka, &["saKye"]);
    assert_has_subantas("saKi", Pum, Panchami, Eka, &["saKyuH"]);
    assert_has_subantas("saKi", Pum, Saptami, Eka, &["saKyO"]);
}

#[test]
fn sutra_1_4_19() {
    assert_has_taddhitanta(&prati("udaSvit"), T::matup, &["udaSvitvat"]);
    assert_has_taddhitanta(&prati("vidyut"), T::matup, &["vidyutvat"]);
    assert_has_taddhitanta(&prati("payas"), T::vini, &["payasvin"]);
    assert_has_taddhitanta(&prati("yaSas"), T::vini, &["yaSasvin"]);
    // tasau?
    assert_has_taddhitanta(&prati("takzan"), T::matup, &["takzavat"]);
}

#[test]
fn sutra_1_4_80() {
    assert_has_lat(&["vi"], &d("liKa~", Tudadi), &["viliKati"]);
    assert_has_lat_p(&["tiras"], &d("qukf\\Y", Tanadi), &["tiraskaroti"]);
}
