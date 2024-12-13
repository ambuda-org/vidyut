extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

#[test]
fn sutra_1_3_1_and_sutra_1_3_2() {
    assert_has_lat(&[], &d("BU", Bhvadi), &["Bavati"]);
    assert_has_lat(&[], &d("eDa~\\", Bhvadi), &["eDate"]);
    assert_has_lat(&[], &d("sparDa~\\", Bhvadi), &["sparDate"]);
}

#[test]
fn sutra_1_3_3() {
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::kvip, &["cit"]);
    assert_has_krdanta(&[], &d("zu\\Y", Svadi), Krt::kvip, &["sut"]);
}

#[test]
fn sutra_1_3_4() {
    // t
    assert_has_sup_5s("vfkza", Pum, &["vfkzAt"]);
    assert_has_sup_5s("plakza", Pum, &["plakzAt"]);
    // s
    let pac = &d("qupa\\ca~^z", Bhvadi);
    assert_has_sup_1p("brAhmaRa", Pum, &["brAhmaRAH"]);
    assert_has_tas(&[], &pac, Lat, &["pacataH"]);
    assert_has_thas(&[], &pac, Lat, &["pacaTaH"]);
    // m
    assert_has_tas(&[], &pac, Lan, &["apacatAm"]);
    assert_has_thas(&[], &pac, Lan, &["apacatam"]);
}

#[test]
fn sutra_1_3_5() {
    // Yi
    assert_has_krdanta(&[], &d("YimidA~\\", Bhvadi), Krt::kta, &["minna"]);
    assert_has_krdanta(&[], &d("YiDfzA~", Svadi), Krt::kta, &["Dfzwa"]);
    assert_has_krdanta(&[], &d("YikzvidA~", Divadi), Krt::kta, &["kzviRRa"]);
    assert_has_krdanta(&[], &d("YiinDI~\\", Rudhadi), Krt::kta, &["idDa"]);
    // wu
    assert_has_krdanta(&[], &d("wuvepf~\\", Bhvadi), Krt::aTuc, &["vepaTu"]);
    assert_has_krdanta(&[], &d("wuo~Svi", Bhvadi), Krt::aTuc, &["SvayaTu"]);
    // qu
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::ktri, &["paktrima"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), Krt::ktri, &["uptrima"]);
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::ktri, &["kftrima"]);
    // TODO: Adi?
}

#[test]
fn sutra_1_3_6() {
    // TODO: change examples to nartakI, etc.
    assert_has_krdanta(&[], &d("nftI~", Divadi), Krt::zvun, &["nartaka"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Bhvadi), Krt::zvun, &["rajaka"]);
    // pratyayasya?
    assert_has_sup_1s("zoqa", Pum, &["zoqaH"]);
    assert_has_sup_1s("zaRqa", Pum, &["zaRqaH"]);
    assert_has_sup_1s("zaqika", Pum, &["zaqikaH"]);
    // AdiH
    assert_has_krdanta(&[], &d("ava~", Bhvadi), Unadi::wizac, &["aviza"]);
    // TODO: right mah?
    assert_has_krdanta(&[], &d("maha~", Bhvadi), Unadi::wizac, &["mahiza"]);
}

#[test]
fn sutra_1_3_7() {
    // ca
    assert_has_taddhita("kuYja", T::cPaY, &["kOYjAyana"]);
    // ja
    assert_has_sup_1p("brAhmaRa", Pum, &["brAhmaRAH"]);
    // Ya
    assert_has_taddhita("SaRqika", T::Yya, &["SARqikya"]);
    // wa
    let car = &d("cara~", Bhvadi);
    assert_has_upapada_krdanta("kuru", &[], &car, Krt::wa, &["kurucara"]);
    assert_has_upapada_krdanta("madra", &[], &car, Krt::wa, &["madracara"]);
    // qa
    let jan = d("janI~\\", Divadi);
    assert_has_upapada_krdanta("upasara", &[], &jan, Krt::qa, &["upasaraja"]);
    assert_has_upapada_krdanta("mandura", &[], &jan, Krt::qa, &["manduraja"]);

    // Ignore Ca, Ja, Wa, Qa,
    assert_has_taddhita("anna", T::Ra, &["Anna"]);
    assert_has_taddhita("keSa", T::cuYcup, &["keSacuYcu"]);
    assert_has_taddhita("keSa", T::caRap, &["keSacaRa"]);
    assert_has_taddhita("ava", T::wIwac, &["avawIwa"]);
    assert_has_taddhita("karman", T::aWac, &["karmaWa"]);
}

