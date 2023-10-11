extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

fn assert_has_pum(prati: &str, expected: &[&str]) {
    assert_has_subantas(prati, Pum, Prathama, Eka, expected);
}

fn assert_has_stri(prati: &str, expected: &[&str]) {
    assert_has_subantas(prati, Stri, Prathama, Eka, expected);
}

fn assert_blocked(text: &str, artha: TaddhitaArtha, t: T) {
    assert_has_artha_taddhita(text, artha, t, &[]);
}

#[test]
fn sutra_4_1_2() {
    // NIp
    let kumari = nyap("kumArI");
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
    let gauri = nyap("gOrI");
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
    let sharngaravi = nyap("SArNgaravI");
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
    let khatva = nyap("KawvA");
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
    let bahuraja = nyap("bahurAjA");
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
    let karishagandhya = nyap("kArIzaganDyA");
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
fn sutra_4_1_83() {
    assert_has_artha_taddhita("upagu", TasyaApatyam, T::aR, &["Opagava"]);
    assert_has_artha_taddhita("kapawu", TasyaApatyam, T::aR, &["kApawava"]);
}

#[test]
fn sutra_4_1_84() {
    assert_has_artha_taddhita("aSvapati", TasyaApatyam, T::aR, &["ASvapata"]);
    assert_has_artha_taddhita("Satapati", TasyaApatyam, T::aR, &["SAtapata"]);
}

#[test]
fn sutra_4_1_85() {
    assert_has_artha_taddhita("diti", TasyaApatyam, T::Rya, &["dEtya"]);
    assert_has_artha_taddhita("aditi", TasyaApatyam, T::Rya, &["Aditya"]);
    assert_has_artha_taddhita("senApati", TasyaApatyam, T::Rya, &["sEnApatya"]);
}

#[test]
fn sutra_4_1_86() {
    assert_has_artha_taddhita("utsa", TasyaApatyam, T::aY, &["Otsa"]);
    assert_has_artha_taddhita("udapAna", TasyaApatyam, T::aY, &["OdapAna"]);
}

#[test]
fn sutra_4_1_87() {
    assert_has_artha_taddhita("strI", TasyaApatyam, T::naY, &["strERa"]);
    assert_has_artha_taddhita("pums", TasyaApatyam, T::snaY, &["pOMsna"]);
}

#[test]
fn sutra_4_1_92() {
    assert_has_artha_taddhita("upagu", TasyaApatyam, T::aR, &["Opagava"]);
    assert_has_artha_taddhita("aSvapati", TasyaApatyam, T::aR, &["ASvapata"]);
    assert_has_artha_taddhita("diti", TasyaApatyam, T::Rya, &["dEtya"]);
    assert_has_artha_taddhita("utsa", TasyaApatyam, T::aY, &["Otsa"]);
    assert_has_artha_taddhita("strI", TasyaApatyam, T::naY, &["strERa"]);
    assert_has_artha_taddhita("pums", TasyaApatyam, T::snaY, &["pOMsna"]);

    assert_has_artha_taddhita("BAnu", TasyaApatyam, T::aR, &["BAnava"]);
    assert_has_artha_taddhita("SyAmagu", TasyaApatyam, T::aR, &["SyAmagava"]);
}

#[test]
fn sutra_4_1_95() {
    assert_has_artha_taddhita("dakza", TasyaApatyam, T::iY, &["dAkzi"]);
    assert_blocked("dakza", TasyaApatyam, T::aR);
    // ataH?
    assert_has_artha_taddhita("SuBamyA", TasyaApatyam, T::iY, &[]);
    assert_has_artha_taddhita("kIlAlapA", TasyaApatyam, T::iY, &[]);
}

#[test]
fn sutra_4_1_96() {
    assert_has_artha_taddhita("bAhu", TasyaApatyam, T::iY, &["bAhavi"]);
    assert_has_artha_taddhita("upabAhu", TasyaApatyam, T::iY, &["OpabAhavi"]);
    // TODO: others
}

#[test]
fn sutra_4_1_97() {
    assert_has_artha_taddhita("suDAtf", TasyaApatyam, T::iY, &["sODAtaki"]);
}

#[test]
fn sutra_4_1_97_v1() {
    assert_has_artha_taddhita("vyAsa", TasyaApatyam, T::iY, &["vEyAsaki"]);
    assert_has_artha_taddhita("varuqa", TasyaApatyam, T::iY, &["vAruqaki"]);
    assert_has_artha_taddhita("nizAda", TasyaApatyam, T::iY, &["nEzAdaki"]);
    assert_has_artha_taddhita("caRqAla", TasyaApatyam, T::iY, &["cARqAlaki"]);
    assert_has_artha_taddhita("bimba", TasyaApatyam, T::iY, &["bEmbaki"]);
}

#[test]
fn sutra_4_1_98() {
    assert_has_artha_taddhita("kuYja", Gotra, T::cPaY, &["kOYjAyana"]);
    assert_has_artha_taddhita("braDna", Gotra, T::cPaY, &["brADnAyana"]);
    // gotre?
    assert_has_artha_taddhita("kuYja", TasyaApatyam, T::iY, &["kOYji"]);
}

#[test]
fn sutra_4_1_99() {
    assert_has_artha_taddhita("naqa", Gotra, T::Pak, &["nAqAyana"]);
    assert_has_artha_taddhita("cara", Gotra, T::Pak, &["cArAyaRa"]);
    // gotre?
    assert_has_artha_taddhita("naqa", TasyaApatyam, T::iY, &["nAqi"]);
    // TODO: others
}

#[test]
fn sutra_4_1_102() {
    assert_has_artha_taddhita("Saradvat", Gotra, T::Pak, &["SAradvatAyana"]);
    assert_has_artha_taddhita("Saradvat", Gotra, T::aY, &["SAradvata"]);
    assert_has_artha_taddhita("Sunaka", Gotra, T::Pak, &["SOnakAyana"]);
    assert_has_artha_taddhita("Sunaka", Gotra, T::aY, &["SOnaka"]);
    assert_has_artha_taddhita("darBa", Gotra, T::Pak, &["dArBAyaRa"]);
    assert_has_artha_taddhita("darBa", TasyaApatyam, T::iY, &["dArBi"]);
}

#[test]
fn sutra_4_1_103() {
    assert_has_artha_taddhita("droRa", Gotra, T::Pak, &["drORAyana"]);
    assert_has_artha_taddhita("droRa", Gotra, T::iY, &["drORi"]);
    assert_has_artha_taddhita("parvata", Gotra, T::Pak, &["pArvatAyana"]);
    assert_has_artha_taddhita("parvata", Gotra, T::iY, &["pArvati"]);
    assert_has_artha_taddhita("jIvanta", Gotra, T::Pak, &["jEvantAyana"]);
    assert_has_artha_taddhita("jIvanta", Gotra, T::iY, &["jEvanti"]);
}

#[test]
fn sutra_4_1_104() {
    assert_has_artha_taddhita("bida", Gotra, T::aY, &["bEda"]);
    assert_has_artha_taddhita("urva", Gotra, T::aY, &["Orva"]);
}

#[test]
fn sutra_4_1_105() {
    assert_has_artha_taddhita("garga", Gotra, T::yaY, &["gArgya"]);
    assert_has_artha_taddhita("vatsa", Gotra, T::yaY, &["vAtsya"]);
}

#[test]
fn sutra_4_1_106() {
    assert_has_artha_taddhita("maDu", Gotra, T::yaY, &["mADavya"]);
    assert_has_artha_taddhita("maDu", Gotra, T::aR, &["mADava"]);
    assert_has_artha_taddhita("baBru", Gotra, T::yaY, &["bABravya"]);
    assert_has_artha_taddhita("baBru", Gotra, T::aR, &["bABrava"]);
    // TODO: bABravyAyaRI
}

#[test]
fn sutra_4_1_107() {
    assert_has_artha_taddhita("kapi", Gotra, T::yaY, &["kApya"]);
    assert_has_artha_taddhita("boDa", Gotra, T::yaY, &["bODya"]);
    // ANgirase
    assert_has_artha_taddhita("kapi", Gotra, T::Qak, &["kApeya"]);
    assert_has_artha_taddhita("boDa", Gotra, T::iY, &["bODi"]);
    // TODO: kApyAyanI
}

#[test]
fn sutra_4_1_108() {
    assert_has_artha_taddhita("vataRqa", Gotra, T::yaY, &["vAtaRqya"]);
    assert_has_artha_taddhita("vataRqa", Gotra, T::aR, &["vAtaRqa"]);
}

#[test]
fn sutra_4_1_110() {
    assert_has_artha_taddhita("aSva", Gotra, T::PaY, &["ASvAyana"]);
    assert_has_artha_taddhita("aSman", Gotra, T::PaY, &["ASmAyana"]);
}

#[test]
fn sutra_4_1_111() {
    assert_has_artha_taddhita("Barga", Gotra, T::PaY, &["BArgAyaRa"]);
    assert_has_artha_taddhita("Barga", Gotra, T::iY, &["BArgi"]);
}

#[test]
fn sutra_4_1_112() {
    assert_has_taddhitanta(&prati("Siva"), T::aR, &["SEva"]);
    assert_has_taddhitanta(&prati("prOzWa"), T::aR, &["prOzWa"]);

    // apavAda
    assert_has_taddhitanta(&prati("Siva"), T::iY, &[]);
    assert_has_taddhitanta(&prati("prOzWa"), T::iY, &[]);
}

#[test]
fn sutra_4_1_116() {
    assert_has_taddhitanta(&prati("kanyA"), T::aR, &["kAnIna"]);
}

#[test]
fn sutra_4_1_117() {
    assert_has_artha_taddhita("vikarRa", TasyaApatyam, T::aR, &["vEkarRa"]);
    assert_has_artha_taddhita("vikarRa", TasyaApatyam, T::iY, &["vEkarRi"]);
    assert_has_artha_taddhita("SuNga", TasyaApatyam, T::aR, &["SONga"]);
    assert_has_artha_taddhita("SuNga", TasyaApatyam, T::iY, &["SONgi"]);
    assert_has_artha_taddhita("Cagala", TasyaApatyam, T::aR, &["CAgala"]);
    assert_has_artha_taddhita("Cagala", TasyaApatyam, T::iY, &["CAgali"]);
}

#[test]
fn sutra_4_1_118() {
    assert_has_taddhitanta(&nyap("pilA"), T::aR, &["pEla"]);
    assert_has_taddhitanta(&nyap("pilA"), T::Qak, &["pEleya"]);
}

#[test]
fn sutra_4_1_119() {
    assert_has_artha_taddhita("maRqUka", TasyaApatyam, T::aR, &["mARqUka"]);
    assert_has_artha_taddhita("maRqUka", TasyaApatyam, T::iY, &["mARqUki"]);
    assert_has_artha_taddhita("maRqUka", TasyaApatyam, T::Qak, &["mARqUkeya"]);
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
    assert_has_artha_taddhita("atri", TasyaApatyam, T::Qak, &["Atreya"]);
    assert_has_artha_taddhita("niDi", TasyaApatyam, T::Qak, &["nEDeya"]);
    // itaH?
    assert_has_artha_taddhita("dakza", TasyaApatyam, T::Qak, &[]);
    assert_has_artha_taddhita("dakza", TasyaApatyam, T::iY, &["dAkzi"]);
    assert_has_artha_taddhita("plakza", TasyaApatyam, T::Qak, &[]);
    assert_has_artha_taddhita("plakza", TasyaApatyam, T::iY, &["plAkzi"]);
    // TODO: an-iYaH
    // dvi-acaH?
    assert_has_artha_taddhita("marIci", TasyaApatyam, T::Qak, &[]);
    assert_has_artha_taddhita("marIci", TasyaApatyam, T::aR, &["mArIca"]);
}

#[test]
fn sutra_4_1_123() {
    assert_has_artha_taddhita("SuBra", TasyaApatyam, T::Qak, &["SOBreya"]);
    assert_has_artha_taddhita("vizwapura", TasyaApatyam, T::Qak, &["vEzwapureya"]);
}

#[test]
fn sutra_4_1_124() {
    assert_has_artha_taddhita("vikarRa", TasyaApatyam, T::Qak, &["vEkarReya"]);
    assert_has_artha_taddhita("kuzItaka", TasyaApatyam, T::Qak, &["kOzItakeya"]);
    assert_has_artha_taddhita("vikarRa", TasyaApatyam, T::iY, &["vEkarRi"]);
    assert_has_artha_taddhita("kuzItaka", TasyaApatyam, T::iY, &["kOzItaki"]);
}

#[ignore]
#[test]
fn sutra_4_1_125() {
    assert_has_artha_taddhita("BrU", TasyaApatyam, T::Qak, &["BrOveya"]);
}

#[ignore]
#[test]
fn sutra_4_1_126() {
    assert_has_artha_taddhita("kalyARI", TasyaApatyam, T::Qak, &["kalyARineya"]);
    assert_has_artha_taddhita("suBagA", TasyaApatyam, T::Qak, &["sOBAgineya"]);
    assert_has_artha_taddhita("durBagA", TasyaApatyam, T::Qak, &["dOrBAgineya"]);
}

#[ignore]
#[test]
fn sutra_4_1_127() {
    assert_has_taddhitanta(&nyap("kulawA"), T::Qak, &["kOlawineya", "kOlaweya"]);
    // TODO: Qrak
}

#[test]
fn sutra_4_1_128() {
    assert_has_artha_taddhita("cawakA", TasyaApatyam, T::Erak, &["cAwakEra"]);
}

#[test]
fn sutra_4_1_128_v1() {
    assert_has_artha_taddhita("cawaka", TasyaApatyam, T::Erak, &["cAwakEra"]);
}

#[test]
fn sutra_4_1_129() {
    assert_has_artha_taddhita("goDA", TasyaApatyam, T::Qrak, &["gODera"]);
}

#[test]
fn sutra_4_1_130() {
    assert_has_artha_taddhita("goDA", TasyaApatyam, T::Arak, &["gODAra"]);
}

#[test]
fn sutra_4_1_132() {
    assert_has_artha_taddhita("pitfzvasf", TasyaApatyam, T::CaR, &["pEtfzvasrIya"]);
}

#[test]
fn sutra_4_1_133() {
    assert_has_artha_taddhita("pitfzvasf", TasyaApatyam, T::Qak, &["pEtfzvaseya"]);
}

#[test]
fn sutra_4_1_134() {
    assert_has_artha_taddhita("mAtfzvasf", TasyaApatyam, T::CaR, &["mAtfzvasrIya"]);
    assert_has_artha_taddhita("mAtfzvasf", TasyaApatyam, T::Qak, &["mAtfzvaseya"]);
}

#[test]
fn sutra_4_1_136() {
    assert_has_artha_taddhita("gfzwi", TasyaApatyam, T::QaY, &["gArzweya"]);
    assert_has_artha_taddhita("hfzwi", TasyaApatyam, T::QaY, &["hArzweya"]);
}

#[ignore]
#[test]
fn sutra_4_1_137() {
    assert_has_artha_taddhita("rAjan", TasyaApatyam, T::yat, &["rAjanya"]);
    assert_has_artha_taddhita("SvaSura", TasyaApatyam, T::yat, &["SvaSurya"]);
}

#[test]
fn sutra_4_1_138() {
    assert_has_artha_taddhita("kzatra", TasyaApatyam, T::Ga, &["kzatriya"]);
}

#[test]
fn sutra_4_1_139() {
    assert_has_artha_taddhita("kula", TasyaApatyam, T::Ka, &["kulIna"]);
}

#[test]
fn sutra_4_1_140() {
    assert_has_artha_taddhita("kula", TasyaApatyam, T::yat, &["kulya"]);
    assert_has_artha_taddhita("kula", TasyaApatyam, T::QakaY, &["kOleyaka"]);
    assert_has_artha_taddhita("kula", TasyaApatyam, T::Ka, &["kulIna"]);
}

#[test]
fn sutra_4_1_141() {
    assert_has_artha_taddhita("mahAkula", TasyaApatyam, T::aY, &["mAhAkula"]);
    assert_has_artha_taddhita("mahAkula", TasyaApatyam, T::KaY, &["mAhAkulIna"]);
    assert_has_artha_taddhita("mahAkula", TasyaApatyam, T::Ka, &["mahAkulIna"]);
}

#[test]
fn sutra_4_1_142() {
    assert_has_artha_taddhita("duzkula", TasyaApatyam, T::Qak, &["dOzkuleya"]);
    assert_has_artha_taddhita("duzkula", TasyaApatyam, T::Ka, &["duzkulIna"]);
}

#[test]
fn sutra_4_1_143() {
    assert_has_artha_taddhita("svasf", TasyaApatyam, T::Ca, &["svasrIya"]);
}

#[test]
fn sutra_4_1_144() {
    assert_has_artha_taddhita("BrAtf", TasyaApatyam, T::Ca, &["BrAtrIya"]);
    assert_has_artha_taddhita("BrAtf", TasyaApatyam, T::vyat, &["BrAtfvya"]);
}

#[test]
fn sutra_4_1_145() {
    assert_has_artha_taddhita("BrAtf", TasyaApatyam, T::vyan, &["BrAtfvya"]);
}

#[test]
fn sutra_4_1_146() {
    assert_has_artha_taddhita("revatI", TasyaApatyam, T::Wak, &["rEvatika"]);
    assert_has_artha_taddhita("aSvapAlI", TasyaApatyam, T::Wak, &["ASvapAlika"]);
}

#[test]
fn sutra_4_1_157() {
    assert_has_artha_taddhita("Amragupta", TasyaApatyam, T::PiY, &["AmraguptAyani"]);
    assert_has_artha_taddhita("grAmarakza", TasyaApatyam, T::PiY, &["grAmarakzAyaRi"]);
}

#[test]
fn sutra_4_1_158() {
    assert_has_artha_taddhita("vAkina", TasyaApatyam, T::PiY, &["vAkinakAyani"]);
    assert_has_artha_taddhita("vAkina", TasyaApatyam, T::iY, &["vAkini"]);
    assert_has_artha_taddhita("gAreDa", TasyaApatyam, T::PiY, &["gAreDakAyani"]);
    assert_has_artha_taddhita("gAreDa", TasyaApatyam, T::iY, &["gAreDi"]);
}

#[test]
fn sutra_4_1_159() {
    assert_has_artha_taddhita(
        "gArgIputra",
        TasyaApatyam,
        T::PiY,
        &["gArgIputrakAyaRi", "gArgIputrAyaRi"],
    );
    assert_has_artha_taddhita("gArgIputra", TasyaApatyam, T::iY, &["gArgIputri"]);
    assert_has_artha_taddhita(
        "vAtsIputra",
        TasyaApatyam,
        T::PiY,
        &["vAtsIputrakAyaRi", "vAtsIputrAyaRi"],
    );
    assert_has_artha_taddhita("vAtsIputra", TasyaApatyam, T::iY, &["vAtsIputri"]);
}

#[test]
fn sutra_4_1_161() {
    assert_has_artha_taddhita("manu", Jatau, T::aY, &["mAnuza"]);
    assert_has_artha_taddhita("manu", Jatau, T::yat, &["manuzya"]);
    // apatya
    assert_has_artha_taddhita("manu", TasyaApatyam, T::aR, &["mAnava"]);
    // TODO: mARava
}

#[test]
fn sutra_4_1_162() {
    assert_has_artha_taddhita("garga", Gotra, T::yaY, &["gArgya"]);
    assert_has_artha_taddhita("vatsa", Gotra, T::yaY, &["vAtsya"]);
    // pOtra-praBfti?
    assert_has_artha_taddhita("kuYja", TasyaApatyam, T::iY, &["kOYji"]);
    assert_has_artha_taddhita("garga", TasyaApatyam, T::iY, &["gArgi"]);
}

#[test]
fn sutra_4_1_168() {
    assert_has_artha_taddhita("paYcAla", Janapada, T::aY, &["pAYcAla"]);
    assert_has_artha_taddhita("ikzvAku", Janapada, T::aY, &["EkzvAka"]);
    assert_has_artha_taddhita("videha", Janapada, T::aY, &["vEdeha"]);
    // apatya
    assert_has_artha_taddhita("druhyu", TasyaApatyam, T::aR, &["drOhyava"]);
    assert_has_artha_taddhita("puru", TasyaApatyam, T::aR, &["pOrava"]);
    // kzatriyasya?
    // TODO: how to derive pAYcAli? paYcAla is utsAdi so isn't iY blocked?
    // assert_has_artha_taddhita("paYcAla", TasyaApatyam, T::iY, &["pAYcAli"]);
    assert_has_artha_taddhita("videha", TasyaApatyam, T::iY, &["vEdehi"]);
}

#[test]
fn sutra_4_1_169() {
    assert_has_artha_taddhita("sAlveya", Janapada, T::aY, &["sAlveya"]);
    assert_has_artha_taddhita("gAnDAri", Janapada, T::aY, &["gAnDAra"]);
}

#[test]
fn sutra_4_1_170() {
    assert_has_artha_taddhita("aNga", Janapada, T::aR, &["ANga"]);
    assert_has_artha_taddhita("vaNga", Janapada, T::aR, &["vANga"]);
    assert_has_artha_taddhita("magaDa", Janapada, T::aR, &["mAgaDa"]);
    assert_has_artha_taddhita("kaliNga", Janapada, T::aR, &["kAliNga"]);
    assert_has_artha_taddhita("sUramasa", Janapada, T::aR, &["sOramasa"]);
    assert_blocked("aNga", Janapada, T::aY);
    assert_blocked("vaNga", Janapada, T::aY);
    assert_blocked("magaDa", Janapada, T::aY);
    assert_blocked("kaliNga", Janapada, T::aY);
    assert_blocked("sUramasa", Janapada, T::aY);
}

#[test]
fn sutra_4_1_171() {
    assert_has_artha_taddhita("AmbazWa", Janapada, T::YyaN, &["AmbazWya"]);
    assert_has_artha_taddhita("sOvIra", Janapada, T::YyaN, &["sOvIrya"]);
    assert_has_artha_taddhita("avanti", Janapada, T::YyaN, &["Avantya"]);
    assert_has_artha_taddhita("kunti", Janapada, T::YyaN, &["kOntya"]);
    assert_has_artha_taddhita("kosala", Janapada, T::YyaN, &["kOsalya"]);
    assert_has_artha_taddhita("ajAda", Janapada, T::YyaN, &["AjAdya"]);
    assert_blocked("AmbazWa", Janapada, T::aY);
}

#[test]
fn sutra_4_1_172() {
    assert_has_artha_taddhita("kuru", Janapada, T::Rya, &["kOravya"]);
    assert_has_artha_taddhita("nizaDa", Janapada, T::Rya, &["nEzaDya"]);
    assert_blocked("kuru", Janapada, T::aR);
    assert_blocked("kuru", Janapada, T::aY);
}

#[test]
fn sutra_4_1_173() {
    assert_has_artha_taddhita("udumbarA", Janapada, T::iY, &["Odumbari"]);
    assert_has_artha_taddhita("tilaKalA", Janapada, T::iY, &["tElaKali"]);
    assert_has_artha_taddhita("madrakArA", Janapada, T::iY, &["mAdrakAri"]);
    assert_has_artha_taddhita("yuganDarA", Janapada, T::iY, &["yOganDari"]);
    assert_has_artha_taddhita("BuliNgA", Janapada, T::iY, &["BOliNgi"]);
    assert_has_artha_taddhita("SaradaRqA", Janapada, T::iY, &["SAradaRqi"]);

    assert_has_artha_taddhita("pratyagraTa", Janapada, T::iY, &["prAtyagraTi"]);
    assert_has_artha_taddhita("kalakUwa", Janapada, T::iY, &["kAlakUwi"]);
    assert_has_artha_taddhita("aSmaka", Janapada, T::iY, &["ASmaki"]);
}

#[test]
fn sutra_4_1_175() {
    assert_has_artha_taddhita("kamboja", Janapada, T::aY, &["kamboja"]);
}
