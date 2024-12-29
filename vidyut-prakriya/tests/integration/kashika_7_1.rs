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
fn sutra_7_1_1() {
    let nand = d("wunadi~", Bhvadi);
    assert_has_krdanta(&[], &nand, Krt::lyu, &["nandana"]);

    let ram = d("ra\\ma~\\", Bhvadi);
    assert_has_krdanta(&[], &ram, Krt::lyu, &["ramaRa"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::Rvul, &["kAraka"]);

    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&[], &hf, Krt::Rvul, &["hAraka"]);
}

#[test]
fn sutra_7_1_2() {
    use Taddhita as T;
    assert_has_taddhita("naqa", T::Pak, &["nAqAyana"]);
    assert_has_taddhita("cara", T::Pak, &["cArAyaRa"]);
    assert_has_taddhita(&nyap("suparRA"), T::Qak, &["sOparReya"]);
    assert_has_taddhita(&nyap("vinatA"), T::Qak, &["vEnateya"]);
    assert_has_taddhita("kula", T::Ka, &["kulIna"]);
    assert_has_taddhita("gArga", T::Ca, &["gArgIya"]);
    assert_has_taddhita("vAtsa", T::Ca, &["vAtsIya"]);
    assert_has_taddhita("kzatra", T::Ga, &["kzatriya"]);

    // pratyayagrahanam
    assert_has_lat(&[], &d("Pakka~", Bhvadi), &["Pakkati"]);
    assert_has_lat(&[], &d("QOkf~\\", Bhvadi), &["QOkate"]);
    assert_has_tip(&[], &d("Kanu~^", Bhvadi), Lat, &["Kanati"]);
    assert_has_tip(&[], &d("Ci\\di~^r", Rudhadi), Lat, &["Cinatti"]);
    assert_has_lat(&[], &d("GurRa~\\", Bhvadi), &["GUrRate"]);

    // TODO: others
}

#[test]
fn sutra_7_1_3() {
    assert_has_jhi(&[], &d("qukf\\Y", Tanadi), Lat, &["kurvanti"]);
    assert_has_jhi(&[], &d("zu\\Y", Svadi), Lat, &["sunvanti"]);
    assert_has_jhi(&[], &d("ci\\Y", Svadi), Lat, &["cinvanti"]);

    // TODO: do the rest.

    // pratyayasya
    let ujjh = d("ujJa", Tudadi);
    assert_has_krdanta(&[], &ujjh, Krt::tfc, &["ujJitf"]);
    assert_has_krdanta(&[], &ujjh, Krt::tumun, &["ujJitum"]);
    assert_has_krdanta(&[], &ujjh, Krt::tavya, &["ujJitavya"]);
}

#[test]
fn sutra_7_1_4() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_jhi(&[], &daa, Lat, &["dadati"]);
    assert_has_jhi(&[], &daa, Lot, &["dadatu"]);

    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_jhi(&[], &dhaa, Lat, &["daDati"]);
    assert_has_jhi(&[], &dhaa, Lot, &["daDatu"]);

    let jaks = d("jakza~", Adadi);
    assert_has_jhi(&[], &jaks, Lat, &["jakzati"]);
    assert_has_jhi(&[], &jaks, Lot, &["jakzatu"]);

    let jaagr = d("jAgf", Adadi);
    assert_has_jhi(&[], &jaagr, Lat, &["jAgrati"]);
    assert_has_jhi(&[], &jaagr, Lot, &["jAgratu"]);

    assert_has_jhi(&[], &dhaa, Lan, &["adaDuH"]);
    assert_has_jhi(&[], &jaagr, Lan, &["ajAgaruH"]);
}

#[test]
fn sutra_7_1_5() {
    let ci = d("ci\\Y", Svadi);
    assert_has_jha(&[], &ci, Lat, &["cinvate"]);
    assert_has_jha(&[], &ci, Lot, &["cinvatAm"]);
    assert_has_jha(&[], &ci, Lan, &["acinvata"]);

    assert_has_jha(&[], &d("pUY", Kryadi), Lat, &["punate"]);

    let lu = d("lUY", Kryadi);
    assert_has_jha(&[], &lu, Lat, &["lunate"]);
    assert_has_jha(&[], &lu, Lot, &["lunatAm"]);
    assert_has_jha(&[], &lu, Lan, &["alunata"]);

    assert_has_jha(&[], &d("cyu\\N", Bhvadi), Lat, &["cyavante"]);
    assert_has_jha(&[], &d("plu\\N", Bhvadi), Lat, &["plavante"]);
}

#[test]
fn sutra_7_1_6() {
    let shi = d("SIN", Adadi);
    assert_has_jha(&[], &shi, Lat, &["Serate"]);
    assert_has_jha(&[], &shi, Lot, &["SeratAm"]);
    assert_has_jha(&[], &shi, Lan, &["aSerata"]);
}

#[test]
fn sutra_7_1_7() {
    let vid = d("vida~", Adadi);
    assert_has_jha(&["sam"], &vid, Lat, &["saMvidate", "saMvidrate"]);
    assert_has_jha(
        &["sam"],
        &vid,
        Lot,
        &["saMvidatAm", "saMvidratAm", "saMvidANkurvatAm"],
    );
    assert_has_jha(&["sam"], &vid, Lan, &["samavidata", "samavidrata"]);
}

// 7.1.8 is chAndasa

#[test]
fn sutra_7_1_9() {
    assert_has_sup_3p("vfkza", Pum, &["vfkzEH"]);
    assert_has_sup_3p("plakza", Pum, &["plakzEH"]);
    assert_has_sup_3p("atijarasa", Pum, &["atijarasEH"]);
}

// 7.1.10 is chAndasa

#[test]
fn sutra_7_1_11() {
    assert_has_sup_3p("idam", Pum, &["eBiH"]);
    assert_has_sup_3p("adas", Pum, &["amIBiH"]);
    // TODO: imaka, amuka
}

#[test]
fn sutra_7_1_12() {
    assert_has_sup_3s("vfkza", Pum, &["vfkzeRa"]);
    assert_has_sup_3s("plakza", Pum, &["plakzeRa"]);
    assert_has_sup_5s("vfkza", Pum, &["vfkzAt"]);
    assert_has_sup_5s("plakza", Pum, &["plakzAt"]);
    assert_has_sup_6s("vfkza", Pum, &["vfkzasya"]);
    assert_has_sup_6s("plakza", Pum, &["plakzasya"]);

    assert_has_sup_3s("saKi", Pum, &["saKyA"]);
    assert_has_sup_3s("pati", Pum, &["patyA"]);
}