#[test]
fn sutra_1_3_8() {
    // la
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::lyuw, &["cayana"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::lyuw, &["jayana"]);
    // Sa
    assert_has_tip(&[], &d("BU", Bhvadi), Lat, &["Bavati"]);
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lat, &["pacati"]);
    // ka
    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_krdanta(&[], &bhuj, Krt::kta, &["Bukta"]);
    assert_has_krdanta(&[], &bhuj, Krt::ktavatu, &["Buktavat"]);
    // Ka
    let vad = d("vada~", Bhvadi);
    assert_has_upapada_krdanta("priya", &[], &vad, Krt::Kac, &["priyaMvada"]);
    assert_has_upapada_krdanta("vaSa", &[], &vad, Krt::Kac, &["vaSaMvada"]);
    // ga
    assert_has_krdanta(&[], &d("glE\\", Bhvadi), Krt::ksnu, &["glAsnu"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::ksnu, &["jizRu"]);
    assert_has_krdanta(&[], &d("BU", Bhvadi), Krt::ksnu, &["BUzRu"]);
    // Ga
    assert_has_krdanta(&[], &d("Ba\\njo~", Rudhadi), Krt::Gurac, &["BaNgura"]);
    // Na (Nasi --> At, Nas --> sya)
    assert_has_sup_5s("vfkza", Pum, &["vfkzAt"]);
    assert_has_sup_6s("vfkza", Pum, &["vfkzasya"]);
    // ataddhite?
    assert_has_taddhita("cUqA", T::lac, &["cUqAla"]);
    assert_has_taddhita("loman", T::Sa, &["lomaSa"]);

    assert_has_taddhita("vfkza", T::kan, &["vfkzaka"]);
}

#[test]
fn skip_sutra_1_3_9() {}

#[test]
fn sutra_1_3_10() {
    use vidyut_prakriya::args::TaddhitaArtha::*;

    // Example: 4.3.94
    assert_has_artha_taddhita("tUdI", AsyaNivasa, T::Qak, &["tOdeya"]);
    assert_has_artha_taddhita("SAlAtura", AsyaNivasa, T::CaR, &["SAlAturIya"]);
    assert_has_artha_taddhita("varmatI", AsyaNivasa, T::QaY, &["vArmateya"]);
    assert_has_artha_taddhita("kUcavAra", AsyaNivasa, T::yak, &["kOcavArya"]);

    // TODO: others
}

#[test]
fn skip_sutra_1_3_11() {}

#[test]
fn sutra_1_3_12() {
    // anudAttet
    assert_has_lat(&[], &d("Asa~\\", Adadi), &["Aste"]);
    assert_has_lat(&[], &d("vasa~\\", Adadi), &["vaste"]);
    // Nit
    assert_has_lat(&[], &d("zUN", Adadi), &["sUte"]);
    assert_has_lat(&[], &d("SIN", Adadi), &["Sete"]);
}

