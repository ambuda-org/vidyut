/// Tests for pada 4.4, including "prAg vahatez Wak".
///
/// Ideally, these tests would include apavada tests verifying that Wak can't be used in certain
/// senses. But because Wak is allowable in so many differest semantic conditions, and since our
/// system does not model semantics, we have no easy way to implement apavada tests in practice.
extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::*;

fn assert_blocked(text: &str, artha: TaddhitaArtha, taddhita: Taddhita) {
    assert_has_artha_taddhita(text, artha, taddhita, &[]);
}

#[test]
fn sutra_4_4_1() {
    assert_has_artha_taddhita("akza", TenaDivyatiJayatiJitam, T::Wak, &["Akzika"]);
}

#[test]
fn sutra_4_4_2() {
    assert_has_artha_taddhita("akza", TenaDivyatiJayatiJitam, T::Wak, &["Akzika"]);
    assert_has_artha_taddhita("SalAkA", TenaDivyatiJayatiJitam, T::Wak, &["SAlAkika"]);
    assert_has_artha_taddhita("aBri", TenaDivyatiJayatiJitam, T::Wak, &["ABrika"]);
    assert_has_artha_taddhita("kuddAla", TenaDivyatiJayatiJitam, T::Wak, &["kOddAlika"]);
}

#[test]
fn sutra_4_4_3() {
    assert_has_artha_taddhita("daDi", TenaSamskrtam, T::Wak, &["dADika"]);
    assert_has_artha_taddhita("SfNgavera", TenaSamskrtam, T::Wak, &["SArNgaverika"]);
    assert_has_artha_taddhita("marici", TenaSamskrtam, T::Wak, &["mAricika"]);
}

#[test]
fn sutra_4_4_4() {
    assert_has_artha_taddhita("kulatTa", TenaSamskrtam, T::aR, &["kOlatTa"]);
    assert_has_artha_taddhita("tittiqIka", TenaSamskrtam, T::aR, &["tEttiqIka"]);
    assert_has_artha_taddhita("dardaBaka", TenaSamskrtam, T::aR, &["dArdaBaka"]);
    assert_blocked("kulatTa", TenaSamskrtam, T::Wak);
}

#[test]
fn sutra_4_4_5() {
    assert_has_artha_taddhita("kARqaplava", TenaTarati, T::Wak, &["kARqaplavika"]);
}

#[test]
fn sutra_4_4_6() {
    assert_has_artha_taddhita("gopucCa", TenaTarati, T::WaY, &["gOpucCika"]);
    assert_blocked("gopucCa", TenaTarati, T::Wak);
}

#[test]
fn sutra_4_4_7() {
    assert_has_artha_taddhita("nO", TenaTarati, T::Wan, &["nAvika"]);
    assert_has_artha_taddhita("Gawa", TenaTarati, T::Wan, &["Gawika"]);
    // TODO: bAhuka
    assert_blocked("nO", TenaTarati, T::Wak);
}

#[test]
fn sutra_4_4_8() {
    assert_has_artha_taddhita("hastin", TenaCarati, T::Wak, &["hAstika"]);
    assert_has_artha_taddhita("SakawA", TenaCarati, T::Wak, &["SAkawika"]);
    assert_has_artha_taddhita("daDi", TenaCarati, T::Wak, &["dADika"]);
}

#[test]
fn sutra_4_4_9() {
    assert_has_artha_taddhita("Akarza", TenaCarati, T::zWal, &["Akarzika"]);
    assert_blocked("Akarza", TenaCarati, T::Wak);
}

#[test]
fn sutra_4_4_10() {
    assert_has_artha_taddhita("parpa", TenaCarati, T::zWan, &["parpika"]);
    assert_has_artha_taddhita("aSva", TenaCarati, T::zWan, &["aSvika"]);
    assert_blocked("parpa", TenaCarati, T::Wak);
}

