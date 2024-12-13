extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::Unadi;
use vidyut_prakriya::args::*;

fn assert_has_pum(prati: &str, expected: &[&str]) {
    assert_has_sup_1s(prati, Pum, expected);
}

fn assert_has_stri(prati: impl Into<SafePratipadika>, expected: &[&str]) {
    assert_has_sup_1s(prati.into().0, Stri, expected);
}

fn assert_blocked(text: &str, artha: TaddhitaArtha, t: T) {
    assert_has_artha_taddhita(text, artha, t, &[]);
}

#[test]
fn skip_sutra_4_1_1() {}

#[test]
fn sutra_4_1_2() {
    // NIp
    let kumari = nyap("kumArI");
    assert_has_sup_1s(&kumari, Stri, &["kumArI"]);
    assert_has_sup_1d(&kumari, Stri, &["kumAryO"]);
    assert_has_sup_1p(&kumari, Stri, &["kumAryaH"]);
    assert_has_sup_2s(&kumari, Stri, &["kumArIm"]);
    assert_has_sup_2d(&kumari, Stri, &["kumAryO"]);
    assert_has_sup_2p(&kumari, Stri, &["kumArIH"]);
    assert_has_sup_3s(&kumari, Stri, &["kumAryA"]);
    assert_has_sup_3d(&kumari, Stri, &["kumArIByAm"]);
    assert_has_sup_3p(&kumari, Stri, &["kumArIBiH"]);
    assert_has_sup_4s(&kumari, Stri, &["kumAryE"]);
    assert_has_sup_4d(&kumari, Stri, &["kumArIByAm"]);
    assert_has_sup_4p(&kumari, Stri, &["kumArIByaH"]);
    assert_has_sup_5s(&kumari, Stri, &["kumAryAH"]);
    assert_has_sup_5d(&kumari, Stri, &["kumArIByAm"]);
    assert_has_sup_5p(&kumari, Stri, &["kumArIByaH"]);
    assert_has_sup_6s(&kumari, Stri, &["kumAryAH"]);
    assert_has_sup_6d(&kumari, Stri, &["kumAryoH"]);
    assert_has_sup_6p(&kumari, Stri, &["kumArIRAm"]);
    assert_has_sup_7s(&kumari, Stri, &["kumAryAm"]);
    assert_has_sup_7d(&kumari, Stri, &["kumAryoH"]);
    assert_has_sup_7p(&kumari, Stri, &["kumArIzu"]);

    // NIz
    let gauri = nyap("gOrI");
    assert_has_sup_1s(&gauri, Stri, &["gOrI"]);
    assert_has_sup_1d(&gauri, Stri, &["gOryO"]);
    assert_has_sup_1p(&gauri, Stri, &["gOryaH"]);
    assert_has_sup_2s(&gauri, Stri, &["gOrIm"]);
    assert_has_sup_2d(&gauri, Stri, &["gOryO"]);
    assert_has_sup_2p(&gauri, Stri, &["gOrIH"]);
    assert_has_sup_3s(&gauri, Stri, &["gOryA"]);
    assert_has_sup_3d(&gauri, Stri, &["gOrIByAm"]);
    assert_has_sup_3p(&gauri, Stri, &["gOrIBiH"]);
    assert_has_sup_4s(&gauri, Stri, &["gOryE"]);
    assert_has_sup_4d(&gauri, Stri, &["gOrIByAm"]);
    assert_has_sup_4p(&gauri, Stri, &["gOrIByaH"]);
    assert_has_sup_5s(&gauri, Stri, &["gOryAH"]);
    assert_has_sup_5d(&gauri, Stri, &["gOrIByAm"]);
    assert_has_sup_5p(&gauri, Stri, &["gOrIByaH"]);
    assert_has_sup_6s(&gauri, Stri, &["gOryAH"]);
    assert_has_sup_6d(&gauri, Stri, &["gOryoH"]);
    assert_has_sup_6p(&gauri, Stri, &["gOrIRAm"]);
    assert_has_sup_7s(&gauri, Stri, &["gOryAm"]);
    assert_has_sup_7d(&gauri, Stri, &["gOryoH"]);
    assert_has_sup_7p(&gauri, Stri, &["gOrIzu"]);

    // NIn
    let sharngaravi = nyap("SArNgaravI");
    assert_has_sup_1s(&sharngaravi, Stri, &["SArNgaravI"]);
    assert_has_sup_1d(&sharngaravi, Stri, &["SArNgaravyO"]);
    assert_has_sup_1p(&sharngaravi, Stri, &["SArNgaravyaH"]);
    assert_has_sup_2s(&sharngaravi, Stri, &["SArNgaravIm"]);
    assert_has_sup_2d(&sharngaravi, Stri, &["SArNgaravyO"]);
    assert_has_sup_2p(&sharngaravi, Stri, &["SArNgaravIH"]);
    assert_has_sup_3s(&sharngaravi, Stri, &["SArNgaravyA"]);
    assert_has_sup_3d(&sharngaravi, Stri, &["SArNgaravIByAm"]);
    assert_has_sup_3p(&sharngaravi, Stri, &["SArNgaravIBiH"]);
    assert_has_sup_4s(&sharngaravi, Stri, &["SArNgaravyE"]);
    assert_has_sup_4d(&sharngaravi, Stri, &["SArNgaravIByAm"]);
    assert_has_sup_4p(&sharngaravi, Stri, &["SArNgaravIByaH"]);
    assert_has_sup_5s(&sharngaravi, Stri, &["SArNgaravyAH"]);
    assert_has_sup_5d(&sharngaravi, Stri, &["SArNgaravIByAm"]);
    assert_has_sup_5p(&sharngaravi, Stri, &["SArNgaravIByaH"]);
    assert_has_sup_6s(&sharngaravi, Stri, &["SArNgaravyAH"]);
    assert_has_sup_6d(&sharngaravi, Stri, &["SArNgaravyoH"]);
    assert_has_sup_6p(&sharngaravi, Stri, &["SArNgaravIRAm"]);
    assert_has_sup_7s(&sharngaravi, Stri, &["SArNgaravyAm"]);
    assert_has_sup_7d(&sharngaravi, Stri, &["SArNgaravyoH"]);
    assert_has_sup_7p(&sharngaravi, Stri, &["SArNgaravIzu"]);

    // wAp
    let khatva = nyap("KawvA");
    assert_has_sup_1s(&khatva, Stri, &["KawvA"]);
    assert_has_sup_1d(&khatva, Stri, &["Kawve"]);
    assert_has_sup_1p(&khatva, Stri, &["KawvAH"]);
    assert_has_sup_2s(&khatva, Stri, &["KawvAm"]);
    assert_has_sup_2d(&khatva, Stri, &["Kawve"]);
    assert_has_sup_2p(&khatva, Stri, &["KawvAH"]);
    assert_has_sup_3s(&khatva, Stri, &["KawvayA"]);
    assert_has_sup_3d(&khatva, Stri, &["KawvAByAm"]);
    assert_has_sup_3p(&khatva, Stri, &["KawvABiH"]);
    assert_has_sup_4s(&khatva, Stri, &["KawvAyE"]);
    assert_has_sup_4d(&khatva, Stri, &["KawvAByAm"]);
    assert_has_sup_4p(&khatva, Stri, &["KawvAByaH"]);
    assert_has_sup_5s(&khatva, Stri, &["KawvAyAH"]);
    assert_has_sup_5d(&khatva, Stri, &["KawvAByAm"]);
    assert_has_sup_5p(&khatva, Stri, &["KawvAByaH"]);
    assert_has_sup_6s(&khatva, Stri, &["KawvAyAH"]);
    assert_has_sup_6d(&khatva, Stri, &["KawvayoH"]);
    assert_has_sup_6p(&khatva, Stri, &["KawvAnAm"]);
    assert_has_sup_7s(&khatva, Stri, &["KawvAyAm"]);
    assert_has_sup_7d(&khatva, Stri, &["KawvayoH"]);
    assert_has_sup_7p(&khatva, Stri, &["KawvAsu"]);

    // qAp
    let bahuraja = nyap("bahurAjA");
    assert_has_sup_1s(&bahuraja, Stri, &["bahurAjA"]);
    assert_has_sup_1d(&bahuraja, Stri, &["bahurAje"]);
    assert_has_sup_1p(&bahuraja, Stri, &["bahurAjAH"]);
    assert_has_sup_2s(&bahuraja, Stri, &["bahurAjAm"]);
    assert_has_sup_2d(&bahuraja, Stri, &["bahurAje"]);
    assert_has_sup_2p(&bahuraja, Stri, &["bahurAjAH"]);
    assert_has_sup_3s(&bahuraja, Stri, &["bahurAjayA"]);
    assert_has_sup_3d(&bahuraja, Stri, &["bahurAjAByAm"]);
    assert_has_sup_3p(&bahuraja, Stri, &["bahurAjABiH"]);
    assert_has_sup_4s(&bahuraja, Stri, &["bahurAjAyE"]);
    assert_has_sup_4d(&bahuraja, Stri, &["bahurAjAByAm"]);
    assert_has_sup_4p(&bahuraja, Stri, &["bahurAjAByaH"]);
    assert_has_sup_5s(&bahuraja, Stri, &["bahurAjAyAH"]);
    assert_has_sup_5d(&bahuraja, Stri, &["bahurAjAByAm"]);
    assert_has_sup_5p(&bahuraja, Stri, &["bahurAjAByaH"]);
    assert_has_sup_6s(&bahuraja, Stri, &["bahurAjAyAH"]);
    assert_has_sup_6d(&bahuraja, Stri, &["bahurAjayoH"]);
    assert_has_sup_6p(&bahuraja, Stri, &["bahurAjAnAm"]);
    assert_has_sup_7s(&bahuraja, Stri, &["bahurAjAyAm"]);
    assert_has_sup_7d(&bahuraja, Stri, &["bahurAjayoH"]);
    assert_has_sup_7p(&bahuraja, Stri, &["bahurAjAsu"]);

    // cAp
    let karishagandhya = nyap("kArIzaganDyA");
    assert_has_sup_1s(&karishagandhya, Stri, &["kArIzaganDyA"]);
    assert_has_sup_1d(&karishagandhya, Stri, &["kArIzaganDye"]);
    assert_has_sup_1p(&karishagandhya, Stri, &["kArIzaganDyAH"]);
    assert_has_sup_2s(&karishagandhya, Stri, &["kArIzaganDyAm"]);
    assert_has_sup_2d(&karishagandhya, Stri, &["kArIzaganDye"]);
    assert_has_sup_2p(&karishagandhya, Stri, &["kArIzaganDyAH"]);
    assert_has_sup_3s(&karishagandhya, Stri, &["kArIzaganDyayA"]);
    assert_has_sup_3d(&karishagandhya, Stri, &["kArIzaganDyAByAm"]);
    assert_has_sup_3p(&karishagandhya, Stri, &["kArIzaganDyABiH"]);
    assert_has_sup_4s(&karishagandhya, Stri, &["kArIzaganDyAyE"]);
    assert_has_sup_4d(&karishagandhya, Stri, &["kArIzaganDyAByAm"]);
    assert_has_sup_4p(&karishagandhya, Stri, &["kArIzaganDyAByaH"]);
    assert_has_sup_5s(&karishagandhya, Stri, &["kArIzaganDyAyAH"]);
    assert_has_sup_5d(&karishagandhya, Stri, &["kArIzaganDyAByAm"]);
    assert_has_sup_5p(&karishagandhya, Stri, &["kArIzaganDyAByaH"]);
    assert_has_sup_6s(&karishagandhya, Stri, &["kArIzaganDyAyAH"]);
    assert_has_sup_6d(&karishagandhya, Stri, &["kArIzaganDyayoH"]);
    assert_has_sup_6p(&karishagandhya, Stri, &["kArIzaganDyAnAm"]);
    assert_has_sup_7s(&karishagandhya, Stri, &["kArIzaganDyAyAm"]);
    assert_has_sup_7d(&karishagandhya, Stri, &["kArIzaganDyayoH"]);
    assert_has_sup_7p(&karishagandhya, Stri, &["kArIzaganDyAsu"]);

    let drshad = phit("dfzad");
    assert_has_sup_1s(&drshad, Pum, &["dfzat"]);
    assert_has_sup_1d(&drshad, Pum, &["dfzadO"]);
    assert_has_sup_1p(&drshad, Pum, &["dfzadaH"]);
    assert_has_sup_2s(&drshad, Pum, &["dfzadam"]);
    assert_has_sup_2d(&drshad, Pum, &["dfzadO"]);
    assert_has_sup_2p(&drshad, Pum, &["dfzadaH"]);
    assert_has_sup_3s(&drshad, Pum, &["dfzadA"]);
    assert_has_sup_3d(&drshad, Pum, &["dfzadByAm"]);
    assert_has_sup_3p(&drshad, Pum, &["dfzadBiH"]);
    assert_has_sup_4s(&drshad, Pum, &["dfzade"]);
    assert_has_sup_4d(&drshad, Pum, &["dfzadByAm"]);
    assert_has_sup_4p(&drshad, Pum, &["dfzadByaH"]);
    assert_has_sup_5s(&drshad, Pum, &["dfzadaH"]);
    assert_has_sup_5d(&drshad, Pum, &["dfzadByAm"]);
    assert_has_sup_5p(&drshad, Pum, &["dfzadByaH"]);
    assert_has_sup_6s(&drshad, Pum, &["dfzadaH"]);
    assert_has_sup_6d(&drshad, Pum, &["dfzadoH"]);
    assert_has_sup_6p(&drshad, Pum, &["dfzadAm"]);
    assert_has_sup_7s(&drshad, Pum, &["dfzadi"]);
    assert_has_sup_7d(&drshad, Pum, &["dfzadoH"]);
    assert_has_sup_7p(&drshad, Pum, &["dfzatsu"]);
}

