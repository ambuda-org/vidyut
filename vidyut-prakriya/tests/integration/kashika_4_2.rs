extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha;
use vidyut_prakriya::args::TaddhitaArtha::*;

fn assert_blocked(text: &str, artha: TaddhitaArtha, t: T) {
    assert_has_artha_taddhita(text, artha, t, &[]);
}

#[test]
fn sutra_4_2_1() {
    assert_has_artha_taddhita("kazAya", TenaRaktam, T::aR, &["kAzAya"]);
    assert_has_artha_taddhita("maYjizWa", TenaRaktam, T::aR, &["mAYjizWa"]);
    assert_has_artha_taddhita("kusumBa", TenaRaktam, T::aR, &["kOsumBa"]);
}

#[test]
fn sutra_4_2_2() {
    assert_has_artha_taddhita("lAkzA", TenaRaktam, T::Wak, &["lAkzika"]);
    assert_has_artha_taddhita("rocanA", TenaRaktam, T::Wak, &["rOcanika"]);
    assert_has_artha_taddhita("Sakala", TenaRaktam, T::Wak, &["SAkalika"]);
    assert_has_artha_taddhita("kardamA", TenaRaktam, T::Wak, &["kArdamika"]);
}

#[test]
fn sutra_4_2_16() {
    assert_has_artha_taddhita("BrAzwa", SamskrtamBhaksha, T::aR, &["BrAzwa"]);
    assert_has_artha_taddhita("kalaSa", SamskrtamBhaksha, T::aR, &["kAlaSa"]);
    assert_has_artha_taddhita("kumBa", SamskrtamBhaksha, T::aR, &["kOmBa"]);
}

#[test]
fn sutra_4_2_17() {
    assert_has_artha_taddhita("SUla", SamskrtamBhaksha, T::yat, &["SUlya"]);
    assert_has_artha_taddhita("uKA", SamskrtamBhaksha, T::yat, &["uKya"]);
}

#[test]
fn sutra_4_2_18() {
    assert_has_artha_taddhita("daDi", SamskrtamBhaksha, T::Wak, &["dADika"]);
}

#[test]
fn sutra_4_2_19() {
    assert_has_artha_taddhita("udaSvit", SamskrtamBhaksha, T::Wak, &["OdaSvitka"]);
    assert_has_artha_taddhita("udaSvit", SamskrtamBhaksha, T::aR, &["OdaSvita"]);
}

#[test]
fn sutra_4_2_20() {
    assert_has_artha_taddhita("kzIra", SamskrtamBhaksha, T::QaY, &["kzEreya"]);
}

#[test]
fn sutra_4_2_24() {
    assert_has_artha_taddhita("indra", SaAsyaDevata, T::aR, &["Endra"]);
    assert_has_artha_taddhita("aditi", SaAsyaDevata, T::Rya, &["Aditya"]);
    assert_has_artha_taddhita("bfhaspati", SaAsyaDevata, T::Rya, &["bArhaspatya"]);
    assert_has_artha_taddhita("prajApati", SaAsyaDevata, T::Rya, &["prAjApatya"]);
}

#[test]
fn sutra_4_2_25() {
    assert_has_artha_taddhita("ka", SaAsyaDevata, T::aR, &["kAya"]);
}

#[test]
fn sutra_4_2_26() {
    assert_has_artha_taddhita("Sukra", SaAsyaDevata, T::Gan, &["Sukriya"]);
}

#[test]
fn sutra_4_2_27() {
    assert_has_artha_taddhita("aponaptAt", SaAsyaDevata, T::Ga, &["aponaptriya"]);
    assert_has_artha_taddhita("apAnnaptAt", SaAsyaDevata, T::Ga, &["apAnnaptriya"]);
}

#[test]
fn sutra_4_2_28() {
    assert_has_artha_taddhita("aponaptAt", SaAsyaDevata, T::Ca, &["aponaptrIya"]);
    assert_has_artha_taddhita("apAnnaptAt", SaAsyaDevata, T::Ca, &["apAnnaptrIya"]);
}

