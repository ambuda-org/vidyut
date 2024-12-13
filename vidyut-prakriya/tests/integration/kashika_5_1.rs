extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::Unadi;

fn assert_blocked(text: &str, artha: TaddhitaArtha, t: T) {
    assert_has_artha_taddhita(text, artha, t, &[]);
}

#[test]
fn sutra_5_1_1() {
    assert_has_artha_taddhita("vatsa", TasmaiHitam, T::Ca, &["vatsIya"]);
    assert_has_artha_taddhita("karaBa", TasmaiHitam, T::Ca, &["karaBIya"]);
    assert_has_artha_taddhita("akaraBa", TasmaiHitam, T::Ca, &["akaraBIya"]);
    assert_has_artha_taddhita("avatsa", TasmaiHitam, T::Ca, &["avatsIya"]);
}

#[test]
fn sutra_5_1_2() {
    assert_has_taddhita("SaNku", T::yat, &["SaNkavya"]);
    assert_has_taddhita("picU", T::yat, &["picavya"]);
    assert_has_taddhita("kamaRqalu", T::yat, &["kamaRqalavya"]);
    assert_has_taddhita("go", T::yat, &["gavya"]);
    // TODO: others
}

#[test]
fn sutra_5_1_3() {
    assert_has_taddhita("kambala", T::yat, &["kambalya"]);
    assert_has_artha_taddhita("kambala", TasmaiHitam, T::Ca, &["kambalIya"]);
}

#[test]
fn sutra_5_1_4() {
    assert_has_artha_taddhita("AmikzA", TasmaiHitam, T::yat, &["Amikzya"]);
    assert_has_artha_taddhita("AmikzA", TasmaiHitam, T::Ca, &["AmikzIya"]);

    assert_has_artha_taddhita("puroqASa", TasmaiHitam, T::yat, &["puroqASya"]);
    assert_has_artha_taddhita("puroqASa", TasmaiHitam, T::Ca, &["puroqASIya"]);

    assert_has_artha_taddhita("apUpa", TasmaiHitam, T::yat, &["apUpya"]);
    assert_has_artha_taddhita("apUpa", TasmaiHitam, T::Ca, &["apUpIya"]);

    assert_has_artha_taddhita("taRqula", TasmaiHitam, T::yat, &["taRqulya"]);
    assert_has_artha_taddhita("taRqula", TasmaiHitam, T::Ca, &["taRqulIya"]);
}

#[ignore]
#[test]
fn sutra_5_1_5() {
    let havis = krdanta(&[], &d("hu\\", Juhotyadi), Unadi::isi);
    assert_has_artha_taddhita("vatsa", TasmaiHitam, T::Ca, &["vatsIya"]);
    assert_has_artha_taddhita("avatsa", TasmaiHitam, T::Ca, &["avatsIya"]);
    assert_has_artha_taddhita("pawu", TasmaiHitam, T::yat, &["pawavya"]);
    assert_has_artha_taddhita("go", TasmaiHitam, T::yat, &["gavya"]);
    assert_has_artha_taddhita(&havis, TasmaiHitam, T::yat, &["havizya"]);
    assert_has_artha_taddhita("apUpa", TasmaiHitam, T::yat, &["apUpya"]);
    assert_has_artha_taddhita("apUpa", TasmaiHitam, T::Ca, &["apUpIya"]);
}

#[test]
fn sutra_5_1_6() {
    assert_has_artha_taddhita("danta", TasmaiHitam, T::yat, &["dantya"]);
    assert_has_artha_taddhita("kaRWa", TasmaiHitam, T::yat, &["kaRWya"]);
    assert_has_artha_taddhita("ozWa", TasmaiHitam, T::yat, &["ozWya"]);
    assert_has_artha_taddhita("nABi", TasmaiHitam, T::yat, &["nABya"]);
    assert_has_artha_taddhita("nAsikA", TasmaiHitam, T::yat, &["nasya"]);
}

#[test]
fn sutra_5_1_6_v1() {
    assert_has_artha_taddhita("nAsikA", TasmaiHitam, T::yat, &["nasya"]);
}

