extern crate test_utils;
use lazy_static::lazy_static;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Pada;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::Unadi;

lazy_static! {
    static ref S: Tester = Tester::with_svara_rules();
}

#[test]
fn sutra_1_1_1() {
    assert_has_taddhita("aSvala", T::Pak, &["ASvalAyana"]);
    assert_has_taddhita("itika", T::Pak, &["EtikAyana"]);
    assert_has_taddhita("upagu", T::aR, &["Opagava"]);
    assert_has_taddhita("upamanyu", T::aR, &["Opamanyava"]);
    assert_has_taddhita("SAlA", T::Ca, &["SAlIya"]);
    assert_has_taddhita("mAlA", T::Ca, &["mAlIya"]);
}

#[test]
fn sutra_1_1_2() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::tfc, &["taritf", "tarItf"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::tfc, &["stotf"]);
    assert_has_jhi(&[], &pac, Lat, &["pacanti"]);
    assert_has_jhi(&[], &d("ji\\", Bhvadi), Lat, &["jayanti"]);
    assert_has_iw(&[], &pac, Lat, &["pace"]);
}

#[test]
fn sutra_1_1_3() {
    // guRa
    assert_has_tip(&[], &d("tF", Bhvadi), Lat, &["tarati"]);
    assert_has_tip(&[], &d("RI\\Y", Bhvadi), Lat, &["nayati"]);
    assert_has_tip(&[], &d("BU", Bhvadi), Lat, &["Bavati"]);
    // vfdDi
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lun, &["akArzIt"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lun, &["ahArzIt"]);
    assert_has_tip(&[], &d("ci\\Y", Svadi), Lun, &["acEzIt"]);
    assert_has_tip(&[], &d("RI\\Y", Bhvadi), Lun, &["anEzIt"]);
    assert_has_tip(&[], &d("lUY", Kryadi), Lun, &["alAvIt"]);
    assert_has_tip(&[], &d("zwu\\Y", Adadi), Lun, &["astAvIt"]);

    // Other examples
    assert_has_tip(&[], &d("YimidA~", Divadi), Lat, &["medyati"]);
    assert_has_jhi(&[], &d("YiBI\\", Juhotyadi), Lan, &["abiBayuH"]);

    // ikaH
    assert_has_krdanta(&[], &d("yA\\", Adadi), Krt::lyuw, &["yAna"]);
    assert_has_tip(&[], &d("glE\\", Bhvadi), Lat, &["glAyati"]);
    assert_has_krdanta(&[], &d("unBa~", Tudadi), Krt::tfc, &["umBitf"]);

    // punargrahaNam of the terms "guna" and "vrddhi"
    assert_has_sup_1s("div", Stri, &["dyOH"]);
    assert_has_sup_1s("paTin", Pum, &["panTAH"]);
    assert_has_sup_1s("tad", Pum, &["saH"]);
    assert_has_sup_2s("idam", Pum, &["imam"]);
}

#[test]
fn sutra_1_1_4() {
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &yan(&lu), Krt::ac, &["loluva"]);
    assert_has_krdanta(&[], &yan(&d("pUY", Kryadi)), Krt::ac, &["popuva"]);
    assert_has_krdanta(&[], &yan(&d("mfjU~", Adadi)), Krt::ac, &["marImfja"]);

    // dhAtu?
    assert_has_krdanta(&[], &lu, Krt::tfc, &["lavitf"]);
    assert_has_krdanta(&[], &d("riza~", Bhvadi), Krt::vic, &["rez"]);
    assert_has_sip(&[], &d("RI\\Y", Bhvadi), VidhiLin, &["nayeH"]);

    // TODO: ArdhadhAtuke?

    // ikaH
    assert_has_ta_k(&[], &d("Ba\\njo~", Rudhadi), Lun, &["aBAji", "aBaYji"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Divadi), Krt::GaY, &["rAga", "raNga"]);

    // bahuvrIhi?
    assert_has_tip(&[], &nic(&d("knUyI~\\", Bhvadi)), Lat, &["knopayati"]);
    // TODO: preddha
}

