extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha as P;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_1_1_1() {
    assert_has_taddhitanta(&prati("aSvala"), T::Pak, &["ASvalAyana"]);
    assert_has_taddhitanta(&prati("itika"), T::Pak, &["EtikAyana"]);
    assert_has_taddhitanta(&prati("upagu"), T::aR, &["Opagava"]);
    assert_has_taddhitanta(&prati("upamanyu"), T::aR, &["Opamanyava"]);
    assert_has_taddhitanta(&prati("SAlA"), T::Ca, &["SAlIya"]);
    assert_has_taddhitanta(&prati("mAlA"), T::Ca, &["mAlIya"]);
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
    assert_has_lat_p(&[], &d("tF", Bhvadi), &["tarati"]);
    assert_has_lat_p(&[], &d("RI\\Y", Bhvadi), &["nayati"]);
    assert_has_lat_p(&[], &d("BU", Bhvadi), &["Bavati"]);
    // vfdDi
    assert_has_lun_p(&[], &d("qukf\\Y", Tanadi), &["akArzIt"]);
    assert_has_lun_p(&[], &d("hf\\Y", Bhvadi), &["ahArzIt"]);
    assert_has_lun_p(&[], &d("ci\\Y", Svadi), &["acEzIt"]);
    assert_has_lun_p(&[], &d("RI\\Y", Bhvadi), &["anEzIt"]);
    assert_has_lun_p(&[], &d("lUY", Kryadi), &["alAvIt"]);
    assert_has_lun_p(&[], &d("zwu\\Y", Adadi), &["astAvIt"]);

    // Other examples
    assert_has_lat_p(&[], &d("YimidA~", Divadi), &["medyati"]);
    assert_has_jhi(&[], &d("YiBI\\", Juhotyadi), Lan, &["abiBayuH"]);

    // ikaH
    assert_has_krdanta(&[], &d("yA\\", Adadi), Krt::lyuw, &["yAna"]);
    assert_has_lat_p(&[], &d("glE\\", Bhvadi), &["glAyati"]);
    assert_has_krdanta(&[], &d("unBa~", Tudadi), Krt::tfc, &["umBitf"]);

    // punargrahaNam of the terms "guna" and "vrddhi"
    assert_has_subantas("div", Stri, Prathama, Eka, &["dyOH"]);
    assert_has_subantas("paTin", Pum, Prathama, Eka, &["panTAH"]);
    assert_has_subantas("tad", Pum, Prathama, Eka, &["saH"]);
    assert_has_subantas("idam", Pum, Dvitiya, Eka, &["imam"]);
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
    assert_has_lun_karmani(&[], &d("Ba\\njo~", Rudhadi), &["aBAji", "aBaYji"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Divadi), Krt::GaY, &["rAga", "raNga"]);

    // bahuvrIhi?
    assert_has_lat_p(&[], &nic(&d("knUyI~\\", Bhvadi)), &["knopayati"]);
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
    assert_has_tinanta(&[], &mfj, Lat, P::Prathama, Dvi, &["mfzwaH"]);
    assert_has_tinanta(&[], &mfj, Lat, P::Prathama, Bahu, &["mfjanti"]);

    // git
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::ksnu, &["jizRu"]);
    assert_has_krdanta(&[], &d("BU", Bhvadi), Krt::ksnu, &["BUzRu"]);

    // ikaH
    assert_has_lat(&[], &d("kamu~\\", Bhvadi), &["kAmayate"]);
    assert_has_taddhitanta(&prati("ligu"), T::Pak, &["lEgavAyana"]);

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

#[test]
fn sutra_1_1_20() {
    assert_has_lat_p(&["pra", "ni"], &d("qudA\\Y", Juhotyadi), &["praRidadAti"]);
    assert_has_krdanta(
        &["pra", "ni"],
        &d("dA\\R", Bhvadi),
        Krt::tfc,
        &["praRidAtf"],
    );
    assert_has_lat(&["pra", "ni"], &d("do\\", Divadi), &["praRidyati"]);
    assert_has_lat(&["pra", "ni"], &d("de\\N", Bhvadi), &["praRidayate"]);
    assert_has_lat_p(&["pra", "ni"], &d("quDA\\Y", Juhotyadi), &["praRidaDAti"]);
    assert_has_lat_p(&["pra", "ni"], &d("De\\w", Bhvadi), &["praRiDayati"]);
    // adAp
    assert_has_krdanta(&[], &d("dA\\p", Adadi), Krt::kta, &["dAta"]);
    assert_has_krdanta(&["ava"], &d("dE\\p", Bhvadi), Krt::kta, &["avadAta"]);
}