#[test]
fn sutra_1_3_13() {
    assert_has_ta_k(&[], &d("glE\\", Bhvadi), Lat, &["glAyate"]);
    assert_has_ta_k(&[], &d("Yizva\\pa~", Adadi), Lat, &["supyate"]);
    assert_has_ta_k(&[], &d("Asa~\\", Adadi), Lat, &["Asyate"]);
    assert_has_ta_k(&[], &d("qukf\\Y", Tanadi), Lat, &["kriyate"]);
    assert_has_ta_k(&[], &d("hf\\Y", Bhvadi), Lat, &["hriyate"]);
    assert_has_ta_k(&[], &d("lUY", Kryadi), Lat, &["lUyate"]);
}

#[test]
fn sutra_1_3_17() {
    let vish = d("vi\\Sa~", Tudadi);
    assert_has_lat(&["ni"], &vish, &["niviSate"]);
    assert_has_lat(&["pra"], &vish, &["praviSati"]);
    assert_has_lan(&["ni"], &vish, &["nyaviSata"]);
}

#[test]
fn sutra_1_3_18() {
    let kri = d("qukrI\\Y", Kryadi);
    assert_has_lat(&["pari"], &kri, &["parikrIRIte"]);
    assert_has_lat(&["vi"], &kri, &["vikrIRIte"]);
    assert_has_lat(&["ava"], &kri, &["avakrIRIte"]);
}

#[test]
fn sutra_1_3_19() {
    let ji = d("ji\\", Bhvadi);
    assert_has_lat(&["vi"], &ji, &["vijayate"]);
    assert_has_lat(&["parA"], &ji, &["parAjayate"]);
}

#[test]
fn sutra_1_3_20() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_lat(&["AN"], &daa, &["Adatte", "AdadAti"]);
}

#[test]
fn sutra_1_3_21() {
    let krid = d("krIqf~", Bhvadi);
    assert_has_lat(&["anu"], &krid, &["anukrIqate"]);
    assert_has_lat(&["sam"], &krid, &["saNkrIqate"]);
    assert_has_lat(&["pari"], &krid, &["parikrIqate"]);
    assert_has_lat(&["AN"], &krid, &["AkrIqate"]);

    // TODO: varttikas
}

#[test]
fn sutra_1_3_22() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&["sam"], &stha, &["santizWate"]);
    assert_has_lat(&["ava"], &stha, &["avatizWate"]);
    assert_has_lat(&["pra"], &stha, &["pratizWate"]);
    assert_has_lat(&["vi"], &stha, &["vitizWate"]);
}

#[test]
fn sutra_1_3_22_v1() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&["AN"], &stha, &["AtizWati", "AtizWate"]);
}

#[test]
fn sutra_1_3_23() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&[], &stha, &["tizWati", "tizWate"]);
}

#[test]
fn sutra_1_3_24() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&["ud"], &stha, &["uttizWati", "uttizWate"]);
}

#[test]
fn sutra_1_3_25_and_sutra_1_3_26() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&["upa"], &stha, &["upatizWati", "upatizWate"]);
}

#[test]
fn sutra_1_3_27() {
    let tap = d("ta\\pa~", Bhvadi);
    assert_has_lat(&["ud"], &tap, &["uttapati", "uttapate"]);
    assert_has_lat(&["vi"], &tap, &["vitapati", "vitapate"]);
}

#[test]
fn sutra_1_3_28() {
    let yam = d("ya\\ma~", Bhvadi);
    let han = d("ha\\na~", Adadi);
    assert_has_lat(&["AN"], &yam, &["AyacCati", "AyacCate"]);
    assert_has_lat(&["AN"], &han, &["Ahanti", "Ahate"]);
}

#[test]
fn sutra_1_3_29() {
    assert_has_lat(
        &["sam"],
        &d("ga\\mx~", Bhvadi),
        &["saNgacCati", "saNgacCate"],
    );
    assert_has_lat(&["sam"], &d("fCa~", Bhvadi), &["samfcCati", "samfcCate"]);
    assert_has_lat(
        &["sam"],
        &d("pra\\Ca~", Tudadi),
        &["sampfcCati", "sampfcCate"],
    );
    assert_has_lat(&["sam"], &d("svf", Bhvadi), &["saMsvarati", "saMsvarate"]);
    assert_has_lat(&["sam"], &d("f\\", Bhvadi), &["samfcCati", "samfcCate"]);
    assert_has_lat(&["sam"], &d("Sru\\", Bhvadi), &["saMSfRoti", "saMSfRute"]);
    assert_has_lat(
        &["sam"],
        &d("vida~", Adadi),
        &["saMvetti", "saMveda", "saMvitte"],
    );
}