#[test]
fn sutra_7_1_13() {
    assert_has_sup_4s("vfkza", Pum, &["vfkzAya"]);
    assert_has_sup_4s("plakza", Pum, &["plakzAya"]);

    assert_has_sup_4s("saKi", Pum, &["saKye"]);
    assert_has_sup_4s("pati", Pum, &["patye"]);
}

#[test]
fn sutra_7_1_14() {
    assert_has_sup_4s("sarva", Pum, &["sarvasmE"]);
    assert_has_sup_4s("viSva", Pum, &["viSvasmE"]);
    assert_has_sup_4s("yad", Pum, &["yasmE"]);
    assert_has_sup_4s("tad", Pum, &["tasmE"]);
    assert_has_sup_4s("kim", Pum, &["kasmE"]);
}

#[test]
fn sutra_7_1_15() {
    assert_has_sup_5s("sarva", Pum, &["sarvasmAt"]);
    assert_has_sup_5s("viSva", Pum, &["viSvasmAt"]);
    assert_has_sup_5s("yad", Pum, &["yasmAt"]);
    assert_has_sup_5s("tad", Pum, &["tasmAt"]);
    assert_has_sup_5s("kim", Pum, &["kasmAt"]);

    assert_has_sup_7s("sarva", Pum, &["sarvasmin"]);
    assert_has_sup_7s("viSva", Pum, &["viSvasmin"]);
    assert_has_sup_7s("yad", Pum, &["yasmin"]);
    assert_has_sup_7s("tad", Pum, &["tasmin"]);
    assert_has_sup_7s("anya", Pum, &["anyasmin"]);
}

#[test]
fn sutra_7_1_16() {
    assert_has_sup_5s("pUrva", Pum, &["pUrvasmAt", "pUrvAt"]);
    assert_has_sup_7s("pUrva", Pum, &["pUrvasmin", "pUrve"]);
    assert_has_sup_5s("para", Pum, &["parasmAt", "parAt"]);
    assert_has_sup_7s("para", Pum, &["parasmin", "pare"]);
    assert_has_sup_5s("avara", Pum, &["avarasmAt", "avarAt"]);
    assert_has_sup_7s("avara", Pum, &["avarasmin", "avare"]);
    assert_has_sup_5s("dakziRa", Pum, &["dakziRasmAt", "dakziRAt"]);
    assert_has_sup_7s("dakziRa", Pum, &["dakziRasmin", "dakziRe"]);
    assert_has_sup_5s("uttara", Pum, &["uttarasmAt", "uttarAt"]);
    assert_has_sup_7s("uttara", Pum, &["uttarasmin", "uttare"]);
    assert_has_sup_5s("apara", Pum, &["aparasmAt", "aparAt"]);
    assert_has_sup_7s("apara", Pum, &["aparasmin", "apare"]);
    assert_has_sup_5s("aDara", Pum, &["aDarasmAt", "aDarAt"]);
    assert_has_sup_7s("aDara", Pum, &["aDarasmin", "aDare"]);
    assert_has_sup_5s("sva", Pum, &["svasmAt", "svAt"]);
    assert_has_sup_7s("sva", Pum, &["svasmin", "sve"]);
    assert_has_sup_5s("antara", Pum, &["antarasmAt", "antarAt"]);
    assert_has_sup_7s("antara", Pum, &["antarasmin", "antare"]);
}

#[test]
fn sutra_7_1_17() {
    assert_has_sup_1p("sarva", Pum, &["sarve"]);
    assert_has_sup_1p("viSva", Pum, &["viSve"]);
    assert_has_sup_1p("yad", Pum, &["ye"]);
    assert_has_sup_1p("tad", Pum, &["te"]);
    assert_has_sup_1p("kim", Pum, &["ke"]);
}

#[test]
fn sutra_7_1_18() {
    assert_has_sup_1d(&nyap("KawvA"), Stri, &["Kawve"]);
    assert_has_sup_2d(&nyap("KawvA"), Stri, &["Kawve"]);
    // TODO: more
}

#[test]
fn sutra_7_1_19() {
    assert_has_sup_1d("kuRqa", Napumsaka, &["kuRqe"]);
    assert_has_sup_2d("kuRqa", Napumsaka, &["kuRqe"]);
    assert_has_sup_1d("daDi", Napumsaka, &["daDinI"]);
    assert_has_sup_1d("maDu", Napumsaka, &["maDunI"]);
    assert_has_sup_1d("trapu", Napumsaka, &["trapuRI"]);
    assert_has_sup_1d("jatu", Napumsaka, &["jatunI"]);
}

#[test]
fn sutra_7_1_20() {
    assert_has_sup_1p("kuRqa", Napumsaka, &["kuRqAni"]);
    assert_has_sup_2p("kuRqa", Napumsaka, &["kuRqAni"]);
    assert_has_sup_1p("daDi", Napumsaka, &["daDIni"]);
    assert_has_sup_1p("maDu", Napumsaka, &["maDUni"]);
    assert_has_sup_1p("trapu", Napumsaka, &["trapURi"]);
    assert_has_sup_1p("jatu", Napumsaka, &["jatUni"]);
}

#[test]
fn sutra_7_1_21() {
    assert_has_sup_1p("azwan", Pum, &["azwO", "azwa"]);
    assert_has_sup_2p("azwan", Pum, &["azwO", "azwa"]);
}

#[test]
fn sutra_7_1_22() {
    assert_has_sup_1p("zaz", Pum, &["zaw"]);
    assert_has_sup_2p("zaz", Pum, &["zaw"]);
    assert_has_sup_1p("paYcan", Pum, &["paYca"]);
    assert_has_sup_1p("saptan", Pum, &["sapta"]);
    assert_has_sup_1p("navan", Pum, &["nava"]);
    assert_has_sup_1p("daSan", Pum, &["daSa"]);
}

#[test]
fn sutra_7_1_23() {
    assert_has_sup_1s("daDi", Napumsaka, &["daDi"]);
    assert_has_sup_2s("daDi", Napumsaka, &["daDi"]);
    assert_has_sup_1s("maDu", Napumsaka, &["maDu"]);
    assert_has_sup_2s("maDu", Napumsaka, &["maDu"]);
    assert_has_sup_1s("trapu", Napumsaka, &["trapu"]);
    assert_has_sup_1s("jatu", Napumsaka, &["jatu"]);
}