#[test]
fn sutra_1_1_5() {
    let ci = d("ci\\Y", Svadi);
    assert_has_krdanta(&[], &ci, Krt::kta, &["cita"]);
    assert_has_krdanta(&[], &ci, Krt::ktavatu, &["citavat"]);

    let stu = d("zwu\\Y", Adadi);
    assert_has_krdanta(&[], &stu, Krt::kta, &["stuta"]);
    assert_has_krdanta(&[], &stu, Krt::ktavatu, &["stutavat"]);

    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_krdanta(&[], &bhid, Krt::kta, &["Binna"]);
    assert_has_krdanta(&[], &bhid, Krt::ktavatu, &["Binnavat"]);

    let mfj = d("mfjU~", Adadi);
    assert_has_krdanta(&[], &mfj, Krt::kta, &["mfzwa"]);
    assert_has_krdanta(&[], &mfj, Krt::ktavatu, &["mfzwavat"]);

    // Niti
    assert_has_tas(&[], &ci, Lat, &["cinutaH"]);
    assert_has_jhi(&[], &ci, Lat, &["cinvanti"]);
    assert_has_tas(&[], &mfj, Lat, &["mfzwaH"]);
    // mArjanti per SK.
    assert_has_jhi(&[], &mfj, Lat, &["mfjanti", "mArjanti"]);

    // git
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::ksnu, &["jizRu"]);
    assert_has_krdanta(&[], &d("BU", Bhvadi), Krt::ksnu, &["BUzRu"]);

    // ikaH
    assert_has_lat(&[], &d("kamu~\\", Bhvadi), &["kAmayate"]);
    assert_has_taddhita("ligu", T::Pak, &["lEgavAyana"]);

    // lakAra-Nit
    assert_has_mip(&[], &ci, Lan, &["acinavam"]);
    assert_has_mip(&[], &d("zu\\Y", Svadi), Lan, &["asunavam"]);
}

#[test]
fn sutra_1_1_6() {
    let didhi = d("dIDIN", Adadi);
    assert_has_krdanta(&["AN"], &didhi, Krt::lyuw, &["AdIDyana"]);
    assert_has_krdanta(&["AN"], &didhi, Krt::Rvul, &["AdIDyaka"]);

    let vevi = d("vevIN", Adadi);
    assert_has_krdanta(&["AN"], &vevi, Krt::lyuw, &["Avevyana"]);
    assert_has_krdanta(&["AN"], &vevi, Krt::Rvul, &["Avevyaka"]);

    assert_has_lut(&[], &d("kaRa~", Bhvadi), &["kaRitA"]);
    assert_has_lut(&[], &d("raRa~", Bhvadi), &["raRitA"]);
}

// No examples available for these sutras.
#[test]
fn skip_sutra_1_1_7_to_sutra_1_1_8() {}

#[test]
fn sutra_1_1_9() {
    let tfp = d("tf\\pa~", Divadi);
    assert_has_krdanta(&[], &tfp, Krt::tfc, &["traptf", "tarptf", "tarpitf"]);
    assert_has_krdanta(&[], &tfp, Krt::tumun, &["traptum", "tarptum", "tarpitum"]);
}

#[test]
fn sutra_1_1_10() {
    assert_has_sandhi("daRqa", "hasta", &["daRqahasta"]);
    assert_has_sandhi("daDi", "SItam", &["daDiSItam"]);
    assert_has_taddhita("vipASa", T::aR, &["vEpASa"]);
    assert_has_taddhita("anaquh", T::aR, &["Anaquha"]);
}

#[test]
fn sutra_1_1_11() {
    let hari = sup_1d("harI", "hari", Pum);
    let etau = sup_1d("etO", "etad", Pum);
    assert_has_vakya(&hari, &etau, &["harI etO"]);
}

#[test]
fn sutra_1_1_12() {
    let ami = sup_1p("amI", "adas", Pum);
    let amu = sup_1d("amU", "adas", Pum);
    let atra = Pada::from_text("atra");
    let asate = Pada::from_text("Asate");

    assert_has_vakya(&ami, &atra, &["amI atra"]);
    assert_has_vakya(&ami, &asate, &["amI Asate"]);
    assert_has_vakya(&amu, &atra, &["amU atra"]);
    assert_has_vakya(&amu, &asate, &["amU Asate"]);

    // adasaH?
    let shami = Pada::from_text("SamI");
    let dadimi = Pada::from_text("dAqimI");
    assert_has_vakya(&shami, &atra, &["Samyatra"]);
    assert_has_vakya(&dadimi, &atra, &["dAqimyatra"]);

    // TODO: amuke
}