/*
#[test]
fn sutra_1_3_28_v1() {
    let dfs = d("ha\\na~", Adadi);
    assert_has_lat(&["sam"], &dfs, &["sampaSyati", "sampaSyate"]);
}
*/

#[test]
fn sutra_1_3_30() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_lat(&["ni"], &hve, &["nihvayate"]);
    assert_has_lat(&["sam"], &hve, &["saMhvayate"]);
    assert_has_lat(&["upa"], &hve, &["upahvayate"]);
    assert_has_lat(&["vi"], &hve, &["vihvayate"]);
}

#[test]
fn sutra_1_3_30_v1() {
    assert_has_lat(&["nis"], &d("asu~", Divadi), &["nirasyati", "nirasyate"]);
    assert_has_lat(&["sam"], &d("Uha~\\", Bhvadi), &["samUhati", "samUhate"]);
}

#[test]
fn sutra_1_3_31() {
    let han = d("hve\\Y", Bhvadi);
    assert_has_lat(&["AN"], &han, &["Ahvayati", "Ahvayate"]);
}

#[test]
fn sutra_1_3_33() {
    assert_has_lit(&["aDi"], &d("qukf\\Y", Tanadi), &["aDicakre", "aDicakAra"]);
    assert_has_lat(&["aDi"], &d("qukf\\Y", Tanadi), &["aDikaroti", "aDikurute"]);
}

#[test]
fn sutra_1_3_34() {
    assert_has_lat(&["vi"], &d("qukf\\Y", Tanadi), &["vikurute", "vikaroti"]);
}

#[test]
fn sutra_1_3_38() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(&[], &kram, &["krAmati", "krAmyati", "kramate", "kramyate"]);
}

#[test]
fn sutra_1_3_39() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(
        &["upa"],
        &kram,
        &["upakrAmati", "upakrAmyati", "upakramate", "upakramyate"],
    );
    assert_has_lat(
        &["parA"],
        &kram,
        &["parAkrAmati", "parAkrAmyati", "parAkramate", "parAkramyate"],
    );
}

#[test]
fn sutra_1_3_40() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(
        &["AN"],
        &kram,
        &["AkrAmati", "AkrAmyati", "Akramate", "Akramyate"],
    );
}

#[test]
fn sutra_1_3_41() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(
        &["vi"],
        &kram,
        &["vikrAmati", "vikrAmyati", "vikramate", "vikramyate"],
    );
}

#[test]
fn sutra_1_3_42() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(
        &["pra"],
        &kram,
        &["prakrAmati", "prakrAmyati", "prakramate", "prakramyate"],
    );
    assert_has_lat(
        &["upa"],
        &kram,
        &["upakrAmati", "upakrAmyati", "upakramate", "upakramyate"],
    );
}

#[test]
fn sutra_1_3_43() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(&[], &kram, &["krAmati", "krAmyati", "kramate", "kramyate"]);
}

#[test]
fn sutra_1_3_44() {
    let jna = d("jYA\\", Kryadi);
    assert_has_lat(&["apa"], &jna, &["apajAnAti", "apajAnIte"]);
}

#[test]
fn sutra_1_3_45() {
    let jna = d("jYA\\", Kryadi);
    assert_has_lat(&[], &jna, &["jAnAti", "jAnIte"]);
}

