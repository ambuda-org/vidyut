extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

fn avyaya(text: &str) -> Pratipadika {
    Pratipadika::avyaya(Slp1String::try_from(text).expect("ok"))
}

#[test]
fn sutra_3_4_12() {
    assert_has_krdanta(&["vi"], &d("Ba\\ja~^", Tudadi), Krt::Ramul, &["viBAjam"]);
    assert_has_krdanta(&["apa"], &d("lu\\px~^", Tudadi), Krt::kamul, &["apalupam"]);
}

#[test]
fn sutra_3_4_17() {
    assert_has_krdanta(&["vi"], &d("sf\\px~", Tudadi), Krt::kasun, &["visfpas"]);
    assert_has_krdanta(&["AN"], &d("u~tfdi~^r", Rudhadi), Krt::kasun, &["Atfdas"]);
}

#[test]
fn sutra_3_4_21() {
    assert_has_krdanta(&[], &d("Bu\\ja~", Rudhadi), Krt::ktvA, &["BuktvA"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), Krt::ktvA, &["pItvA"]);
    assert_has_krdanta(&[], &d("zRA\\", Adadi), Krt::ktvA, &["snAtvA"]);
}

#[test]
fn sutra_3_4_22() {
    let bhuj = d("Bu\\ja~", Rudhadi);
    let paa = d("pA\\", Bhvadi);
    assert_has_krdanta(&[], &bhuj, Krt::Ramul, &["Bojam"]);
    assert_has_krdanta(&[], &bhuj, Krt::ktvA, &["BuktvA"]);
    assert_has_krdanta(&[], &paa, Krt::Ramul, &["pAyam"]);
    assert_has_krdanta(&[], &paa, Krt::ktvA, &["pItvA"]);
}

#[test]
fn sutra_3_4_67() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::Rvul, &["kAraka"]);
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kartf"]);

    assert_has_krdanta(&[], &d("wunadi~", Bhvadi), Krt::lyu, &["nandana"]);
    assert_has_krdanta(&[], &d("graha~^", Kryadi), Krt::Rini, &["grAhin"]);
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::ac, &["paca"]);
}

#[test]
fn sutra_3_4_69() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_ta_k(&[], &gam, Lat, &["gamyate"]);
    assert_has_tip(&[], &gam, Lat, &["gacCati"]);

    let aas = d("Asa~\\", Adadi);
    assert_has_ta_k(&[], &aas, Lat, &["Asyate"]);
    assert_has_ta(&[], &aas, Lat, &["Aste"]);
}

#[test]
fn sutra_3_4_70() {
    let kf = &d("qukf\\Y", Tanadi);
    let bhuj = &d("Bu\\ja~", Rudhadi);
    let aas = &d("Asa~\\", Tanadi);
    let shi = &d("SIN", Adadi);
    let bhu = &d("BU", Bhvadi);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    assert_has_krdanta(&[], &bhuj, Krt::tavya, &["Boktavya"]);
    assert_has_krdanta(&[], &aas, Krt::tavya, &["Asitavya"]);
    assert_has_krdanta(&[], &shi, Krt::tavya, &["Sayitavya"]);
    // kta
    assert_has_krdanta(&[], &kf, Krt::kta, &["kfta"]);
    assert_has_krdanta(&[], &bhuj, Krt::kta, &["Bukta"]);
    assert_has_krdanta(&[], &aas, Krt::kta, &["Asita"]);
    assert_has_krdanta(&[], &shi, Krt::kta, &["Sayita"]);

    assert_has_upapada_krdanta(avyaya("Izat"), &[], &kf, Krt::Kal, &["Izatkara"]);
    assert_has_upapada_krdanta(avyaya("su"), &[], &kf, Krt::Kal, &["sukara"]);
    assert_has_upapada_krdanta(avyaya("dur"), &[], &kf, Krt::Kal, &["duzkara"]);
    assert_has_upapada_krdanta(avyaya("dur"), &[], &kf, Krt::Kal, &["duzkara"]);
    assert_has_krdanta(&["IzadAQya"], bhu, Krt::Kal, &["IzadAQyamBava"]);
    assert_has_krdanta(&["svAQya"], bhu, Krt::Kal, &["svAQyamBava"]);
}