#[test]
fn skip_sutra_4_1_3() {}

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
fn sutra_4_1_6() {
    let bhavat = create_krdanta("Bavat", &[], &d("BA\\", Adadi), Unadi::qavatu);
    assert_has_stri(&bhavat, &["BavatI"]);

    let abhibhavat = create_avyaya_tatpurusha("atiBavat", "ati", &bhavat);
    assert_has_stri(&abhibhavat, &["atiBavatI"]);

    let pacat = create_krdanta("pacat", &[], &d("qupa\\ca~^z", Bhvadi), Krt::Satf);
    assert_has_stri(&pacat, &["pacantI"]);

    let yajat = create_krdanta("yajat", &[], &d("ya\\ja~", Bhvadi), Krt::Satf);
    assert_has_stri(&yajat, &["yajantI"]);
}

#[test]
fn sutra_4_1_7() {
    assert_has_stri("DIvan", &["DIvarI"]);
    assert_has_stri("pIvan", &["pIvarI"]);
    assert_has_stri("Sarvan", &["SarvarI"]);
}

#[test]
fn sutra_4_1_8_to_sutra_4_1_9() {
    let dvipad = create_bahuvrihi("dvipAd", "dvi", "pAda");
    let tripad = create_bahuvrihi("tripAd", "tri", "pAda");
    let catushpad = create_bahuvrihi("catuzpAd", "catur", "pAda");
    assert_has_stri(&dvipad, &["dvipAt", "dvipadI", "dvipadA"]);
    assert_has_stri(&tripad, &["tripAt", "tripadI", "tripadA"]);
    assert_has_stri(&catushpad, &["catuzpAt", "catuzpadI", "catuzpadA"]);
}