#[test]
fn sutra_7_1_24() {
    assert_has_sup_1s("kuRqa", Napumsaka, &["kuRqam"]);
    assert_has_sup_2s("kuRqa", Napumsaka, &["kuRqam"]);
}

#[test]
fn sutra_7_1_25() {
    assert_has_sup_1s("katara", Napumsaka, &["katarat"]);
    assert_has_sup_2s("katara", Napumsaka, &["katarat"]);
    assert_has_sup_1s("katama", Napumsaka, &["katamat"]);
    assert_has_sup_2s("katama", Napumsaka, &["katamat"]);
    assert_has_sup_1s("itara", Napumsaka, &["itarat"]);
    assert_has_sup_1s("anyatara", Napumsaka, &["anyatarat"]);
    assert_has_sup_1s("anya", Napumsaka, &["anyat"]);

    assert_has_sup_1s("nema", Napumsaka, &["nemam"]);
    assert_has_sup_2s("nema", Napumsaka, &["nemam"]);
}

// 7.1.26 is chAndasa.

#[test]
fn sutra_7_1_27() {
    assert_has_sup_6s("asmad", Pum, &["mama"]);
    assert_has_sup_6s("yuzmad", Pum, &["tava"]);
}

#[test]
fn sutra_7_1_28() {
    assert_has_sup_4s("asmad", Pum, &["mahyam"]);
    assert_has_sup_4s("yuzmad", Pum, &["tuByam"]);

    assert_has_sup_1s("yuzmad", Pum, &["tvam"]);
    assert_has_sup_1s("asmad", Pum, &["aham"]);
    assert_has_sup_1d("yuzmad", Pum, &["yuvAm"]);
    assert_has_sup_1d("asmad", Pum, &["AvAm"]);
    assert_has_sup_1p("yuzmad", Pum, &["yUyam"]);
    assert_has_sup_1p("asmad", Pum, &["vayam"]);
    assert_has_sup_2s("yuzmad", Pum, &["tvAm"]);
    assert_has_sup_2s("asmad", Pum, &["mAm"]);
    assert_has_sup_2d("yuzmad", Pum, &["yuvAm"]);
    assert_has_sup_2d("asmad", Pum, &["AvAm"]);
}

#[test]
fn sutra_7_1_29() {
    assert_has_sup_2p("asmad", Pum, &["asmAn"]);
    assert_has_sup_2p("yuzmad", Pum, &["yuzmAn"]);
}

#[test]
fn sutra_7_1_30() {
    assert_has_sup_4p("asmad", Pum, &["asmaByam"]);
    assert_has_sup_4p("yuzmad", Pum, &["yuzmaByam"]);
}

#[test]
fn sutra_7_1_31() {
    assert_has_sup_5p("asmad", Pum, &["asmat"]);
    assert_has_sup_5p("yuzmad", Pum, &["yuzmat"]);
}

#[test]
fn sutra_7_1_32() {
    assert_has_sup_5s("asmad", Pum, &["mat"]);
    assert_has_sup_5s("yuzmad", Pum, &["tvat"]);
}

#[test]
fn sutra_7_1_33() {
    assert_has_sup_6p("asmad", Pum, &["asmAkam"]);
    assert_has_sup_6p("yuzmad", Pum, &["yuzmAkam"]);
}

#[test]
fn sutra_7_1_34() {
    assert_has_tip(&[], &d("pE\\", Bhvadi), Lit, &["papO"]);
    assert_has_tip(&[], &d("zWA\\", Bhvadi), Lit, &["tasTO"]);
    assert_has_tip(&[], &d("glE\\", Bhvadi), Lit, &["jaglO"]);
    assert_has_tip(&[], &d("mlE\\", Bhvadi), Lit, &["mamlO"]);
}

#[test]
fn sutra_7_1_35() {
    assert_has_lot(&[], &d("jIva~", Bhvadi), &["jIvatAt", "jIvatu"]);
    assert_has_sip(&[], &d("jIva~", Bhvadi), Lot, &["jIvatAt", "jIva"]);
    assert_has_tip(&[], &d("brUY", Adadi), Lot, &["brUtAt", "bravItu"]);
}

#[test]
fn sutra_7_1_36() {
    let vid = d("vida~", Adadi);
    assert_has_krdanta(&[], &vid, Krt::Satf, &["vidat", "vidvas"]);
}

#[ignore]
#[test]
fn sutra_7_1_37() {
    use Krt::ktvA;
    let kf = d("qukf\\Y", Tanadi);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&["pra"], &kf, ktvA, &["prakftya"]);
    assert_has_krdanta(&["pra"], &hf, ktvA, &["prahftya"]);
    assert_has_upapada_krdanta(avyaya("pArSvataH"), &[], &kf, ktvA, &["pArSvataHkftya"]);
    assert_has_krdanta(&["nAnA"], &kf, ktvA, &["nAnAkftya"]);
    assert_has_krdanta(&["dviDA"], &kf, ktvA, &["dviDAkftya"]);

    // samAse
    assert_has_krdanta(&[], &kf, ktvA, &["kftvA"]);
    assert_has_krdanta(&[], &hf, ktvA, &["hftvA"]);

    // TODO: others
}

// 7.1.38 - 7.1.50 are chAndasa.

#[test]
fn sutra_7_1_52() {
    assert_has_sup_6p("sarva", Pum, &["sarvezAm"]);
    assert_has_sup_6p("viSva", Pum, &["viSvezAm"]);
    assert_has_sup_6p("yad", Pum, &["yezAm"]);
    assert_has_sup_6p("tad", Pum, &["tezAm"]);

    assert_has_sup_6p("sarva", Stri, &["sarvAsAm"]);
    assert_has_sup_6p("yad", Stri, &["yAsAm"]);
    assert_has_sup_6p("tad", Stri, &["tAsAm"]);

    assert_has_sup_6p("Bavat", Pum, &["BavatAm"]);
}

#[test]
fn sutra_7_1_53() {
    assert_has_sup_6p("tri", Pum, &["trayARAm"]);
}

