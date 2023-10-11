extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
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
    assert_has_subantas_p(&nyap("kumArI"), Stri, Prathama, Eka, &["kumArI"]);
    assert_has_subantas_p(&nyap("gOrI"), Stri, Prathama, Eka, &["gOrI"]);
    assert_has_subantas("lakzmI", Stri, Prathama, Eka, &["lakzmIH"]);
    assert_has_subantas_p(&nyap("SArNgaravI"), Stri, Prathama, Eka, &["SArNgaravI"]);
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

#[test]
fn sutra_1_4_4() {
    let shri = Pratipadika::builder()
        .text("SrI")
        .is_dhatu(true)
        .build()
        .unwrap();
    assert_has_subantas_p(&shri, Stri, Sambodhana, Eka, &["SrIH"]);
    assert_has_subantas("BrU", Stri, Sambodhana, Eka, &["BrUH"]);
    // astrI
    assert_has_subantas("strI", Stri, Sambodhana, Eka, &["stri"]);
}

#[test]
fn sutra_1_4_5() {
    let shri = dhatu_prati("SrI");
    assert_has_subantas_p(&shri, Stri, Sasthi, Bahu, &["SriyAm", "SrIRAm"]);
    assert_has_subantas("BrU", Stri, Sasthi, Bahu, &["BruvAm", "BrURAm"]);
    // astrI
    assert_has_subantas("strI", Stri, Sasthi, Bahu, &["strIRAm"]);
}

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
    assert_has_subantas("Banu", Pum, Caturthi, Eka, &["Banave"]);
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
fn sutra_1_4_8() {
    assert_has_subantas("pati", Pum, Trtiya, Eka, &["patyA"]);
    assert_has_subantas("pati", Pum, Caturthi, Eka, &["patye"]);
    assert_has_subantas("pati", Pum, Panchami, Eka, &["patyuH"]);
    assert_has_subantas("pati", Pum, Sasthi, Eka, &["patyuH"]);
    assert_has_subantas("pati", Pum, Saptami, Eka, &["patyO"]);
    // TODO: test when in samasa.
}

#[test]
fn sutra_1_4_10() {
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &d("Ci\\di~^r", Rudhadi), Krt::tfc, &["Cettf"]);
    assert_has_lun_p(&[], &nic(&d("qukf\\Y", Tanadi)), &["acIkarat"]);
    assert_has_lun_p(&[], &nic(&d("hf\\Y", Bhvadi)), &["ajIharat"]);
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
    assert_has_lrt_p(&[], &kf, &["karizyati"]);
    assert_has_lrt_p(&[], &hf, &["harizyati"]);
    assert_has_lrn_p(&[], &kf, &["akarizyat"]);
    assert_has_taddhitanta(&prati("upagu"), T::aR, &["Opagava"]);
    assert_has_taddhitanta(&prati("kapawu"), T::aR, &["kApawava"]);

    // pratyayagrahana
    assert_has_lan(&["ni"], &d("vi\\Sa~", Tudadi), &["nyaviSata"]);
    assert_has_lan(&["vi"], &d("qukrI\\Y", Kryadi), &["vyakrIRIta"]);

    assert_has_vas(&[], &kf, Lrt, &["karizyAvaH"]);
    assert_has_mas(&[], &kf, Lrt, &["karizyAmaH"]);
    assert_has_subantas("kuRqa", Napumsaka, Prathama, Bahu, &["kuRqAni"]);
    // TODO: others;
}

#[test]
fn sutra_1_4_14() {
    assert_has_subantas("brAhmaRa", Pum, Prathama, Bahu, &["brAhmaRAH"]);
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
    assert_has_subantas("brAhmaRa", Pum, Prathama, Bahu, &["brAhmaRAH"]);
    assert_has_jhi(&[], &d("paWa~", Bhvadi), Lat, &["paWanti"]);
}

#[test]
fn sutra_1_4_22() {
    let path = d("paWa~", Bhvadi);
    assert_has_subantas("brAhmaRa", Pum, Prathama, Dvi, &["brAhmaRO"]);
    assert_has_tas(&[], &path, Lat, &["paWataH"]);

    assert_has_subantas("brAhmaRa", Pum, Prathama, Eka, &["brAhmaRaH"]);
    assert_has_tip(&[], &path, Lat, &["paWati"]);
}

#[test]
fn sutra_1_4_59() {
    let ni = d("RI\\Y", Bhvadi);
    assert_has_lat_p(&["pra"], &ni, &["praRayati"]);
    assert_has_lat_p(&["pari"], &ni, &["pariRayati"]);
    assert_has_krdanta(&["pra"], &ni, Krt::Rvul, &["praRAyaka"]);
    assert_has_krdanta(&["pari"], &ni, Krt::Rvul, &["pariRAyaka"]);
}

#[test]
fn sutra_1_4_80() {
    assert_has_lat(&["vi"], &d("liKa~", Tudadi), &["viliKati"]);
    assert_has_lat_p(&["tiras"], &d("qukf\\Y", Tanadi), &["tiraskaroti"]);
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
