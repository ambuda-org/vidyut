extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::Unadi;

#[test]
fn sutra_5_4_3() {
    assert_has_taddhitanta(&prati("sTUla"), T::kan, &["sTUlaka"]);
    assert_has_taddhitanta(&prati("aRu"), T::kan, &["aRuka"]);
    assert_has_taddhitanta(&prati("mAza"), T::kan, &["mAzaka"]);
    assert_has_taddhitanta(&prati("gomUtra"), T::kan, &["gomUtraka"]);
}

#[test]
fn sutra_5_4_3_v1() {
    assert_has_taddhitanta(&prati("caYcut"), T::kan, &["caYcutka"]);
    assert_has_taddhitanta(&prati("bfhat"), T::kan, &["bfhatka"]);
}

#[test]
fn sutra_5_4_17() {
    assert_has_taddhitanta(&prati("paYcan"), T::kftvasuc, &["paYcakftvas"]);
    assert_has_taddhitanta(&prati("saptan"), T::kftvasuc, &["saptakftvas"]);
    // TODO: saNKyAyAH?

    // For these, see 5.4.18.
    assert_has_taddhitanta(&prati("dvi"), T::kftvasuc, &[]);
    assert_has_taddhitanta(&prati("tri"), T::kftvasuc, &[]);
    assert_has_taddhitanta(&prati("catur"), T::kftvasuc, &[]);
}

#[ignore]
#[test]
fn sutra_5_4_18() {
    assert_has_taddhitanta(&prati("dvi"), T::suc, &["dviH"]);
    assert_has_taddhitanta(&prati("tri"), T::suc, &["triH"]);
    assert_has_taddhitanta(&prati("catur"), T::suc, &["catuH"]);
}

#[ignore]
#[test]
fn sutra_5_4_19() {
    assert_has_taddhitanta(&prati("eka"), T::suc, &["sakft"]);
}

#[test]
fn sutra_5_4_21() {
    assert_has_taddhitanta(&prati("anna"), T::mayaw, &["annamaya"]);
    assert_has_taddhitanta(&prati("apUpa"), T::mayaw, &["apUpamaya"]);
    assert_has_taddhitanta(&prati("vawaka"), T::mayaw, &["vawakamaya"]);
}

#[test]
fn sutra_5_4_23() {
    assert_has_taddhitanta(&prati("ananta"), T::Yya, &["Anantya"]);
    assert_has_taddhitanta(&prati("AvasaTa"), T::Yya, &["AvasaTya"]);
    assert_has_taddhitanta(&prati("itiha"), T::Yya, &["Etihya"]);
    assert_has_taddhitanta(&prati("Bezaja"), T::Yya, &["BEzajya"]);
}

#[test]
fn sutra_5_4_26() {
    assert_has_taddhitanta(&prati("atiTi"), T::Yya, &["AtiTya"]);
}

#[test]
fn sutra_5_4_27() {
    assert_has_taddhitanta("deva", T::tal, &["devatA"]);
}

#[test]
fn sutra_5_4_28() {
    assert_has_taddhitanta("avi", T::ka, &["avika"]);
}

#[test]
fn sutra_5_4_29() {
    assert_has_taddhitanta("yAva", T::ka, &["yAvaka"]);
    assert_has_taddhitanta("maRi", T::ka, &["maRika"]);
}

#[test]
fn sutra_5_4_30_to_sutra_5_4_32() {
    assert_has_taddhitanta("lohita", T::ka, &["lohitaka"]);
}

#[test]
fn sutra_5_4_33() {
    assert_has_taddhitanta("kAla", T::ka, &["kAlaka"]);
}

#[test]
fn sutra_5_4_34() {
    assert_has_taddhitanta("vinaya", T::Wak, &["vEnayika"]);
    assert_has_taddhitanta("samaya", T::Wak, &["sAmayika"]);
    assert_has_taddhitanta("upAya", T::Wak, &["OpAyika"]);
}

#[test]
fn sutra_5_4_35() {
    assert_has_taddhitanta("vAc", T::Wak, &["vAcika"]);
}

#[ignore]
#[test]
fn sutra_5_4_39() {
    assert_has_taddhitanta(&prati("mft"), T::tikan, &["mfttikA"]);
}

#[ignore]
#[test]
fn sutra_5_4_43() {
    assert_has_taddhitanta(&prati("dvi"), T::Sas, &["dviSaH"]);
    assert_has_taddhitanta(&prati("tri"), T::Sas, &["triSaH"]);
    assert_has_taddhitanta(&prati("kArzApaRa"), T::Sas, &["kArzApaRaSaH"]);
    assert_has_taddhitanta(&prati("mAza"), T::Sas, &["mAzaSaH"]);
    assert_has_taddhitanta(&prati("pAda"), T::Sas, &["pAdaSaH"]);
}