#[test]
fn sutra_4_4_11() {
    assert_has_artha_taddhita("SvagaRa", TenaCarati, T::zWan, &["SvagaRika"]);
    assert_has_artha_taddhita("SvagaRa", TenaCarati, T::WaY, &["SvAgaRika"]);
    assert_blocked("SvagaRa", TenaCarati, T::Wak);
}

#[test]
fn sutra_4_4_12() {
    assert_has_artha_taddhita("vetana", TenaJivati, T::Wak, &["vEtanika"]);
    assert_has_artha_taddhita("DanurdaRqa", TenaJivati, T::Wak, &["DAnurdaRqika"]);
    // TODO: others
}

#[test]
fn sutra_4_4_13() {
    assert_has_artha_taddhita("vasna", TenaJivati, T::Wan, &["vasnika"]);
    assert_has_artha_taddhita("kraya", TenaJivati, T::Wan, &["krayika"]);
    assert_has_artha_taddhita("vikraya", TenaJivati, T::Wan, &["vikrayika"]);
    assert_has_artha_taddhita("krayavikraya", TenaJivati, T::Wan, &["krayavikrayika"]);
}

#[test]
fn sutra_4_4_14() {
    assert_has_artha_taddhita("AyuDa", TenaJivati, T::Wan, &["AyuDika"]);
    assert_has_artha_taddhita("AyuDa", TenaJivati, T::Ca, &["AyuDIya"]);
}

#[test]
fn sutra_4_4_15() {
    assert_has_artha_taddhita("utsaNga", TenaHarati, T::Wak, &["OtsaNgika"]);
    assert_has_artha_taddhita("uqupa", TenaHarati, T::Wak, &["Oqupika"]);
}

#[test]
fn sutra_4_4_16() {
    assert_has_artha_taddhita("BastrA", TenaHarati, T::zWan, &["Bastrika"]);
    assert_has_artha_taddhita("Barawa", TenaHarati, T::zWan, &["Barawika"]);
}

#[test]
fn sutra_4_4_17() {
    assert_has_artha_taddhita("vivaDa", TenaHarati, T::zWan, &["vivaDika"]);
    assert_has_artha_taddhita("vivaDa", TenaHarati, T::Wak, &["vEvaDika"]);
    assert_has_artha_taddhita("vIvaDa", TenaHarati, T::zWan, &["vIvaDika"]);
    assert_has_artha_taddhita("vIvaDa", TenaHarati, T::Wak, &["vEvaDika"]);
}

#[test]
fn sutra_4_4_18() {
    assert_has_artha_taddhita("kuwilikA", TenaHarati, T::aR, &["kOwilika"]);
}

#[test]
fn sutra_4_4_19() {
    assert_has_artha_taddhita("akzadyUta", TenaNirvrtte, T::Wak, &["AkzadyUtika"]);
    assert_has_artha_taddhita("jAnuprahfta", TenaNirvrtte, T::Wak, &["jAnuprahftika"]);
}

#[test]
fn sutra_4_4_20() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::ktri, &["paktrima"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), Krt::ktri, &["uptrima"]);
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::ktri, &["kftrima"]);
}

#[test]
fn sutra_4_4_21() {
    assert_has_artha_taddhita("apamitya", TenaNirvrtte, T::kak, &["Apamityaka"]);
    assert_has_artha_taddhita("yAcita", TenaNirvrtte, T::kan, &["yAcitaka"]);
}

#[test]
fn sutra_4_4_22() {
    assert_has_artha_taddhita("daDi", TenaSamsrshte, T::Wak, &["dADika"]);
    assert_has_artha_taddhita("marici", TenaSamsrshte, T::Wak, &["mAricika"]);
    assert_has_artha_taddhita("SfNgavera", TenaSamsrshte, T::Wak, &["SArNgaverika"]);
    assert_has_artha_taddhita("pippala", TenaSamsrshte, T::Wak, &["pEppalika"]);
}

#[test]
fn sutra_4_4_23() {
    assert_has_artha_taddhita("cUrRa", TenaSamsrshte, T::ini, &["cUrRin"]);
}