#[test]
fn sutra_1_3_46() {
    let jna = d("jYA\\", Kryadi);
    assert_has_lat(&["sam"], &jna, &["saYjAnAti", "saYjAnIte"]);
    assert_has_lat(&["prati"], &jna, &["pratijAnAti", "pratijAnIte"]);
}

#[test]
fn sutra_1_3_47() {
    let vad = d("vada~", Bhvadi);
    assert_has_lat(&[], &vad, &["vadati", "vadate"]);
}

#[test]
fn sutra_1_3_48() {
    let vad = d("vada~", Bhvadi);
    assert_has_lat(&["sam", "pra"], &vad, &["sampravadati", "sampravadate"]);
}

#[test]
fn sutra_1_3_49() {
    let vad = d("vada~", Bhvadi);
    assert_has_lat(&["anu"], &vad, &["anuvadati", "anuvadate"]);
}

#[test]
fn sutra_1_3_50() {
    let vad = d("vada~", Bhvadi);
    assert_has_jhi(&["vi", "pra"], &vad, Lat, &["vipravadanti"]);
    assert_has_jha(&["vi", "pra"], &vad, Lat, &["vipravadante"]);
}

#[test]
fn sutra_1_3_51() {
    let gf = d("gF", Tudadi);
    assert_has_lat(&["ava"], &gf, &["avagirate", "avagilate"]);
}

#[test]
fn sutra_1_3_52() {
    let gf = d("gF", Tudadi);
    assert_has_lat(
        &["sam"],
        &gf,
        &["saNgirati", "saNgilati", "saNgirate", "saNgilate"],
    );
}

#[test]
fn sutra_1_3_53() {
    let car = d("cara~", Bhvadi);
    assert_has_lat(&["ud"], &car, &["uccarati", "uccarate"]);
}

#[test]
fn sutra_1_3_54() {
    let car = d("cara~", Bhvadi);
    assert_has_lat(&["sam"], &car, &["saYcarati", "saYcarate"]);
}

#[test]
fn sutra_1_3_55() {
    let daa = d("dA\\R", Bhvadi);
    assert_has_lat(&["sam", "pra"], &daa, &["samprayacCati", "samprayacCate"]);
}

#[test]
fn sutra_1_3_56() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_lat(&["upa"], &yam, &["upayacCati", "upayacCate"]);
}

#[test]
fn sutra_1_3_57() {
    let jna = d("jYA\\", Kryadi);
    assert_has_lat(&[], &san(&jna), &["jijYAsate"]);

    let sru = d("Sru\\", Svadi);
    assert_has_lat(&[], &san(&sru), &["SuSrUzate"]);

    let smf = d("smf\\", Bhvadi);
    assert_has_lat(&[], &san(&smf), &["susmUrzate"]);

    let dfs = d("df\\Si~r", Bhvadi);
    assert_has_lat(&[], &san(&dfs), &["didfkzate"]);
}

#[test]
fn sutra_1_3_58() {
    let jna = d("jYA\\", Kryadi);
    assert_has_lat(&["anu"], &san(&jna), &["anujijYAsati"]);
}

#[test]
fn sutra_1_3_59() {
    let sru = d("Sru\\", Svadi);
    assert_has_lat(&["prati"], &san(&sru), &["pratiSuSrUzati"]);
    assert_has_lat(&["AN"], &san(&sru), &["ASuSrUzati"]);
}

#[test]
fn sutra_1_3_60() {
    let shad = d("Sa\\dx~", Tudadi);
    assert_has_lat(&[], &shad, &["SIyate"]);
    assert_has_lrn(&[], &shad, &["aSatsyat"]);
    assert_has_lrt(&[], &shad, &["Satsyati"]);
    assert_has_lat(&[], &san(&shad), &["SiSatsati"]);
}

#[test]
fn sutra_1_3_61() {
    let mr = d("mf\\N", Tudadi);
    assert_has_lun(&[], &mr, &["amfta"]);
    assert_has_ashirlin(&[], &mr, &["mfzIzwa"]);
    assert_has_lat(&[], &mr, &["mriyate"]);
    assert_has_lrt(&[], &mr, &["marizyati"]);
    assert_has_lrn(&[], &mr, &["amarizyat"]);
}