#[test]
fn sutra_4_1_10() {
    // zaw
    assert_has_sup_1p("paYcan", Stri, &["paYca"]);
    assert_has_sup_1p("saptan", Stri, &["sapta"]);
    assert_has_sup_1p("navan", Stri, &["nava"]);
    assert_has_sup_1p("daSan", Stri, &["daSa"]);

    // svasrAdi
    assert_has_stri("svasf", &["svasA"]);
    assert_has_stri("duhitf", &["duhitA"]);
    assert_has_stri("nanAndf", &["nanAndA"]);
    assert_has_stri("yAtf", &["yAtA"]);
    assert_has_stri("mAtf", &["mAtA"]);
    assert_has_sup_1p("tri", Stri, &["tisraH"]);
    assert_has_sup_1p("catur", Stri, &["catasraH"]);
}

#[ignore]
#[test]
fn sutra_4_1_11() {
    assert_has_sup_1s("dAman", Stri, &["dAmA"]);
    assert_has_sup_1d("dAman", Stri, &["dAmAnO"]);
    assert_has_sup_1p("dAman", Stri, &["dAmAnaH"]);

    assert_has_sup_1s("pAman", Stri, &["pAmA"]);
    assert_has_sup_1d("pAman", Stri, &["pAmAnO"]);
    assert_has_sup_1p("pAman", Stri, &["pAmAnaH"]);

    assert_has_sup_1s("sIman", Stri, &["sImA"]);
    assert_has_sup_1d("sIman", Stri, &["sImAnO"]);
    assert_has_sup_1p("sIman", Stri, &["sImAnaH"]);

    assert_has_sup_1s("atimahiman", Stri, &["atimahimA"]);
    assert_has_sup_1d("atimahiman", Stri, &["atimahimAnO"]);
    assert_has_sup_1p("atimahiman", Stri, &["atimahimAnaH"]);
}