#[test]
fn sutra_1_1_14() {
    let a = Pada::nipata("a");
    let apehi = Pada::from_text("apehi");
    assert_has_vakya(&a, &apehi, &["a apehi"]);

    let i = Pada::nipata("i");
    let indra = Pada::from_text("indram");
    assert_has_vakya(&i, &indra, &["i indram"]);

    let u = Pada::nipata("u");
    let uttistha = Pada::from_text("uttizWa");
    assert_has_vakya(&u, &uttistha, &["u uttizWa"]);

    let aa = Pada::nipata("A");
    let evam = Pada::from_text("evam");
    assert_has_vakya(&aa, &evam, &["A evam"]);

    // nipAta?
    let cakara = Pada::from_text("cakAra");
    let jahara = Pada::from_text("jahAra");
    let atra = Pada::from_text("atra");
    assert_has_vakya(&cakara, &atra, &["cakArAtra"]);
    assert_has_vakya(&jahara, &atra, &["jahArAtra"]);

    // ekAc?
    let pra = Pada::nipata("pra");
    let agnaye = Pada::from_text("agnaye");
    assert_has_vakya(&pra, &agnaye, &["prAgnaye"]);

    // anAN?
    let aan = Pada::nipata("AN");
    let udakantat = Pada::from_text("udakAntAt");
    assert_has_vakya(&aa, &udakantat, &["A udakAntAt"]);
    assert_has_vakya(&aan, &udakantat, &["odakAntAt"]);
}

#[test]
fn sutra_1_1_15() {
    let aho = Pada::nipata("Aho");
    let utaho = Pada::nipata("utAho");
    let iti = Pada::from_text("iti");

    assert_has_vakya(&aho, &iti, &["Aho iti"]);
    assert_has_vakya(&utaho, &iti, &["utAho iti"]);
}

#[test]
fn sutra_1_1_17_to_sutra_1_1_18() {
    let un = Pada::nipata("uY");
    let iti = Pada::from_text("iti");
    assert_has_vakya(&un, &iti, &["u iti", "U~ iti", "viti"]);
}

#[test]
fn sutra_1_1_20() {
    assert_has_tip(
        &["pra", "ni"],
        &d("qudA\\Y", Juhotyadi),
        Lat,
        &["praRidadAti"],
    );
    assert_has_krdanta(
        &["pra", "ni"],
        &d("dA\\R", Bhvadi),
        Krt::tfc,
        &["praRidAtf"],
    );
    assert_has_lat(&["pra", "ni"], &d("do\\", Divadi), &["praRidyati"]);
    assert_has_lat(&["pra", "ni"], &d("de\\N", Bhvadi), &["praRidayate"]);
    assert_has_tip(
        &["pra", "ni"],
        &d("quDA\\Y", Juhotyadi),
        Lat,
        &["praRidaDAti"],
    );
    assert_has_tip(&["pra", "ni"], &d("De\\w", Bhvadi), Lat, &["praRiDayati"]);
    // adAp
    assert_has_krdanta(&[], &d("dA\\p", Adadi), Krt::kta, &["dAta"]);
    assert_has_krdanta(&["ava"], &d("dE\\p", Bhvadi), Krt::kta, &["avadAta"]);
}

#[test]
fn sutra_1_1_21() {
    S.assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tavya, &["karta/vya"]);
    S.assert_has_taddhita("upagu", T::aR, &["Opagava/"]);
    assert_has_sup_3d("vfkza", Pum, &["vfkzAByAm"]);
}

#[test]
fn sutra_1_1_22() {
    assert_has_taddhita("kumArI", T::tarap, &["kumAritara"]);
    assert_has_taddhita("kumArI", T::tamap, &["kumAritama"]);
    assert_has_taddhita("brAhmaRI", T::tarap, &["brAhmaRitara"]);
    assert_has_taddhita("brAhmaRI", T::tamap, &["brAhmaRitama"]);
}