#[test]
fn sutra_4_2_28_v2() {
    assert_has_artha_taddhita("Satarudra", SaAsyaDevata, T::Ga, &["Satarudriya"]);
    assert_has_artha_taddhita("Satarudra", SaAsyaDevata, T::Ca, &["SatarudrIya"]);
}

#[test]
fn sutra_4_2_29() {
    assert_has_artha_taddhita("mahendra", SaAsyaDevata, T::Ga, &["mahendriya"]);
    assert_has_artha_taddhita("mahendra", SaAsyaDevata, T::aR, &["mAhendra"]);
    assert_has_artha_taddhita("mahendra", SaAsyaDevata, T::Ca, &["mahendrIya"]);
}

#[test]
fn sutra_4_2_30() {
    assert_has_artha_taddhita("soma", SaAsyaDevata, T::wyaR, &["sOmya"]);
}

#[test]
fn sutra_4_2_31() {
    assert_has_artha_taddhita("vAyu", SaAsyaDevata, T::yat, &["vAyavya"]);
    assert_has_artha_taddhita("ftu", SaAsyaDevata, T::yat, &["ftavya"]);
    assert_has_artha_taddhita("pitf", SaAsyaDevata, T::yat, &["pitrya"]);
    assert_has_artha_taddhita("uzas", SaAsyaDevata, T::yat, &["uzasya"]);
}

#[test]
fn sutra_4_2_32() {
    fn assert_has_yat_and_cha(prati: &str, cha: &str, yat: &str) {
        assert_has_artha_taddhita(prati, SaAsyaDevata, T::Ca, &[cha]);
        assert_has_artha_taddhita(prati, SaAsyaDevata, T::yat, &[yat]);
        assert_blocked(prati, SaAsyaDevata, T::aR);
        assert_blocked(prati, SaAsyaDevata, T::Rya);
    }
    assert_has_yat_and_cha("dyAvApfTivI", "dyAvApfTivIya", "dyAvApfTivya");
    assert_has_yat_and_cha("SunAsIra", "SunAsIrIya", "SunAsIrya");
    assert_has_yat_and_cha("marutvat", "marutvatIya", "marutvatya");
    assert_has_yat_and_cha("agnIzoma", "agnIzomIya", "agnIzomya");
    assert_has_yat_and_cha("vAstozpati", "vAstozpatIya", "vAstozpatya");
    assert_has_yat_and_cha("gfhameDa", "gfhameDIya", "gfhameDya");
}

#[test]
fn sutra_4_2_33() {
    assert_has_artha_taddhita("agni", SaAsyaDevata, T::Qak, &["Agneya"]);
}

#[test]
fn sutra_4_2_35() {
    assert_has_artha_taddhita("mahArAja", SaAsyaDevata, T::WaY, &["mAhArAjika"]);
    assert_has_artha_taddhita("prozWapada", SaAsyaDevata, T::WaY, &["prOzWapadika"]);
}

#[test]
fn sutra_4_2_37() {
    assert_has_artha_taddhita("kAka", TasyaSamuha, T::aR, &["kAka"]);
    assert_has_artha_taddhita("baka", TasyaSamuha, T::aR, &["bAka"]);
}

#[test]
fn sutra_4_2_38() {
    assert_has_artha_taddhita("BikzA", TasyaSamuha, T::aR, &["BEkza"]);
    assert_has_artha_taddhita("garBiRI", TasyaSamuha, T::aR, &["gArBiRa"]);
    assert_has_artha_taddhita("yuvati", TasyaSamuha, T::aR, &["yOvata"]);
}

#[test]
fn sutra_4_2_40() {
    assert_has_artha_taddhita("kedAra", TasyaSamuha, T::vuY, &["kEdAraka"]);
    assert_has_artha_taddhita("kedAra", TasyaSamuha, T::yaY, &["kEdArya"]);
}

