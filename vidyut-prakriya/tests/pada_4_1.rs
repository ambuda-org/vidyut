extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

fn assert_has_pum(prati: &str, expected: &[&str]) {
    assert_has_subantas(prati, Pum, Prathama, Eka, expected);
}

fn assert_has_stri(prati: &str, expected: &[&str]) {
    assert_has_subantas(prati, Stri, Prathama, Eka, expected);
}

#[test]
fn sutra_4_1_2() {
    // NIp
    let kumari = Pratipadika::builder()
        .text("kumArI")
        .is_nyap(true)
        .build()
        .unwrap();
    assert_has_subantas_p(&kumari, Stri, Prathama, Eka, &["kumArI"]);
    assert_has_subantas_p(&kumari, Stri, Prathama, Dvi, &["kumAryO"]);
    assert_has_subantas_p(&kumari, Stri, Prathama, Bahu, &["kumAryaH"]);
    assert_has_subantas_p(&kumari, Stri, Dvitiya, Eka, &["kumArIm"]);
    assert_has_subantas_p(&kumari, Stri, Dvitiya, Dvi, &["kumAryO"]);
    assert_has_subantas_p(&kumari, Stri, Dvitiya, Bahu, &["kumArIH"]);
    assert_has_subantas_p(&kumari, Stri, Trtiya, Eka, &["kumAryA"]);
    assert_has_subantas_p(&kumari, Stri, Trtiya, Dvi, &["kumArIByAm"]);
    assert_has_subantas_p(&kumari, Stri, Trtiya, Bahu, &["kumArIBiH"]);
    assert_has_subantas_p(&kumari, Stri, Caturthi, Eka, &["kumAryE"]);
    assert_has_subantas_p(&kumari, Stri, Caturthi, Dvi, &["kumArIByAm"]);
    assert_has_subantas_p(&kumari, Stri, Caturthi, Bahu, &["kumArIByaH"]);
    assert_has_subantas_p(&kumari, Stri, Panchami, Eka, &["kumAryAH"]);
    assert_has_subantas_p(&kumari, Stri, Panchami, Dvi, &["kumArIByAm"]);
    assert_has_subantas_p(&kumari, Stri, Panchami, Bahu, &["kumArIByaH"]);
    assert_has_subantas_p(&kumari, Stri, Sasthi, Eka, &["kumAryAH"]);
    assert_has_subantas_p(&kumari, Stri, Sasthi, Dvi, &["kumAryoH"]);
    assert_has_subantas_p(&kumari, Stri, Sasthi, Bahu, &["kumArIRAm"]);
    assert_has_subantas_p(&kumari, Stri, Saptami, Eka, &["kumAryAm"]);
    assert_has_subantas_p(&kumari, Stri, Saptami, Dvi, &["kumAryoH"]);
    assert_has_subantas_p(&kumari, Stri, Saptami, Bahu, &["kumArIzu"]);

    // NIz
    let gauri = Pratipadika::builder()
        .text("gOrI")
        .is_nyap(true)
        .build()
        .unwrap();
    assert_has_subantas_p(&gauri, Stri, Prathama, Eka, &["gOrI"]);
    assert_has_subantas_p(&gauri, Stri, Prathama, Dvi, &["gOryO"]);
    assert_has_subantas_p(&gauri, Stri, Prathama, Bahu, &["gOryaH"]);
    assert_has_subantas_p(&gauri, Stri, Dvitiya, Eka, &["gOrIm"]);
    assert_has_subantas_p(&gauri, Stri, Dvitiya, Dvi, &["gOryO"]);
    assert_has_subantas_p(&gauri, Stri, Dvitiya, Bahu, &["gOrIH"]);
    assert_has_subantas_p(&gauri, Stri, Trtiya, Eka, &["gOryA"]);
    assert_has_subantas_p(&gauri, Stri, Trtiya, Dvi, &["gOrIByAm"]);
    assert_has_subantas_p(&gauri, Stri, Trtiya, Bahu, &["gOrIBiH"]);
    assert_has_subantas_p(&gauri, Stri, Caturthi, Eka, &["gOryE"]);
    assert_has_subantas_p(&gauri, Stri, Caturthi, Dvi, &["gOrIByAm"]);
    assert_has_subantas_p(&gauri, Stri, Caturthi, Bahu, &["gOrIByaH"]);
    assert_has_subantas_p(&gauri, Stri, Panchami, Eka, &["gOryAH"]);
    assert_has_subantas_p(&gauri, Stri, Panchami, Dvi, &["gOrIByAm"]);
    assert_has_subantas_p(&gauri, Stri, Panchami, Bahu, &["gOrIByaH"]);
    assert_has_subantas_p(&gauri, Stri, Sasthi, Eka, &["gOryAH"]);
    assert_has_subantas_p(&gauri, Stri, Sasthi, Dvi, &["gOryoH"]);
    assert_has_subantas_p(&gauri, Stri, Sasthi, Bahu, &["gOrIRAm"]);
    assert_has_subantas_p(&gauri, Stri, Saptami, Eka, &["gOryAm"]);
    assert_has_subantas_p(&gauri, Stri, Saptami, Dvi, &["gOryoH"]);
    assert_has_subantas_p(&gauri, Stri, Saptami, Bahu, &["gOrIzu"]);

    // NIn
    let sharngaravi = Pratipadika::builder()
        .text("SArNgaravI")
        .is_nyap(true)
        .build()
        .unwrap();
    assert_has_subantas_p(&sharngaravi, Stri, Prathama, Eka, &["SArNgaravI"]);
    assert_has_subantas_p(&sharngaravi, Stri, Prathama, Dvi, &["SArNgaravyO"]);
    assert_has_subantas_p(&sharngaravi, Stri, Prathama, Bahu, &["SArNgaravyaH"]);
    assert_has_subantas_p(&sharngaravi, Stri, Dvitiya, Eka, &["SArNgaravIm"]);
    assert_has_subantas_p(&sharngaravi, Stri, Dvitiya, Dvi, &["SArNgaravyO"]);
    assert_has_subantas_p(&sharngaravi, Stri, Dvitiya, Bahu, &["SArNgaravIH"]);
    assert_has_subantas_p(&sharngaravi, Stri, Trtiya, Eka, &["SArNgaravyA"]);
    assert_has_subantas_p(&sharngaravi, Stri, Trtiya, Dvi, &["SArNgaravIByAm"]);
    assert_has_subantas_p(&sharngaravi, Stri, Trtiya, Bahu, &["SArNgaravIBiH"]);
    assert_has_subantas_p(&sharngaravi, Stri, Caturthi, Eka, &["SArNgaravyE"]);
    assert_has_subantas_p(&sharngaravi, Stri, Caturthi, Dvi, &["SArNgaravIByAm"]);
    assert_has_subantas_p(&sharngaravi, Stri, Caturthi, Bahu, &["SArNgaravIByaH"]);
    assert_has_subantas_p(&sharngaravi, Stri, Panchami, Eka, &["SArNgaravyAH"]);
    assert_has_subantas_p(&sharngaravi, Stri, Panchami, Dvi, &["SArNgaravIByAm"]);
    assert_has_subantas_p(&sharngaravi, Stri, Panchami, Bahu, &["SArNgaravIByaH"]);
    assert_has_subantas_p(&sharngaravi, Stri, Sasthi, Eka, &["SArNgaravyAH"]);
    assert_has_subantas_p(&sharngaravi, Stri, Sasthi, Dvi, &["SArNgaravyoH"]);
    assert_has_subantas_p(&sharngaravi, Stri, Sasthi, Bahu, &["SArNgaravIRAm"]);
    assert_has_subantas_p(&sharngaravi, Stri, Saptami, Eka, &["SArNgaravyAm"]);
    assert_has_subantas_p(&sharngaravi, Stri, Saptami, Dvi, &["SArNgaravyoH"]);
    assert_has_subantas_p(&sharngaravi, Stri, Saptami, Bahu, &["SArNgaravIzu"]);

    // wAp
    let khatva = Pratipadika::builder()
        .text("KawvA")
        .is_nyap(true)
        .build()
        .unwrap();
    assert_has_subantas_p(&khatva, Stri, Prathama, Eka, &["KawvA"]);
    assert_has_subantas_p(&khatva, Stri, Prathama, Dvi, &["Kawve"]);
    assert_has_subantas_p(&khatva, Stri, Prathama, Bahu, &["KawvAH"]);
    assert_has_subantas_p(&khatva, Stri, Dvitiya, Eka, &["KawvAm"]);
    assert_has_subantas_p(&khatva, Stri, Dvitiya, Dvi, &["Kawve"]);
    assert_has_subantas_p(&khatva, Stri, Dvitiya, Bahu, &["KawvAH"]);
    assert_has_subantas_p(&khatva, Stri, Trtiya, Eka, &["KawvayA"]);
    assert_has_subantas_p(&khatva, Stri, Trtiya, Dvi, &["KawvAByAm"]);
    assert_has_subantas_p(&khatva, Stri, Trtiya, Bahu, &["KawvABiH"]);
    assert_has_subantas_p(&khatva, Stri, Caturthi, Eka, &["KawvAyE"]);
    assert_has_subantas_p(&khatva, Stri, Caturthi, Dvi, &["KawvAByAm"]);
    assert_has_subantas_p(&khatva, Stri, Caturthi, Bahu, &["KawvAByaH"]);
    assert_has_subantas_p(&khatva, Stri, Panchami, Eka, &["KawvAyAH"]);
    assert_has_subantas_p(&khatva, Stri, Panchami, Dvi, &["KawvAByAm"]);
    assert_has_subantas_p(&khatva, Stri, Panchami, Bahu, &["KawvAByaH"]);
    assert_has_subantas_p(&khatva, Stri, Sasthi, Eka, &["KawvAyAH"]);
    assert_has_subantas_p(&khatva, Stri, Sasthi, Dvi, &["KawvayoH"]);
    assert_has_subantas_p(&khatva, Stri, Sasthi, Bahu, &["KawvAnAm"]);
    assert_has_subantas_p(&khatva, Stri, Saptami, Eka, &["KawvAyAm"]);
    assert_has_subantas_p(&khatva, Stri, Saptami, Dvi, &["KawvayoH"]);
    assert_has_subantas_p(&khatva, Stri, Saptami, Bahu, &["KawvAsu"]);

    // qAp
    let bahuraja = Pratipadika::builder()
        .text("bahurAjA")
        .is_nyap(true)
        .build()
        .unwrap();
    assert_has_subantas_p(&bahuraja, Stri, Prathama, Eka, &["bahurAjA"]);
    assert_has_subantas_p(&bahuraja, Stri, Prathama, Dvi, &["bahurAje"]);
    assert_has_subantas_p(&bahuraja, Stri, Prathama, Bahu, &["bahurAjAH"]);
    assert_has_subantas_p(&bahuraja, Stri, Dvitiya, Eka, &["bahurAjAm"]);
    assert_has_subantas_p(&bahuraja, Stri, Dvitiya, Dvi, &["bahurAje"]);
    assert_has_subantas_p(&bahuraja, Stri, Dvitiya, Bahu, &["bahurAjAH"]);
    assert_has_subantas_p(&bahuraja, Stri, Trtiya, Eka, &["bahurAjayA"]);
    assert_has_subantas_p(&bahuraja, Stri, Trtiya, Dvi, &["bahurAjAByAm"]);
    assert_has_subantas_p(&bahuraja, Stri, Trtiya, Bahu, &["bahurAjABiH"]);
    assert_has_subantas_p(&bahuraja, Stri, Caturthi, Eka, &["bahurAjAyE"]);
    assert_has_subantas_p(&bahuraja, Stri, Caturthi, Dvi, &["bahurAjAByAm"]);
    assert_has_subantas_p(&bahuraja, Stri, Caturthi, Bahu, &["bahurAjAByaH"]);
    assert_has_subantas_p(&bahuraja, Stri, Panchami, Eka, &["bahurAjAyAH"]);
    assert_has_subantas_p(&bahuraja, Stri, Panchami, Dvi, &["bahurAjAByAm"]);
    assert_has_subantas_p(&bahuraja, Stri, Panchami, Bahu, &["bahurAjAByaH"]);
    assert_has_subantas_p(&bahuraja, Stri, Sasthi, Eka, &["bahurAjAyAH"]);
    assert_has_subantas_p(&bahuraja, Stri, Sasthi, Dvi, &["bahurAjayoH"]);
    assert_has_subantas_p(&bahuraja, Stri, Sasthi, Bahu, &["bahurAjAnAm"]);
    assert_has_subantas_p(&bahuraja, Stri, Saptami, Eka, &["bahurAjAyAm"]);
    assert_has_subantas_p(&bahuraja, Stri, Saptami, Dvi, &["bahurAjayoH"]);
    assert_has_subantas_p(&bahuraja, Stri, Saptami, Bahu, &["bahurAjAsu"]);

    // cAp
    let karishagandhya = Pratipadika::builder()
        .text("kArIzaganDyA")
        .is_nyap(true)
        .build()
        .unwrap();
    assert_has_subantas_p(&karishagandhya, Stri, Prathama, Eka, &["kArIzaganDyA"]);
    assert_has_subantas_p(&karishagandhya, Stri, Prathama, Dvi, &["kArIzaganDye"]);
    assert_has_subantas_p(&karishagandhya, Stri, Prathama, Bahu, &["kArIzaganDyAH"]);
    assert_has_subantas_p(&karishagandhya, Stri, Dvitiya, Eka, &["kArIzaganDyAm"]);
    assert_has_subantas_p(&karishagandhya, Stri, Dvitiya, Dvi, &["kArIzaganDye"]);
    assert_has_subantas_p(&karishagandhya, Stri, Dvitiya, Bahu, &["kArIzaganDyAH"]);
    assert_has_subantas_p(&karishagandhya, Stri, Trtiya, Eka, &["kArIzaganDyayA"]);
    assert_has_subantas_p(&karishagandhya, Stri, Trtiya, Dvi, &["kArIzaganDyAByAm"]);
    assert_has_subantas_p(&karishagandhya, Stri, Trtiya, Bahu, &["kArIzaganDyABiH"]);
    assert_has_subantas_p(&karishagandhya, Stri, Caturthi, Eka, &["kArIzaganDyAyE"]);
    assert_has_subantas_p(&karishagandhya, Stri, Caturthi, Dvi, &["kArIzaganDyAByAm"]);
    assert_has_subantas_p(&karishagandhya, Stri, Caturthi, Bahu, &["kArIzaganDyAByaH"]);
    assert_has_subantas_p(&karishagandhya, Stri, Panchami, Eka, &["kArIzaganDyAyAH"]);
    assert_has_subantas_p(&karishagandhya, Stri, Panchami, Dvi, &["kArIzaganDyAByAm"]);
    assert_has_subantas_p(&karishagandhya, Stri, Panchami, Bahu, &["kArIzaganDyAByaH"]);
    assert_has_subantas_p(&karishagandhya, Stri, Sasthi, Eka, &["kArIzaganDyAyAH"]);
    assert_has_subantas_p(&karishagandhya, Stri, Sasthi, Dvi, &["kArIzaganDyayoH"]);
    assert_has_subantas_p(&karishagandhya, Stri, Sasthi, Bahu, &["kArIzaganDyAnAm"]);
    assert_has_subantas_p(&karishagandhya, Stri, Saptami, Eka, &["kArIzaganDyAyAm"]);
    assert_has_subantas_p(&karishagandhya, Stri, Saptami, Dvi, &["kArIzaganDyayoH"]);
    assert_has_subantas_p(&karishagandhya, Stri, Saptami, Bahu, &["kArIzaganDyAsu"]);

    let drshad = Pratipadika::new("dfzad");
    assert_has_subantas_p(&drshad, Pum, Prathama, Eka, &["dfzat"]);
    assert_has_subantas_p(&drshad, Pum, Prathama, Dvi, &["dfzadO"]);
    assert_has_subantas_p(&drshad, Pum, Prathama, Bahu, &["dfzadaH"]);
    assert_has_subantas_p(&drshad, Pum, Dvitiya, Eka, &["dfzadam"]);
    assert_has_subantas_p(&drshad, Pum, Dvitiya, Dvi, &["dfzadO"]);
    assert_has_subantas_p(&drshad, Pum, Dvitiya, Bahu, &["dfzadaH"]);
    assert_has_subantas_p(&drshad, Pum, Trtiya, Eka, &["dfzadA"]);
    assert_has_subantas_p(&drshad, Pum, Trtiya, Dvi, &["dfzadByAm"]);
    assert_has_subantas_p(&drshad, Pum, Trtiya, Bahu, &["dfzadBiH"]);
    assert_has_subantas_p(&drshad, Pum, Caturthi, Eka, &["dfzade"]);
    assert_has_subantas_p(&drshad, Pum, Caturthi, Dvi, &["dfzadByAm"]);
    assert_has_subantas_p(&drshad, Pum, Caturthi, Bahu, &["dfzadByaH"]);
    assert_has_subantas_p(&drshad, Pum, Panchami, Eka, &["dfzadaH"]);
    assert_has_subantas_p(&drshad, Pum, Panchami, Dvi, &["dfzadByAm"]);
    assert_has_subantas_p(&drshad, Pum, Panchami, Bahu, &["dfzadByaH"]);
    assert_has_subantas_p(&drshad, Pum, Sasthi, Eka, &["dfzadaH"]);
    assert_has_subantas_p(&drshad, Pum, Sasthi, Dvi, &["dfzadoH"]);
    assert_has_subantas_p(&drshad, Pum, Sasthi, Bahu, &["dfzadAm"]);
    assert_has_subantas_p(&drshad, Pum, Saptami, Eka, &["dfzadi"]);
    assert_has_subantas_p(&drshad, Pum, Saptami, Dvi, &["dfzadoH"]);
    assert_has_subantas_p(&drshad, Pum, Saptami, Bahu, &["dfzatsu"]);
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
fn sutra_4_1_98() {
    assert_has_taddhitanta(&prati("kuYja"), T::cPaY, &["kOYjAyana"]);
    assert_has_taddhitanta(&prati("braDna"), T::cPaY, &["brADnAyana"]);
    // gotre?
    assert_has_taddhitanta(&prati("kuYja"), T::iY, &["kOYji"]);
}

#[test]
fn sutra_4_1_105() {
    assert_has_taddhitanta(&prati("garga"), T::yaY, &["gArgya"]);
    assert_has_taddhitanta(&prati("vatsa"), T::yaY, &["vAtsya"]);
}

#[test]
fn sutra_4_1_106() {
    assert_has_taddhitanta(&prati("maDu"), T::yaY, &["mADavya"]);
    assert_has_taddhitanta(&prati("maDu"), T::aR, &["mADava"]);
    assert_has_taddhitanta(&prati("baBru"), T::yaY, &["bABravya"]);
    assert_has_taddhitanta(&prati("baBru"), T::aR, &["bABrava"]);
    // TODO: bABravyAyaRI
}

#[test]
fn sutra_4_1_107() {
    assert_has_taddhitanta(&prati("kapi"), T::yaY, &["kApya"]);
    assert_has_taddhitanta(&prati("boDa"), T::yaY, &["bODya"]);
    // ANgirase
    assert_has_taddhitanta(&prati("kapi"), T::Qak, &["kApeya"]);
    assert_has_taddhitanta(&prati("boDa"), T::iY, &["bODi"]);
    // TODO: kApyAyanI
}

#[test]
fn sutra_4_1_108() {
    assert_has_taddhitanta(&prati("vataRqa"), T::yaY, &["vAtaRqya"]);
    assert_has_taddhitanta(&prati("vataRqa"), T::aR, &["vAtaRqa"]);
}

#[test]
fn sutra_4_1_110() {
    assert_has_taddhitanta(&prati("aSva"), T::PaY, &["ASvAyana"]);
    assert_has_taddhitanta(&prati("aSman"), T::PaY, &["ASmAyana"]);
}

#[test]
fn sutra_4_1_111() {
    assert_has_taddhitanta(&prati("Barga"), T::PaY, &["BArgAyaRa"]);
    assert_has_taddhitanta(&prati("Barga"), T::iY, &["BArgi"]);
}

#[test]
fn sutra_4_1_120() {
    assert_has_taddhitanta(&nyap("suparRA"), T::Qak, &["sOparReya"]);
    assert_has_taddhitanta(&nyap("vinatA"), T::Qak, &["vEnateya"]);
    assert_has_taddhitanta(&nyap("vAqavA"), T::Qak, &["vAqaveya"]);
}

#[test]
fn sutra_4_1_121() {
    assert_has_taddhitanta(&nyap("dattA"), T::Qak, &["dAtteya"]);
    assert_has_taddhitanta(&nyap("gopI"), T::Qak, &["gOpeya"]);
    // dvi-acaH?
    // TODO: is yamunA nyAbanta?
    assert_has_taddhitanta(&prati("yamunA"), T::Qak, &[]);
    assert_has_taddhitanta(&prati("yamunA"), T::aR, &["yAmuna"]);
}

#[test]
fn sutra_4_1_122() {
    assert_has_taddhitanta(&prati("atri"), T::Qak, &["Atreya"]);
    assert_has_taddhitanta(&prati("niDi"), T::Qak, &["nEDeya"]);
    // itaH?
    assert_has_taddhitanta(&prati("dakza"), T::Qak, &[]);
    assert_has_taddhitanta(&prati("dakza"), T::iY, &["dAkzi"]);
    assert_has_taddhitanta(&prati("plakza"), T::Qak, &[]);
    assert_has_taddhitanta(&prati("plakza"), T::iY, &["plAkzi"]);
    // TODO: an-iYaH
    // dvi-acaH?
    assert_has_taddhitanta(&prati("marIci"), T::Qak, &[]);
    assert_has_taddhitanta(&prati("marIci"), T::aR, &["mArIca"]);
}

#[test]
fn sutra_4_1_123() {
    assert_has_taddhitanta(&prati("SuBra"), T::Qak, &["SOBreya"]);
    assert_has_taddhitanta(&prati("vizwapura"), T::Qak, &["vEzwapureya"]);
}

#[test]
fn sutra_4_1_124() {
    assert_has_taddhitanta(&prati("vikarRa"), T::Qak, &["vEkarReya"]);
    assert_has_taddhitanta(&prati("kuzItaka"), T::Qak, &["kOzItakeya"]);
    assert_has_taddhitanta(&prati("vikarRa"), T::iY, &["vEkarRi"]);
    assert_has_taddhitanta(&prati("kuzItaka"), T::iY, &["kOzItaki"]);
}

#[ignore]
#[test]
fn sutra_4_1_125() {
    assert_has_taddhitanta(&prati("BrU"), T::Qak, &["BrOveya"]);
}

#[ignore]
#[test]
fn sutra_4_1_126() {
    assert_has_taddhitanta(&prati("kalyARI"), T::Qak, &["kalyARineya"]);
    assert_has_taddhitanta(&prati("suBagA"), T::Qak, &["sOBAgineya"]);
    assert_has_taddhitanta(&prati("durBagA"), T::Qak, &["dOrBAgineya"]);
}

#[ignore]
#[test]
fn sutra_4_1_127() {
    assert_has_taddhitanta(&nyap("kulawA"), T::Qak, &["kOlawineya", "kOlaweya"]);
    // TODO: Qrak
}

#[test]
fn sutra_4_1_128() {
    assert_has_taddhitanta(&prati("cawakA"), T::Erak, &["cAwakEra"]);
}

#[test]
fn sutra_4_1_128_v1() {
    assert_has_taddhitanta(&prati("cawaka"), T::Erak, &["cAwakEra"]);
}

#[test]
fn sutra_4_1_129() {
    assert_has_taddhitanta(&prati("goDA"), T::Qrak, &["gODera"]);
}

#[test]
fn sutra_4_1_130() {
    assert_has_taddhitanta(&prati("goDA"), T::Arak, &["gODAra"]);
}

#[test]
fn sutra_4_1_132() {
    assert_has_taddhitanta(&prati("pitfzvasf"), T::CaR, &["pEtfzvasrIya"]);
}

#[test]
fn sutra_4_1_133() {
    assert_has_taddhitanta(&prati("pitfzvasf"), T::Qak, &["pEtfzvaseya"]);
}

#[test]
fn sutra_4_1_134() {
    assert_has_taddhitanta(&prati("mAtfzvasf"), T::CaR, &["mAtfzvasrIya"]);
    assert_has_taddhitanta(&prati("mAtfzvasf"), T::Qak, &["mAtfzvaseya"]);
}

#[test]
fn sutra_4_1_136() {
    assert_has_taddhitanta(&prati("gfzwi"), T::QaY, &["gArzweya"]);
    assert_has_taddhitanta(&prati("hfzwi"), T::QaY, &["hArzweya"]);
}

#[ignore]
#[test]
fn sutra_4_1_137() {
    assert_has_taddhitanta(&prati("rAjan"), T::yat, &["rAjanya"]);
    assert_has_taddhitanta(&prati("SvaSura"), T::yat, &["SvaSurya"]);
}

#[test]
fn sutra_4_1_138() {
    assert_has_taddhitanta(&prati("kzatra"), T::Ga, &["kzatriya"]);
}

#[test]
fn sutra_4_1_139() {
    assert_has_taddhitanta(&prati("kula"), T::Ka, &["kulIna"]);
}

#[test]
fn sutra_4_1_146() {
    assert_has_taddhitanta(&prati("revatI"), T::Wak, &["rEvatika"]);
    assert_has_taddhitanta(&prati("aSvapAlI"), T::Wak, &["ASvapAlika"]);
}