#[test]
fn sutra_1_1_23() {
    // TODO: others
    assert_has_taddhita("bahu", T::kftvasuc, &["bahukftvas"]);
    assert_has_taddhita("bahu", T::DA, &["bahuDA"]);
    assert_has_taddhita("bahu", T::kan, &["bahuka"]);
    // TODO: assert_has_taddhita("bahu", T::Sas, &["bahuSas"]);

    assert_has_taddhita("gaRa", T::kftvasuc, &["gaRakftvas"]);
    assert_has_taddhita("gaRa", T::DA, &["gaRaDA"]);
    assert_has_taddhita("gaRa", T::kan, &["gaRaka"]);
    // TODO: assert_has_taddhita("gaRa", T::Sas, &["gaRaSas"]);

    // TODO:
    // let taavat = prati("tAvat");
    // assert_has_taddhitanta(&taavat, T::kftvasuc, &["tAvatkftvas"]);
    // assert_has_taddhitanta(&taavat, T::DA, &["tAvadDA"]);
    // assert_has_taddhitanta(&taavat, T::kan, &["tAvatka"]);
    // assert_has_taddhitanta(&taavat, T::Sas, &["tAvacCas"]);

    // let kati = prati("kati");
    // assert_has_taddhitanta(&kati, T::kftvasuc, &["katikftvas"]);
    // assert_has_taddhitanta(&kati, T::DA, &["katiDA"]);
    // assert_has_taddhitanta(&kati, T::kan, &["katika"]);
    // assert_has_taddhitanta(&kati, T::Sas, &["katizas"]);
}

#[test]
fn sutra_1_1_24() {
    assert_has_sup_1p("zaz", Napumsaka, &["zaw"]);
    assert_has_sup_1p("paYcan", Napumsaka, &["paYca"]);
    assert_has_sup_1p("saptan", Napumsaka, &["sapta"]);
    assert_has_sup_1p("navan", Napumsaka, &["nava"]);
    assert_has_sup_1p("daSan", Napumsaka, &["daSa"]);
    // others
    assert_has_sup_1p("Sata", Napumsaka, &["SatAni"]);
    assert_has_sup_1p("sahasra", Napumsaka, &["sahasrARi"]);
}

#[test]
fn sutra_1_1_25() {
    assert_has_taddhita("kim", T::qati, &["kati"]);
}

#[test]
fn sutra_1_1_26() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::kta, &["kfta"]);
    assert_has_krdanta(&[], &kf, Krt::ktavatu, &["kftavat"]);

    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_krdanta(&[], &bhuj, Krt::kta, &["Bukta"]);
    assert_has_krdanta(&[], &bhuj, Krt::ktavatu, &["Buktavat"]);
}

#[test]
fn sutra_1_1_27() {
    // use Taddhita as T;
    assert_has_sup_1s("sarva", Pum, &["sarvaH"]);
    assert_has_sup_1d("sarva", Pum, &["sarvO"]);
    assert_has_sup_1p("sarva", Pum, &["sarve"]);
    assert_has_sup_4s("sarva", Pum, &["sarvasmE"]);
    assert_has_sup_5s("sarva", Pum, &["sarvasmAt"]);
    assert_has_sup_6p("sarva", Pum, &["sarvezAm"]);
    assert_has_sup_7s("sarva", Pum, &["sarvasmin"]);
    // assert_has_taddhitanta("sarva", T::akac, &["sarvaka"]);

    assert_has_sup_1s("viSva", Pum, &["viSvaH"]);
    assert_has_sup_1d("viSva", Pum, &["viSvO"]);
    assert_has_sup_1p("viSva", Pum, &["viSve"]);
    assert_has_sup_4s("viSva", Pum, &["viSvasmE"]);
    assert_has_sup_5s("viSva", Pum, &["viSvasmAt"]);
    assert_has_sup_6p("viSva", Pum, &["viSvezAm"]);
    assert_has_sup_7s("viSva", Pum, &["viSvasmin"]);
    // assert_has_taddhitanta("viSva", T::akac, &["viSvaka"]);

    assert_has_sup_1p("uBaya", Pum, &["uBaye"]);
    assert_has_sup_4s("uBaya", Pum, &["uBayasmE"]);
    assert_has_sup_5s("uBaya", Pum, &["uBayasmAt"]);
    assert_has_sup_6p("uBaya", Pum, &["uBayezAm"]);
    assert_has_sup_7s("uBaya", Pum, &["uBayasmin"]);
    // qatara
    assert_has_sup_4s("katara", Pum, &["katarasmE"]);
    assert_has_sup_4s("katama", Pum, &["katamasmE"]);
    // itara, etc.
    assert_has_sup_4s("itara", Pum, &["itarasmE"]);
    assert_has_sup_4s("anya", Pum, &["anyasmE"]);
    assert_has_sup_4s("anyatara", Pum, &["anyatarasmE"]);

    // TODO: others
}