#[test]
fn sutra_4_4_24() {
    assert_has_artha_taddhita("lavaRa", TenaSamsrshte, T::Wak, &["lavaRa"]);
}

#[test]
fn sutra_4_4_25() {
    assert_has_artha_taddhita("mudra", TenaSamsrshte, T::aR, &["mOdra"]);
}

#[test]
fn sutra_4_4_26() {
    assert_has_artha_taddhita("daDi", TenaUpasikte, T::Wak, &["dADika"]);
    assert_has_artha_taddhita("sUpa", TenaUpasikte, T::Wak, &["sOpika"]);
    assert_has_artha_taddhita("Kara", TenaUpasikte, T::Wak, &["KArika"]);
}

#[test]
fn sutra_4_4_27() {
    assert_has_artha_taddhita("ojas", Vartate, T::Wak, &["Ojasika"]);
    assert_has_artha_taddhita("sahas", Vartate, T::Wak, &["sAhasika"]);
    assert_has_artha_taddhita("amBas", Vartate, T::Wak, &["AmBasika"]);
}

#[test]
fn sutra_4_4_28() {
    assert_has_artha_taddhita("pratIpa", Vartate, T::Wak, &["prAtIpika"]);
    assert_has_artha_taddhita("anvIpa", Vartate, T::Wak, &["AnvIpika"]);
    assert_has_artha_taddhita("pratiloma", Vartate, T::Wak, &["prAtilomika"]);
    assert_has_artha_taddhita("anuloma", Vartate, T::Wak, &["Anulomika"]);
    assert_has_artha_taddhita("pratikUla", Vartate, T::Wak, &["prAtikUlika"]);
    assert_has_artha_taddhita("anukUla", Vartate, T::Wak, &["AnukUlika"]);
}

#[test]
fn sutra_4_4_29() {
    assert_has_artha_taddhita("parimuKa", Vartate, T::Wak, &["pArimuKika"]);
}

#[test]
fn sutra_4_4_30() {
    assert_has_artha_taddhita("dviguRa", PrayacchatiGarhyam, T::Wak, &["dvEguRika"]);
    assert_has_artha_taddhita("triguRa", PrayacchatiGarhyam, T::Wak, &["trEguRika"]);
}

#[test]
fn sutra_4_4_31() {
    assert_has_artha_taddhita("kusIda", PrayacchatiGarhyam, T::zWan, &["kusIdika"]);
    assert_has_artha_taddhita("daSEkAdaSa", PrayacchatiGarhyam, T::zWac, &["daSEkAdaSika"]);
}

#[test]
fn sutra_4_4_32() {
    assert_has_artha_taddhita("badara", Unchati, T::Wak, &["bAdarika"]);
    assert_has_artha_taddhita("SyAmAka", Unchati, T::Wak, &["SyAmAkika"]);
    assert_has_artha_taddhita("kaRa", Unchati, T::Wak, &["kARika"]);
}

#[test]
fn sutra_4_4_33() {
    assert_has_artha_taddhita("samAja", TadRakshati, T::Wak, &["sAmAjika"]);
    assert_has_artha_taddhita("sanniveSa", TadRakshati, T::Wak, &["sAnniveSika"]);
}

#[test]
fn sutra_4_4_34() {
    assert_has_taddhitanta(&prati("Sabda"), T::Wak, &["SAbdika"]);
    assert_has_taddhitanta(&prati("dardura"), T::Wak, &["dArdurika"]);
}