#[test]
fn sutra_4_2_40_v1() {
    assert_has_artha_taddhita("gaRikA", TasyaSamuha, T::yaY, &["gARikya"]);
}

#[test]
fn sutra_4_2_41() {
    assert_has_artha_taddhita("kavacin", TasyaSamuha, T::WaY, &["kAvacika"]);
    assert_has_artha_taddhita("kedAra", TasyaSamuha, T::WaY, &["kEdArika"]);
}

#[test]
fn sutra_4_2_42() {
    assert_has_artha_taddhita("brAhmaRa", TasyaSamuha, T::yan, &["brAhmaRya"]);
    assert_has_artha_taddhita("mARava", TasyaSamuha, T::yan, &["mARavya"]);
    assert_has_artha_taddhita("vAqava", TasyaSamuha, T::yan, &["vAqavya"]);
}

#[test]
fn sutra_4_2_42_v1() {
    assert_has_artha_taddhita("pfzWa", TasyaSamuha, T::yan, &["pfzWya"]);
}

#[test]
fn sutra_4_2_43() {
    assert_has_artha_taddhita("grAma", TasyaSamuha, T::tal, &["grAmatA"]);
    assert_has_artha_taddhita("jana", TasyaSamuha, T::tal, &["janatA"]);
    assert_has_artha_taddhita("banDu", TasyaSamuha, T::tal, &["banDutA"]);
    assert_has_artha_taddhita("sahAya", TasyaSamuha, T::tal, &["sahAyatA"]);
}

#[test]
fn sutra_4_2_43_v1() {
    assert_has_artha_taddhita("gaja", TasyaSamuha, T::tal, &["gajatA"]);
}

#[test]
fn sutra_4_2_45() {
    assert_has_artha_taddhita("KaRqikA", TasyaSamuha, T::aY, &["KARqika"]);
    assert_has_artha_taddhita("vaqavA", TasyaSamuha, T::aY, &["vAqava"]);
}

#[test]
fn sutra_4_2_48() {
    assert_has_artha_taddhita("keSa", TasyaSamuha, T::yaY, &["kESya"]);
    assert_has_artha_taddhita("keSa", TasyaSamuha, T::Wak, &["kESika"]);
    assert_has_artha_taddhita("aSva", TasyaSamuha, T::Ca, &["aSvIya"]);
    assert_has_artha_taddhita("aSva", TasyaSamuha, T::aR, &["ASva"]);
}

#[test]
fn sutra_4_2_49() {
    assert_has_artha_taddhita("pASa", TasyaSamuha, T::ya, &["pASyA"]);
    assert_has_artha_taddhita("tfRa", TasyaSamuha, T::ya, &["tfRyA"]);
}

#[test]
fn sutra_4_2_50() {
    assert_has_artha_taddhita("Kala", TasyaSamuha, T::ya, &["KalyA"]);
    assert_has_artha_taddhita("go", TasyaSamuha, T::ya, &["gavyA"]);
    assert_has_artha_taddhita("raTa", TasyaSamuha, T::ya, &["raTyA"]);
}

#[test]
fn sutra_4_2_52() {
    let artha = TasyaVishayoDeshe;
    assert_has_artha_taddhita("Sibi", artha, T::aR, &["SEba"]);
    assert_has_artha_taddhita("uzwra", artha, T::aR, &["Ozwra"]);
}

#[test]
fn sutra_4_2_53() {
    let artha = TasyaVishayoDeshe;
    assert_has_artha_taddhita("rAjanya", artha, T::vuY, &["rAjanyaka"]);
    assert_has_artha_taddhita("devayAna", artha, T::vuY, &["dEvayAnaka"]);
    assert_has_artha_taddhita("mAlava", artha, T::vuY, &["mAlavaka"]);
    assert_has_artha_taddhita("virAwa", artha, T::vuY, &["vErAwaka"]);
    assert_has_artha_taddhita("trigarta", artha, T::vuY, &["trEgartaka"]);
    assert_blocked("rAjanya", artha, T::aR);
}