#[test]
fn sutra_5_1_7() {
    assert_has_artha_taddhita("Kala", TasmaiHitam, T::yat, &["Kalya"]);
    assert_has_artha_taddhita("yava", TasmaiHitam, T::yat, &["yavya"]);
    assert_has_artha_taddhita("mAza", TasmaiHitam, T::yat, &["mAzya"]);
    assert_has_artha_taddhita("tila", TasmaiHitam, T::yat, &["tilya"]);
    assert_has_artha_taddhita("vfza", TasmaiHitam, T::yat, &["vfzya"]);
    assert_has_artha_taddhita("brahman", TasmaiHitam, T::yat, &["brahmaRya"]);
    assert_blocked("Kala", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_8() {
    assert_has_artha_taddhita("aja", TasmaiHitam, T::Tyan, &["ajaTya"]);
    assert_has_artha_taddhita("avi", TasmaiHitam, T::Tyan, &["aviTya"]);
    assert_blocked("aja", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_9() {
    assert_has_artha_taddhita("Atman", TasmaiHitam, T::Ka, &["AtmanIna"]);
    assert_has_artha_taddhita("viSvajana", TasmaiHitam, T::Ka, &["viSvajanIna"]);
    assert_has_artha_taddhita("mAtfBoga", TasmaiHitam, T::Ka, &["mAtfBogIRa"]);
    assert_has_artha_taddhita("pitfBoga", TasmaiHitam, T::Ka, &["pitfBogIRa"]);
    assert_blocked("Atman", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_10() {
    assert_has_artha_taddhita("sarva", TasmaiHitam, T::Ra, &["sArva"]);
    assert_has_artha_taddhita("puruza", TasmaiHitam, T::QaY, &["pOruzeya"]);
    assert_blocked("sarva", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_11() {
    assert_has_artha_taddhita("mARava", TasmaiHitam, T::KaY, &["mARavIna"]);
    assert_has_artha_taddhita("caraka", TasmaiHitam, T::KaY, &["cArakIRa"]);
    assert_blocked("mARava", TasmaiHitam, T::Ca)
}

#[test]
fn sutra_5_1_12() {
    let artha = TadarthamVikrtehPrakrtau;
    assert_has_artha_taddhita("aNgAra", artha, T::Ca, &["aNgArIya"]);
    assert_has_artha_taddhita("prAkAra", artha, T::Ca, &["prAkArIya"]);
    assert_has_artha_taddhita("Sanku", artha, T::yat, &["SaNkavya"]);
    assert_has_artha_taddhita("picU", artha, T::yat, &["picavya"]);
}

#[ignore]
#[test]
fn sutra_5_1_13() {
    let artha = TadarthamVikrtehPrakrtau;
    let chadis = create_krdanta("Cadis", &[], &d("Cada~", Curadi), Unadi::isi);
    assert_has_artha_taddhita(&chadis, artha, T::QaY, &["CAdizeya"]);
    assert_has_artha_taddhita("upaDi", artha, T::QaY, &["OpaDeya"]);
    assert_has_artha_taddhita("bali", artha, T::QaY, &["bAleya"]);
    assert_blocked("Cadis", artha, T::Ca);
}

#[test]
fn sutra_5_1_14() {
    let artha = TadarthamVikrtehPrakrtau;
    assert_has_artha_taddhita("fzaBa", artha, T::Yya, &["ArzaBya"]);
    assert_has_artha_taddhita("upAnah", artha, T::Yya, &["OpAnahya"]);
}

#[test]
fn sutra_5_1_15() {
    let artha = TadarthamVikrtehPrakrtau;
    assert_has_artha_taddhita("varDrI", artha, T::aY, &["vArDra"]);
    assert_has_artha_taddhita("varatrA", artha, T::aY, &["vAratra"]);
    assert_blocked("varDrI", artha, T::Ca);
}

#[test]
fn sutra_5_1_16() {
    let artha = TadAsyaTadAsminSyat;
    assert_has_artha_taddhita("prAkAra", artha, T::Ca, &["prAkArIya"]);
    assert_has_artha_taddhita("prAsAda", artha, T::Ca, &["prAsAdIya"]);
}

#[test]
fn sutra_5_1_17() {
    let artha = TadAsyaTadAsminSyat;
    assert_has_artha_taddhita("pariKA", artha, T::QaY, &["pAriKeya"]);
    assert_blocked("pariKA", artha, T::Ca);
}

#[test]
fn sutra_5_1_18() {
    assert_has_artha_taddhita("pArAyaRa", TadVartayati, T::WaY, &["pArAyaRika"]);
    assert_has_artha_taddhita("turAyaRa", TadVartayati, T::WaY, &["tOrAyaRika"]);
    assert_has_artha_taddhita("cAndrAyaRa", TadVartayati, T::WaY, &["cAndrAyaRika"]);
}

#[ignore]
#[test]
fn sutra_5_1_19() {
    assert_has_artha_taddhita("nizka", TenaKritam, T::Wak, &["nEzkika"]);
    assert_has_artha_taddhita("paRa", TenaKritam, T::Wak, &["pARika"]);

    assert_has_artha_taddhita("gopucCa", TenaKritam, T::Wak, &[]);
    assert_has_artha_taddhita("gopucCa", TenaKritam, T::WaY, &["gOpucCika"]);
    assert_has_artha_taddhita("zazwi", TenaKritam, T::Wak, &[]);
    assert_has_artha_taddhita("zazwi", TenaKritam, T::WaY, &["zAzwika"]);
}

#[ignore]
#[test]
fn sutra_5_1_20() {
    assert_has_artha_taddhita("nizka", TenaKritam, T::Wak, &["nEzkika"]);
    assert_has_artha_taddhita("paRa", TenaKritam, T::Wak, &["pARika"]);
    assert_has_artha_taddhita("pAda", TenaKritam, T::Wak, &["pAdika"]);
    assert_has_artha_taddhita("mAza", TenaKritam, T::Wak, &["mAzika"]);
    // TODO: others
}

#[test]
fn sutra_5_1_21() {
    assert_has_artha_taddhita("Sata", TenaKritam, T::Wan, &["Satika"]);
    assert_has_artha_taddhita("Sata", TenaKritam, T::yat, &["Satya"]);
    assert_has_artha_taddhita("Sata", TenaKritam, T::kan, &["Sataka"]);
}

#[test]
fn sutra_5_1_22() {
    assert_has_taddhita("paYca", T::kan, &["paYcaka"]);
    assert_has_taddhita("bahu", T::kan, &["bahuka"]);
    assert_has_taddhita("gaRa", T::kan, &["gaRaka"]);
    // a-ti-Sat
    assert_has_taddhita("saptati", T::WaY, &["sAptatika"]);
    assert_has_taddhita("catvAriMSat", T::WaY, &["cAtvAriMSatka"]);
}

#[test]
fn sutra_5_1_23() {
    let tavat = artha_taddhitanta("tad", Parimana, T::vatup);
    assert_has_artha_taddhita(&tavat, TenaKritam, T::kan, &["tAvatka", "tAvatika"]);

    let yavat = artha_taddhitanta("yad", Parimana, T::vatup);
    assert_has_artha_taddhita(&yavat, TenaKritam, T::kan, &["yAvatka", "yAvatika"]);
}

#[test]
fn sutra_5_1_25() {
    assert_has_artha_taddhita("kaMsa", TenaKritam, T::wiWan, &["kaMsika"]);
    assert_has_artha_taddhita("kaMsa", TenaKritam, T::WaY, &[]);
}

#[test]
fn sutra_5_1_26() {
    assert_has_artha_taddhita("SUrpa", TenaKritam, T::aY, &["SOrpa"]);
    assert_has_artha_taddhita("SUrpa", TenaKritam, T::WaY, &["SOrpika"]);
}

#[test]
fn sutra_5_1_27() {
    assert_has_artha_taddhita("SatamAna", TenaKritam, T::aR, &["SAtamAna"]);
    assert_has_artha_taddhita("viMSatika", TenaKritam, T::aR, &["vEMSatika"]);
    assert_has_artha_taddhita("sahasra", TenaKritam, T::aR, &["sAhasra"]);
    assert_has_artha_taddhita("vasana", TenaKritam, T::aR, &["vAsana"]);
}

#[test]
fn sutra_5_1_37() {
    let artha = TenaKritam;
    assert_has_artha_taddhita("saptati", artha, T::WaY, &["sAptatika"]);
    assert_has_artha_taddhita("aSIti", artha, T::WaY, &["ASItika"]);
    assert_has_artha_taddhita("nizka", artha, T::Wak, &["nEzkika"]);
    assert_has_artha_taddhita("paRa", artha, T::Wak, &["pARika"]);
    assert_has_artha_taddhita("pAda", artha, T::Wak, &["pAdika"]);
    assert_has_artha_taddhita("mAza", artha, T::Wak, &["mAzika"]);
    assert_has_artha_taddhita("Sata", artha, T::yat, &["Satya"]);
    assert_has_artha_taddhita("Sata", artha, T::Wan, &["Satika"]);

    assert_has_artha_taddhita("dvi", artha, T::kan, &["dvika"]);
    assert_has_artha_taddhita("tri", artha, T::kan, &["trika"]);
    assert_has_artha_taddhita("paYcan", artha, T::kan, &["paYcaka"]);

    assert_has_artha_taddhita("mudra", artha, T::Wak, &["mOdrika"]);
}

#[test]
fn sutra_5_1_38() {
    let artha = TasyaNimittamSamyogotpattau;
    assert_has_artha_taddhita("Sata", artha, T::yat, &["Satya"]);
    assert_has_artha_taddhita("Sata", artha, T::Wan, &["Satika"]);
    assert_has_artha_taddhita("sahasra", artha, T::aR, &["sAhasra"]);
}

#[test]
fn sutra_5_1_38_v2() {
    let artha = TasyaNimittamSamyogotpattau;
    assert_has_artha_taddhita("sannipAta", artha, T::Wak, &["sAnnipAtika"]);
}

#[test]
fn sutra_5_1_40() {
    let artha = TasyaNimittamSamyogotpattau;
    assert_has_artha_taddhita("putra", artha, T::Ca, &["putrIya"]);
    assert_has_artha_taddhita("putra", artha, T::yat, &["putrya"]);
}

#[ignore]
#[test]
fn sutra_5_1_41() {
    let artha = TasyaNimittamSamyogotpattau;
    assert_has_artha_taddhita("sarvaBUmi", artha, T::aR, &["sArvaBOma"]);
    assert_has_artha_taddhita("pfTivI", artha, T::aY, &["pArTiva"]);
}

#[test]
fn sutra_5_1_45() {
    let artha = TasyaVapa;
    assert_has_artha_taddhita("prasTa", artha, T::WaY, &["prAsTika"]);
    assert_has_artha_taddhita("droRa", artha, T::Wak, &["drORika"]);
    assert_has_artha_taddhita("KArI", artha, T::Ikan, &["KArIka"]);
}

#[test]
fn sutra_5_1_46() {
    let artha = TasyaVapa;
    let patrika = artha_taddhitanta("pAtra", artha, T::zWan);
    assert_has_sup_1s(&patrika, Pum, &["pAtrikaH"]);
    assert_has_sup_1s(&patrika, Stri, &["pAtrikI"]);
}

#[test]
fn sutra_5_1_47() {
    let artha = TadAsminVrddhiAyaLabhaSulkaUpada;
    assert_has_artha_taddhita("paYcan", artha, T::kan, &["paYcaka"]);
    assert_has_artha_taddhita("saptan", artha, T::kan, &["saptaka"]);
    assert_has_artha_taddhita("Sata", artha, T::yat, &["Satya"]);
    assert_has_artha_taddhita("Sata", artha, T::Wan, &["Satika"]);
    assert_has_artha_taddhita("sahasra", artha, T::aR, &["sAhasra"]);
}

#[test]
fn sutra_5_1_49() {
    let artha = TadAsminVrddhiAyaLabhaSulkaUpada;
    assert_has_artha_taddhita("BAga", artha, T::yat, &["BAgya"]);
    assert_has_artha_taddhita("BAga", artha, T::Wan, &["BAgika"]);
    assert_has_artha_taddhita("BAga", artha, T::WaY, &[]);
}

#[ignore]
#[test]
fn sutra_5_1_50() {
    let kd = |x, y| karmadharaya(x, y);

    let artha = TadDharatiVahatiAvahati;
    assert_has_artha_taddhita(kd("vaMSa", "BAra"), artha, T::Wak, &["vAMSaBArika"]);
    assert_has_artha_taddhita(kd("kuwaja", "BAra"), artha, T::Wak, &["kOwajaBArika"]);
    assert_has_artha_taddhita(kd("balvaja", "BAra"), artha, T::Wak, &["bAlvajaBArika"]);

    assert_has_artha_taddhita(kd("vrIhi", "BAra"), artha, T::Wak, &[]);

    assert_has_artha_taddhita("vaMSa", artha, T::Wak, &["vAMSika"]);
    assert_has_artha_taddhita("kuwaja", artha, T::Wak, &["kOwajika"]);
    assert_has_artha_taddhita("balvaja", artha, T::Wak, &["bAlvajika"]);
}

#[test]
fn sutra_5_1_51() {
    let artha = TadDharatiVahatiAvahati;
    assert_has_artha_taddhita("vasna", artha, T::Wan, &["vasnika"]);
    assert_has_artha_taddhita("dravya", artha, T::kan, &["dravyaka"]);
}

#[test]
fn sutra_5_1_52() {
    let artha = SambhavatiAharatiPacati;
    assert_has_artha_taddhita("prasTa", artha, T::WaY, &["prAsTika"]);
    assert_has_artha_taddhita("kuqava", artha, T::WaY, &["kOqavika"]);
    assert_has_artha_taddhita("KArI", artha, T::Ikan, &["KArIka"]);
}

#[test]
fn sutra_5_1_53() {
    let artha = SambhavatiAharatiPacati;
    assert_has_artha_taddhita("AQaka", artha, T::Ka, &["AQakIna"]);
    assert_has_artha_taddhita("AQaka", artha, T::Wak, &["AQakika"]);

    assert_has_artha_taddhita("Acita", artha, T::Ka, &["AcitIna"]);
    assert_has_artha_taddhita("Acita", artha, T::Wak, &["Acitika"]);

    assert_has_artha_taddhita("pAtra", artha, T::Ka, &["pAtrIRa"]);
    assert_has_artha_taddhita("pAtra", artha, T::Wak, &["pAtrika"]);
}

#[ignore]
#[test]
fn sutra_5_1_57() {
    let artha = TadAsyaParimanam;
    assert_has_artha_taddhita("prasTa", artha, T::WaY, &["prAsTika"]);

    assert_has_artha_taddhita("Sata", artha, T::Wan, &["Satika"]);
    assert_has_artha_taddhita("Sata", artha, T::yat, &["Satya"]);
    assert_has_artha_taddhita("sahasra", artha, T::aR, &["sAhasra"]);
    assert_has_artha_taddhita("droRa", artha, T::Wak, &["drORika"]);
    assert_has_artha_taddhita("kuqava", artha, T::WaY, &["kOqavika"]);
}

#[test]
fn sutra_5_1_63() {
    assert_has_artha_taddhita("SvetacCatra", TadArhati, T::Wak, &["SvEtacCatrika"]);
    assert_has_artha_taddhita("vastrayugma", TadArhati, T::Wak, &["vAstrayugmika"]);
    assert_has_artha_taddhita("Sata", TadArhati, T::Wan, &["Satika"]);
    assert_has_artha_taddhita("Sata", TadArhati, T::yat, &["Satya"]);
    assert_has_artha_taddhita("sahasra", TadArhati, T::aR, &["sAhasra"]);
}

#[test]
fn sutra_5_1_64() {
    assert_has_artha_taddhita("Ceda", TadArhati, T::Wak, &["CEdika"]);
    assert_has_artha_taddhita("Beda", TadArhati, T::Wak, &["BEdika"]);
    assert_has_artha_taddhita("viraNg", TadArhati, T::Wak, &["vEraNgika"]);
}

#[test]
fn sutra_5_1_65() {
    assert_has_artha_taddhita("SIrSacCeda", TadArhati, T::Wak, &["SErSacCedika"]);
    assert_has_artha_taddhita("SIrSacCeda", TadArhati, T::yat, &["SIrSacCedya"]);
}

#[test]
fn sutra_5_1_66() {
    assert_has_artha_taddhita("daRqa", TadArhati, T::yat, &["daRqya"]);
    assert_has_artha_taddhita("musala", TadArhati, T::yat, &["musalya"]);
    assert_blocked("daRqa", TadArhati, T::Wak);
}

#[test]
fn sutra_5_1_67() {
    let t = Tester::with_chaandasa();
    t.assert_has_artha_taddhita("udaka", TadArhati, T::yat, &["udakya"]);
    t.assert_has_artha_taddhita("yUpa", TadArhati, T::yat, &["yUpya"]);
}

#[test]
fn sutra_5_1_68() {
    assert_has_artha_taddhita("pAtra", TadArhati, T::Gan, &["pAtriya"]);
    assert_has_artha_taddhita("pAtra", TadArhati, T::yat, &["pAtrya"]);
    assert_blocked("pAtra", TadArhati, T::Wak);
    assert_blocked("pAtra", TadArhati, T::WaY);
}

#[test]
fn sutra_5_1_69() {
    assert_has_artha_taddhita("kaqaNkara", TadArhati, T::Ca, &["kaqaNkarIya"]);
    assert_has_artha_taddhita("kaqaNkara", TadArhati, T::yat, &["kaqaNkarya"]);
    assert_has_artha_taddhita("dakziRA", TadArhati, T::Ca, &["dakziRIya"]);
    assert_has_artha_taddhita("dakziRA", TadArhati, T::yat, &["dakziRya"]);
    assert_blocked("kaqaNkara", TadArhati, T::Wak);
}

#[test]
fn sutra_5_1_70() {
    assert_has_artha_taddhita("sTAlIbila", TadArhati, T::Ca, &["sTAlIbilIya"]);
    assert_has_artha_taddhita("sTAlIbila", TadArhati, T::yat, &["sTAlIbilya"]);
    assert_blocked("sTAlIbila", TadArhati, T::Wak);
}

#[test]
fn sutra_5_1_71() {
    assert_has_artha_taddhita("yajYa", TadArhati, T::Ga, &["yajYiya"]);
    assert_has_artha_taddhita("ftvij", TadArhati, T::KaY, &["ArtvijIna"]);
    assert_blocked("yajYa", TadArhati, T::Wak);
    assert_blocked("ftvij", TadArhati, T::Wak);
}

#[test]
fn sutra_5_1_72() {
    assert_has_artha_taddhita("pArAyaRa", TadVartayati, T::WaY, &["pArAyaRika"]);
    assert_has_artha_taddhita("turAyaRa", TadVartayati, T::WaY, &["tOrAyaRika"]);
    assert_has_artha_taddhita("cAndrAyaRa", TadVartayati, T::WaY, &["cAndrAyaRika"]);
}

#[test]
fn sutra_5_1_73() {
    assert_has_artha_taddhita("saMSaya", Apanna, T::WaY, &["sAMSayika"]);
}

#[test]
fn sutra_5_1_74() {
    assert_has_artha_taddhita("yojana", Gacchati, T::WaY, &["yOjanika"]);
}

#[test]
fn sutra_5_1_74_v1() {
    let artha = AbhigamanamArhati;
    assert_has_artha_taddhita("kroSaSata", artha, T::WaY, &["krOSaSatika"]);
    assert_has_artha_taddhita("yojanaSata", artha, T::WaY, &["yOjanaSatika"]);
}

#[test]
fn sutra_5_1_75() {
    let pathika = create_artha_taddhita("paTika", "paTin", Gacchati, T::zkan);
    assert_has_sup_1s(&pathika, Pum, &["paTikaH"]);
    assert_has_sup_1s(&pathika, Stri, &["paTikI"]);
}

#[test]
fn sutra_5_1_76() {
    assert_has_artha_taddhita("paTin", Gacchati, T::Ra, &["pAnTa"]);
    // nityam?
    assert_has_artha_taddhita("paTin", Gacchati, T::zkan, &["paTika"]);
}

#[test]
fn sutra_5_1_77() {
    assert_has_artha_taddhita("uttarapaTa", Gacchati, T::WaY, &["OttarapaTika"]);
    assert_has_artha_taddhita("uttarapaTa", Ahrtam, T::WaY, &["OttarapaTika"]);
}

#[test]
fn sutra_5_1_77_v1() {
    assert_has_artha_taddhita("vAripaTa", Gacchati, T::WaY, &["vAripaTika"]);
    assert_has_artha_taddhita("vAripaTa", Ahrtam, T::WaY, &["vAripaTika"]);

    assert_has_artha_taddhita("jaNgalapaTa", Gacchati, T::WaY, &["jANgalapaTika"]);
    assert_has_artha_taddhita("jaNgalapaTa", Ahrtam, T::WaY, &["jANgalapaTika"]);

    assert_has_artha_taddhita("sTalapaTa", Gacchati, T::WaY, &["sTAlapaTika"]);
    assert_has_artha_taddhita("sTalapaTa", Ahrtam, T::WaY, &["sTAlapaTika"]);

    assert_has_artha_taddhita("kAntArapaTa", Gacchati, T::WaY, &["kAntArapaTika"]);
    assert_has_artha_taddhita("kAntArapaTa", Ahrtam, T::WaY, &["kAntArapaTika"]);
}

#[test]
fn sutra_5_1_77_v2() {
    assert_has_artha_taddhita("ajapaTa", Gacchati, T::WaY, &["AjapaTika"]);
    assert_has_artha_taddhita("ajapaTa", Ahrtam, T::WaY, &["AjapaTika"]);

    assert_has_artha_taddhita("SaNkupaTa", Gacchati, T::WaY, &["SANkupaTika"]);
    assert_has_artha_taddhita("SaNkupaTa", Ahrtam, T::WaY, &["SANkupaTika"]);
}

#[test]
fn sutra_5_1_77_v3() {
    assert_has_taddhita("sTalapaTa", T::aR, &["sTAlapaTa"]);
}

#[test]
fn sutra_5_1_79() {
    assert_has_artha_taddhita("mAsa", TenaNirvrttam, T::WaY, &["mAsika"]);
    assert_has_artha_taddhita("arDamAsa", TenaNirvrttam, T::WaY, &["ArDamAsika"]);
    assert_has_artha_taddhita("saMvatsara", TenaNirvrttam, T::WaY, &["sAMvatsarika"]);
    assert_has_artha_taddhita("ahan", TenaNirvrttam, T::WaY, &["Ahnika"]);
}

#[test]
fn sutra_5_1_80() {
    let artha = TamAdhisteBhrtoBhutoBhavi;
    assert_has_artha_taddhita("mAsa", artha, T::WaY, &["mAsika"]);
}

#[test]
fn sutra_5_1_81() {
    assert_has_artha_taddhita("mAsa", Vayasi, T::yat, &["mAsya"]);
    assert_has_artha_taddhita("mAsa", Vayasi, T::KaY, &["mAsIna"]);

    // vayasi?
    assert_has_artha_taddhita("mAsa", TenaNirvrttam, T::WaY, &["mAsika"]);
}

#[test]
fn sutra_5_1_85() {
    let artha = TamAdhisteBhrtoBhutoBhavi;
    assert_has_artha_taddhita("samA", artha, T::Ka, &["samIna"]);
    assert_blocked("samA", artha, T::WaY);
}

#[test]
fn sutra_5_1_105() {
    assert_has_taddhita("ftu", T::aR, &["Artava"]);
}

#[test]
fn sutra_5_1_106() {
    assert_has_taddhita("ftu", T::Gas, &["ftviya"]);
}

#[test]
fn sutra_5_1_119() {
    assert_has_taddhita("aSva", T::tva, &["aSvatva"]);
    assert_has_taddhita("aSva", T::tal, &["aSvatA"]);
    assert_has_taddhita("go", T::tva, &["gotva"]);
    assert_has_taddhita("go", T::tal, &["gotA"]);
}

#[test]
fn sutra_5_1_122() {
    assert_has_taddhita("pfTu", T::imanic, &["praTiman"]);
    assert_has_taddhita("pfTu", T::aR, &["pArTava"]);
    assert_has_taddhita("mfdu", T::imanic, &["mradiman"]);
    assert_has_taddhita("mfdu", T::aR, &["mArdava"]);
}