#[test]
fn sutra_1_1_33() {
    assert_has_sup_1p("praTama", Pum, &["praTame", "praTamAH"]);
    assert_has_sup_1p("carama", Pum, &["carame", "caramAH"]);
    assert_has_sup_1p("alpa", Pum, &["alpe", "alpAH"]);
    assert_has_sup_1p("arDa", Pum, &["arDe", "arDAH"]);
    assert_has_sup_1p("katipaya", Pum, &["katipaye", "katipayAH"]);
    assert_has_sup_1p("nema", Pum, &["neme", "nemAH"]);

    // TODO: taya
}

#[test]
fn sutra_1_1_34() {
    assert_has_sup_1p("pUrva", Pum, &["pUrve", "pUrvAH"]);
    assert_has_sup_1p("para", Pum, &["pare", "parAH"]);
    assert_has_sup_1p("avara", Pum, &["avare", "avarAH"]);
    assert_has_sup_1p("dakziRa", Pum, &["dakziRe", "dakziRAH"]);
    assert_has_sup_1p("uttara", Pum, &["uttare", "uttarAH"]);
    assert_has_sup_1p("apara", Pum, &["apare", "aparAH"]);
    assert_has_sup_1p("aDara", Pum, &["aDare", "aDarAH"]);
}

#[test]
fn skip_sutra_1_1_37() {}

#[test]
fn sutra_1_1_38() {
    assert_has_taddhita("tad", T::tasil, &["tatas"]);
    assert_has_taddhita("yad", T::tasil, &["yatas"]);
    assert_has_taddhita("tad", T::tral, &["tatra"]);
    assert_has_taddhita("yad", T::tral, &["yatra"]);
    assert_has_taddhita("tad", T::dA, &["tadA"]);
    assert_has_taddhita("yad", T::dA, &["yadA"]);
    assert_has_taddhita("sarva", T::dA, &["sarvadA", "sadA"]);

    // taddhite?
    assert_has_sup_1s("eka", Pum, &["ekaH"]);
    assert_has_sup_1d("dvi", Pum, &["dvO"]);
    assert_has_sup_1p("bahu", Pum, &["bahavaH"]);

    // asarvavibhakti?
    let aupagava = taddhitanta("upagu", T::aR);
    assert_has_sup_1s(&aupagava, Pum, &["OpagavaH"]);
    assert_has_sup_1d(&aupagava, Pum, &["OpagavO"]);
    assert_has_sup_1p(&aupagava, Pum, &["OpagavAH"]);
}

#[test]
fn sutra_1_1_40() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::ktvA, &["kftvA"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::ktvA, &["hftvA"]);
    // TODO: tosun

    // kasun
    assert_has_krdanta(&["vi"], &d("sf\\px~", Tudadi), Krt::kasun, &["visfpas"]);
    assert_has_krdanta(&["AN"], &d("u~tfdi~^r", Rudhadi), Krt::kasun, &["Atfdas"]);
}

#[test]
fn sutra_1_1_41() {
    assert_has_avyayibhava("upa", "agni", &["upAgni"]);
    assert_has_avyayibhava("prati", "agni", &["pratyagni"]);
    assert_has_avyayibhava("upa", "payas", &["upapayaH"]);

    // TODO: others
}

#[test]
fn sutra_1_1_42() {
    assert_has_sup_1p("kuRqa", Napumsaka, &["kuRqAni"]);
    assert_has_sup_2p("kuRqa", Napumsaka, &["kuRqAni"]);
    assert_has_sup_1p("daDi", Napumsaka, &["daDIni"]);
    assert_has_sup_1p("maDu", Napumsaka, &["maDUni"]);
    assert_has_sup_1p("trapu", Napumsaka, &["trapURi"]);
    assert_has_sup_1p("jatu", Napumsaka, &["jatUni"]);
}

#[test]
fn sutra_1_1_43() {
    assert_has_sup_1s("rAjan", Pum, &["rAjA"]);
    assert_has_sup_1d("rAjan", Pum, &["rAjAnO"]);
    assert_has_sup_1p("rAjan", Pum, &["rAjAnaH"]);
    assert_has_sup_2s("rAjan", Pum, &["rAjAnam"]);
    assert_has_sup_2d("rAjan", Pum, &["rAjAnO"]);
    // suw
    assert_has_sup_2p("rAjan", Pum, &["rAjYaH"]);
    // anapuMsakasya
    assert_has_sup_1d("sAman", Napumsaka, &["sAmanI", "sAmnI"]);
    assert_has_sup_1d("veman", Napumsaka, &["vemanI", "vemnI"]);
}