#[test]
fn sutra_5_4_44() {
    assert_has_taddhitanta(&prati("vAsudeva"), T::tasi, &["vAsudevatas"]);
    assert_has_taddhitanta(&prati("arjuna"), T::tasi, &["arjunatas"]);

    // Akrtigana
    assert_has_taddhitanta(&prati("Adi"), T::tasi, &["Aditas"]);
    assert_has_taddhitanta(&prati("maDya"), T::tasi, &["maDyatas"]);
    assert_has_taddhitanta(&prati("pArSva"), T::tasi, &["pArSvatas"]);
    assert_has_taddhitanta(&prati("pfzWa"), T::tasi, &["pfzWatas"]);
}

#[test]
fn sutra_5_4_50() {
    assert_has_artha_taddhita("Sukla", AbhutaTadbhava, T::cvi, &["SuklI"]);
    assert_has_artha_taddhita("Gawa", AbhutaTadbhava, T::cvi, &["GawI"]);
}

#[test]
fn sutra_5_4_51() {
    assert_has_artha_taddhita("arus", AbhutaTadbhava, T::cvi, &["arU"]);
    assert_has_artha_taddhita("unmanas", AbhutaTadbhava, T::cvi, &["unmanI"]);
    assert_has_artha_taddhita("uccakzus", AbhutaTadbhava, T::cvi, &["uccakzU"]);
    assert_has_artha_taddhita("vicetas", AbhutaTadbhava, T::cvi, &["vicetI"]);
    assert_has_artha_taddhita("virahas", AbhutaTadbhava, T::cvi, &["virahI"]);
    assert_has_artha_taddhita("virajas", AbhutaTadbhava, T::cvi, &["virajI"]);
}

#[test]
fn sutra_5_4_52() {
    assert_has_artha_taddhita("agni", AbhutaTadbhava, T::sAti, &["agnisAt"]);
    assert_has_artha_taddhita("udaka", AbhutaTadbhava, T::sAti, &["udakasAt"]);
}

#[test]
fn sutra_5_4_54() {
    assert_has_taddhitanta(&prati("agni"), T::sAti, &["agnisAt"]);
    assert_has_taddhitanta(&prati("udaka"), T::sAti, &["udakasAt"]);
}

#[test]
fn sutra_5_4_55() {
    assert_has_artha_taddhita("brAhmaRa", AbhutaTadbhava, T::trA, &["brAhmaRatrA"]);
}

#[test]
fn sutra_5_4_78() {
    assert_has_sasthi_tatpurusha("brahman", "varcas", &["brahmavarcasa"]);
    assert_has_sasthi_tatpurusha("hastin", "varcas", &["hastivarcasa"]);
}

#[test]
fn sutra_5_4_78_v1() {
    assert_has_sasthi_tatpurusha("pallya", "varcas", &["pallyavarcasa"]);
    assert_has_sasthi_tatpurusha("rAjan", "varcas", &["rAjavarcasa"]);
}

#[test]
fn sutra_5_4_79() {
    assert_has_avyaya_tatpurusha("ava", "tamas", &["avatamasa"]);
    assert_has_avyaya_tatpurusha("sam", "tamas", &["santamasa"]);
    // TODO: how to derive this?
    assert_has_misc_tatpurusha("anDa", "tamas", &["anDatamasa"]);
}

#[test]
fn sutra_5_4_80() {
    assert_has_misc_tatpurusha("Svas", "vasIya", &["SvovasIya"]);
    assert_has_misc_tatpurusha("Svas", "Sreyas", &["SvaHSreyasa", "SvaSSreyasa"]);
}

#[test]
fn sutra_5_4_81() {
    assert_has_avyaya_tatpurusha("anu", "rahas", &["anurahasa"]);
    assert_has_avyaya_tatpurusha("ava", "rahas", &["avarahasa"]);
    assert_has_misc_tatpurusha("tapta", "rahas", &["taptarahasa"]);
}

#[test]
fn sutra_5_4_91() {
    assert_has_karmadharaya("mahat", "rAjan", &["mahArAja"]);
    assert_has_sasthi_tatpurusha("madra", "rAjan", &["madrarAja"]);
    assert_has_karmadharaya("parama", "ahan", &["paramAha"]);
    assert_has_karmadharaya("uttama", "ahan", &["uttamAha"]);
    assert_has_sasthi_tatpurusha("rAjan", "saKi", &["rAjasaKa"]);
    assert_has_sasthi_tatpurusha("brAhmaRa", "saKi", &["brAhmaRasaKa"]);

    // TODO: others
}