#[test]
fn sutra_1_3_62() {
    let san = |u, gana| d(u, gana).with_sanadi(&[Sanadi::san]);

    assert_has_lat(&[], &san("Asa~\\", Adadi), &["Asisizate"]);
    assert_has_lat(&[], &san("SIN", Adadi), &["SiSayizate"]);
    assert_has_lat(&["ni"], &san("vi\\Sa~", Tudadi), &["nivivikzate"]);
    assert_has_lat(
        &["AN"],
        &san("kramu~", Bhvadi),
        &["AcikraMsate", "Acikramizati"],
    );

    // From S. C. Vasu's commentary
    assert_has_lat(&[], &d("gupa~\\", Bhvadi), &["jugupsate"]);
    assert_has_lat(&[], &d("tija~\\", Bhvadi), &["titikzate"]);
    assert_has_lat(&[], &d("mAna~\\", Bhvadi), &["mImAMsate"]);
    assert_has_lat(&[], &d("baDa~\\", Bhvadi), &["bIBatsate"]);
}

#[test]
fn sutra_1_3_63() {
    assert_has_lit(
        &[],
        &d("Ikza~\\", Bhvadi),
        &["IkzAYcakre", "IkzAmbaBUva", "IkzAmAsa"],
    );
    assert_has_lit(
        &[],
        &d("Iha~\\", Bhvadi),
        &["IhAYcakre", "IhAmbaBUva", "IhAmAsa"],
    );
}

#[test]
fn sutra_1_3_64() {
    let yuj = d("yu\\ji~^r", Rudhadi);

    assert_has_lat(&["pra"], &yuj, &["prayuNkte", "prayunakti"]);
    assert_has_lat(&["upa"], &yuj, &["upayuNkte", "upayunakti"]);
}

#[test]
fn sutra_1_3_65() {
    let ksnu = d("kzRu", Adadi);

    assert_has_lat(&["sam"], &ksnu, &["saNkzRute"]);
}

// TODO: 1.3.64.v1

#[test]
fn sutra_1_3_66() {
    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_lat(&[], &bhuj, &["Bunakti", "BuNkte"]);
}

#[test]
fn sutra_1_3_68() {
    assert_has_lat(
        &[],
        &nic(&d("YiBI\\", Juhotyadi)),
        &["BAyayati", "BAyayate", "BIzayate", "BApayate"],
    );
    assert_has_lat(
        &["vi"],
        &nic(&d("zmi\\N", Bhvadi)),
        &["vismAyayati", "vismAyayate", "vismApayate"],
    );
}

#[test]
fn sutra_1_3_69() {
    assert_has_lat(&[], &nic(&d("gfDU~", Divadi)), &["garDayati", "garDayate"]);
    assert_has_lat(&[], &nic(&d("vancu~", Bhvadi)), &["vaYcayati", "vaYcayate"]);
}

#[test]
fn sutra_1_3_70() {
    let li = d("lI\\N", Divadi);
    assert_has_lat(
        &["AN"],
        &nic(&li),
        &[
            "AlApayati",
            "AlApayate",
            "AlAlayati",
            "AlAlayate",
            "AlAyayati",
            "AlAyayate",
            "AlInayati",
            "AlInayate",
        ],
    );
    assert_has_lat(
        &["ud"],
        &nic(&li),
        &[
            "ullApayati",
            "ullApayate",
            "ullAlayati",
            "ullAlayate",
            "ullAyayati",
            "ullAyayate",
            "ullInayati",
            "ullInayate",
        ],
    );
}

