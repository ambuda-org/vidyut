extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

pub fn assert_has_lat_p_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lat)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lit_p_1d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Uttama)
        .vacana(Vacana::Dvi)
        .lakara(Lakara::Lit)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();

    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lot_2s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lot)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lot_p_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lot)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lan_p_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lan)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lat_a_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lat)
        .pada(Pada::Atmane)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lot_a_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lot)
        .pada(Pada::Atmane)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lan_a_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lan)
        .pada(Pada::Atmane)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

#[test]
fn sutra_7_1_1() {
    let nand = Dhatu::new("wunadi~", Gana::Bhvadi);
    assert_has_krdanta(&[], &nand, Krt::lyu, &["nandana"]);

    let ram = Dhatu::new("ra\\ma~\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &ram, Krt::lyu, &["ramaRa"]);

    let kf = Dhatu::new("qukf\\Y", Gana::Tanadi);
    assert_has_krdanta(&[], &kf, Krt::Rvul, &["kAraka"]);

    let hf = Dhatu::new("hf\\Y", Gana::Bhvadi);
    assert_has_krdanta(&[], &hf, Krt::Rvul, &["hAraka"]);
}

#[test]
fn sutra_7_1_3() {
    let d = Dhatu::new;
    assert_has_lat_p_3p(&[], &d("qukf\\Y", Gana::Tanadi), &["kurvanti"]);
    assert_has_lat_p_3p(&[], &d("zu\\Y", Gana::Svadi), &["sunvanti"]);
    assert_has_lat_p_3p(&[], &d("ci\\Y", Gana::Svadi), &["cinvanti"]);

    // TODO: do the rest.
}

#[test]
fn sutra_7_1_4() {
    let d = Dhatu::new;

    let daa = d("qudA\\Y", Gana::Juhotyadi);
    assert_has_lat_p_3p(&[], &daa, &["dadati"]);
    assert_has_lot_p_3p(&[], &daa, &["dadatu"]);

    let dhaa = d("quDA\\Y", Gana::Juhotyadi);
    assert_has_lat_p_3p(&[], &dhaa, &["daDati"]);
    assert_has_lot_p_3p(&[], &dhaa, &["daDatu"]);

    let jaks = d("jakza~", Gana::Adadi);
    assert_has_lat_p_3p(&[], &jaks, &["jakzati"]);
    assert_has_lot_p_3p(&[], &jaks, &["jakzatu"]);

    let jaagr = d("jAgf", Gana::Adadi);
    assert_has_lat_p_3p(&[], &jaagr, &["jAgrati"]);
    assert_has_lot_p_3p(&[], &jaagr, &["jAgratu"]);

    assert_has_lan_p_3p(&[], &dhaa, &["adaDuH"]);
    assert_has_lan_p_3p(&[], &jaagr, &["ajAgaruH"]);
}

#[test]
fn sutra_7_1_5() {
    let ci = Dhatu::new("ci\\Y", Gana::Svadi);
    assert_has_lat_a_3p(&[], &ci, &["cinvate"]);
    assert_has_lot_a_3p(&[], &ci, &["cinvatAm"]);
    assert_has_lan_a_3p(&[], &ci, &["acinvata"]);

    assert_has_lat_a_3p(&[], &Dhatu::new("pUY", Gana::Kryadi), &["punate"]);

    let lu = Dhatu::new("lUY", Gana::Kryadi);
    assert_has_lat_a_3p(&[], &lu, &["lunate"]);
    assert_has_lot_a_3p(&[], &lu, &["lunatAm"]);
    assert_has_lan_a_3p(&[], &lu, &["alunata"]);

    assert_has_lat_a_3p(&[], &Dhatu::new("cyu\\N", Gana::Bhvadi), &["cyavante"]);
    assert_has_lat_a_3p(&[], &Dhatu::new("plu\\N", Gana::Bhvadi), &["plavante"]);
}

#[test]
fn sutra_7_1_6() {
    let shi = Dhatu::new("SIN", Gana::Adadi);
    assert_has_lat_a_3p(&[], &shi, &["Serate"]);
    assert_has_lot_a_3p(&[], &shi, &["SeratAm"]);
    assert_has_lan_a_3p(&[], &shi, &["aSerata"]);
}

#[test]
fn sutra_7_1_7() {
    let vid = Dhatu::new("vida~", Gana::Adadi);
    assert_has_lat_a_3p(&["sam"], &vid, &["saMvidate", "saMvidrate"]);
    assert_has_lot_a_3p(
        &["sam"],
        &vid,
        &["saMvidatAm", "saMvidratAm", "saMvidANkurvatAm"],
    );
    assert_has_lan_a_3p(&["sam"], &vid, &["samavidata", "samavidrata"]);
}

#[test]
fn sutra_7_1_8() {
    assert_has_subantas("vfkza", Pum, Trtiya, Bahu, &["vfkzEH"]);
    assert_has_subantas("plakza", Pum, Trtiya, Bahu, &["plakzEH"]);
    assert_has_subantas("atijarasa", Pum, Trtiya, Bahu, &["atijarasEH"]);
}

#[ignore]
#[test]
fn sutra_7_1_11() {
    assert_has_subantas("idam", Pum, Trtiya, Bahu, &["eBiH"]);
    assert_has_subantas("adas", Pum, Trtiya, Bahu, &["adBiH"]);
    // TODO: imaka, amuka
}

#[test]
fn sutra_7_1_12() {
    assert_has_subantas("vfkza", Pum, Trtiya, Eka, &["vfkzeRa"]);
    assert_has_subantas("plakza", Pum, Trtiya, Eka, &["plakzeRa"]);
    assert_has_subantas("vfkza", Pum, Panchami, Eka, &["vfkzAt"]);
    assert_has_subantas("plakza", Pum, Panchami, Eka, &["plakzAt"]);
    assert_has_subantas("vfkza", Pum, Sasthi, Eka, &["vfkzasya"]);
    assert_has_subantas("plakza", Pum, Sasthi, Eka, &["plakzasya"]);

    assert_has_subantas("saKi", Pum, Trtiya, Eka, &["saKyA"]);
    // TODO: patyA
}

#[test]
fn sutra_7_1_13() {
    assert_has_subantas("vfkza", Pum, Caturthi, Eka, &["vfkzAya"]);
    assert_has_subantas("plakza", Pum, Caturthi, Eka, &["plakzAya"]);

    assert_has_subantas("saKi", Pum, Caturthi, Eka, &["saKye"]);
    // TODO: patye
}

#[ignore]
#[test]
fn sutra_7_1_14() {
    assert_has_subantas("sarva", Pum, Caturthi, Eka, &["sarvasmE"]);
    assert_has_subantas("viSva", Pum, Caturthi, Eka, &["viSvasmE"]);
    assert_has_subantas("yad", Pum, Caturthi, Eka, &["yasmE"]);
    assert_has_subantas("tad", Pum, Caturthi, Eka, &["tasmE"]);
    assert_has_subantas("kim", Pum, Caturthi, Eka, &["kasmE"]);
}

#[ignore]
#[test]
fn sutra_7_1_15() {
    assert_has_subantas("sarva", Pum, Panchami, Eka, &["sarvasmAt"]);
    assert_has_subantas("viSva", Pum, Panchami, Eka, &["viSvasmAt"]);
    assert_has_subantas("yad", Pum, Panchami, Eka, &["yasmAt"]);
    assert_has_subantas("tad", Pum, Panchami, Eka, &["tasmAt"]);
    assert_has_subantas("kim", Pum, Panchami, Eka, &["kasmAt"]);

    assert_has_subantas("sarva", Pum, Panchami, Eka, &["sarvasmin"]);
    assert_has_subantas("viSva", Pum, Panchami, Eka, &["viSvasmin"]);
    assert_has_subantas("yad", Pum, Panchami, Eka, &["yasmin"]);
    assert_has_subantas("tad", Pum, Panchami, Eka, &["tasmin"]);
    assert_has_subantas("anya", Pum, Panchami, Eka, &["anyasmin"]);
}

#[test]
fn sutra_7_1_16() {
    assert_has_subantas("pUrva", Pum, Panchami, Eka, &["pUrvasmAt", "pUrvAt"]);
    assert_has_subantas("pUrva", Pum, Saptami, Eka, &["pUrvasmin", "pUrve"]);
    assert_has_subantas("para", Pum, Panchami, Eka, &["parasmAt", "parAt"]);
    assert_has_subantas("para", Pum, Saptami, Eka, &["parasmin", "pare"]);
    assert_has_subantas("avara", Pum, Panchami, Eka, &["avarasmAt", "avarAt"]);
    assert_has_subantas("avara", Pum, Saptami, Eka, &["avarasmin", "avare"]);
    assert_has_subantas("dakziRa", Pum, Panchami, Eka, &["dakziRasmAt", "dakziRAt"]);
    assert_has_subantas("dakziRa", Pum, Saptami, Eka, &["dakziRasmin", "dakziRe"]);
    assert_has_subantas("uttara", Pum, Panchami, Eka, &["uttarasmAt", "uttarAt"]);
    assert_has_subantas("uttara", Pum, Saptami, Eka, &["uttarasmin", "uttare"]);
    assert_has_subantas("apara", Pum, Panchami, Eka, &["aparasmAt", "aparAt"]);
    assert_has_subantas("apara", Pum, Saptami, Eka, &["aparasmin", "apare"]);
    assert_has_subantas("aDara", Pum, Panchami, Eka, &["aDarasmAt", "aDarAt"]);
    assert_has_subantas("aDara", Pum, Saptami, Eka, &["aDarasmin", "aDare"]);
    assert_has_subantas("sva", Pum, Panchami, Eka, &["svasmAt", "svAt"]);
    assert_has_subantas("sva", Pum, Saptami, Eka, &["svasmin", "sve"]);
    assert_has_subantas("antara", Pum, Panchami, Eka, &["antarasmAt", "antarAt"]);
    assert_has_subantas("antara", Pum, Saptami, Eka, &["antarasmin", "antare"]);
}

#[ignore]
#[test]
fn sutra_7_1_17() {
    assert_has_subantas("sarva", Pum, Prathama, Bahu, &["sarve"]);
    assert_has_subantas("viSva", Pum, Prathama, Bahu, &["viSve"]);
    assert_has_subantas("yad", Pum, Prathama, Bahu, &["ye"]);
    assert_has_subantas("tad", Pum, Prathama, Bahu, &["te"]);
    assert_has_subantas("kim", Pum, Prathama, Bahu, &["ke"]);
}

#[test]
fn sutra_7_1_19() {
    assert_has_subantas("kuRqa", Napumsaka, Prathama, Dvi, &["kuRqe"]);
    assert_has_subantas("kuRqa", Napumsaka, Dvitiya, Dvi, &["kuRqe"]);
    assert_has_subantas("daDi", Napumsaka, Prathama, Dvi, &["daDinI"]);
    assert_has_subantas("maDu", Napumsaka, Prathama, Dvi, &["maDunI"]);
    assert_has_subantas("trapu", Napumsaka, Prathama, Dvi, &["trapuRI"]);
    assert_has_subantas("jatu", Napumsaka, Prathama, Dvi, &["jatunI"]);
}

#[test]
fn sutra_7_1_20() {
    assert_has_subantas("kuRqa", Napumsaka, Prathama, Bahu, &["kuRqAni"]);
    assert_has_subantas("kuRqa", Napumsaka, Dvitiya, Bahu, &["kuRqAni"]);
    assert_has_subantas("daDi", Napumsaka, Prathama, Bahu, &["daDIni"]);
    assert_has_subantas("maDu", Napumsaka, Prathama, Bahu, &["maDUni"]);
    assert_has_subantas("trapu", Napumsaka, Prathama, Bahu, &["trapURi"]);
    assert_has_subantas("jatu", Napumsaka, Prathama, Bahu, &["jatUni"]);
}

#[ignore]
#[test]
fn sutra_7_1_21() {
    assert_has_subantas("azwan", Pum, Prathama, Bahu, &["azwO"]);
    assert_has_subantas("azwan", Pum, Dvitiya, Bahu, &["azwO"]);
}

#[test]
fn sutra_7_1_22() {
    assert_has_subantas("zaz", Pum, Prathama, Bahu, &["zaw"]);
    assert_has_subantas("zaz", Pum, Dvitiya, Bahu, &["zaw"]);
}

#[test]
fn sutra_7_1_23() {
    assert_has_subantas("daDi", Napumsaka, Prathama, Eka, &["daDi"]);
    assert_has_subantas("daDi", Napumsaka, Dvitiya, Eka, &["daDi"]);
    assert_has_subantas("maDu", Napumsaka, Prathama, Eka, &["maDu"]);
    assert_has_subantas("maDu", Napumsaka, Dvitiya, Eka, &["maDu"]);
    assert_has_subantas("trapu", Napumsaka, Prathama, Eka, &["trapu"]);
    assert_has_subantas("jatu", Napumsaka, Prathama, Eka, &["jatu"]);
}

#[test]
fn sutra_7_1_24() {
    assert_has_subantas("kuRqa", Napumsaka, Prathama, Eka, &["kuRqam"]);
    assert_has_subantas("kuRqa", Napumsaka, Dvitiya, Eka, &["kuRqam"]);
}

#[ignore]
#[test]
fn sutra_7_1_25() {
    assert_has_subantas("katara", Napumsaka, Prathama, Eka, &["katarat"]);
    assert_has_subantas("katara", Napumsaka, Dvitiya, Eka, &["katarat"]);
    assert_has_subantas("katama", Napumsaka, Prathama, Eka, &["katamat"]);
    assert_has_subantas("katama", Napumsaka, Dvitiya, Eka, &["katamat"]);
    assert_has_subantas("itara", Napumsaka, Prathama, Eka, &["itarat"]);
    assert_has_subantas("anyatara", Napumsaka, Prathama, Eka, &["anyatarat"]);
    assert_has_subantas("anya", Napumsaka, Prathama, Eka, &["anyat"]);

    assert_has_subantas("nema", Napumsaka, Prathama, Eka, &["nemam"]);
    assert_has_subantas("nema", Napumsaka, Dvitiya, Eka, &["nemam"]);
}

#[ignore]
#[test]
fn sutra_7_1_27() {
    assert_has_subantas("asmad", Pum, Sasthi, Eka, &["mama"]);
    assert_has_subantas("yuzmad", Pum, Sasthi, Eka, &["tava"]);
}

#[test]
fn sutra_7_1_28() {
    assert_has_subantas("asmad", Pum, Caturthi, Eka, &["mahyam"]);
    assert_has_subantas("yuzmad", Pum, Caturthi, Eka, &["tuByam"]);

    assert_has_subantas("yuzmad", Pum, Prathama, Eka, &["tvam"]);
    assert_has_subantas("asmad", Pum, Prathama, Eka, &["aham"]);
    assert_has_subantas("yuzmad", Pum, Prathama, Dvi, &["yuvAm"]);
    assert_has_subantas("asmad", Pum, Prathama, Dvi, &["AvAm"]);
    assert_has_subantas("yuzmad", Pum, Prathama, Bahu, &["yUyam"]);
    assert_has_subantas("asmad", Pum, Prathama, Bahu, &["vayam"]);
    assert_has_subantas("yuzmad", Pum, Dvitiya, Eka, &["tvAm"]);
    assert_has_subantas("asmad", Pum, Dvitiya, Eka, &["mAm"]);
    assert_has_subantas("yuzmad", Pum, Dvitiya, Dvi, &["yuvAm"]);
    assert_has_subantas("asmad", Pum, Dvitiya, Dvi, &["AvAm"]);
}

#[test]
fn sutra_7_1_29() {
    assert_has_subantas("asmad", Pum, Dvitiya, Bahu, &["asmAn"]);
    assert_has_subantas("yuzmad", Pum, Dvitiya, Bahu, &["yuzmAn"]);
}

#[ignore]
#[test]
fn sutra_7_1_30() {
    assert_has_subantas("asmad", Pum, Caturthi, Bahu, &["asmaByam"]);
    assert_has_subantas("yuzmad", Pum, Caturthi, Bahu, &["yuzmaByam"]);
}

#[test]
fn sutra_7_1_31() {
    assert_has_subantas("asmad", Pum, Panchami, Bahu, &["asmat"]);
    assert_has_subantas("yuzmad", Pum, Panchami, Bahu, &["yuzmat"]);
}

#[test]
fn sutra_7_1_32() {
    assert_has_subantas("asmad", Pum, Panchami, Eka, &["mat"]);
    assert_has_subantas("yuzmad", Pum, Panchami, Eka, &["tvat"]);
}

#[ignore]
#[test]
fn sutra_7_1_33() {
    assert_has_subantas("asmad", Pum, Sasthi, Bahu, &["asmAkam"]);
    assert_has_subantas("yuzmad", Pum, Sasthi, Bahu, &["yuzmAkam"]);
}

#[test]
fn sutra_7_1_34() {
    let d = Dhatu::new;
    assert_has_lit_p(&[], &d("pE\\", Gana::Bhvadi), &["papO"]);
    assert_has_lit_p(&[], &d("zWA\\", Gana::Bhvadi), &["tasTO"]);
    assert_has_lit_p(&[], &d("glE\\", Gana::Bhvadi), &["jaglO"]);
    assert_has_lit_p(&[], &d("mlE\\", Gana::Bhvadi), &["mamlO"]);
}

#[test]
fn sutra_7_1_35() {
    let d = Dhatu::new;
    assert_has_lot(&[], &d("jIva~", Gana::Bhvadi), &["jIvatAt", "jIvatu"]);
    assert_has_lot_2s(&[], &d("jIva~", Gana::Bhvadi), &["jIvatAt", "jIva"]);
    assert_has_lot_p(&[], &d("brUY", Gana::Adadi), &["brUtAt", "bravItu"]);
}

#[test]
fn sutra_7_1_36() {
    let vid = Dhatu::new("vida~", Gana::Adadi);
    assert_has_krdanta(&[], &vid, Krt::Satf, &["vidat", "vidvas"]);
}

// 7.1.38 - 7.1.50 are *chandasi*.

#[ignore]
#[test]
fn sutra_7_1_52() {
    assert_has_subantas("sarva", Pum, Sasthi, Bahu, &["sarvezAm"]);
    assert_has_subantas("viSva", Pum, Sasthi, Bahu, &["viSvezAm"]);
    assert_has_subantas("yad", Pum, Sasthi, Bahu, &["yezAm"]);
    assert_has_subantas("tad", Pum, Sasthi, Bahu, &["tezAm"]);

    assert_has_subantas("sarva", Stri, Sasthi, Bahu, &["sarvAsAm"]);
    assert_has_subantas("yad", Stri, Sasthi, Bahu, &["yAsAm"]);
    assert_has_subantas("tad", Stri, Sasthi, Bahu, &["tAsAm"]);

    assert_has_subantas("Bavat", Pum, Sasthi, Bahu, &["BavatAm"]);
}

#[ignore]
#[test]
fn sutra_7_1_53() {
    assert_has_subantas("tri", Pum, Sasthi, Bahu, &["trayARAm"]);
}

#[test]
fn sutra_7_1_54() {
    assert_has_subantas("plakza", Pum, Sasthi, Bahu, &["plakzARAm"]);
    assert_has_subantas("agni", Pum, Sasthi, Bahu, &["agnInAm"]);
    assert_has_subantas("vAyu", Pum, Sasthi, Bahu, &["vAyUnAm"]);
    assert_has_subantas("kartf", Pum, Sasthi, Bahu, &["kartFRAm"]);

    // TODO: nadI, Ap
}

#[ignore]
#[test]
fn sutra_7_1_55() {
    assert_has_subantas("zaz", Pum, Sasthi, Bahu, &["zaRRAm"]);
    assert_has_subantas("paYcan", Pum, Sasthi, Bahu, &["paYcAnAm"]);
    assert_has_subantas("saptan", Pum, Sasthi, Bahu, &["saptAnAm"]);
    assert_has_subantas("navan", Pum, Sasthi, Bahu, &["navAnAm"]);
    assert_has_subantas("daSan", Pum, Sasthi, Bahu, &["daSAnAm"]);
    assert_has_subantas("catur", Pum, Sasthi, Bahu, &["caturRAm"]);
}

#[test]
fn sutra_7_1_58() {
    let kund = Dhatu::new("kuqi~\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &kund, Krt::tfc, &["kuRqitf"]);
    assert_has_krdanta(&[], &kund, Krt::tumun, &["kuRqitum"]);
    assert_has_krdanta(&[], &kund, Krt::tavya, &["kuRqitavya"]);

    let hund = Dhatu::new("huqi~\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &hund, Krt::tfc, &["huRqitf"]);
    assert_has_krdanta(&[], &hund, Krt::tumun, &["huRqitum"]);
    assert_has_krdanta(&[], &hund, Krt::tavya, &["huRqitavya"]);

    // TODO: kundA, hundA
}

#[test]
fn sutra_7_1_59() {
    let d = Dhatu::new;
    let muc = d("mu\\cx~^", Gana::Tudadi);
    assert_has_lat_p(&[], &muc, &["muYcati"]);
    assert_has_lat_p(&[], &d("lu\\px~^", Gana::Tudadi), &["lumpati"]);
    assert_has_lat_p(&[], &d("vidx~^", Gana::Tudadi), &["vindati"]);
    assert_has_lat_p(&[], &d("li\\pa~^", Gana::Tudadi), &["limpati"]);
    assert_has_lat_p(&[], &d("zi\\ca~^", Gana::Tudadi), &["siYcati"]);
    assert_has_lat_p(&[], &d("kftI~", Gana::Tudadi), &["kfntati"]);
    assert_has_lat_p(&[], &d("Ki\\da~", Gana::Tudadi), &["Kindati"]);
    assert_has_lat_p(&[], &d("piSa~", Gana::Tudadi), &["piMSati"]);

    assert_has_krdanta(&[], &muc, Krt::tfc, &["moktf"]);
    assert_has_krdanta(&[], &muc, Krt::tumun, &["moktum"]);
    assert_has_krdanta(&[], &muc, Krt::tavya, &["moktavya"]);

    assert_has_lat_p(&[], &d("tu\\da~^", Gana::Tudadi), &["tudati"]);
    assert_has_lat_p(&[], &d("Ru\\da~^", Gana::Tudadi), &["nudati"]);

    // TODO: varttika
}

#[test]
fn sutra_7_1_60() {
    let masj = Dhatu::new("wuma\\sjo~", Gana::Tudadi);
    assert_has_krdanta(&[], &masj, Krt::tfc, &["maNktf"]);
    assert_has_krdanta(&[], &masj, Krt::tumun, &["maNktum"]);
    assert_has_krdanta(&[], &masj, Krt::tavya, &["maNktavya"]);

    let nas = Dhatu::new("Ra\\Sa~", Gana::Divadi);
    assert_has_krdanta(&[], &nas, Krt::tfc, &["naMzwf", "naSitf"]);
    assert_has_krdanta(&[], &nas, Krt::tumun, &["naMzwum", "naSitum"]);
    assert_has_krdanta(&[], &nas, Krt::tavya, &["naMzwavya", "naSitavya"]);

    assert_has_krdanta(&[], &masj, Krt::lyuw, &["majjana"]);
    assert_has_krdanta(&[], &nas, Krt::lyuw, &["naSana"]);

    // TODO: enable these.
    // assert_has_krdanta(&[], &masj, Krt::kta, &["magna"]);
    // assert_has_krdanta(&[], &masj, Krt::ktavatu, &["magnavat"]);
}

#[test]
fn sutra_7_1_61() {
    let radh = Dhatu::new("ra\\Da~", Gana::Divadi);
    let jabh = Dhatu::new("jaBI~\\", Gana::Bhvadi);
    assert_has_lat_p(
        &[],
        &radh.clone().with_sanadi(&[Sanadi::Nic]),
        &["ranDayati"],
    );
    assert_has_krdanta(&[], &radh, Krt::Rvul, &["ranDaka"]);
    assert_has_krdanta(&[], &radh, Krt::GaY, &["ranDa"]);

    assert_has_lat_p(
        &[],
        &jabh.clone().with_sanadi(&[Sanadi::Nic]),
        &["jamBayati"],
    );
    assert_has_krdanta(&[], &jabh, Krt::Rvul, &["jamBaka"]);
    assert_has_krdanta(&[], &jabh, Krt::GaY, &["jamBa"]);

    assert_has_krdanta(&[], &radh, Krt::tfc, &["radDf", "raDitf"]);
    assert_has_krdanta(&[], &jabh, Krt::yat, &["jaBya"]);
    // TODO: other radh/jabh padas
}

#[test]
fn sutra_7_1_62() {
    let radh = Dhatu::new("ra\\Da~", Gana::Divadi);
    assert_has_krdanta(&[], &radh, Krt::tfc, &["radDf", "raDitf"]);
    assert_has_krdanta(&[], &radh, Krt::tumun, &["raDitum", "radDum"]);
    assert_has_krdanta(&[], &radh, Krt::tavya, &["raDitavya", "radDavya"]);
    assert_has_krdanta(&[], &radh, Krt::lyuw, &["ranDana"]);
    assert_has_krdanta(&[], &radh, Krt::Rvul, &["ranDaka"]);
    assert_has_lit_p_1d(&[], &radh, &["raranDiva"]);
    // TODO: other radh padas
}

#[test]
fn sutra_7_1_63() {
    let rabh = Dhatu::new("ra\\Ba~\\", Gana::Bhvadi);
    assert_has_lat_p(
        &["AN"],
        &rabh.clone().with_sanadi(&[Sanadi::Nic]),
        &["AramBayati"],
    );
    assert_has_krdanta(&["AN"], &rabh, Krt::Rvul, &["AramBaka"]);
    assert_has_krdanta(&["AN"], &rabh, Krt::GaY, &["AramBa"]);
    assert_has_lat(&["AN"], &rabh, &["AraBate"]);
    assert_has_lit(&["AN"], &rabh, &["AreBe"]);
    assert_has_krdanta(&["AN"], &rabh, Krt::tfc, &["ArabDf"]);
    // TODO: other rabh padas
}

#[test]
fn sutra_7_1_64() {
    let labh = Dhatu::new("qula\\Ba~\\z", Gana::Bhvadi);
    assert_has_lat_p(
        &[],
        &labh.clone().with_sanadi(&[Sanadi::Nic]),
        &["lamBayati"],
    );
    assert_has_krdanta(&[], &labh, Krt::Rvul, &["lamBaka"]);
    assert_has_krdanta(&[], &labh, Krt::GaY, &["lamBa"]);
    assert_has_lat(&[], &labh, &["laBate"]);
    assert_has_lit(&[], &labh, &["leBe"]);
    assert_has_krdanta(&[], &labh, Krt::tfc, &["labDf"]);
    // TODO: other labh padas
}

#[ignore]
#[test]
fn sutra_7_1_65() {
    // TODO: num-Agama is added before adding the affix.
    let labh = Dhatu::new("qula\\Ba~\\z", Gana::Bhvadi);
    assert_has_krdanta(&["AN"], &labh, Krt::yat, &["AlamBya"]);
    assert_has_krdanta(&[], &labh, Krt::yat, &["laBya"]);
}

#[ignore]
#[test]
fn sutra_7_1_66() {
    // TODO: num-Agama is added before adding the affix.
    let labh = Dhatu::new("qula\\Ba~\\z", Gana::Bhvadi);
    assert_has_krdanta(&["upa"], &labh, Krt::yat, &["upalamBya", "upalaBya"]);
}

#[test]
fn sutra_7_1_100() {
    let d = Dhatu::new;
    assert_has_lat(&[], &d("kF", Gana::Tudadi), &["kirati"]);
    assert_has_lat(&[], &d("gF", Gana::Tudadi), &["girati", "gilati"]);
    assert_has_krdanta(&["AN"], &d("stFY", Gana::Kryadi), Krt::kta, &["AstIrRa"]);
    assert_has_krdanta(&["vi"], &d("SF", Gana::Kryadi), Krt::kta, &["viSIrRa"]);
}

#[test]
fn sutra_7_1_101() {
    assert_has_lat_p(&[], &Dhatu::new("kFta~", Gana::Curadi), &["kIrtayati"]);
}