#[test]
fn sutra_4_4_35() {
    assert_has_artha_taddhita("pakzin", Hanti, T::Wak, &["pAkzika"]);
    assert_has_artha_taddhita("Sakuni", Hanti, T::Wak, &["SAkunika"]);
    assert_has_artha_taddhita("mayUra", Hanti, T::Wak, &["mAyUrika"]);
    assert_has_artha_taddhita("tittiri", Hanti, T::Wak, &["tEttirika"]);
    assert_has_artha_taddhita("matsya", Hanti, T::Wak, &["mAtsyika"]);
    assert_has_artha_taddhita("mIna", Hanti, T::Wak, &["mEnika"]);
    assert_has_artha_taddhita("SaPara", Hanti, T::Wak, &["SAParika"]);
    assert_has_artha_taddhita("Sakuli", Hanti, T::Wak, &["SAkulika"]);
    assert_has_artha_taddhita("mfga", Hanti, T::Wak, &["mArgika"]);
    assert_has_artha_taddhita("hariRa", Hanti, T::Wak, &["hAriRika"]);
    assert_has_artha_taddhita("sUkara", Hanti, T::Wak, &["sOkarika"]);
    assert_has_artha_taddhita("sAraNga", Hanti, T::Wak, &["sAraNgika"]);
}

#[test]
fn sutra_4_4_36() {
    assert_has_artha_taddhita("paripanTa", Hanti, T::Wak, &["pAripanTika"]);
}

#[test]
fn sutra_4_4_37() {
    assert_has_artha_taddhita("daRqamATa", Hanti, T::Wak, &["dARqamATika"]);
    assert_has_artha_taddhita("SulkamATa", Hanti, T::Wak, &["SOlkamATika"]);
    assert_has_artha_taddhita("padavI", Hanti, T::Wak, &["pAdavika"]);
    assert_has_artha_taddhita("anupada", Hanti, T::Wak, &["Anupadika"]);
}

#[test]
fn sutra_4_4_38() {
    assert_has_artha_taddhita("Akranda", Hanti, T::Wak, &["Akrandika"]);
    assert_has_artha_taddhita("Akranda", Hanti, T::WaY, &["Akrandika"]);
}

#[test]
fn sutra_4_4_39() {
    assert_has_artha_taddhita("pUrvapada", Grhnati, T::Wak, &["pOrvapadika"]);
    assert_has_artha_taddhita("uttarapada", Grhnati, T::Wak, &["Ottarapadika"]);
    // TODO: bahupada
}

#[test]
fn sutra_4_4_40() {
    assert_has_artha_taddhita("pratikaRWA", Grhnati, T::Wak, &["prAtikaRWika"]);
    assert_has_artha_taddhita("arTa", Grhnati, T::Wak, &["ArTika"]);
    assert_has_artha_taddhita("lalAma", Grhnati, T::Wak, &["lAlAmika"]);
}

#[test]
fn sutra_4_4_41() {
    assert_has_artha_taddhita("Darma", Carati, T::Wak, &["DArmika"]);
}

#[test]
fn sutra_4_4_41_v1() {
    assert_has_artha_taddhita("aDarma", Carati, T::Wak, &["ADarmika"]);
}

#[test]
fn sutra_4_4_42() {
    assert_has_artha_taddhita("pratipaTa", Eti, T::Wak, &["prAtipaTika"]);
    assert_has_artha_taddhita("pratipaTa", Eti, T::Wan, &["pratipaTika"]);
}

#[test]
fn sutra_4_4_43() {
    assert_has_artha_taddhita("samavAya", Samavaiti, T::Wak, &["sAmavAyika"]);
    assert_has_artha_taddhita("samAja", Samavaiti, T::Wak, &["sAmAjika"]);
    assert_has_artha_taddhita("samUha", Samavaiti, T::Wak, &["sAmUhika"]);
    assert_has_artha_taddhita("sanniveSa", Samavaiti, T::Wak, &["sAnniveSika"]);
}

#[test]
fn sutra_4_4_44() {
    assert_has_artha_taddhita("parizad", Samavaiti, T::Rya, &["pArizadya"]);
}

#[test]
fn sutra_4_4_45() {
    assert_has_artha_taddhita("senA", Samavaiti, T::Rya, &["sEnya"]);
    assert_has_artha_taddhita("senA", Samavaiti, T::Wak, &["sEnika"]);
}

#[test]
fn sutra_4_4_46() {
    assert_has_artha_taddhita("lalAwa", Pashyati, T::Wak, &["lAlAwika"]);
    assert_has_artha_taddhita("kukkuwI", Pashyati, T::Wak, &["kOkkuwika"]);
}