#[test]
fn sutra_4_2_54() {
    let artha = TasyaVishayoDeshe;
    assert_has_artha_taddhita("BOriki", artha, T::viDal, &["BOrikiviDa"]);
    assert_has_artha_taddhita("vEpeya", artha, T::viDal, &["vEpeyaviDa"]);
    assert_blocked("BOriki", artha, T::aR);
    assert_has_artha_taddhita("EzukAri", artha, T::Baktal, &["EzukAriBakta"]);
    assert_has_artha_taddhita("sArasyAyana", artha, T::Baktal, &["sArasyAyanaBakta"]);
    assert_blocked("BOriki", artha, T::aR);
    assert_blocked("EzukAri", artha, T::aR);
}

#[test]
fn sutra_4_2_59() {
    let artha = TadAdhiteTadVeda;
    assert_has_artha_taddhita("Candas", artha, T::aR, &["CAndasa"]);
    assert_has_artha_taddhita("vyAkaraRa", artha, T::aR, &["vEyAkaraRa"]);
    assert_has_artha_taddhita("nirukta", artha, T::aR, &["nErukta"]);
    assert_has_artha_taddhita("nimitta", artha, T::aR, &["nEmitta"]);
    assert_has_artha_taddhita("muhUrta", artha, T::aR, &["mOhUrta"]);
    assert_has_artha_taddhita("utpAta", artha, T::aR, &["OtpAta"]);
}

#[test]
fn sutra_4_2_61() {
    let artha = TadAdhiteTadVeda;
    assert_has_artha_taddhita("krama", artha, T::vun, &["kramaka"]);
    assert_has_artha_taddhita("pada", artha, T::vun, &["padaka"]);
    assert_blocked("krama", artha, T::aR)
}

#[test]
fn sutra_4_2_67() {
    assert_has_taddhita("udumbara", T::aR, &["Odumbara"]);
    assert_has_taddhita("balbaja", T::aR, &["bAlbaja"]);
    assert_has_taddhita("parvata", T::aR, &["pArvata"]);
}

#[test]
fn sutra_4_2_68() {
    assert_has_taddhita("kuSAmba", T::aR, &["kOSAmba"]);
    assert_has_taddhita("sahasra", T::aR, &["sAhasra"]);
}

#[test]
fn sutra_4_2_69() {
    assert_has_taddhita("fjunO", T::aR, &["ArjunAva"]);
    assert_has_taddhita("Siba", T::aR, &["SEba"]);
}

#[test]
fn sutra_4_2_70() {
    assert_has_taddhita("vidiSA", T::aR, &["vEdiSa"]);
    assert_has_taddhita("himavat", T::aR, &["hEmavata"]);
}

#[test]
fn sutra_4_2_71() {
    assert_has_taddhita("araqu", T::aY, &["Araqava"]);
    assert_has_taddhita("kakzatu", T::aY, &["kAkzatava"]);
    assert_has_taddhita("karkawelu", T::aY, &["kArkawelava"]);
}

#[test]
fn sutra_4_2_72() {
    assert_has_taddhita("izukAvat", T::aY, &["EzukAvata"]);
    assert_has_taddhita("siDrAkAvat", T::aY, &["sEDrAkAvata"]);

    // bahu-ac?
    assert_has_taddhita("ahimat", T::aR, &["Ahimata"]);
    assert_has_taddhita("yavamat", T::aR, &["yAvamata"]);
    assert_has_taddhita("mAlAvat", T::aR, &["mAlAvata"]);
}

#[test]
fn sutra_4_2_75() {
    assert_has_taddhita("saNkala", T::aY, &["sANkala"]);
    assert_has_taddhita("puzkala", T::aY, &["pOzkala"]);
}