#[test]
fn sutra_3_4_78() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lat, &["pacati"]);
    assert_has_tas(&[], &pac, Lat, &["pacataH"]);
    assert_has_jhi(&[], &pac, Lat, &["pacanti"]);
    assert_has_sip(&[], &pac, Lat, &["pacasi"]);
    assert_has_thas(&[], &pac, Lat, &["pacaTaH"]);
    assert_has_tha(&[], &pac, Lat, &["pacaTa"]);
    assert_has_mip(&[], &pac, Lat, &["pacAmi"]);
    assert_has_vas(&[], &pac, Lat, &["pacAvaH"]);
    assert_has_mas(&[], &pac, Lat, &["pacAmaH"]);
    assert_has_ta(&[], &pac, Lat, &["pacate"]);
    assert_has_aataam(&[], &pac, Lat, &["pacete"]);
    assert_has_jha(&[], &pac, Lat, &["pacante"]);
    assert_has_thaas(&[], &pac, Lat, &["pacase"]);
    assert_has_aathaam(&[], &pac, Lat, &["paceTe"]);
    assert_has_dhvam(&[], &pac, Lat, &["pacaDve"]);
    assert_has_iw(&[], &pac, Lat, &["pace"]);
    assert_has_vahi(&[], &pac, Lat, &["pacAvahe"]);
    assert_has_mahin(&[], &pac, Lat, &["pacAmahe"]);
}

#[test]
fn sutra_3_4_79() {
    // Counterexamples
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::SAnac, &["pacamAna"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::SAnac, &["yajamAna"]);
}

#[test]
fn sutra_3_4_80() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_thaas(&[], &pac, Lat, &["pacase"]);
    assert_has_thaas(&[], &pac, Lit, &["pecize"]);
    assert_has_thaas(&[], &pac, Lut, &["paktAse"]);
    assert_has_thaas(&[], &pac, Lrt, &["pakzyase"]);
}

#[test]
fn sutra_3_4_81() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_ta(&[], &pac, Lit, &["pece"]);
    assert_has_aataam(&[], &pac, Lit, &["pecAte"]);
    assert_has_jha(&[], &pac, Lit, &["pecire"]);
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_ta(&[], &labh, Lit, &["leBe"]);
    assert_has_aataam(&[], &labh, Lit, &["leBAte"]);
    assert_has_jha(&[], &labh, Lit, &["leBire"]);
}

#[test]
fn sutra_3_4_82() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lit, &["papAca"]);
    assert_has_tas(&[], &pac, Lit, &["pecatuH"]);
    assert_has_jhi(&[], &pac, Lit, &["pecuH"]);
    assert_has_sip(&[], &pac, Lit, &["peciTa", "papakTa"]);
    assert_has_thas(&[], &pac, Lit, &["pecaTuH"]);
    assert_has_tha(&[], &pac, Lit, &["peca"]);
    assert_has_mip(&[], &pac, Lit, &["papAca", "papaca"]);
    assert_has_vas(&[], &pac, Lit, &["peciva"]);
    assert_has_mas(&[], &pac, Lit, &["pecima"]);
}

#[test]
fn sutra_3_4_83() {
    let vid = d("vida~", Adadi);
    assert_has_tip(&[], &vid, Lat, &["veda", "vetti"]);
    assert_has_tas(&[], &vid, Lat, &["vidatuH", "vittaH"]);
    assert_has_jhi(&[], &vid, Lat, &["viduH", "vidanti"]);
    assert_has_sip(&[], &vid, Lat, &["vetTa", "vetsi"]);
    assert_has_thas(&[], &vid, Lat, &["vidaTuH", "vitTaH"]);
    assert_has_tha(&[], &vid, Lat, &["vida", "vitTa"]);
    assert_has_mip(&[], &vid, Lat, &["veda", "vedmi"]);
    assert_has_vas(&[], &vid, Lat, &["vidva", "vidvaH"]);
    assert_has_mas(&[], &vid, Lat, &["vidma", "vidmaH"]);
}