#[test]
fn sutra_4_4_47() {
    assert_has_artha_taddhita("SulkaSAlA", TasyaDharmyam, T::Wak, &["SOlkaSAlika"]);
    assert_has_artha_taddhita("Akara", TasyaDharmyam, T::Wak, &["Akarika"]);
    assert_has_artha_taddhita("ApaRa", TasyaDharmyam, T::Wak, &["ApaRika"]);
    assert_has_artha_taddhita("gulma", TasyaDharmyam, T::Wak, &["gOlmika"]);
}

#[test]
fn sutra_4_4_48() {
    assert_has_artha_taddhita("mahizI", TasyaDharmyam, T::aR, &["mAhiza"]);
    assert_has_artha_taddhita("prajAvatI", TasyaDharmyam, T::aR, &["prAjAvata"]);
}

#[test]
fn sutra_4_4_49() {
    assert_has_artha_taddhita("potf", TasyaDharmyam, T::aY, &["pOtra"]);
    assert_has_artha_taddhita("udgAtf", TasyaDharmyam, T::aY, &["OdgAtra"]);
}

#[test]
fn sutra_4_4_49_v1() {
    assert_has_artha_taddhita("nara", TasyaDharmyam, T::aY, &["nAra"]);
}

#[test]
fn sutra_4_4_50() {
    assert_has_artha_taddhita("SulkaSAlA", Avakraya, T::Wak, &["SOlkaSAlika"]);
    assert_has_artha_taddhita("Akara", Avakraya, T::Wak, &["Akarika"]);
    assert_has_artha_taddhita("ApaRa", Avakraya, T::Wak, &["ApaRika"]);
    assert_has_artha_taddhita("gulma", Avakraya, T::Wak, &["gOlmika"]);
}

#[test]
fn sutra_4_4_51() {
    assert_has_artha_taddhita("apUpa", TadAsyaPanyam, T::Wak, &["ApUpika"]);
    assert_has_artha_taddhita("Sazkula", TadAsyaPanyam, T::Wak, &["SAzkulika"]);
    assert_has_artha_taddhita("modaka", TadAsyaPanyam, T::Wak, &["mOdakika"]);
}

#[test]
fn sutra_4_4_52() {
    assert_has_artha_taddhita("lavaRa", TadAsyaPanyam, T::WaY, &["lAvaRika"]);
}

#[test]
fn sutra_4_4_53() {
    // TODO: strI-linga
    assert_has_artha_taddhita("kiSara", TadAsyaPanyam, T::zWan, &["kiSarika"]);
    assert_has_artha_taddhita("narada", TadAsyaPanyam, T::zWan, &["naradika"]);
}

#[test]
fn sutra_4_4_54() {
    assert_has_artha_taddhita("SalAlu", TadAsyaPanyam, T::zWan, &["SalAluka"]);
    assert_has_artha_taddhita("SalAlu", TadAsyaPanyam, T::Wak, &["SAlAluka"]);
}

#[test]
fn sutra_4_4_55() {
    assert_has_artha_taddhita("mfdaNga", Shilpam, T::Wak, &["mArdaNgika"]);
    assert_has_artha_taddhita("paRavA", Shilpam, T::Wak, &["pARavika"]);
    assert_has_artha_taddhita("vIRA", Shilpam, T::Wak, &["vERika"]);
}

#[test]
fn sutra_4_4_56() {
    assert_has_artha_taddhita("maqquka", Shilpam, T::aR, &["mAqquka"]);
    assert_has_artha_taddhita("maqquka", Shilpam, T::Wak, &["mAqqukika"]);
    assert_has_artha_taddhita("JarJara", Shilpam, T::aR, &["JArJara"]);
    assert_has_artha_taddhita("JarJara", Shilpam, T::Wak, &["JArJarika"]);
}