#[test]
fn sutra_4_2_77() {
    assert_has_taddhita("suvAstu", T::aR, &["sOvAstava"]);
    assert_has_taddhita("varRu", T::aR, &["vArRava"]);
}

#[test]
fn sutra_4_2_78() {
    assert_has_taddhita("roRI", T::aR, &["rORa"]);
    assert_has_taddhita("ajakaroRI", T::aR, &["AjakaroRa"]);
    assert_has_taddhita("sihikaroRI", T::aR, &["sEhikaroRa"]);
}

#[test]
fn sutra_4_2_79() {
    assert_has_taddhita("karRacCidrika", T::aR, &["kArRacCidrika"]);
    assert_has_taddhita("karRavezwaka", T::aR, &["kArRavezwaka"]);
    assert_has_taddhita("triSaNku", T::aR, &["trESaNkava"]);
}

#[test]
fn sutra_4_2_84() {
    assert_has_taddhita("SarkarA", T::Wak, &["SArkarika"]);
    assert_has_taddhita("SarkarA", T::Ca, &["SarkarIya"]);
}

#[test]
fn sutra_4_2_86() {
    assert_has_taddhita("maDu", T::matup, &["maDumat"]);
    assert_has_taddhita("bisa", T::matup, &["bisavat"]);
}

#[ignore]
#[test]
fn sutra_4_2_87() {
    assert_has_taddhita("kumuda", T::qmatup, &["kumudvat"]);
    assert_has_taddhita("naqa", T::qmatup, &["maqvat"]);
    assert_has_taddhita("vetasa", T::qmatup, &["vetasvat"]);
}

// Seze
// ----

// TODO: cAkzuza
#[ignore]
#[test]
fn sutra_4_2_92() {
    assert_has_taddhita("rAzwra", T::Ga, &["rAzwriya"]);
    assert_has_taddhita("avArapAra", T::Ka, &["avArapArIRa"]);

    assert_has_taddhita("cakzus", T::aR, &["cAkzuza"]);
    assert_has_taddhita("SravaRa", T::aR, &["SrAvaRa"]);
    assert_has_taddhita("dfzad", T::aR, &["dArzada"]);
    assert_has_taddhita("ulUkala", T::aR, &["OlUKala"]);
    assert_has_taddhita("aSva", T::aR, &["ASva"]);
    assert_has_taddhita("catur", T::aR, &["cAtura"]);
    assert_has_taddhita("caturdaSI", T::aR, &["cAturdaSa"]);
}

#[test]
fn sutra_4_2_93() {
    assert_has_taddhita("rAzwra", T::Ga, &["rAzwriya"]);
    assert_has_taddhita("avArapAra", T::Ka, &["avArapArIRa"]);
}

#[test]
fn sutra_4_2_93_v1() {
    assert_has_taddhita("avAra", T::Ka, &["avArIRa"]);
    assert_has_taddhita("pAra", T::Ka, &["pArIRa"]);
    assert_has_taddhita("pArAvara", T::Ka, &["pArAvarIRa"]);
}

#[test]
fn sutra_4_2_94() {
    assert_has_taddhita("grAma", T::ya, &["grAmya"]);
    assert_has_taddhita("grAma", T::KaY, &["grAmIRa"]);
}

#[test]
fn sutra_4_2_95() {
    assert_has_taddhita("katri", T::QakaY, &["kAtreyaka"]);
    assert_has_taddhita("umBi", T::QakaY, &["OmBeyaka"]);
}

#[test]
fn sutra_4_2_97() {
    assert_has_taddhita("nadI", T::Qak, &["nAdeya"]);
    assert_has_taddhita("mahI", T::Qak, &["mAheya"]);
}

#[test]
fn sutra_4_2_98() {
    assert_has_taddhita("dakziRA", T::tyak, &["dAkziRAtya"]);
    assert_has_taddhita("paScAt", T::tyak, &["pAScAttya"]);
    assert_has_taddhita("puras", T::tyak, &["pOrastya"]);
}