#[test]
fn sutra_1_3_72() {
    // svaritet
    assert_has_lat(&[], &d("ya\\ja~^", Bhvadi), &["yajati", "yajate"]);
    assert_has_lat(&[], &d("qupa\\ca~^z", Bhvadi), &["pacati", "pacate"]);
    // Yit
    assert_has_lat(&[], &d("zu\\Y", Svadi), &["sunoti", "sunute"]);
    assert_has_lat(&[], &d("qukf\\Y", Tanadi), &["karoti", "kurute"]);
}

#[test]
fn sutra_1_3_73() {
    let vad = d("vada~", Bhvadi);
    assert_has_lat(&["apa"], &vad, &["apavadati", "apavadate"]);
}

#[test]
fn sutra_1_3_74() {
    assert_has_lat(
        &[],
        &nic(&d("qupa\\ca~^z", Bhvadi)),
        &["pAcayate", "pAcayati"],
    );
    assert_has_lat(&[], &nic(&d("qukf\\Y", Tanadi)), &["kArayate", "kArayati"]);
}

#[test]
fn sutra_1_3_75() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_lat(&["sam"], &yam, &["saMyacCati", "saMyacCate"]);
    assert_has_lat(&["ud"], &yam, &["udyacCati", "udyacCate"]);
    assert_has_lat(&["AN"], &yam, &["AyacCati", "AyacCate"]);
}

#[test]
fn sutra_1_3_76() {
    let jna = d("jYA\\", Kryadi);
    assert_has_lat(&[], &jna, &["jAnAti", "jAnIte"]);
    assert_has_lat(&["pra"], &jna, &["prajAnAti"]);
}

#[test]
fn sutra_1_3_78() {
    assert_has_lat(&[], &d("yA\\", Adadi), &["yAti"]);
    assert_has_lat(&[], &d("vA\\", Adadi), &["vAti"]);

    let vish = d("vi\\Sa~", Tudadi);
    assert_has_lat(&["AN"], &vish, &["AviSati"]);
    assert_has_lat(&["pra"], &vish, &["praviSati"]);
}

#[test]
fn sutra_1_3_79() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_lat(&["anu"], &kr, &["anukaroti"]);
    assert_has_lat(&["parA"], &kr, &["parAkaroti"]);
}

#[test]
fn sutra_1_3_80() {
    let ksip = d("kzi\\pa~^", Tudadi);
    assert_has_lat(&["aBi"], &ksip, &["aBikzipati"]);
    assert_has_lat(&["prati"], &ksip, &["pratikzipati"]);
    assert_has_lat(&["ati"], &ksip, &["atikzipati"]);
    // abhi-prati-atiByaH?
    assert_has_lat(&["AN"], &ksip, &["Akzipati", "Akzipate"]);
}

#[test]
fn sutra_1_3_81() {
    let vah = d("va\\ha~^", Bhvadi);
    assert_has_lat(&["pra"], &vah, &["pravahati"]);
    assert_has_lat(&["AN"], &vah, &["Avahati", "Avahate"]);
}

#[test]
fn sutra_1_3_82() {
    let mfs = d("mfza~^", Divadi);
    assert_has_lat(&["pari"], &mfs, &["parimfzyati"]);
    assert_has_lat(&["AN"], &mfs, &["Amfzyati", "Amfzyate"]);
}

#[test]
fn sutra_1_3_83() {
    let ram = d("ra\\mu~\\", Bhvadi);
    assert_has_lat(&["vi"], &ram, &["viramati"]);
    assert_has_lat(&["AN"], &ram, &["Aramati"]);
    assert_has_lat(&["pari"], &ram, &["pariramati"]);
    assert_has_lat(&["aBi"], &ram, &["aBiramate"]);
}

#[test]
fn sutra_1_3_84_and_sutra_1_3_85() {
    let ram = d("ra\\ma~\\", Bhvadi);
    assert_has_lat(&["upa"], &ram, &["uparamati", "uparamate"]);
}