#[test]
fn sutra_7_1_54() {
    assert_has_sup_6p("plakza", Pum, &["plakzARAm"]);
    assert_has_sup_6p("agni", Pum, &["agnInAm"]);
    assert_has_sup_6p("vAyu", Pum, &["vAyUnAm"]);
    assert_has_sup_6p("kartf", Pum, &["kartFRAm"]);

    assert_has_sup_6p(&nyap("kumArI"), Stri, &["kumArIRAm"]);
    assert_has_sup_6p(&nyap("kiSorI"), Stri, &["kiSorIRAm"]);
    assert_has_sup_6p(&nyap("gOrI"), Stri, &["gOrIRAm"]);
    assert_has_sup_6p(&nyap("SArNgaravI"), Stri, &["SArNgaravIRAm"]);
    assert_has_sup_6p(&nyap("lakzmI"), Stri, &["lakzmIRAm"]);
    assert_has_sup_6p(&nyap("brahmabanDU"), Stri, &["brahmabanDUnAm"]);
    assert_has_sup_6p(&nyap("vIrabanDU"), Stri, &["vIrabanDUnAm"]);
    assert_has_sup_6p(&nyap("KawvA"), Stri, &["KawvAnAm"]);
    assert_has_sup_6p(&nyap("mAlA"), Stri, &["mAlAnAm"]);
    assert_has_sup_6p(&nyap("bahurAjA"), Stri, &["bahurAjAnAm"]);
    assert_has_sup_6p(&nyap("kArIzaganDyA"), Stri, &["kArIzaganDyAnAm"]);
}

#[test]
fn sutra_7_1_55() {
    assert_has_sup_6p("zaz", Pum, &["zaRRAm"]);
    assert_has_sup_6p("paYcan", Pum, &["paYcAnAm"]);
    assert_has_sup_6p("saptan", Pum, &["saptAnAm"]);
    assert_has_sup_6p("navan", Pum, &["navAnAm"]);
    assert_has_sup_6p("daSan", Pum, &["daSAnAm"]);
    assert_has_sup_6p("catur", Pum, &["caturRAm"]);
}

#[test]
fn sutra_7_1_58() {
    let kund = d("kuqi~\\", Bhvadi);
    assert_has_krdanta(&[], &kund, Krt::tfc, &["kuRqitf"]);
    assert_has_krdanta(&[], &kund, Krt::tumun, &["kuRqitum"]);
    assert_has_krdanta(&[], &kund, Krt::tavya, &["kuRqitavya"]);
    assert_has_krdanta(&[], &kund, Krt::a, &["kuRqA"]);

    let hund = d("huqi~\\", Bhvadi);
    assert_has_krdanta(&[], &hund, Krt::tfc, &["huRqitf"]);
    assert_has_krdanta(&[], &hund, Krt::tumun, &["huRqitum"]);
    assert_has_krdanta(&[], &hund, Krt::tavya, &["huRqitavya"]);
    assert_has_krdanta(&[], &hund, Krt::a, &["huRqA"]);

    // no idit
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lat, &["pacati"]);
    assert_has_tip(&[], &d("paWa~", Bhvadi), Lat, &["paWati"]);

    // irit is not idit
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &d("Ci\\di~^r", Rudhadi), Krt::tfc, &["Cettf"]);
}

#[test]
fn sutra_7_1_59() {
    let muc = d("mu\\cx~^", Tudadi);
    assert_has_tip(&[], &muc, Lat, &["muYcati"]);
    assert_has_tip(&[], &d("lu\\px~^", Tudadi), Lat, &["lumpati"]);
    assert_has_tip(&[], &d("vi\\dx~^", Tudadi), Lat, &["vindati"]);
    assert_has_tip(&[], &d("li\\pa~^", Tudadi), Lat, &["limpati"]);
    assert_has_tip(&[], &d("zi\\ca~^", Tudadi), Lat, &["siYcati"]);
    assert_has_tip(&[], &d("kftI~", Tudadi), Lat, &["kfntati"]);
    assert_has_tip(&[], &d("Ki\\da~", Tudadi), Lat, &["Kindati"]);
    assert_has_tip(&[], &d("piSa~", Tudadi), Lat, &["piMSati"]);

    assert_has_krdanta(&[], &muc, Krt::tfc, &["moktf"]);
    assert_has_krdanta(&[], &muc, Krt::tumun, &["moktum"]);
    assert_has_krdanta(&[], &muc, Krt::tavya, &["moktavya"]);

    assert_has_tip(&[], &d("tu\\da~^", Tudadi), Lat, &["tudati"]);
    assert_has_tip(&[], &d("Ru\\da~^", Tudadi), Lat, &["nudati"]);

    // TODO: varttika
}

#[test]
fn sutra_7_1_60() {
    let masj = d("wuma\\sjo~", Tudadi);
    assert_has_krdanta(&[], &masj, Krt::tfc, &["maNktf"]);
    assert_has_krdanta(&[], &masj, Krt::tumun, &["maNktum"]);
    assert_has_krdanta(&[], &masj, Krt::tavya, &["maNktavya"]);

    let nas = d("Ra\\Sa~", Divadi);
    assert_has_krdanta(&[], &nas, Krt::tfc, &["naMzwf", "naSitf"]);
    assert_has_krdanta(&[], &nas, Krt::tumun, &["naMzwum", "naSitum"]);
    assert_has_krdanta(&[], &nas, Krt::tavya, &["naMzwavya", "naSitavya"]);

    assert_has_krdanta(&[], &masj, Krt::lyuw, &["majjana"]);
    assert_has_krdanta(&[], &nas, Krt::lyuw, &["naSana"]);

    assert_has_krdanta(&[], &masj, Krt::kta, &["magna"]);
    assert_has_krdanta(&[], &masj, Krt::ktavatu, &["magnavat"]);
}

#[test]
fn sutra_7_1_61() {
    let radh = d("ra\\Da~", Divadi);
    let jabh = d("jaBI~\\", Bhvadi);
    assert_has_tip(
        &[],
        &radh.clone().with_sanadi(&[Sanadi::Ric]),
        Lat,
        &["ranDayati"],
    );
    assert_has_krdanta(&[], &radh, Krt::Rvul, &["ranDaka"]);
    assert_has_krdanta(&[], &radh, Krt::Rini, &["ranDin"]);
    assert_has_krdanta(&[], &radh, Krt::Ramul, &["ranDam"]);
    assert_has_krdanta(&[], &radh, Krt::GaY, &["ranDa"]);

    assert_has_tip(
        &[],
        &jabh.clone().with_sanadi(&[Sanadi::Ric]),
        Lat,
        &["jamBayati"],
    );
    assert_has_krdanta(&[], &jabh, Krt::Rvul, &["jamBaka"]);
    assert_has_krdanta(&[], &jabh, Krt::Rini, &["jamBin"]);
    assert_has_krdanta(&[], &jabh, Krt::Ramul, &["jamBam"]);
    assert_has_krdanta(&[], &jabh, Krt::GaY, &["jamBa"]);

    assert_has_krdanta(&[], &radh, Krt::tfc, &["radDf", "raDitf"]);
    assert_has_krdanta(&[], &jabh, Krt::yat, &["jaBya"]);
}