#[test]
fn sutra_1_1_22() {
    assert_has_taddhitanta(&prati("kumArI"), T::tarap, &["kumAritara"]);
    assert_has_taddhitanta(&prati("kumArI"), T::tamap, &["kumAritama"]);
    assert_has_taddhitanta(&prati("brAhmaRI"), T::tarap, &["brAhmaRitara"]);
    assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRitama"]);
}

#[test]
fn sutra_1_1_23() {
    // TODO: others
    let bahu = prati("bahu");
    assert_has_taddhitanta(&bahu, T::kftvasuc, &["bahukftvas"]);
    assert_has_taddhitanta(&bahu, T::DA, &["bahuDA"]);
    assert_has_taddhitanta(&bahu, T::kan, &["bahuka"]);
    // TODO: assert_has_taddhitanta(&bahu, T::Sas, &["bahuSas"]);

    let gana = prati("gaRa");
    assert_has_taddhitanta(&gana, T::kftvasuc, &["gaRakftvas"]);
    assert_has_taddhitanta(&gana, T::DA, &["gaRaDA"]);
    assert_has_taddhitanta(&gana, T::kan, &["gaRaka"]);
    // TODO: assert_has_taddhitanta(&gana, T::Sas, &["gaRaSas"]);

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
    assert_has_subantas("zaz", Napumsaka, Prathama, Bahu, &["zaw"]);
    assert_has_subantas("paYcan", Napumsaka, Prathama, Bahu, &["paYca"]);
    assert_has_subantas("saptan", Napumsaka, Prathama, Bahu, &["sapta"]);
    assert_has_subantas("navan", Napumsaka, Prathama, Bahu, &["nava"]);
    assert_has_subantas("daSan", Napumsaka, Prathama, Bahu, &["daSa"]);
    // others
    assert_has_subantas("Sata", Napumsaka, Prathama, Bahu, &["SatAni"]);
    assert_has_subantas("sahasra", Napumsaka, Prathama, Bahu, &["sahasrARi"]);
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
    assert_has_subantas("sarva", Pum, Prathama, Eka, &["sarvaH"]);
    assert_has_subantas("sarva", Pum, Prathama, Dvi, &["sarvO"]);
    assert_has_subantas("sarva", Pum, Prathama, Bahu, &["sarve"]);
    assert_has_subantas("sarva", Pum, Caturthi, Eka, &["sarvasmE"]);
    assert_has_subantas("sarva", Pum, Panchami, Eka, &["sarvasmAt"]);
    assert_has_subantas("sarva", Pum, Sasthi, Bahu, &["sarvezAm"]);
    assert_has_subantas("sarva", Pum, Saptami, Eka, &["sarvasmin"]);
    // assert_has_taddhitanta(&prati("sarva"), T::akac, &["sarvaka"]);

    assert_has_subantas("viSva", Pum, Prathama, Eka, &["viSvaH"]);
    assert_has_subantas("viSva", Pum, Prathama, Dvi, &["viSvO"]);
    assert_has_subantas("viSva", Pum, Prathama, Bahu, &["viSve"]);
    assert_has_subantas("viSva", Pum, Caturthi, Eka, &["viSvasmE"]);
    assert_has_subantas("viSva", Pum, Panchami, Eka, &["viSvasmAt"]);
    assert_has_subantas("viSva", Pum, Sasthi, Bahu, &["viSvezAm"]);
    assert_has_subantas("viSva", Pum, Saptami, Eka, &["viSvasmin"]);
    // assert_has_taddhitanta(&prati("viSva"), T::akac, &["viSvaka"]);

    assert_has_subantas("uBaya", Pum, Prathama, Bahu, &["uBaye"]);
    assert_has_subantas("uBaya", Pum, Caturthi, Eka, &["uBayasmE"]);
    assert_has_subantas("uBaya", Pum, Panchami, Eka, &["uBayasmAt"]);
    assert_has_subantas("uBaya", Pum, Sasthi, Bahu, &["uBayezAm"]);
    assert_has_subantas("uBaya", Pum, Saptami, Eka, &["uBayasmin"]);
    // qatara
    assert_has_subantas("katara", Pum, Caturthi, Eka, &["katarasmE"]);
    assert_has_subantas("katama", Pum, Caturthi, Eka, &["katamasmE"]);
    // itara, etc.
    assert_has_subantas("itara", Pum, Caturthi, Eka, &["itarasmE"]);
    assert_has_subantas("anya", Pum, Caturthi, Eka, &["anyasmE"]);
    assert_has_subantas("anyatara", Pum, Caturthi, Eka, &["anyatarasmE"]);

    // TODO: others
}