#[test]
fn sutra_1_1_44() {
    let svi = d("wuo~Svi", Bhvadi);
    assert_has_tip(&[], &svi, Lit, &["SuSAva", "SiSvAya"]);
    assert_has_tas(&[], &svi, Lit, &["SuSuvatuH", "SiSviyatuH"]);
}

#[test]
fn sutra_1_1_45() {
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_krdanta(&[], &yaj, Krt::kta, &["izwa"]);
    let vap = d("quva\\pa~^", Bhvadi);
    assert_has_krdanta(&[], &vap, Krt::kta, &["upta"]);
    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&[], &grah, Krt::kta, &["gfhIta"]);
}

#[test]
fn sutra_1_1_46() {
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::tfc, &["lavitf"]);

    assert_has_lat(
        &[],
        &nic(&d("YiBI\\", Juhotyadi)),
        &["BAyayati", "BAyayate", "BIzayate", "BApayate"],
    );
}

#[test]
fn sutra_1_1_47() {
    assert_has_tip(&["vi"], &d("ru\\Di~^r", Rudhadi), Lat, &["viruRadDi"]);
    assert_has_tip(&[], &d("mu\\cx~^", Tudadi), Lat, &["muYcati"]);
    assert_has_sup_1p("payas", Napumsaka, &["payAMsi"]);
}

#[test]
fn sutra_1_1_47_v1() {
    let masj = d("wuma\\sjo~", Tudadi);
    assert_has_krdanta(&[], &masj, Krt::kta, &["magna"]);
    assert_has_krdanta(&[], &masj, Krt::ktavatu, &["magnavat"]);
    assert_has_krdanta(&[], &masj, Krt::tfc, &["maNktf"]);
    assert_has_krdanta(&[], &masj, Krt::tumun, &["maNktum"]);
}

#[test]
fn sutra_1_1_48() {
    assert_has_sup_1s("atirE", Napumsaka, &["atiri"]);
    assert_has_sup_1s("atinO", Napumsaka, &["atinu"]);
    assert_has_sup_1s("upago", Napumsaka, &["upagu"]);
    // ecaH?
    // TODO: assert_has_sup_1s("atiKawvA", Pum, &["atiKawvaH"]);
    // TODO: assert_has_sup_1s("atimAlA", Pum, &["atimAlaH"]);
}

#[test]
fn sutra_1_1_49() {
    let as_ = d("asa~", Adadi);
    assert_has_krdanta(&[], &as_, Krt::tfc, &["Bavitf"]);
    assert_has_krdanta(&[], &as_, Krt::tumun, &["Bavitum"]);
    assert_has_krdanta(&[], &as_, Krt::tavya, &["Bavitavya"]);

    let bru = d("brUY", Adadi);
    assert_has_krdanta(&[], &bru, Krt::tfc, &["vaktf"]);
    assert_has_krdanta(&[], &bru, Krt::tumun, &["vaktum"]);
    assert_has_krdanta(&[], &bru, Krt::tavya, &["vaktavya"]);
}

#[test]
fn sutra_1_1_50() {
    assert_has_sandhi("daRqa", "agram", &["daRqAgram"]);
    assert_has_sandhi("yUpa", "agram", &["yUpAgram"]);

    // TODO: arthataH

    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Bhvadi), Krt::GaY, &["tyAga"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Bhvadi), Krt::GaY, &["rAga", "raNga"]);

    assert_has_sup_4s("adas", Pum, &["amuzmE"]);
    assert_has_sup_4d("adas", Pum, &["amUByAm"]);

    assert_has_sandhi("vAk", "hasati", &["vAg Gasati", "vAg hasati"]);
    assert_has_sandhi("trizWup", "hasati", &["trizWub Basati", "trizWub hasati"]);
}

#[test]
fn sutra_1_1_51() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::tfc, &["hartf"]);
    assert_has_lat(&[], &d("kF", Tudadi), &["kirati"]);
    assert_has_lat(&[], &d("gF", Tudadi), &["girati", "gilati"]);

    // TODO: others

    // uH
    assert_has_krdanta(&[], &d("Kanu~^", Bhvadi), Krt::kyap, &["Keya"]);
    assert_has_krdanta(&[], &d("gE\\", Bhvadi), Krt::yat, &["geya"]);
}