#[test]
fn sutra_7_1_62() {
    let radh = d("ra\\Da~", Divadi);
    assert_has_krdanta(&[], &radh, Krt::tfc, &["radDf", "raDitf"]);
    assert_has_krdanta(&[], &radh, Krt::tumun, &["raDitum", "radDum"]);
    assert_has_krdanta(&[], &radh, Krt::tavya, &["raDitavya", "radDavya"]);
    assert_has_krdanta(&[], &radh, Krt::lyuw, &["ranDana"]);
    assert_has_krdanta(&[], &radh, Krt::Rvul, &["ranDaka"]);

    // reDva, reDma by 7.2.45.
    assert_has_vas(&[], &radh, Lit, &["raranDiva", "reDva"]);
    assert_has_mas(&[], &radh, Lit, &["raranDima", "reDma"]);

    // TODO: redhivas (we get *redhvas instead)
    assert_has_krdanta(&[], &radh, Krt::kvasu, &["reDivas"]);
}

#[test]
fn sutra_7_1_63() {
    let rabh = d("ra\\Ba~\\", Bhvadi);
    assert_has_tip(&["AN"], &nic(&rabh), Lat, &["AramBayati"]);
    assert_has_krdanta(&["AN"], &rabh, Krt::Rvul, &["AramBaka"]);
    assert_has_krdanta(&["AN"], &rabh, Krt::Rini, &["AramBin"]);
    assert_has_krdanta(&["AN"], &rabh, Krt::Ramul, &["AramBam"]);
    assert_has_krdanta(&["AN"], &rabh, Krt::GaY, &["AramBa"]);

    // a-Sap-liwoH?
    assert_has_lat(&["AN"], &rabh, &["AraBate"]);
    assert_has_lit(&["AN"], &rabh, &["AreBe"]);
    // aci?
    assert_has_krdanta(&["AN"], &rabh, Krt::tfc, &["ArabDf"]);
}

#[test]
fn sutra_7_1_64() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_tip(&[], &nic(&labh), Lat, &["lamBayati"]);
    assert_has_krdanta(&[], &labh, Krt::Rvul, &["lamBaka"]);
    assert_has_krdanta(&[], &labh, Krt::Rini, &["lamBin"]);
    assert_has_krdanta(&[], &labh, Krt::Ramul, &["lamBam", "lABam"]);
    assert_has_krdanta(&[], &labh, Krt::ac, &["lamBa"]);

    // a-Sap-liwoH
    assert_has_lat(&[], &labh, &["laBate"]);
    assert_has_lit(&[], &labh, &["leBe"]);
    // aci
    assert_has_krdanta(&[], &labh, Krt::tfc, &["labDf"]);
}

#[test]
fn sutra_7_1_65() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_krdanta(&["AN"], &labh, Krt::yat, &["AlamBya"]);
    assert_has_krdanta(&[], &labh, Krt::yat, &["laBya"]);
}

#[test]
fn sutra_7_1_66() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_krdanta(&["upa"], &labh, Krt::yat, &["upalamBya", "upalaBya"]);
}

#[test]
fn sutra_7_1_67() {
    let labh = d("qula\\Ba~\\z", Bhvadi);

    // Kal
    assert_has_upapada_krdanta(avyaya("Izat"), &["pra"], &labh, Krt::Kal, &["IzatpralamBa"]);
    assert_has_upapada_krdanta(avyaya("su"), &["pra"], &labh, Krt::Kal, &["supralamBa"]);

    // GaY
    assert_has_krdanta(&["pra"], &labh, Krt::GaY, &["pralamBa"]);
    assert_has_krdanta(&["vi", "pra"], &labh, Krt::GaY, &["vipralamBa"]);

    // otherwise, ...
    assert_has_upapada_krdanta(avyaya("Izat"), &[], &labh, Krt::Kal, &["IzallaBa"]);
    assert_has_krdanta(&[], &labh, Krt::GaY, &["lABa"]);
}

#[test]
fn sutra_7_1_68() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_krdanta(&["su"], &labh, Krt::GaY, &["sulABa"]);
    assert_has_krdanta(&["dur"], &labh, Krt::GaY, &["durlABa"]);

    // kevalAbhyAm
    assert_has_krdanta(&["su", "pra"], &labh, Krt::GaY, &["supralamBa"]);
    assert_has_krdanta(&["dur", "pra"], &labh, Krt::GaY, &["duzpralamBa"]);
    assert_has_krdanta(&["ati", "su"], &labh, Krt::GaY, &["atisulamBa"]);
}

#[test]
fn sutra_7_1_69() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_ta_k(&[], &labh, Lun, &["alamBi", "alABi"]);
    assert_has_krdanta(&[], &labh, Krt::Ramul, &["lABam", "lamBam"]);

    // nitya for sopasarga
    assert_has_ta_k(&["pra"], &labh, Lun, &["prAlamBi"]);
    assert_has_krdanta(&["pra"], &labh, Krt::Ramul, &["pralamBam"]);
}

#[test]
fn sutra_7_1_70() {
    let bhavat = create_krdanta("Bavat", &[], &d("BA\\", Adadi), Unadi::qavatu);
    assert_has_sup_1s(&bhavat, Pum, &["BavAn"]);
    assert_has_sup_1d(&bhavat, Pum, &["BavantO"]);
    assert_has_sup_1p(&bhavat, Pum, &["BavantaH"]);

    let shreyas = create_taddhitanta("Sreyas", "praSasya", T::Iyasun);
    assert_has_sup_1s(&shreyas, Pum, &["SreyAn"]);
    assert_has_sup_1d(&shreyas, Pum, &["SreyAMsO"]);
    assert_has_sup_1p(&shreyas, Pum, &["SreyAMsaH"]);

    let prac = create_krdanta("prAc", &["pra"], &d("ancu~", Bhvadi), Krt::kvin);
    assert_has_sup_1s(&prac, Pum, &["prAN"]);
    assert_has_sup_1d(&prac, Pum, &["prAYcO"]);
    assert_has_sup_1p(&prac, Pum, &["prAYcaH"]);

    let pacat = create_krdanta("pacat", &[], &d("qupa\\ca~^z", Bhvadi), Krt::Satf);
    assert_has_sup_1s(&pacat, Pum, &["pacan"]);
    assert_has_sup_1d(&pacat, Pum, &["pacantO"]);
    assert_has_sup_1p(&pacat, Pum, &["pacantaH"]);

    // ugidacAm?
    assert_has_sup_1s("dfzad", Pum, &["dfzat"]);
    assert_has_sup_1d("dfzad", Pum, &["dfzadO"]);
    assert_has_sup_1p("dfzad", Pum, &["dfzadaH"]);

    // sarvanAmasthAne?
    assert_has_sup_2p(&bhavat, Pum, &["BavataH"]);
    assert_has_sup_2p(&shreyas, Pum, &["SreyasaH"]);
}