#[test]
fn sutra_5_4_95() {
    assert_has_sasthi_tatpurusha("grAma", "takzan", &["grAmatakza"]);
    assert_has_sasthi_tatpurusha("kOwa", "takzan", &["kOwatakza"]);

    assert_has_sasthi_tatpurusha("rAjan", "takzan", &["rAjatakzan"]);
}

#[test]
fn sutra_5_4_96() {
    assert_has_avyaya_tatpurusha("ati", "Svan", &["atiSva"]);
}

#[test]
fn sutra_5_4_98() {
    assert_has_karmadharaya("uttama", "sakTi", &["uttamasakTa"]);
    assert_has_karmadharaya("mfga", "sakTi", &["mfgasakTa"]);
    assert_has_karmadharaya("pUrva", "sakTi", &["pUrvasakTa"]);
    assert_has_karmadharaya("Palaka", "sakTi", &["PalakasakTa"]);
}

#[test]
fn sutra_5_4_100() {
    assert_has_misc_tatpurusha("arDa", "nO", &["arDanAva"]);
}

#[test]
fn sutra_5_4_106() {
    assert_has_samahara_dvandva(&["vAc", "tvac"], &["vAktvaca"]);
    assert_has_samahara_dvandva(&["tvac", "sraj"], &["tvaksraja"]);

    assert_has_samahara_dvandva(&["samid", "dfzada"], &["samiddfzada"]);
    assert_has_samahara_dvandva(&["sampad", "dvipad"], &["sampaddvipada"]);

    assert_has_samahara_dvandva(&["vAc", "vipruz"], &["vAgvipruza"]);
    assert_has_samahara_dvandva(&["CAtra", "upAnah"], &["CAtropAnaha"]);
    assert_has_samahara_dvandva(&["Denu", "goduh"], &["Denugoduha"]);

    // TODO: others
}

#[ignore]
#[test]
fn sutra_5_4_108() {
    assert_has_avyayibhava("upa", "rAjan", &["uparAjam"]);
    assert_has_avyayibhava("prati", "rAjan", &["pratirAjam"]);
    assert_has_avyayibhava("aDi", "Atman", &["aDyAtmam"]);
    assert_has_avyayibhava("prati", "Atman", &["pratyAtmam"]);
}

#[test]
fn sutra_5_4_132() {
    assert_has_bahuvrihi("SArNga", "Danus", &["SArNgaDanvan"]);
    assert_has_bahuvrihi("aDijya", "Danus", &["aDijyaDanvan"]);
}

#[ignore]
#[test]
fn sutra_5_4_134() {
    assert_has_bahuvrihi("yuvati", "jAyA", &["yuvajAni"]);
    assert_has_bahuvrihi("vfdDa", "jAyA", &["vfdDajAni"]);
}

#[test]
fn sutra_5_4_135() {
    assert_has_bahuvrihi("ud", "ganDa", &["udganDi"]);
    assert_has_bahuvrihi("pUti", "ganDa", &["pUtiganDi"]);
    assert_has_bahuvrihi("su", "ganDa", &["suganDi"]);
    assert_has_bahuvrihi("suraBi", "ganDa", &["suraBiganDi"]);

    assert_has_bahuvrihi("tIvra", "ganDa", &["tIvraganDa"]);
    // TODO: suganDa in other sense
}

#[test]
fn sutra_5_4_148() {
    assert_has_bahuvrihi("ud", "kAkuda", &["utkAkud"]);
    assert_has_bahuvrihi("vi", "kAkuda", &["vikAkud"]);
}

#[test]
fn sutra_5_4_149() {
    assert_has_bahuvrihi("pUrRa", "kAkuda", &["pUrRakAkud", "pUrRakAkuda"]);
}

#[test]
fn sutra_5_4_150() {
    assert_has_bahuvrihi("su", "hfdaya", &["suhfd", "suhfdaya"]);
}

#[ignore]
#[test]
fn sutra_5_4_151() {
    let sarpis = create_krdanta("sarpis", &[], &d("sf\\px~", Bhvadi), Unadi::isi);
    let upanah = create_krdanta("upAnah", &["upa", "AN"], &d("Ra\\ha~^", Divadi), Krt::kvip);

    assert_has_bahuvrihi("vyUQa", "uras", &["vyUQoraska"]);
    assert_has_bahuvrihi("priya", &sarpis, &["priyasarpizka"]);
    assert_has_bahuvrihi("avamukta", &upanah, &["avamuktopAnatka"]);
}