#[test]
fn sutra_3_4_84() {
    let bru = d("brUY", Adadi);
    assert_has_tip(&[], &bru, Lat, &["Aha", "bravIti"]);
    assert_has_tas(&[], &bru, Lat, &["AhatuH", "brUtaH"]);
    assert_has_jhi(&[], &bru, Lat, &["AhuH", "bruvanti"]);
    assert_has_sip(&[], &bru, Lat, &["AtTa", "bravIzi"]);
    assert_has_thas(&[], &bru, Lat, &["AhaTuH", "brUTaH"]);
    assert_has_tha(&[], &bru, Lat, &["brUTa"]);
    assert_has_mip(&[], &bru, Lat, &["bravImi"]);
    assert_has_vas(&[], &bru, Lat, &["brUvaH"]);
    assert_has_mas(&[], &bru, Lat, &["brUmaH"]);
}

#[test]
fn sutra_3_4_85() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tas(&[], &pac, Lot, &["pacatAm"]);
    assert_has_thas(&[], &pac, Lot, &["pacatam"]);
    assert_has_tha(&[], &pac, Lot, &["pacata"]);
    assert_has_vas(&[], &pac, Lot, &["pacAva"]);
    assert_has_mas(&[], &pac, Lot, &["pacAma"]);
    //
    assert_has_jhi(&[], &d("yA\\", Adadi), Lot, &["yAntu"]);
    assert_has_jhi(&[], &d("vA\\", Adadi), Lot, &["vAntu"]);
}

#[test]
fn sutra_3_4_86() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lot, &["pacatu", "pacatAt"]);
    assert_has_jhi(&[], &pac, Lot, &["pacantu"]);
}

#[test]
fn sutra_3_4_87() {
    assert_has_sip(&[], &d("lUY", Kryadi), Lot, &["lunIhi", "lunItAt"]);
    assert_has_sip(&[], &d("pUY", Kryadi), Lot, &["punIhi", "punItAt"]);
    assert_has_sip(&[], &d("rA\\Da~", Svadi), Lot, &["rADnuhi", "rADnutAt"]);
    assert_has_sip(
        &[],
        &d("takzU~", Bhvadi),
        Lot,
        &["takzRuhi", "takzRutAt", "takza", "takzatAt"],
    );
}

// 3.4.88 is chAndasa.

#[test]
fn sutra_3_4_89() {
    assert_has_mip(&[], &d("qupa\\ca~^z", Bhvadi), Lot, &["pacAni"]);
    assert_has_mip(&[], &d("paWa~", Bhvadi), Lot, &["paWAni"]);
}

#[test]
fn sutra_3_4_90() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_ta(&[], &pac, Lot, &["pacatAm"]);
    assert_has_aataam(&[], &pac, Lot, &["pacetAm"]);
    assert_has_jha(&[], &pac, Lot, &["pacantAm"]);
}

#[test]
fn sutra_3_4_91() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_thaas(&[], &pac, Lot, &["pacasva"]);
    assert_has_dhvam(&[], &pac, Lot, &["pacaDvam"]);
}

#[test]
fn sutra_3_4_92() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_mip(&[], &kf, Lot, &["karavARi"]);
    assert_has_vas(&[], &kf, Lot, &["karavAva"]);
    assert_has_mas(&[], &kf, Lot, &["karavAma"]);
    assert_has_iw(&[], &kf, Lot, &["karavE"]);
    assert_has_vahi(&[], &kf, Lot, &["karavAvahE"]);
    assert_has_mahin(&[], &kf, Lot, &["karavAmahE"]);
}

#[test]
fn sutra_3_4_93() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_iw(&[], &kf, Lot, &["karavE"]);
    assert_has_vahi(&[], &kf, Lot, &["karavAvahE"]);
    assert_has_mahin(&[], &kf, Lot, &["karavAmahE"]);
}