#[test]
fn sutra_7_1_71() {
    let yuj = krdanta(&[], &d("yu\\ji~^r", Rudhadi), Krt::kvin);
    assert_has_sup_1s(&yuj, Pum, &["yuN"]);
    assert_has_sup_1d(&yuj, Pum, &["yuYjO"]);
    assert_has_sup_1p(&yuj, Pum, &["yuYjaH"]);

    // asamAse?
    let ashvayuj = upapada_krdanta("aSva", &[], &d("yu\\ji~^r", Rudhadi), Krt::kvip);
    assert_has_sup_1s(&ashvayuj, Pum, &["aSvayuk"]);
    assert_has_sup_1d(&ashvayuj, Pum, &["aSvayujO"]);
    assert_has_sup_1p(&ashvayuj, Pum, &["aSvayujaH"]);
}

#[test]
fn sutra_7_1_72() {
    assert_has_sup_1p("udaSvit", Napumsaka, &["udaSvinti"]);
    assert_has_sup_1p("Sakft", Napumsaka, &["Sakfnti"]);
    assert_has_sup_1p("yaSas", Napumsaka, &["yaSAMsi"]);
    assert_has_sup_1p("payas", Napumsaka, &["payAMsi"]);
    // ajantasya
    assert_has_sup_1p("kuRqa", Napumsaka, &["kuRqAni"]);
    assert_has_sup_1p("vana", Napumsaka, &["vanAni"]);
    assert_has_sup_1p("trapu", Napumsaka, &["trapURi"]);
    assert_has_sup_1p("jatu", Napumsaka, &["jatUni"]);

    // napuMsakasya
    assert_has_sup_1s("agnicit", Pum, &["agnicit"]);

    // jhal-acaH
    assert_has_sup_1p("bahupur", Napumsaka, &["bahupuri"]);
    assert_has_sup_1p("vimaladiv", Napumsaka, &["vimaladivi"]);
    assert_has_sup_1p("catur", Napumsaka, &["catvAri"]);
    assert_has_sup_1p("ahan", Napumsaka, &["ahAni"]);

    assert_has_sup_1p("Sreyas", Napumsaka, &["SreyAMsi"]);
    assert_has_sup_1p("BUyas", Napumsaka, &["BUyAMsi"]);
    assert_has_sup_1p("kurvat", Napumsaka, &["kurvanti"]);
    assert_has_sup_1p("kfzat", Napumsaka, &["kfzanti"]);

    // TODO: bahUrji
}

#[test]
fn sutra_7_1_73() {
    assert_has_sup_1d("trapu", Napumsaka, &["trapuRI"]);
    assert_has_sup_1d("jatu", Napumsaka, &["jatunI"]);
    assert_has_sup_1d("tumburu", Napumsaka, &["tumburuRI"]);
    assert_has_sup_4s("trapu", Napumsaka, &["trapuRe"]);
    assert_has_sup_4s("jatu", Napumsaka, &["jatune"]);
    assert_has_sup_4s("tumburu", Napumsaka, &["tumburuRe"]);
    // ikaH
    assert_has_sup_1d("kuRqa", Napumsaka, &["kuRqe"]);
    assert_has_sup_1d("pIWa", Napumsaka, &["pIWe"]);
}

#[test]
fn sutra_7_1_75() {
    assert_has_sup_3s("asTi", Pum, &["asTnA"]);
    assert_has_sup_4s("asTi", Pum, &["asTne"]);
    assert_has_sup_3s("daDi", Pum, &["daDnA"]);
    assert_has_sup_4s("daDi", Pum, &["daDne"]);
    assert_has_sup_3s("sakTi", Pum, &["sakTnA"]);
    assert_has_sup_4s("sakTi", Pum, &["sakTne"]);
    assert_has_sup_3s("akzi", Pum, &["akzRA"]);
    assert_has_sup_4s("akzi", Pum, &["akzRe"]);
}

#[test]
fn sutra_7_1_78() {
    let dadat = create_krdanta("dadat", &[], &d("qudA\\Y", Juhotyadi), Krt::Satf);
    assert_has_sup_1s(&dadat, Pum, &["dadat"]);
    assert_has_sup_1d(&dadat, Pum, &["dadatO"]);
    assert_has_sup_1p(&dadat, Pum, &["dadataH"]);

    let dadhat = create_krdanta("daDat", &[], &d("quDA\\Y", Juhotyadi), Krt::Satf);
    assert_has_sup_1s(&dadhat, Pum, &["daDat"]);
    assert_has_sup_1d(&dadhat, Pum, &["daDatO"]);
    assert_has_sup_1p(&dadhat, Pum, &["daDataH"]);

    let jakshat = create_krdanta("jakzat", &[], &d("jakza~", Adadi), Krt::Satf);
    assert_has_sup_1s(&jakshat, Pum, &["jakzat"]);
    assert_has_sup_1d(&jakshat, Pum, &["jakzatO"]);
    assert_has_sup_1p(&jakshat, Pum, &["jakzataH"]);

    let jagrat = create_krdanta("jAgrat", &[], &d("jAgf", Adadi), Krt::Satf);
    assert_has_sup_1s(&jagrat, Pum, &["jAgrat"]);
    assert_has_sup_1d(&jagrat, Pum, &["jAgratO"]);
    assert_has_sup_1p(&jagrat, Pum, &["jAgrataH"]);
}