#[ignore]
#[test]
fn sutra_4_1_14() {
    let kurucara = create_upapada_krdanta("kurucara", "kuru", &[], &d("cara~", Bhvadi), Krt::wa);
    let madracara = create_upapada_krdanta("madracara", "madra", &[], &d("cara~", Bhvadi), Krt::wa);
    assert_has_stri(&kurucara, &["kurucarI"]);
    assert_has_stri(&madracara, &["madracarI"]);
}

#[ignore]
#[test]
fn sutra_4_1_15() {
    let kurucara = create_upapada_krdanta("kurucara", "kuru", &[], &d("cara~", Bhvadi), Krt::wa);
    let madracara = create_upapada_krdanta("madracara", "madra", &[], &d("cara~", Bhvadi), Krt::wa);
    assert_has_stri(&kurucara, &["kurucarI"]);
    assert_has_stri(&madracara, &["madracarI"]);

    let pacamana = create_krdanta("pacamAna", &[], &d("qupa\\ca~^z", Bhvadi), Krt::SAnac);
    let yajamana = create_krdanta("yajamAna", &[], &d("ya\\ja~^", Bhvadi), Krt::SAnac);
    assert_has_stri(&pacamana, &["pacamAnA"]);
    assert_has_stri(&yajamana, &["yajamAnA"]);

    let sauparneya = create_taddhitanta("sOparReya", &nyap("suparRA"), T::Qak);
    let vainateya = create_taddhitanta("vEnateya", &nyap("vinatA"), T::Qak);
    assert_has_stri(&sauparneya, &["sOparReyI"]);
    assert_has_stri(&vainateya, &["vEnateyI"]);

    let kr = d("qukf\\Y", Tanadi);
    let kumbhakara = create_upapada_krdanta("kumBakAra", "kumBa", &[], &kr, Krt::aR);
    let nagarakara = create_upapada_krdanta("nagarakAra", "nagara", &[], &kr, Krt::aR);
    assert_has_stri(&kumbhakara, &["kumBakArI"]);
    assert_has_stri(&nagarakara, &["nagarakArI"]);

    let aupagava = create_taddhitanta("Opagava", "upagu", T::aR);
    assert_has_stri(&aupagava, &["OpagavI"]);

    // TODO: many, many others
}