#[test]
fn sutra_1_1_33() {
    assert_has_subantas("praTama", Pum, Prathama, Bahu, &["praTame", "praTamAH"]);
    assert_has_subantas("carama", Pum, Prathama, Bahu, &["carame", "caramAH"]);
    assert_has_subantas("alpa", Pum, Prathama, Bahu, &["alpe", "alpAH"]);
    assert_has_subantas("arDa", Pum, Prathama, Bahu, &["arDe", "arDAH"]);
    assert_has_subantas("katipaya", Pum, Prathama, Bahu, &["katipaye", "katipayAH"]);
    assert_has_subantas("nema", Pum, Prathama, Bahu, &["neme", "nemAH"]);

    // TODO: taya
}

#[test]
fn sutra_1_1_34() {
    assert_has_subantas("pUrva", Pum, Prathama, Bahu, &["pUrve", "pUrvAH"]);
    assert_has_subantas("para", Pum, Prathama, Bahu, &["pare", "parAH"]);
    assert_has_subantas("avara", Pum, Prathama, Bahu, &["avare", "avarAH"]);
    assert_has_subantas("dakziRa", Pum, Prathama, Bahu, &["dakziRe", "dakziRAH"]);
    assert_has_subantas("uttara", Pum, Prathama, Bahu, &["uttare", "uttarAH"]);
    assert_has_subantas("apara", Pum, Prathama, Bahu, &["apare", "aparAH"]);
    assert_has_subantas("aDara", Pum, Prathama, Bahu, &["aDare", "aDarAH"]);
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
fn sutra_1_1_42() {
    assert_has_subantas("kuRqa", Napumsaka, Prathama, Bahu, &["kuRqAni"]);
    assert_has_subantas("kuRqa", Napumsaka, Dvitiya, Bahu, &["kuRqAni"]);
    assert_has_subantas("daDi", Napumsaka, Prathama, Bahu, &["daDIni"]);
    assert_has_subantas("maDu", Napumsaka, Prathama, Bahu, &["maDUni"]);
    assert_has_subantas("trapu", Napumsaka, Prathama, Bahu, &["trapURi"]);
    assert_has_subantas("jatu", Napumsaka, Prathama, Bahu, &["jatUni"]);
}

#[test]
fn sutra_1_1_43() {
    assert_has_subantas("rAjan", Pum, Prathama, Eka, &["rAjA"]);
    assert_has_subantas("rAjan", Pum, Prathama, Dvi, &["rAjAnO"]);
    assert_has_subantas("rAjan", Pum, Prathama, Bahu, &["rAjAnaH"]);
    assert_has_subantas("rAjan", Pum, Dvitiya, Eka, &["rAjAnam"]);
    assert_has_subantas("rAjan", Pum, Dvitiya, Dvi, &["rAjAnO"]);
    // suw
    assert_has_subantas("rAjan", Pum, Dvitiya, Bahu, &["rAjYaH"]);
    // anapuMsakasya
    assert_has_subantas("sAman", Napumsaka, Prathama, Dvi, &["sAmanI", "sAmnI"]);
    assert_has_subantas("veman", Napumsaka, Prathama, Dvi, &["vemanI", "vemnI"]);
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
    assert_has_lat_p(&["vi"], &d("ru\\Di~^r", Rudhadi), &["viruRadDi"]);
    assert_has_lat_p(&[], &d("mu\\cx~^", Tudadi), &["muYcati"]);
    assert_has_subantas("payas", Napumsaka, Prathama, Bahu, &["payAMsi"]);
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
    use Vibhakti::*;
    assert_has_subantas("atirE", Napumsaka, Prathama, Eka, &["atiri"]);
    assert_has_subantas("atinO", Napumsaka, Prathama, Eka, &["atinu"]);
    assert_has_subantas("upago", Napumsaka, Prathama, Eka, &["upagu"]);
    // ecaH?
    // TODO: assert_has_subantas("atiKawvA", Pum, Prathama, Eka, &["atiKawvaH"]);
    // TODO: assert_has_subantas("atimAlA", Pum, Prathama, Eka, &["atimAlaH"]);
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
    assert_has_subantas("kuRqa", Napumsaka, Prathama, Bahu, &["kuRqAni"]);
    assert_has_subantas("kuRqa", Napumsaka, Dvitiya, Bahu, &["kuRqAni"]);
}

#[test]
fn sutra_1_1_56() {
    assert_has_krdanta(&[], &d("asa~", Adadi), Krt::tfc, &["Bavitf"]);
    assert_has_krdanta(&[], &d("asa~", Adadi), Krt::tumun, &["Bavitum"]);
    assert_has_krdanta(&[], &d("asa~", Adadi), Krt::tavya, &["Bavitavya"]);

    assert_has_krdanta(&[], &d("brUY", Adadi), Krt::tfc, &["vaktf"]);
    assert_has_krdanta(&[], &d("brUY", Adadi), Krt::tumun, &["vaktum"]);
    assert_has_krdanta(&[], &d("brUY", Adadi), Krt::tavya, &["vaktavya"]);

    assert_has_subantas("kim", Pum, Trtiya, Eka, &["kena"]);
    assert_has_subantas("kim", Pum, Trtiya, Dvi, &["kAByAm"]);
    assert_has_subantas("kim", Pum, Trtiya, Bahu, &["kEH"]);

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

    assert_has_subantas("vfkza", Pum, Caturthi, Eka, &["vfkzAya"]);
    assert_has_subantas("plakza", Pum, Caturthi, Eka, &["plakzAya"]);

    assert_has_tas(&[], &kf, Lan, &["akurutAm"]);
    assert_has_thas(&[], &kf, Lan, &["akurutam"]);

    // TODO: others
}

#[test]
fn sutra_1_1_59() {
    let pai = d("pE\\", Bhvadi);
    assert_has_tinanta(&[], &pai, Lit, P::Prathama, Dvi, &["papatuH"]);
    assert_has_tinanta(&[], &pai, Lit, P::Prathama, Bahu, &["papuH"]);

    let han = d("ha\\na~", Adadi);
    assert_has_tinanta(&[], &han, Lit, P::Prathama, Dvi, &["jaGnatuH"]);
    assert_has_tinanta(&[], &han, Lit, P::Prathama, Bahu, &["jaGnuH"]);

    let aw = d("awa~", Bhvadi);
    assert_has_lun_p(&[], &nic(&aw), &["Awiwat"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_tas(&[], &kf, Lit, &["cakratuH"]);
    assert_has_jhi(&[], &kf, Lit, &["cakruH"]);

    assert_has_mip(&[], &d("RI\\Y", Bhvadi), Lit, &["ninaya", "ninAya"]);
    assert_has_mip(&[], &d("lUY", Kryadi), Lit, &["lulava", "lulAva"]);

    // dvirvacane
    assert_has_lit_karmani(&[], &d("glE\\", Bhvadi), &["jagle"]);
    assert_has_lit_karmani(&[], &d("mlE\\", Bhvadi), &["mamle"]);

    // dvirvacananimitte
    assert_has_lat(&[], &san(&d("divu~", Divadi)), &["didevizati", "dudyUzati"]);

    // aci
    assert_has_lat(&[], &yan(&d("GrA\\", Bhvadi)), &["jeGrIyate"]);
    assert_has_lat(&[], &yan(&d("DmA\\", Bhvadi)), &["deDmIyate"]);
}

#[test]
fn sutra_1_1_61() {
    assert_has_lat_p(&[], &d("a\\da~", Adadi), &["atti"]);
    assert_has_lat_p(&[], &d("hu\\", Juhotyadi), &["juhoti"]);
    // TODO: lup
}

#[test]
fn sutra_1_1_73() {
    assert_has_taddhitanta(&nyap("SAlA"), T::Ca, &["SAlIya"]);
    assert_has_taddhitanta(&nyap("mAlA"), T::Ca, &["mAlIya"]);
    assert_has_taddhitanta(&prati("Opagava"), T::Ca, &["OpagavIya"]);
    assert_has_taddhitanta(&prati("kApawava"), T::Ca, &["kApawavIya"]);
    // TODO: do others
}

#[test]
fn sutra_1_1_74() {
    assert_has_taddhitanta(&prati("tyad"), T::Ca, &["tyadIya"]);
    assert_has_taddhitanta(&prati("tad"), T::Ca, &["tadIya"]);
    assert_has_taddhitanta(&prati("etad"), T::Ca, &["etadIya"]);
    assert_has_taddhitanta(&prati("idam"), T::Ca, &["idamIya"]);
    assert_has_taddhitanta(&prati("adas"), T::Ca, &["adasIya"]);
    // TODO: enable these.
    // assert_has_taddhitanta(&prati("yuzmad"), T::Ca, &["tvadIya"]);
    // assert_has_taddhitanta(&prati("yuzmad"), T::PiY, &["tvAdAyani"]);
    // assert_has_taddhitanta(&prati("asmad"), T::Ca, &["madIya"]);
    // assert_has_taddhitanta(&prati("asmad"), T::PiY, &["mAdAyani"]);
    // assert_has_taddhitanta(&prati("Bavatu~"), T::Ca, &["BavadIya"]);
    assert_has_taddhitanta(&prati("kim"), T::Ca, &["kimIya"]);
}