#[test]
fn sutra_7_1_79() {
    let dadat = create_krdanta("dadat", &[], &d("qudA\\Y", Juhotyadi), Krt::Satf);
    assert_has_sup_1p(&dadat, Napumsaka, &["dadati", "dadanti"]);

    let dadhat = create_krdanta("daDat", &[], &d("quDA\\Y", Juhotyadi), Krt::Satf);
    assert_has_sup_1p(&dadhat, Napumsaka, &["daDati", "daDanti"]);

    let jakshat = create_krdanta("jakzat", &[], &d("jakza~", Adadi), Krt::Satf);
    assert_has_sup_1p(&jakshat, Napumsaka, &["jakzati", "jakzanti"]);

    let jagrat = create_krdanta("jAgrat", &[], &d("jAgf", Adadi), Krt::Satf);
    assert_has_sup_1p(&jagrat, Napumsaka, &["jAgrati", "jAgranti"]);
}

#[test]
fn sutra_7_1_80() {
    let tudat = create_krdanta("tudat", &[], &d("tu\\da~^", Tudadi), Krt::Satf);
    assert_has_sup_1d(&tudat, Napumsaka, &["tudatI", "tudantI"]);
    assert_has_sup_1s(&tudat, Stri, &["tudatI", "tudantI"]);

    let yaat = create_krdanta("yAt", &[], &d("yA\\", Adadi), Krt::Satf);
    assert_has_sup_1d(&yaat, Napumsaka, &["yAtI", "yAntI"]);
    assert_has_sup_1s(&yaat, Stri, &["yAtI", "yAntI"]);
}

#[test]
fn sutra_7_1_81() {
    let pacat = create_krdanta("pacat", &[], &d("qupa\\ca~^z", Bhvadi), Krt::Satf);
    assert_has_sup_1d(&pacat, Napumsaka, &["pacantI"]);
    assert_has_sup_1s(&pacat, Stri, &["pacantI"]);

    let divyat = create_krdanta("dIvyat", &[], &d("divu~", Divadi), Krt::Satf);
    assert_has_sup_1d(&divyat, Napumsaka, &["dIvyantI"]);
    assert_has_sup_1s(&divyat, Stri, &["dIvyantI"]);

    let sivyat = create_krdanta("sIvyat", &[], &d("zivu~", Divadi), Krt::Satf);
    assert_has_sup_1d(&sivyat, Napumsaka, &["sIvyantI"]);
    assert_has_sup_1s(&sivyat, Stri, &["sIvyantI"]);
}

#[test]
fn sutra_7_1_82() {
    assert_has_sup_1s("anaquh", Pum, &["anaqvAn"]);
    assert_has_sup_ss("anaquh", Pum, &["anaqvan"]);
}

#[test]
fn sutra_7_1_84() {
    assert_has_sup_1s("div", Stri, &["dyOH"]);
}

#[test]
fn sutra_7_1_85() {
    assert_has_sup_1s("paTin", Pum, &["panTAH"]);
    assert_has_sup_1s("maTin", Pum, &["manTAH"]);
    assert_has_sup_1s("fBukzin", Pum, &["fBukzAH"]);
}

#[test]
fn sutra_7_1_86() {
    assert_has_sup_1s("paTin", Pum, &["panTAH"]);
    assert_has_sup_1d("paTin", Pum, &["panTAnO"]);
    assert_has_sup_1p("paTin", Pum, &["panTAnaH"]);
    assert_has_sup_2s("paTin", Pum, &["panTAnam"]);
    assert_has_sup_2d("paTin", Pum, &["panTAnO"]);

    assert_has_sup_1s("maTin", Pum, &["manTAH"]);
    assert_has_sup_1d("maTin", Pum, &["manTAnO"]);
    assert_has_sup_1p("maTin", Pum, &["manTAnaH"]);
    assert_has_sup_2s("maTin", Pum, &["manTAnam"]);
    assert_has_sup_2d("maTin", Pum, &["manTAnO"]);

    assert_has_sup_1s("fBukzin", Pum, &["fBukzAH"]);
    assert_has_sup_1d("fBukzin", Pum, &["fBukzARO"]);
    assert_has_sup_1p("fBukzin", Pum, &["fBukzARaH"]);
    assert_has_sup_2s("fBukzin", Pum, &["fBukzARam"]);
    assert_has_sup_2d("fBukzin", Pum, &["fBukzARO"]);
}

#[test]
fn sutra_7_1_87() {
    assert_has_sup_1s("paTin", Pum, &["panTAH"]);
    assert_has_sup_1d("paTin", Pum, &["panTAnO"]);
    assert_has_sup_1p("paTin", Pum, &["panTAnaH"]);

    assert_has_sup_1s("maTin", Pum, &["manTAH"]);
    assert_has_sup_1d("maTin", Pum, &["manTAnO"]);
    assert_has_sup_1p("maTin", Pum, &["manTAnaH"]);
}

#[test]
fn sutra_7_1_88() {
    assert_has_sup_2p("paTin", Pum, &["paTaH"]);
    assert_has_sup_3s("paTin", Pum, &["paTA"]);
    assert_has_sup_4s("paTin", Pum, &["paTe"]);

    assert_has_sup_2p("maTin", Pum, &["maTaH"]);
    assert_has_sup_3s("maTin", Pum, &["maTA"]);
    assert_has_sup_4s("maTin", Pum, &["maTe"]);

    assert_has_sup_2p("fBukzin", Pum, &["fBukzaH"]);
    assert_has_sup_3s("fBukzin", Pum, &["fBukzA"]);
    assert_has_sup_4s("fBukzin", Pum, &["fBukze"]);
}

#[test]
fn sutra_7_1_89() {
    assert_has_sup_1s("pums", Pum, &["pumAn"]);
    assert_has_sup_1d("pums", Pum, &["pumAMsO"]);
    assert_has_sup_1p("pums", Pum, &["pumAMsaH"]);
}

#[test]
fn sutra_7_1_90() {
    assert_has_sup_1s("go", Pum, &["gOH"]);
    assert_has_sup_1d("go", Pum, &["gAvO"]);
    assert_has_sup_1p("go", Pum, &["gAvaH"]);
    // TODO: others
}

#[test]
fn sutra_7_1_91() {
    assert_has_mip(&[], &d("qukf\\Y", Tanadi), Lit, &["cakara", "cakAra"]);
    assert_has_mip(&[], &d("qupa\\ca~^z", Bhvadi), Lit, &["papaca", "papAca"]);
}

#[test]
fn sutra_7_1_92() {
    assert_has_sup_1d("saKi", Pum, &["saKAyO"]);
    assert_has_sup_1p("saKi", Pum, &["saKAyaH"]);
    // asambuddhau?
    assert_has_sup_ss("saKi", Pum, &["saKe"]);
}