#[test]
fn sutra_4_1_41() {
    // zit
    let nartaka = create_krdanta("nartaka", &[], &d("nftI~", Divadi), Krt::zvun);
    assert_has_stri(&nartaka, &["nartakI"]);
    let khanaka = create_krdanta("Kanaka", &[], &d("Kanu~^", Bhvadi), Krt::zvun);
    assert_has_stri(&khanaka, &["KanakI"]);
    let rajaka = create_krdanta("rajaka", &[], &d("ra\\nja~^", Bhvadi), Krt::zvun);
    assert_has_stri(&rajaka, &["rajakI"]);

    // gaurAdi
    assert_has_stri("gOra", &["gOrI"]);
    assert_has_stri("matsya", &["matsI"]);
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
}

#[test]
fn sutra_4_1_62() {
    assert_has_stri("saKi", &["saKI"]);
    assert_has_stri("aSiSu", &["aSiSvI"]);
}

#[test]
fn sutra_4_1_68() {
    assert_has_stri("paNgu", &["paNgUH"]);
}

#[test]
fn sutra_4_1_68_v1() {
    assert_has_stri("SvaSura", &["SvaSrUH"]);
}

#[test]
fn sutra_4_1_71() {
    let c = Tester::with_chaandasa();
    c.assert_has_sup_1s("kadru", Stri, &["kadrUH"]);
    c.assert_has_sup_1s("kamaRqalu", Stri, &["kamaRqalUH"]);

    // BAzAyAm
    assert_has_sup_1s("kadru", Stri, &["kadruH"]);
    assert_has_sup_1s("kamaRqalu", Stri, &["kamaRqaluH"]);
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
    assert_has_taddhita("Siva", T::aR, &["SEva"]);
    assert_has_taddhita("prOzWa", T::aR, &["prOzWa"]);

    // apavAda
    assert_has_taddhita("Siva", T::iY, &[]);
    assert_has_taddhita("prOzWa", T::iY, &[]);
}