#[test]
fn sutra_4_4_57() {
    assert_has_artha_taddhita("asi", Praharanam, T::Wak, &["Asika"]);
    assert_has_artha_taddhita("prAsi", Praharanam, T::Wak, &["prAsika"]);
    assert_has_artha_taddhita("cakra", Praharanam, T::Wak, &["cAkrika"]);
    assert_has_artha_taddhita("Danus", Praharanam, T::Wak, &["DAnuzka"]);
}

#[test]
fn sutra_4_4_58() {
    assert_has_artha_taddhita("paraSvaDa", Praharanam, T::WaY, &["pAraSvaDika"]);
    assert_has_artha_taddhita("paraSvaDa", Praharanam, T::Wak, &["pAraSvaDika"]);
}

#[test]
fn sutra_4_4_59() {
    assert_has_artha_taddhita("Sakti", Praharanam, T::Ikak, &["SAktIka"]);
    assert_has_artha_taddhita("yazwi", Praharanam, T::Ikak, &["yAzwIka"]);
}

#[test]
fn sutra_4_4_60() {
    assert_has_artha_taddhita("asti", Mati, T::Wak, &["Astika"]);
    assert_has_artha_taddhita("nAsti", Mati, T::Wak, &["nAstika"]);
    assert_has_artha_taddhita("dizwa", Mati, T::Wak, &["dEzwika"]);
}

#[test]
fn sutra_4_4_61() {
    assert_has_artha_taddhita("apUpa", Shilam, T::Wak, &["ApUpika"]);
    assert_has_artha_taddhita("Sazkula", Shilam, T::Wak, &["SAzkulika"]);
    assert_has_artha_taddhita("modaka", Shilam, T::Wak, &["mOdakika"]);
}

#[test]
fn sutra_4_4_62() {
    assert_has_artha_taddhita("Catra", Shilam, T::Ra, &["CAtra"]);
}

#[test]
fn sutra_4_4_65() {
    assert_has_taddhitanta(&prati("apUpa"), T::Wak, &["ApUpika"]);
    assert_has_taddhitanta(&prati("Sazkula"), T::Wak, &["SAzkulika"]);
    assert_has_taddhitanta(&prati("modaka"), T::Wak, &["mOdakika"]);
}

#[test]
fn sutra_4_4_66() {
    let asmai = TadAsmaiDiyateNiyuktam;
    assert_has_artha_taddhita("apUpa", asmai, T::Wak, &["ApUpika"]);
    assert_has_artha_taddhita("Sazkula", asmai, T::Wak, &["SAzkulika"]);
}

#[test]
fn sutra_4_4_67() {
    let asmai = TadAsmaiDiyateNiyuktam;
    assert_has_artha_taddhita("SrARA", asmai, T::wiWan, &["SrARika"]);
    assert_has_artha_taddhita("mAMsOdana", asmai, T::wiWan, &["mAMsOdanika"]);
}

#[test]
fn sutra_4_4_68() {
    let asmai = TadAsmaiDiyateNiyuktam;
    assert_has_artha_taddhita("Bakta", asmai, T::aR, &["BAkta"]);
    assert_has_artha_taddhita("Bakta", asmai, T::Wak, &["BAktika"]);
}

#[test]
fn sutra_4_4_69() {
    assert_has_artha_taddhita("SulkaSAlA", Niyuktam, T::Wak, &["SOlkaSAlika"]);
    assert_has_artha_taddhita("Akara", Niyuktam, T::Wak, &["Akarika"]);
    assert_has_artha_taddhita("ApaRa", Niyuktam, T::Wak, &["ApaRika"]);
    assert_has_artha_taddhita("gulma", Niyuktam, T::Wak, &["gOlmika"]);
    assert_has_artha_taddhita("dvAra", Niyuktam, T::Wak, &["dOvArika"]);
}

#[test]
fn sutra_4_4_70() {
    assert_has_artha_taddhita("devAgAra", Niyuktam, T::Wan, &["devAgArika"]);
    assert_has_artha_taddhita("kozWAgAra", Niyuktam, T::Wan, &["kozWAgArika"]);
    assert_has_artha_taddhita("BARqAgAra", Niyuktam, T::Wan, &["BARqAgArika"]);
}