#[test]
fn sutra_7_1_93() {
    assert_has_sup_1s("saKi", Pum, &["saKA"]);
    // asambuddhau?
    assert_has_sup_ss("saKi", Pum, &["saKe"]);
}

#[test]
fn sutra_7_1_94() {
    assert_has_sup_1s("kartf", Pum, &["kartA"]);
    assert_has_sup_1s("hartf", Pum, &["hartA"]);
    assert_has_sup_1s("mAtf", Pum, &["mAtA"]);
    assert_has_sup_1s("pitf", Pum, &["pitA"]);
    assert_has_sup_1s("BrAtf", Pum, &["BrAtA"]);
    assert_has_sup_1s("uSanas", Pum, &["uSanA"]);
    assert_has_sup_1s("purudaMsas", Pum, &["purudaMsA"]);
    assert_has_sup_1s("anehas", Pum, &["anehA"]);

    // asambuddhau?
    assert_has_sup_ss("kartf", Pum, &["kartaH"]);
    assert_has_sup_ss("mAtf", Pum, &["mAtaH"]);
    assert_has_sup_ss("pitf", Pum, &["pitaH"]);
    assert_has_sup_ss("purudaMsas", Pum, &["purudaMsaH"]);
    assert_has_sup_ss("anehas", Pum, &["anehaH"]);
    assert_has_sup_ss("uSanas", Pum, &["uSanaH"]);
}

#[test]
fn sutra_7_1_95() {
    assert_has_sup_1s("krozwu", Pum, &["krozwA"]);
    assert_has_sup_1d("krozwu", Pum, &["krozwArO"]);
    assert_has_sup_1p("krozwu", Pum, &["krozwAraH"]);
    assert_has_sup_2s("krozwu", Pum, &["krozwAram"]);
    assert_has_sup_2d("krozwu", Pum, &["krozwArO"]);
    assert_has_sup_2p("krozwu", Pum, &["krozwUn"]);

    // asambuddhau?
    assert_has_sup_ss("krozwu", Pum, &["krozwo"]);
}

#[test]
fn sutra_7_1_96() {
    // TODO: see KV commentary on whether krozwu is part of gaurAdi.
    assert_has_sup_1s("krozwu", Stri, &["krozwrI"]);
    assert_has_sup_3d("krozwu", Stri, &["krozwrIByAm"]);
    assert_has_sup_3p("krozwu", Stri, &["krozwrIBiH"]);
}

#[test]
fn sutra_7_1_97() {
    assert_has_sup_3s("krozwu", Pum, &["krozwrA", "krozwunA"]);
    assert_has_sup_4s("krozwu", Pum, &["krozwre", "krozwave"]);
    assert_has_sup_5s("krozwu", Pum, &["krozwuH", "krozwoH"]);
    assert_has_sup_7s("krozwu", Pum, &["krozwari", "krozwO"]);
    assert_has_sup_7d("krozwu", Pum, &["krozwroH", "krozwvoH"]);

    // tftIyAdizu?
    assert_has_sup_2p("krozwu", Pum, &["krozwUn"]);

    // aci?
    assert_has_sup_3d("krozwu", Pum, &["krozwuByAm"]);
    assert_has_sup_3p("krozwu", Pum, &["krozwuBiH"]);
}

#[test]
fn sutra_7_1_98() {
    assert_has_sup_1p("catur", Pum, &["catvAraH"]);

    assert_has_sup_1s("anaquh", Pum, &["anaqvAn"]);
    assert_has_sup_1d("anaquh", Pum, &["anaqvAhO"]);
    assert_has_sup_1p("anaquh", Pum, &["anaqvAhaH"]);
    assert_has_sup_2s("anaquh", Pum, &["anaqvAham"]);

    // tadantaviDiratra izyate
    let priyacatur = create_bahuvrihi("priyacatur", "priya", "catur");
    assert_has_sup_1s(&priyacatur, Pum, &["priyacatvAH"]);
    assert_has_sup_1d(&priyacatur, Pum, &["priyacatvArO"]);
    assert_has_sup_1p(&priyacatur, Pum, &["priyacatvAraH"]);

    let priyanaduh = create_bahuvrihi("priyAnaquh", "priya", "anaquh");
    assert_has_sup_1s(&priyanaduh, Pum, &["priyAnaqvAn"]);
    assert_has_sup_1d(&priyanaduh, Pum, &["priyAnaqvAhO"]);
    assert_has_sup_1p(&priyanaduh, Pum, &["priyAnaqvAhaH"]);
}

#[test]
fn sutra_7_1_99() {
    let priyacatur = create_bahuvrihi("priyacatur", "priya", "catur");
    assert_has_sup_ss(&priyacatur, Pum, &["priyacatvaH"]);

    let priyanaduh = create_bahuvrihi("priyAnaquh", "priya", "anaquh");
    assert_has_sup_ss(&priyanaduh, Pum, &["priyAnaqvan"]);
}

#[test]
fn sutra_7_1_100() {
    assert_has_lat(&[], &d("kF", Tudadi), &["kirati"]);
    assert_has_lat(&[], &d("gF", Tudadi), &["girati", "gilati"]);
    assert_has_krdanta(&["AN"], &d("stFY", Kryadi), Krt::kta, &["AstIrRa"]);
    assert_has_krdanta(&["vi"], &d("SF", Kryadi), Krt::kta, &["viSIrRa"]);
}

#[test]
fn sutra_7_1_101() {
    assert_has_tip(&[], &d("kFta~", Curadi), Lat, &["kIrtayati"]);
}

#[test]
fn sutra_7_1_102() {
    let pf = d("pF", Juhotyadi);
    assert_has_krdanta(&[], &pf, Krt::kta, &["pUrta"]);
    // piparizati and piparIzati are by 7.2.41.
    assert_has_tip(
        &[],
        &san(&pf),
        Lat,
        &["pupUrzati", "piparizati", "piparIzati"],
    );
    assert_has_tip(&[], &san(&d("mf\\N", Tudadi)), Lat, &["mumUrzati"]);
    // Not to be confused with svf!
    // sisvarIzati, etc. are allowable (I think) by 7.2.41
    assert_has_tip(
        &[],
        &san(&d("svF", Kryadi)),
        Lat,
        &["susvUrzati", "sisvarizati", "sisvarIzati"],
    );

    let vr = d("vf", Svadi);
    assert_has_tip(&[], &san(&vr), Lat, &["vuvUrzati"]);
    assert_has_tip(&["pra", "AN"], &san(&vr), Lat, &["prAvuvUrzati"]);
}