#[test]
fn sutra_4_2_99() {
    assert_has_taddhita("kApiSI", T::zPak, &["kApiSAyana"]);
    // TODO: stri
}

#[test]
fn sutra_4_2_100() {
    assert_has_taddhita("raNku", T::zPak, &["rANkavAyaRa"]);
    assert_has_taddhita("raNku", T::aR, &["rANkava"]);
}

#[ignore]
#[test]
fn sutra_4_2_101() {
    assert_has_taddhita("dyu", T::yat, &["divya"]);
    assert_has_taddhita("prAc", T::yat, &["prAcya"]);
    assert_has_taddhita("apAc", T::yat, &["apAcya"]);
    assert_has_taddhita("udac", T::yat, &["udIcya"]);
    assert_has_taddhita("pratyac", T::yat, &["pratIcya"]);
}

#[test]
fn sutra_4_2_102() {
    assert_has_taddhita("kanTA", T::Wak, &["kAnTika"]);
}

#[test]
fn sutra_4_2_103() {
    assert_has_taddhita("kanTA", T::vuk, &["kAnTaka"]);
}

#[test]
fn sutra_4_2_105() {
    assert_has_taddhita("Ezamas", T::tyap, &["Ezamastya"]);
    assert_has_taddhita("hyas", T::tyap, &["hyastya"]);
    assert_has_taddhita("Svas", T::tyap, &["Svastya"]);
    // TODO: others
}

#[test]
fn sutra_4_2_106() {
    assert_has_artha_taddhita("kAkatIra", TatraJata, T::aY, &["kAkatIra"]);
    assert_has_artha_taddhita("palvalatIra", TatraJata, T::aY, &["pAlvalatIra"]);
    assert_has_artha_taddhita("vfkarUpya", TatraJata, T::Ya, &["vArkarUpya"]);
    assert_has_artha_taddhita("SivarUpya", TatraJata, T::Ya, &["SEvarUpya"]);
}

#[ignore]
#[test]
fn sutra_4_2_114() {
    assert_has_taddhita("gArgya", T::Ca, &["gArgIya"]);
    assert_has_taddhita("vAtsa", T::Ca, &["vAtsIya"]);
    assert_has_taddhita("SAla", T::Ca, &["SAlIya"]);
    assert_has_taddhita("mAla", T::Ca, &["mAlIya"]);
}

#[ignore]
#[test]
fn sutra_4_2_115() {
    assert_has_taddhita("Bavat", T::Wak, &["BAvatka"]);
    assert_has_taddhita("Bavat", T::Cas, &["BavadIya"]);
    assert_has_taddhita("Bavat", T::Ca, &[]);
}

#[test]
fn sutra_4_2_131() {
    assert_has_artha_taddhita("madra", TatraJata, T::kan, &["madraka"]);
    assert_has_artha_taddhita("vfji", TatraJata, T::kan, &["vfjika"]);
}

#[test]
fn sutra_4_2_132() {
    assert_has_artha_taddhita("fzika", TatraJata, T::aR, &["Arzika"]);
    assert_has_artha_taddhita("mahizika", TatraJata, T::aR, &["mAhizika"]);
}

#[test]
fn sutra_4_2_133() {
    assert_has_artha_taddhita("kacCa", TatraJata, T::aR, &["kAcCa"]);
    assert_has_artha_taddhita("sinDu", TatraJata, T::aR, &["sEnDava"]);
    assert_has_artha_taddhita("varRu", TatraJata, T::aR, &["vArRava"]);
}

#[test]
fn sutra_4_2_138() {
    assert_has_artha_taddhita("gaha", TatraJata, T::Ca, &["gahIya"]);
    assert_has_artha_taddhita("antaHsTa", TatraJata, T::Ca, &["antaHsTIya"]);
}

#[test]
fn sutra_4_2_143() {
    assert_has_artha_taddhita("parvata", TatraJata, T::Ca, &["parvatIya"]);
}