#[test]
fn sutra_1_3_86() {
    assert_has_lat(&[], &nic(&d("bu\\Da~\\", Divadi)), &["boDayati"]);
    assert_has_lat(&[], &nic(&d("yu\\Da~\\", Divadi)), &["yoDayati"]);
    assert_has_lat(&[], &nic(&d("Ra\\Sa~", Divadi)), &["nASayati"]);
    assert_has_lat(&[], &nic(&d("janI~\\", Divadi)), &["janayati"]);
    assert_has_lat(&["aDi"], &nic(&d("i\\N", Adadi)), &["aDyApayati"]);
    assert_has_lat(&[], &nic(&d("pru\\N", Bhvadi)), &["prAvayati"]);
    assert_has_lat(&[], &nic(&d("dru\\", Bhvadi)), &["drAvayati"]);
    assert_has_lat(&[], &nic(&d("sru\\", Bhvadi)), &["srAvayati"]);
}

#[ignore]
#[test]
fn sutra_1_3_89() {
    assert_has_lat(&[], &nic(&d("pA\\", Bhvadi)), &["pAyayate"]);
    assert_has_lat(&[], &nic(&d("damu~", Divadi)), &["damayate"]);
    assert_has_lat(&["AN"], &nic(&d("yama~", Bhvadi)), &["AyAmayate"]);
    assert_has_lat(&["AN"], &nic(&d("yasu~", Divadi)), &["AyAsayate"]);
    assert_has_lat(&["pari"], &nic(&d("mu\\ha~", Divadi)), &["parimohayate"]);
    assert_has_lat(&[], &nic(&d("ruca~\\", Bhvadi)), &["rocayate"]);
    assert_has_lat(&[], &nic(&d("nftI~", Divadi)), &["nartayate"]);
    assert_has_lat(&[], &nic(&d("vada~", Bhvadi)), &["vAdayate"]);
    assert_has_lat(&[], &nic(&d("va\\sa~", Bhvadi)), &["vAsayate"]);
}

#[ignore]
#[test]
fn sutra_1_3_89_v1() {
    assert_has_lat(&[], &nic(&d("De\\w", Bhvadi)), &["DApayate"]);
}

#[test]
fn sutra_1_3_90() {
    let kyas = |prati| Dhatu::nama(phit(prati), None);
    assert_has_lat(&[], &kyas("lohita"), &["lohitAyati", "lohitAyate"]);
}

#[test]
fn sutra_1_3_91() {
    let dyut = &d("dyuta~\\", Bhvadi);
    assert_has_lun(&["vi"], &dyut, &["vyadyutat", "vyadyotizwa"]);
    assert_has_lun(&[], &d("luWa~\\", Bhvadi), &["aluWat", "aloWizwa"]);
    // luNi?
    assert_has_lat(&[], &dyut, &["dyotate"]);
}

#[test]
fn sutra_1_3_92() {
    let dyut = &d("dyuta~\\", Bhvadi);
    assert_has_lun(&["vi"], &dyut, &["vyadyutat", "vyadyotizwa"]);
    assert_has_lun(&[], &d("luWa~\\", Bhvadi), &["aluWat", "aloWizwa"]);
    // luNi?
    assert_has_lat(&[], &dyut, &["dyotate"]);
}

#[test]
fn sutra_1_3_93() {
    let vft = d("vftu~\\", Bhvadi);
    assert_has_lrt(&[], &vft, &["vartsyati", "vartizyate"]);
    assert_has_lrn(&[], &vft, &["avartsyat", "avartizyata"]);
    assert_has_lat(&[], &san(&vft), &["vivftsati", "vivartizate"]);

    let vfdh = d("vfDu~\\", Bhvadi);
    assert_has_lrt(&[], &vfdh, &["vartsyati", "varDizyate"]);
    assert_has_lrn(&[], &vfdh, &["avartsyat", "avarDizyata"]);
    assert_has_lat(&[], &san(&vfdh), &["vivftsati", "vivarDizate"]);
}