#[test]
fn sutra_1_1_55() {
    let asa = d("asa~", Adadi);
    // aneka-al
    assert_has_krdanta(&[], &asa, Krt::tfc, &["Bavitf"]);
    assert_has_krdanta(&[], &asa, Krt::tumun, &["Bavitum"]);
    assert_has_krdanta(&[], &asa, Krt::tavya, &["Bavitavya"]);
    // Sit
    assert_has_sup_1p("kuRqa", Napumsaka, &["kuRqAni"]);
    assert_has_sup_2p("kuRqa", Napumsaka, &["kuRqAni"]);

    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::tfc, &["stotf"]);
}

#[test]
fn sutra_1_1_56() {
    assert_has_krdanta(&[], &d("asa~", Adadi), Krt::tfc, &["Bavitf"]);
    assert_has_krdanta(&[], &d("asa~", Adadi), Krt::tumun, &["Bavitum"]);
    assert_has_krdanta(&[], &d("asa~", Adadi), Krt::tavya, &["Bavitavya"]);

    assert_has_krdanta(&[], &d("brUY", Adadi), Krt::tfc, &["vaktf"]);
    assert_has_krdanta(&[], &d("brUY", Adadi), Krt::tumun, &["vaktum"]);
    assert_has_krdanta(&[], &d("brUY", Adadi), Krt::tavya, &["vaktavya"]);

    assert_has_sup_3s("kim", Pum, &["kena"]);
    assert_has_sup_3d("kim", Pum, &["kAByAm"]);
    assert_has_sup_3p("kim", Pum, &["kEH"]);

    let kf = d("qukf\\Y", Tanadi);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&["pra"], &kf, Krt::ktvA, &["prakftya"]);
    assert_has_krdanta(&["pra"], &hf, Krt::ktvA, &["prahftya"]);

    // TODO: taddhita tests

    let stu = d("zwu\\Y", Adadi);
    assert_has_krdanta(&["pra"], &kf, Krt::ktvA, &["prakftya"]);
    assert_has_krdanta(&["pra"], &stu, Krt::ktvA, &["prastutya"]);
    assert_has_krdanta(&["pra"], &hf, Krt::ktvA, &["prahftya"]);
    assert_has_krdanta(&["upa"], &hf, Krt::ktvA, &["upahftya"]);
    assert_has_krdanta(&["upa"], &stu, Krt::ktvA, &["upastutya"]);

    assert_has_sup_4s("vfkza", Pum, &["vfkzAya"]);
    assert_has_sup_4s("plakza", Pum, &["plakzAya"]);

    assert_has_tas(&[], &kf, Lan, &["akurutAm"]);
    assert_has_thas(&[], &kf, Lan, &["akurutam"]);

    // TODO: others
}