#[test]
fn sutra_3_4_94() {
    // No `\\` to force parasmaipada
    // -ti forms are not attested but optional by 3.4.97.
    assert_has_tip(&[], &d("juzI~", Tudadi), Let, &["jozizat"]);
    assert_has_tip(&[], &d("tF", Bhvadi), Let, &["tArizat"]);
    assert_has_tip(&[], &d("madi~", Bhvadi), Let, &["mandizat"]);

    assert_has_tip(&[], &d("patx~", Bhvadi), Let, &["patAti"]);
    assert_has_tip(&[], &nic(&d("cyu\\N", Bhvadi)), Let, &["cyAvayAti"]);
}

#[test]
fn sutra_3_4_95() {
    assert_has_aataam(&[], &d("matri~", Curadi), Let, &["mantrayEte"]);
    assert_has_aathaam(&[], &d("matri~", Curadi), Let, &["mantrayETe"]);
    assert_has_aataam(&[], &d("qukf\\Y", Tanadi), Let, &["karavEte"]);
    assert_has_aathaam(&[], &d("qukf\\Y", Tanadi), Let, &["karavETe"]);
}

#[ignore]
#[test]
fn sutra_3_4_96() {
    assert_has_aataam(&[], &d("", Curadi), Let, &["SAsE"]);
    assert_has_aataam(&[], &d("", Curadi), Let, &["ISE"]);
    assert_has_aataam_k(&[], &d("", Curadi), Let, &["gfhyAntE"]);
    assert_has_aataam_k(&[], &d("", Curadi), Let, &["ucyAntE"]);

    // na
    assert_has_aataam_k(&[], &d("", Curadi), Let, &["daDase"]);

    // anyatra?
    assert_has_aataam(&[], &d("matri~", Curadi), Let, &["mantrayEte"]);
    assert_has_aathaam(&[], &d("matri~", Curadi), Let, &["mantrayETe"]);
}

#[test]
fn sutra_3_4_97() {
    // No `\\` to force parasmaipada
    assert_has_tip(&[], &d("juzI~", Tudadi), Let, &["jozizat"]);
    assert_has_tip(&[], &d("tF", Bhvadi), Let, &["tArizat"]);
    assert_has_tip(&[], &d("madi~", Bhvadi), Let, &["mandizat"]);

    assert_has_tip(&[], &d("patx~", Bhvadi), Let, &["patAti"]);
    assert_has_tip(&[], &nic(&d("cyu\\N", Bhvadi)), Let, &["cyAvayAti"]);
}

#[test]
fn sutra_3_4_98() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_vas(&[], &kf, Let, &["karavAva", "karavAvaH"]);
    assert_has_mas(&[], &kf, Let, &["karavAma", "karavAmaH"]);
}

#[test]
fn sutra_3_4_99() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_vas(&[], &pac, Lan, &["apacAva"]);
    assert_has_mas(&[], &pac, Lan, &["apacAma"]);
}

#[test]
fn sutra_3_4_100() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lan, &["apacat"]);
    assert_has_tip(&[], &pac, Lun, &["apAkzIt"]);
    assert_has_vahi(&[], &pac, Lan, &["apacAvahi"]);
    assert_has_mahin(&[], &pac, Lan, &["apacAmahi"]);
}

#[test]
fn sutra_3_4_101() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tas(&[], &pac, Lan, &["apacatAm"]);
    assert_has_thas(&[], &pac, Lan, &["apacatam"]);
    assert_has_tha(&[], &pac, Lan, &["apacata"]);
    assert_has_mip(&[], &pac, Lan, &["apacam"]);
    assert_has_tas(&[], &pac, Lun, &["apAktAm"]);
    assert_has_thas(&[], &pac, Lun, &["apAktam"]);
    assert_has_tha(&[], &pac, Lun, &["apAkta"]);
    assert_has_mip(&[], &pac, Lun, &["apAkzam"]);
}