#[test]
fn sutra_4_1_116() {
    assert_has_taddhita("kanyA", T::aR, &["kAnIna"]);
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
    assert_has_taddhita(&nyap("pIlA"), T::aR, &["pEla"]);
    assert_has_taddhita(&nyap("pIlA"), T::Qak, &["pEleya"]);
}

#[test]
fn sutra_4_1_119() {
    assert_has_artha_taddhita("maRqUka", TasyaApatyam, T::aR, &["mARqUka"]);
    assert_has_artha_taddhita("maRqUka", TasyaApatyam, T::iY, &["mARqUki"]);
    assert_has_artha_taddhita("maRqUka", TasyaApatyam, T::Qak, &["mARqUkeya"]);
}

#[test]
fn sutra_4_1_120() {
    assert_has_taddhita(&nyap("suparRA"), T::Qak, &["sOparReya"]);
    assert_has_taddhita(&nyap("vinatA"), T::Qak, &["vEnateya"]);
    assert_has_taddhita(&nyap("vAqavA"), T::Qak, &["vAqaveya"]);
}

#[test]
fn sutra_4_1_121() {
    assert_has_taddhita(&nyap("dattA"), T::Qak, &["dAtteya"]);
    assert_has_taddhita(&nyap("gopI"), T::Qak, &["gOpeya"]);
    // dvi-acaH?
    // TODO: is yamunA nyAbanta?
    assert_has_taddhita("yamunA", T::Qak, &[]);
    assert_has_taddhita("yamunA", T::aR, &["yAmuna"]);
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
    assert_has_taddhita(&nyap("kulawA"), T::Qak, &["kOlawineya", "kOlaweya"]);
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