#[test]
fn sutra_1_1_59() {
    let pai = d("pE\\", Bhvadi);
    assert_has_tas(&[], &pai, Lit, &["papatuH"]);
    assert_has_jhi(&[], &pai, Lit, &["papuH"]);

    let han = d("ha\\na~", Adadi);
    assert_has_tas(&[], &han, Lit, &["jaGnatuH"]);
    assert_has_jhi(&[], &han, Lit, &["jaGnuH"]);

    let aw = d("awa~", Bhvadi);
    assert_has_tip(&[], &nic(&aw), Lun, &["Awiwat"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_tas(&[], &kf, Lit, &["cakratuH"]);
    assert_has_jhi(&[], &kf, Lit, &["cakruH"]);

    assert_has_mip(&[], &d("RI\\Y", Bhvadi), Lit, &["ninaya", "ninAya"]);
    assert_has_mip(&[], &d("lUY", Kryadi), Lit, &["lulava", "lulAva"]);

    // dvirvacane
    assert_has_ta_k(&[], &d("glE\\", Bhvadi), Lit, &["jagle"]);
    assert_has_ta_k(&[], &d("mlE\\", Bhvadi), Lit, &["mamle"]);

    // dvirvacananimitte
    assert_has_lat(&[], &san(&d("divu~", Divadi)), &["didevizati", "dudyUzati"]);

    // aci
    assert_has_lat(&[], &yan(&d("GrA\\", Bhvadi)), &["jeGrIyate"]);
    assert_has_lat(&[], &yan(&d("DmA\\", Bhvadi)), &["deDmIyate"]);
}

#[test]
fn sutra_1_1_60() {
    assert_has_artha_taddhita("goDA", TasyaApatyam, T::Qrak, &["gODera"]);
    assert_has_jha(&[], &d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceran"]);
    assert_has_krdanta(&[], &d("jIva~", Bhvadi), Unadi::radAnuk, &["jIradAnu"]);
}

#[test]
fn sutra_1_1_61() {
    assert_has_tip(&[], &d("a\\da~", Adadi), Lat, &["atti"]);
    assert_has_tip(&[], &d("hu\\", Juhotyadi), Lat, &["juhoti"]);
    // TODO: lup
}

#[test]
fn sutra_1_1_62() {
    assert_has_upapada_krdanta("agni", &[], &d("ci\\Y", Svadi), Krt::kvip, &["agnicit"]);
    assert_has_upapada_krdanta("soma", &[], &d("zu\\Y", Svadi), Krt::kvip, &["somasut"]);
    let duh = d("du\\ha~^", Adadi);
    assert_has_tip(&[], &duh, Lan, &["aDok"]);

    // TODO: how to derive AGnIya, AgmIya?

    assert_has_sasthi_tatpurusha("rE", "kula", &["rEkula"]);
    assert_has_caturthi_tatpurusha("go", "hita", &["gohita"]);
}

#[test]
fn sutra_1_1_64() {
    assert_has_upapada_krdanta("agni", &[], &d("ci\\Y", Svadi), Krt::kvip, &["agnicit"]);
    assert_has_upapada_krdanta("soma", &[], &d("zu\\Y", Svadi), Krt::kvip, &["somasut"]);

    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_aataam(&[], &pac, Lat, &["pacete"]);
    assert_has_aathaam(&[], &pac, Lat, &["paceTe"]);
}

#[test]
fn skip_sutra_1_1_65() {}

#[test]
fn sutra_1_1_66() {
    assert_has_sandhi("daDi", "udakam", &["daDyudakam"]);
    assert_has_sandhi("maDu", "idam", &["maDvidam"]);
    assert_has_sandhi("pacati", "odanam", &["pacatyodanam"]);
    assert_has_sandhi("agnicit", "atra", &["agnicid atra"]);
}

#[test]
fn sutra_1_1_70() {
    assert_has_sup_3p("vfkza", Pum, &["vfkzEH"]);
    assert_has_sup_3p("plakza", Pum, &["plakzEH"]);

    let jan = d("janI~\\", Divadi);
    let abja = create_upapada_krdanta("abjA", "ap", &[], &jan, Krt::viw);
    let goja = create_upapada_krdanta("gojA", "go", &[], &jan, Krt::viw);
    assert_has_sup_1s(&abja, Pum, &["abjAH"]);
    assert_has_sup_1s(&goja, Pum, &["gojAH"]);

    // tatkAlasya?
    assert_has_sup_3p("KawvA", Stri, &["KawvABiH"]);
    assert_has_sup_3p("mAlA", Stri, &["mAlABiH"]);
}

#[test]
fn skip_sutra_1_1_71() {}

#[test]
fn sutra_1_1_73() {
    assert_has_taddhita(&nyap("SAlA"), T::Ca, &["SAlIya"]);
    assert_has_taddhita(&nyap("mAlA"), T::Ca, &["mAlIya"]);
    assert_has_taddhita("Opagava", T::Ca, &["OpagavIya"]);
    assert_has_taddhita("kApawava", T::Ca, &["kApawavIya"]);
}

#[test]
fn sutra_1_1_74() {
    assert_has_taddhita("tyad", T::Ca, &["tyadIya"]);
    assert_has_taddhita("tad", T::Ca, &["tadIya"]);
    assert_has_taddhita("etad", T::Ca, &["etadIya"]);
    assert_has_taddhita("idam", T::Ca, &["idamIya"]);
    assert_has_taddhita("adas", T::Ca, &["adasIya"]);
    // TODO: enable these.
    // assert_has_taddhitanta("yuzmad", T::Ca, &["tvadIya"]);
    // assert_has_taddhitanta("yuzmad", T::PiY, &["tvAdAyani"]);
    // assert_has_taddhitanta("asmad", T::Ca, &["madIya"]);
    // assert_has_taddhitanta("asmad", T::PiY, &["mAdAyani"]);
    // assert_has_taddhitanta("Bavatu~", T::Ca, &["BavadIya"]);
    assert_has_taddhita("kim", T::Ca, &["kimIya"]);
}