#[test]
fn sutra_3_4_102() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_ta(&[], &pac, VidhiLin, &["paceta"]);
    assert_has_aataam(&[], &pac, VidhiLin, &["paceyAtAm"]);
    assert_has_jha(&[], &pac, VidhiLin, &["paceran"]);
    assert_has_ta(&[], &pac, AshirLin, &["pakzIzwa"]);
    assert_has_aataam(&[], &pac, AshirLin, &["pakzIyAstAm"]);
    assert_has_jha(&[], &pac, AshirLin, &["pakzIran"]);
}

#[test]
fn sutra_3_4_103() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tip(&[], &kf, VidhiLin, &["kuryAt"]);
    assert_has_tas(&[], &kf, VidhiLin, &["kuryAtAm"]);
    assert_has_jhi(&[], &kf, VidhiLin, &["kuryuH"]);
    // Counterexamples
    assert_has_mip(&[], &d("ci\\Y", Svadi), Lan, &["acinavam"]);
    assert_has_mip(&[], &kf, Lan, &["akaravam"]);
}

#[test]
fn sutra_3_4_104() {
    let iz = d("izu~", Tudadi);
    assert_has_tip(&[], &iz, AshirLin, &["izyAt"]);
    assert_has_tas(&[], &iz, AshirLin, &["izyAstAm"]);
    assert_has_jhi(&[], &iz, AshirLin, &["izyAsuH"]);
    let jagf = d("jAgf", Adadi);
    assert_has_tip(&[], &jagf, AshirLin, &["jAgaryAt"]);
    assert_has_tas(&[], &jagf, AshirLin, &["jAgaryAstAm"]);
    assert_has_jhi(&[], &jagf, AshirLin, &["jAgaryAsuH"]);
    // Counterexamples
    assert_has_tip(&[], &d("va\\ca~", Adadi), VidhiLin, &["vacyAt"]);
    assert_has_tip(&[], &jagf, VidhiLin, &["jAgfyAt"]);
}

