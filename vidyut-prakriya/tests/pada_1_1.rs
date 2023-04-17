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

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic])
}

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan])
}

fn prati(text: &str) -> Pratipadika {
    Pratipadika::new(text)
}

fn stri(text: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(text)
        .is_nyap(true)
        .build()
        .unwrap()
}

#[test]
fn sutra_1_1_1() {
    // TODO: others
    assert_has_taddhitanta(&prati("upagu"), T::aR, &["Opagava"]);
}

#[test]
fn sutra_1_1_2() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::tfc, &["taritf", "tarItf"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::tfc, &["stotf"]);
    assert_has_parasmai_tinanta(&[], &pac, Lat, P::Prathama, Bahu, &["pacanti"]);
    assert_has_tinanta(
        &[],
        &d("ji\\", Bhvadi),
        Lat,
        P::Prathama,
        Bahu,
        &["jayanti"],
    );
    assert_has_atmane_tinanta(&[], &pac, Lat, P::Uttama, Eka, &["pace"]);
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
    assert_has_parasmai_tinanta(
        &[],
        &d("YiBI\\", Juhotyadi),
        Lan,
        P::Prathama,
        Bahu,
        &["abiBayuH"],
    );

    // ikaH
    assert_has_krdanta(&[], &d("yA\\", Adadi), Krt::lyuw, &["yAna"]);
    assert_has_lat_p(&[], &d("glE\\", Bhvadi), &["glAyati"]);
    assert_has_krdanta(&[], &d("unBa~", Tudadi), Krt::tfc, &["umBitf"]);

    // TODO: others
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
    assert_has_parasmai_tinanta(&[], &ci, Lat, P::Prathama, Dvi, &["cinutaH"]);
    assert_has_parasmai_tinanta(&[], &ci, Lat, P::Prathama, Bahu, &["cinvanti"]);
    assert_has_tinanta(&[], &mfj, Lat, P::Prathama, Dvi, &["mfzwaH"]);
    assert_has_tinanta(&[], &mfj, Lat, P::Prathama, Bahu, &["mfjanti"]);

    // git
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::ksnu, &["jizRu"]);
    assert_has_krdanta(&[], &d("BU", Bhvadi), Krt::ksnu, &["BUzRu"]);

    // ikaH
    assert_has_lat(&[], &d("kamu~\\", Bhvadi), &["kAmayate"]);
    assert_has_taddhitanta(&prati("ligu"), T::Pak, &["lEgavAyana"]);

    // lakAra-Nit
    assert_has_parasmai_tinanta(&[], &ci, Lan, P::Uttama, Eka, &["acinavam"]);
    assert_has_parasmai_tinanta(&[], &d("zu\\Y", Svadi), Lan, P::Uttama, Eka, &["asunavam"]);
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

    assert_has_parasmai_tinanta(&[], &kf, Lan, P::Prathama, Dvi, &["akurutAm"]);
    assert_has_parasmai_tinanta(&[], &kf, Lan, P::Madhyama, Dvi, &["akurutam"]);

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
    assert_has_parasmai_tinanta(&[], &kf, Lit, P::Prathama, Dvi, &["cakratuH"]);
    assert_has_parasmai_tinanta(&[], &kf, Lit, P::Prathama, Bahu, &["cakruH"]);

    assert_has_parasmai_tinanta(
        &[],
        &d("RI\\Y", Bhvadi),
        Lit,
        P::Uttama,
        Eka,
        &["ninaya", "ninAya"],
    );
    assert_has_parasmai_tinanta(
        &[],
        &d("lUY", Kryadi),
        Lit,
        P::Uttama,
        Eka,
        &["lulava", "lulAva"],
    );

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
fn sutra_1_1_73() {
    assert_has_taddhitanta(&stri("SAlA"), T::Ca, &["SAlIya"]);
    assert_has_taddhitanta(&stri("mAlA"), T::Ca, &["mAlIya"]);
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