#[test]
fn sutra_4_4_71() {
    assert_has_taddhitanta(&prati("SmaSAna"), T::Wak, &["SmASAnika"]);
    assert_has_taddhitanta(&prati("catuzpaTa"), T::Wak, &["cAtuzpaTika"]);
    assert_has_taddhitanta(&prati("caturdaSa"), T::Wak, &["cAturdaSika"]);
    assert_has_taddhitanta(&prati("amAvAsyA"), T::Wak, &["AmAvAsyika"]);
}

#[test]
fn sutra_4_4_72() {
    assert_has_artha_taddhita("vaMSakaWina", Vyavaharati, T::Wak, &["vAMSakaWinika"]);
    assert_has_artha_taddhita("vfDrakaWina", Vyavaharati, T::Wak, &["vArDrakaWinika"]);
    assert_has_artha_taddhita("prastAra", Vyavaharati, T::Wak, &["prAstArika"]);
    assert_has_artha_taddhita("saMsTAna", Vyavaharati, T::Wak, &["sAMsTAnika"]);
}

#[test]
fn sutra_4_4_73() {
    assert_has_artha_taddhita("nikawa", Vasati, T::Wak, &["nEkawika"]);
}

#[test]
fn sutra_4_4_74() {
    assert_has_artha_taddhita("AvasaTa", Vasati, T::zWal, &["AvasaTika"]);
}

#[test]
fn sutra_4_4_76() {
    assert_has_artha_taddhita("raTa", TadVahati, T::yat, &["raTya"]);
    assert_has_artha_taddhita("yuga", TadVahati, T::yat, &["yugya"]);
    assert_has_artha_taddhita("prAsaNga", TadVahati, T::yat, &["prAsaNgya"]);
}

#[test]
fn sutra_4_4_77() {
    assert_has_artha_taddhita("Dur", TadVahati, T::yat, &["Durya"]);
    assert_has_artha_taddhita("Dur", TadVahati, T::Qak, &["DOreya"]);
}

#[test]
fn sutra_4_4_78() {
    assert_has_artha_taddhita("sarvaDurA", TadVahati, T::Ka, &["sarvaDurIRa"]);
    assert_has_artha_taddhita("uttaraDurA", TadVahati, T::Ka, &["uttaraDurIRa"]);
    assert_has_artha_taddhita("dakziRaDurA", TadVahati, T::Ka, &["dakziRaDurIRa"]);
}

#[test]
fn sutra_4_4_80() {
    assert_has_artha_taddhita("Sakawa", TadVahati, T::aR, &["SAkawa"]);
}

#[test]
fn sutra_4_4_81() {
    assert_has_artha_taddhita("hala", TadVahati, T::Wak, &["hAlika"]);
    assert_has_artha_taddhita("sIra", TadVahati, T::Wak, &["sErika"]);
}

#[test]
fn sutra_4_4_82() {
    assert_has_artha_taddhita("janI", TadVahati, T::yat, &["janya"]);
}

#[test]
fn sutra_4_4_83() {
    assert_has_artha_taddhita("pAda", TadVidhyati, T::yat, &["padya"]);
    assert_has_artha_taddhita("Uru", TadVidhyati, T::yat, &["Uravya"]);
    // TODO: a-DanuzA
}

#[test]
fn sutra_4_4_91() {
    assert_has_taddhitanta(&prati("nO"), T::yat, &["nAvya"]);
    assert_has_taddhitanta(&prati("vayas"), T::yat, &["vayasya"]);
    assert_has_taddhitanta(&prati("Darma"), T::yat, &["Darmya"]);
    assert_has_taddhitanta(&prati("viza"), T::yat, &["vizya"]);
    assert_has_taddhitanta(&prati("mUla"), T::yat, &["mUlya"]);
    assert_has_taddhitanta(&prati("sItA"), T::yat, &["sItya"]);
    assert_has_taddhitanta(&prati("tulA"), T::yat, &["tulya"]);
    // TODO: others
}