#[test]
fn sutra_3_4_105() {
    assert_has_jha(&[], &d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceran"]);
    assert_has_jha(&[], &d("ya\\ja~^", Bhvadi), VidhiLin, &["yajeran"]);
    assert_has_jha(&[], &d("qukf\\Y", Tanadi), AshirLin, &["kfzIran"]);
}

#[test]
fn sutra_3_4_106() {
    let assert_has_iw = |dhatu, lakara, exp| {
        assert_has_iw(&[], &dhatu, lakara, exp);
    };
    assert_has_iw(d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceya"]);
    assert_has_iw(d("ya\\ja~^", Bhvadi), VidhiLin, &["yajeya"]);
    assert_has_iw(d("qukf\\Y", Tanadi), AshirLin, &["kfzIya"]);
    assert_has_iw(d("hf\\Y", Bhvadi), AshirLin, &["hfzIya"]);
}

#[test]
fn sutra_3_4_107() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_ta(&[], &kf, AshirLin, &["kfzIzwa"]);
    assert_has_aataam(&[], &kf, AshirLin, &["kfzIyAstAm"]);
    assert_has_thaas(&[], &kf, AshirLin, &["kfzIzWAH"]);
    assert_has_aathaam(&[], &kf, AshirLin, &["kfzIyAsTAm"]);
}

#[test]
fn sutra_3_4_108() {
    assert_has_jhi(&[], &d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceyuH"]);
    assert_has_jhi(&[], &d("ya\\ja~^", Bhvadi), VidhiLin, &["yajeyuH"]);
}

#[test]
fn sutra_3_4_109() {
    assert_has_jhi(&[], &d("qukf\\Y", Tanadi), Lun, &["akArzuH"]);
    assert_has_jhi(&[], &d("hf\\Y", Bhvadi), Lun, &["ahArzuH"]);
    assert_has_jhi(&[], &d("YiBI\\", Juhotyadi), Lan, &["abiBayuH"]);
    assert_has_jhi(&[], &d("hrI\\", Juhotyadi), Lan, &["ajihrayuH"]);
    assert_has_jhi(&[], &d("jAgf", Adadi), Lan, &["ajAgaruH"]);
    assert_has_jhi(&[], &d("vida~", Adadi), Lan, &["aviduH"]);
}

#[test]
fn sutra_3_4_110() {
    assert_has_jhi(&[], &d("qudA\\Y", Juhotyadi), Lun, &["aduH"]);
    assert_has_jhi(&[], &d("quDA\\Y", Juhotyadi), Lun, &["aDuH"]);
    assert_has_jhi(&[], &d("zWA\\", Bhvadi), Lun, &["asTuH"]);

    // AtaH, sic-luk
    assert_has_jhi(&[], &d("BU", Bhvadi), Lun, &["aBUvan"]);
    assert_has_jhi(&[], &d("qukf\\Y", Tanadi), Lun, &["akArzuH"]);
    assert_has_jhi(&[], &d("hf\\Y", Bhvadi), Lun, &["ahArzuH"]);
}

#[test]
fn sutra_3_4_111() {
    let yaa = d("yA\\", Adadi);
    assert_has_jhi(&[], &yaa, Lan, &["ayuH", "ayAn"]);
    assert_has_jhi(&[], &d("dA\\p", Adadi), Lan, &["aduH", "adAn"]);

    assert_has_jhi(&[], &yaa, Lot, &["yAntu"]);
    assert_has_jhi(&[], &d("vA\\", Adadi), Lot, &["vAntu"]);

    // not for Lot
    assert_has_jhi(&[], &d("YiBI\\", Juhotyadi), Lot, &["biByatu"]);
    assert_has_jhi(&[], &d("jAgf", Adadi), Lot, &["jAgratu"]);
    assert_has_jhi(&[], &d("vida~", Adadi), Lot, &["vidantu", "vidANkurvantu"]);
}

#[test]
fn sutra_3_4_112() {
    assert_has_jhi(&[], &d("dvi\\za~^", Adadi), Lan, &["advizuH", "advizan"]);
}

#[test]
fn sutra_3_4_113() {
    assert_has_tip(&[], &d("BU", Bhvadi), Lat, &["Bavati"]);
    assert_has_tip(&[], &d("RI\\Y", Bhvadi), Lat, &["nayati"]);
    assert_has_tip(&[], &d("Yizva\\pa~", Adadi), Lat, &["svapiti"]);
    assert_has_tip(&[], &d("rudi~r", Adadi), Lat, &["roditi"]);
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::SAnac, &["pacamAna"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::SAnac, &["yajamAna"]);
}

#[test]
fn sutra_3_4_114() {
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::tfc, &["lavitf"]);
    assert_has_krdanta(&[], &lu, Krt::tumun, &["lavitum"]);
    assert_has_krdanta(&[], &lu, Krt::tavya, &["lavitavya"]);

    // DAtoH
    assert_has_taddhita("vfkza", T::tva, &["vfkzatva"]);
    assert_has_taddhita("vfkza", T::tal, &["vfkzatA"]);
    assert_has_sup_3d("lU", Pum, &["lUByAm"]);
    assert_has_sup_3p("lU", Pum, &["lUBiH"]);
    assert_has_lat(&[], &d("gupa~\\", Bhvadi), &["jugupsate"]);
}

#[test]
fn sutra_3_4_115() {
    assert_has_sip(&[], &d("qupa\\ca~^z", Adadi), Lit, &["peciTa", "papakTa"]);
    assert_has_sip(&[], &d("Sa\\kx~", Adadi), Lit, &["SekiTa", "SaSakTa"]);
    assert_has_ta_k(&[], &d("glE\\", Bhvadi), Lit, &["jagle"]);
    assert_has_ta_k(&[], &d("mlE\\", Bhvadi), Lit, &["mamle"]);
}

#[test]
fn sutra_3_4_116() {
    let lu = d("lUY", Gana::Kryadi);
    let pu = d("pUY", Gana::Kryadi);

    assert_has_ta(&[], &lu, AshirLin, &["lavizIzwa"]);
    assert_has_ta(&[], &pu, AshirLin, &["pavizIzwa"]);

    assert_has_tip(&[], &lu, VidhiLin, &["lunIyAt"]);
    assert_has_tip(&[], &pu, VidhiLin, &["punIyAt"]);
}

// 3.4.117 is chAndasa.
