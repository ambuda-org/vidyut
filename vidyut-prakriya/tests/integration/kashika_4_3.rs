extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha;
use vidyut_prakriya::args::TaddhitaArtha::*;

fn assert_blocked(text: &str, artha: TaddhitaArtha, t: T) {
    assert_has_artha_taddhita(text, artha, t, &[]);
}

#[allow(non_snake_case)]
fn assert_no_aR(text: &str, artha: TaddhitaArtha) {
    assert_blocked(text, artha, T::aR);
}

#[test]
fn sutra_4_3_1_and_sutra_4_3_2() {
    assert_has_artha_taddhita("yuzmad", TatraJata, T::KaY, &["yOzmAkIRa"]);
    assert_has_artha_taddhita("asmad", TatraJata, T::KaY, &["AsmAkIna"]);
    assert_has_artha_taddhita("yuzmad", TatraJata, T::Ca, &["yuzmadIya"]);
    assert_has_artha_taddhita("asmad", TatraJata, T::Ca, &["asmadIya"]);
    assert_has_artha_taddhita("yuzmad", TatraJata, T::aR, &["yOzmAka"]);
    assert_has_artha_taddhita("asmad", TatraJata, T::aR, &["AsmAka"]);
}

#[test]
fn sutra_4_3_4() {
    assert_has_artha_taddhita("arDa", TatraJata, T::yat, &["arDya"]);
    assert_blocked("arDa", TatraJata, T::aR);
}

#[test]
fn sutra_4_3_5() {
    assert_has_artha_taddhita("parArDa", TatraJata, T::yat, &["parArDya"]);
    assert_has_artha_taddhita("avarArDa", TatraJata, T::yat, &["avarArDya"]);
    assert_has_artha_taddhita("aDamArDa", TatraJata, T::yat, &["aDamArDya"]);
    assert_has_artha_taddhita("uttamArDa", TatraJata, T::yat, &["uttamArDya"]);
}

#[ignore]
#[test]
fn sutra_4_3_8() {
    assert_has_artha_taddhita("maDya", TatraJata, T::aR, &["maDyama"]);
}

#[test]
fn sutra_4_3_14() {
    assert_has_artha_taddhita("niSA", TatraBhava, T::aR, &["nESa"]);
    assert_has_artha_taddhita("niSA", TatraBhava, T::WaY, &["nESika"]);
    assert_has_artha_taddhita("pradoza", TatraBhava, T::aR, &["prAdoza"]);
    assert_has_artha_taddhita("pradoza", TatraBhava, T::WaY, &["prAdozika"]);
}

#[ignore]
#[test]
fn sutra_4_3_15() {
    assert_has_artha_taddhita("Svas", TatraBhava, T::WaY, &["SOvastika"]);
    assert_has_artha_taddhita("Svas", TatraBhava, T::tyap, &["Svastya"]);
    assert_has_artha_taddhita("Svas", TatraBhava, T::tyu, &["Svastana"]);
    assert_has_artha_taddhita("Svas", TatraBhava, T::tyul, &["Svastana"]);
}

#[test]
fn sutra_4_3_17() {
    assert_has_artha_taddhita("prAvfz", TatraSambhute, T::eRya, &["prAvfzeRya"]);
}

#[test]
fn sutra_4_3_18() {
    assert_has_artha_taddhita("varzA", TatraJata, T::Wak, &["vArzika"]);
}

#[test]
fn sutra_4_3_22() {
    assert_has_artha_taddhita("hemanta", TatraJata, T::aR, &["hEmana"]);
}

#[test]
fn sutra_4_3_25() {
    assert_has_artha_taddhita("sruGna", TatraJata, T::aR, &["srOGna"]);
    assert_has_artha_taddhita("maTura", TatraJata, T::aR, &["mATura"]);
    assert_has_artha_taddhita("utsa", TatraJata, T::aR, &["Otsa"]);
    assert_has_artha_taddhita("udapAna", TatraJata, T::aR, &["OdapAna"]);
    assert_has_artha_taddhita("rAzwra", TatraJata, T::Ga, &["rAzwriya"]);
    assert_has_artha_taddhita("avArapAra", TatraJata, T::Ka, &["avArapArIRa"]);
    assert_has_artha_taddhita("SAkala", TatraJata, T::WaY, &["SAkalika"]);
    assert_has_artha_taddhita("mAkala", TatraJata, T::WaY, &["mAkalika"]);
    assert_has_artha_taddhita("grAma", TatraJata, T::ya, &["grAmya"]);
    assert_has_artha_taddhita("grAma", TatraJata, T::KaY, &["grAmIRa"]);
    assert_has_artha_taddhita("katri", TatraJata, T::QakaY, &["kAtreyaka"]);
    assert_has_artha_taddhita("umBi", TatraJata, T::QakaY, &["OmBeyaka"]);
}

#[test]
fn sutra_4_3_26() {
    assert_has_artha_taddhita("prAvfz", TatraJata, T::Wap, &["prAvfzika"]);
}

#[test]
fn sutra_4_3_27() {
    assert_has_artha_taddhita("Sarad", TatraJata, T::vuY, &["SAradaka"]);
    // samjnayam?
    assert_has_artha_taddhita("Sarad", TatraJata, T::aR, &["SArada"]);
}

#[test]
fn sutra_4_3_28() {
    assert_has_artha_taddhita("pUrvAhRa", TatraJata, T::vun, &["pUrvAhRaka"]);
    assert_has_artha_taddhita("aparAhRa", TatraJata, T::vun, &["aparAhRaka"]);
    assert_has_artha_taddhita("ArdrA", TatraJata, T::vun, &["Ardraka"]);
    assert_has_artha_taddhita("mUlA", TatraJata, T::vun, &["mUlaka"]);
    assert_has_artha_taddhita("pradoza", TatraJata, T::vun, &["pradozaka"]);
    assert_has_artha_taddhita("avaskara", TatraJata, T::vun, &["avaskaraka"]);
}

#[test]
fn sutra_4_3_29() {
    assert_has_artha_taddhita("paTi", TatraJata, T::vun, &["panTaka"]);
    assert_blocked("paTi", TatraJata, T::aR);
}

#[test]
fn sutra_4_3_30() {
    assert_has_artha_taddhita("amAvAsyA", TatraJata, T::vun, &["amAvAsyaka"]);
    assert_has_artha_taddhita("amAvAsyA", TatraJata, T::aR, &["AmAvAsya"]);
}

#[test]
fn sutra_4_3_31() {
    assert_has_artha_taddhita("amAvAsyA", TatraJata, T::a, &["amAvAsya"]);
}

#[test]
fn sutra_4_3_32() {
    assert_has_artha_taddhita("sinDu", TatraJata, T::kan, &["sinDuka"]);
    assert_has_artha_taddhita("apakara", TatraJata, T::kan, &["apakaraka"]);
}

#[test]
fn sutra_4_3_33() {
    assert_has_artha_taddhita("sinDu", TatraJata, T::aR, &["sEnDava"]);
    assert_has_artha_taddhita("sinDu", TatraJata, T::aY, &["sEnDava"]);
    assert_has_artha_taddhita("apakara", TatraJata, T::aR, &["Apakara"]);
    assert_has_artha_taddhita("apakara", TatraJata, T::aY, &["Apakara"]);
}

#[test]
fn sutra_4_3_38() {
    assert_has_artha_taddhita("sruGna", TatraJata, T::aR, &["srOGna"]);
    assert_has_artha_taddhita("maTura", TatraJata, T::aR, &["mATura"]);
    assert_has_artha_taddhita("rAzwra", TatraJata, T::Ga, &["rAzwriya"]);
}

#[test]
fn sutra_4_3_39() {
    assert_has_artha_taddhita("sruGna", TatraPrayabhava, T::aR, &["srOGna"]);
    assert_has_artha_taddhita("maTura", TatraPrayabhava, T::aR, &["mATura"]);
    assert_has_artha_taddhita("rAzwra", TatraPrayabhava, T::Ga, &["rAzwriya"]);
}

#[test]
fn sutra_4_3_40() {
    assert_has_artha_taddhita("upajAnu", TatraPrayabhava, T::Wak, &["OpajAnuka"]);
    assert_has_artha_taddhita("upakarRa", TatraPrayabhava, T::Wak, &["OpakarRika"]);
    assert_has_artha_taddhita("upanIvi", TatraPrayabhava, T::Wak, &["OpanIvika"]);
}

#[test]
fn sutra_4_3_41() {
    assert_has_artha_taddhita("sruGna", TatraSambhute, T::aR, &["srOGna"]);
    assert_has_artha_taddhita("maTura", TatraSambhute, T::aR, &["mATura"]);
    assert_has_artha_taddhita("rAzwra", TatraSambhute, T::Ga, &["rAzwriya"]);
}

#[test]
fn sutra_4_3_42() {
    assert_has_artha_taddhita("koSa", TatraSambhute, T::QaY, &["kOSeya"]);
    assert_blocked("koSa", TatraSambhute, T::aR);
}

#[test]
fn sutra_4_3_45() {
    assert_has_artha_taddhita("ASvayujI", TatraSambhute, T::vuY, &["ASvayujaka"]);
    assert_blocked("ASvayujI", TatraSambhute, T::WaY);
}

#[test]
fn sutra_4_3_46() {
    assert_has_artha_taddhita("grIzma", TatraSambhute, T::vuY, &["grEzmaka"]);
    assert_has_artha_taddhita("grIzma", TatraSambhute, T::aR, &["grEzma"]);
    assert_has_artha_taddhita("vasanta", TatraSambhute, T::vuY, &["vAsantaka"]);
    assert_has_artha_taddhita("vasanta", TatraSambhute, T::aR, &["vAsanta"]);
}

#[test]
fn sutra_4_3_53() {
    assert_has_artha_taddhita("sruGna", TatraBhava, T::aR, &["srOGna"]);
    assert_has_artha_taddhita("maTura", TatraBhava, T::aR, &["mATura"]);
    assert_has_artha_taddhita("rAzwra", TatraBhava, T::Ga, &["rAzwriya"]);
}

#[test]
fn sutra_4_3_54() {
    assert_has_artha_taddhita("diS", TatraBhava, T::yat, &["diSya"]);
    assert_has_artha_taddhita("varga", TatraBhava, T::yat, &["vargya"]);
}

#[test]
fn sutra_4_3_56() {
    assert_has_artha_taddhita("dfti", TatraBhava, T::QaY, &["dArteya"]);
    assert_has_artha_taddhita("kukzi", TatraBhava, T::QaY, &["kOkzeya"]);
    assert_has_artha_taddhita("kalaSi", TatraBhava, T::QaY, &["kAlaSeya"]);
    assert_has_artha_taddhita("vasti", TatraBhava, T::QaY, &["vAsteya"]);
    assert_has_artha_taddhita("asti", TatraBhava, T::QaY, &["Asteya"]);
    assert_has_artha_taddhita("ahi", TatraBhava, T::QaY, &["Aheya"]);
}

#[test]
fn sutra_4_3_57() {
    assert_has_artha_taddhita("grIvA", TatraBhava, T::aR, &["grEva"]);
    assert_has_artha_taddhita("grIvA", TatraBhava, T::QaY, &["grEveya"]);
}

#[test]
fn sutra_4_3_58() {
    assert_has_artha_taddhita("gamBIra", TatraBhava, T::Yya, &["gAmBIrya"]);
}

#[test]
fn sutra_4_3_61() {
    assert_has_artha_taddhita("parigrAma", TatraBhava, T::WaY, &["pArigrAmika"]);
    assert_has_artha_taddhita("anugrAma", TatraBhava, T::WaY, &["AnugrAmika"]);
    assert_blocked("parigrAma", TatraBhava, T::aR);
    assert_blocked("anugrAma", TatraBhava, T::aR);
}

#[test]
fn sutra_4_3_62() {
    assert_has_artha_taddhita("jihvAmUla", TatraBhava, T::Ca, &["jihvAmUlIya"]);
    assert_has_artha_taddhita("aNguli", TatraBhava, T::Ca, &["aNgulIya"]);
    assert_blocked("jihvAmUla", TatraBhava, T::yat);
    assert_blocked("aNguli", TatraBhava, T::yat);
}

#[test]
fn sutra_4_3_63() {
    assert_has_artha_taddhita("kavarga", TatraBhava, T::Ca, &["kavargIya"]);
    assert_has_artha_taddhita("cavarga", TatraBhava, T::Ca, &["cavargIya"]);
    assert_blocked("kavarga", TatraBhava, T::aR);
    assert_blocked("cavarga", TatraBhava, T::aR);
}

#[ignore]
#[test]
fn sutra_4_3_65() {
    assert_has_artha_taddhita("karRa", TatraBhava, T::kan, &["karRaka"]);
    assert_has_artha_taddhita("lalAwa", TatraBhava, T::kan, &["lalAwaka"]);
    // TODO: others
}

#[test]
fn sutra_4_3_74() {
    assert_has_artha_taddhita("sruGna", TataAgata, T::aR, &["srOGna"]);
    assert_has_artha_taddhita("maTura", TataAgata, T::aR, &["mATura"]);
    assert_has_artha_taddhita("rAzwra", TataAgata, T::Ga, &["rAzwriya"]);
}

#[test]
fn sutra_4_3_76() {
    assert_has_artha_taddhita("SuRqika", TataAgata, T::aR, &["SORqika"]);
    assert_has_artha_taddhita("kfkaRa", TataAgata, T::aR, &["kArkaRa"]);
    assert_has_artha_taddhita("udapAna", TataAgata, T::aR, &["OdapAna"]);
}

#[test]
fn sutra_4_3_79() {
    assert_has_artha_taddhita("pitf", TataAgata, T::yat, &["pitrya"]);
    assert_has_artha_taddhita("pitf", TataAgata, T::WaY, &["pEtfka"]);
}

#[test]
fn sutra_4_3_89() {
    assert_has_artha_taddhita("sruGna", AsyaNivasa, T::aR, &["srOGna"]);
    assert_has_artha_taddhita("maTura", AsyaNivasa, T::aR, &["mATura"]);
    assert_has_artha_taddhita("rAzwra", AsyaNivasa, T::Ga, &["rAzwriya"]);
}

#[test]
fn sutra_4_3_92() {
    assert_has_artha_taddhita("SaRqika", AsyaNivasa, T::Yya, &["SARqikya"]);
    assert_has_artha_taddhita("sarvasena", AsyaNivasa, T::Yya, &["sArvasenya"]);
}

#[test]
fn sutra_4_3_93() {
    assert_has_artha_taddhita("sinDu", AsyaNivasa, T::aR, &["sEnDava"]);
    assert_has_artha_taddhita("sinDu", AsyaNivasa, T::aY, &["sEnDava"]);
    assert_has_artha_taddhita("varRu", AsyaNivasa, T::aR, &["vArRava"]);
    assert_has_artha_taddhita("varRu", AsyaNivasa, T::aY, &["vArRava"]);
}

#[test]
fn sutra_4_3_94() {
    assert_has_artha_taddhita("tUdI", AsyaNivasa, T::Qak, &["tOdeya"]);
    assert_has_artha_taddhita("SAlAtura", AsyaNivasa, T::CaR, &["SAlAturIya"]);
    assert_has_artha_taddhita("varmatI", AsyaNivasa, T::QaY, &["vArmateya"]);
    assert_has_artha_taddhita("kUcavAra", AsyaNivasa, T::yak, &["kOcavArya"]);
}

#[test]
fn sutra_4_3_98() {
    assert_has_artha_taddhita("vAsudeva", Bhakti, T::vun, &["vAsudevaka"]);
    assert_has_artha_taddhita("arjuna", Bhakti, T::vun, &["arjunaka"]);
}

#[test]
fn sutra_4_3_102() {
    assert_has_artha_taddhita("tittiri", TenaProktam, T::CaR, &["tEttirIya"]);
    assert_has_artha_taddhita("varatantu", TenaProktam, T::CaR, &["vAratantavIya"]);
    assert_has_artha_taddhita("KaRqika", TenaProktam, T::CaR, &["KARqikIya"]);
    assert_has_artha_taddhita("uKa", TenaProktam, T::CaR, &["OKIya"]);
}

#[test]
fn sutra_4_3_103() {
    assert_has_artha_taddhita("kASyapa", TenaProktam, T::Rini, &["kASyapin"]);
    assert_has_artha_taddhita("kOSika", TenaProktam, T::Rini, &["kOSikin"]);
}

#[test]
fn sutra_4_3_107() {
    assert_has_artha_taddhita("kaWa", TenaProktam, T::Rini, &["kaWa"]);
    assert_has_artha_taddhita("caraka", TenaProktam, T::aR, &["caraka"]);
}

#[ignore]
#[test]
fn sutra_4_3_108() {
    assert_has_artha_taddhita("kalApin", TenaProktam, T::aR, &["kAlApina"]);
}

#[test]
fn sutra_4_3_109() {
    assert_has_artha_taddhita("Cagalin", TenaProktam, T::Qinuk, &["CAgaleyin"]);
}

#[ignore]
#[test]
fn sutra_4_3_110() {
    assert_has_artha_taddhita("pArASarya", TenaProktam, T::Rini, &["pArASarin"]);
    assert_has_artha_taddhita("SilAlin", TenaProktam, T::Rini, &["SElAlin"]);
}

#[test]
fn sutra_4_3_111() {
    assert_has_artha_taddhita("karmanda", TenaProktam, T::ini, &["karmandin"]);
    assert_has_artha_taddhita("kfSASva", TenaProktam, T::ini, &["kfSASvin"]);
}

#[test]
fn sutra_4_3_118() {
    assert_has_artha_taddhita("kulAla", TenaKrte, T::vuY, &["kOlAlaka"]);
    assert_has_artha_taddhita("varuqa", TenaKrte, T::vuY, &["vAruqaka"]);
}

#[test]
fn sutra_4_3_119() {
    assert_has_artha_taddhita("kzudrA", TenaKrte, T::aY, &["kzOdra"]);
    assert_has_artha_taddhita("Bramara", TenaKrte, T::aY, &["BrAmara"]);
    assert_has_artha_taddhita("vawara", TenaKrte, T::aY, &["vAwara"]);
    assert_has_artha_taddhita("pAdapa", TenaKrte, T::aY, &["pAdapa"]);
}

#[test]
fn sutra_4_3_121() {
    assert_has_artha_taddhita("raTa", TasyaIdam, T::yat, &["raTya"]);
    assert_no_aR("raTa", TasyaIdam);
}

#[test]
fn sutra_4_3_123() {
    assert_has_artha_taddhita("aDvaryu", TasyaIdam, T::aY, &["ADvaryava"]);
    assert_has_artha_taddhita("parizad", TasyaIdam, T::aY, &["pArizada"]);
    assert_no_aR("aDvaryu", TasyaIdam);
    assert_no_aR("parizad", TasyaIdam);
    // TODO: others
}

#[test]
fn sutra_4_3_124() {
    assert_has_artha_taddhita("hala", TasyaIdam, T::Wak, &["hAlika"]);
    assert_has_artha_taddhita("sIra", TasyaIdam, T::Wak, &["sErika"]);
    assert_no_aR("hala", TasyaIdam);
    assert_no_aR("sIra", TasyaIdam);
}

#[test]
fn sutra_4_3_129() {
    assert_has_artha_taddhita("Candoga", TasyaIdam, T::Yya, &["CAndogya"]);
    assert_has_artha_taddhita("OkTika", TasyaIdam, T::Yya, &["OkTikya"]);
    assert_has_artha_taddhita("yAjYika", TasyaIdam, T::Yya, &["yAjYikya"]);
    assert_has_artha_taddhita("bahvfca", TasyaIdam, T::Yya, &["bAhvfcya"]);
    assert_has_artha_taddhita("nawa", TasyaIdam, T::Yya, &["nAwya"]);
    assert_no_aR("Candoga", TasyaIdam);
    assert_no_aR("OkTika", TasyaIdam);
    assert_no_aR("yAjYika", TasyaIdam);
    assert_no_aR("bahvfca", TasyaIdam);
    assert_no_aR("nawa", TasyaIdam);
}

#[test]
fn sutra_4_3_131() {
    assert_has_artha_taddhita("rEvatika", TasyaIdam, T::Ca, &["rEvatikIya"]);
    assert_has_artha_taddhita("svApiSi", TasyaIdam, T::Ca, &["svApiSIya"]);
}

#[test]
fn sutra_4_3_132() {
    assert_has_artha_taddhita("kOpiYjala", TasyaIdam, T::aR, &["kOpiYjala"]);
    assert_has_artha_taddhita("hAstipada", TasyaIdam, T::aR, &["hAstipada"]);
}

#[test]
fn sutra_4_3_135() {
    assert_has_artha_taddhita("kapota", TasyaVikara, T::aR, &["kApota"]);
    assert_has_artha_taddhita("mayUra", TasyaVikara, T::aR, &["mAyUra"]);
    assert_has_artha_taddhita("tittiri", TasyaVikara, T::aR, &["tEttira"]);
    assert_has_artha_taddhita("murva", TasyaVikara, T::aR, &["mOrva"]);
    assert_has_artha_taddhita("karIra", TasyaVikara, T::aR, &["kArIra"]);
}

#[test]
fn sutra_4_3_136() {
    assert_has_artha_taddhita("bilva", TasyaVikara, T::aR, &["bElva"]);
    assert_blocked("bilva", TasyaVikara, T::mayaw);
    // TODO: other blocks
}

#[test]
fn sutra_4_3_137() {
    assert_has_artha_taddhita("tarku", TasyaVikara, T::aR, &["tArkava"]);
    assert_has_artha_taddhita("tittiqIka", TasyaVikara, T::aR, &["tEttiqIka"]);
    assert_has_artha_taddhita("maRqUka", TasyaVikara, T::aR, &["mARqUka"]);
    assert_has_artha_taddhita("dardurUka", TasyaVikara, T::aR, &["dArdurUka"]);
    assert_has_artha_taddhita("maDuka", TasyaVikara, T::aR, &["mADuka"]);
}

#[test]
fn sutra_4_3_138() {
    assert_has_artha_taddhita("trapu", TasyaVikara, T::aR, &["trApuza"]);
    assert_has_artha_taddhita("jatu", TasyaVikara, T::aR, &["jAtuza"]);
}

#[test]
fn sutra_4_3_139() {
    assert_has_artha_taddhita("devadAru", TasyaVikara, T::aY, &["dEvadArava"]);
    assert_has_artha_taddhita("BadradAru", TasyaVikara, T::aY, &["BAdradArava"]);
}

#[test]
fn sutra_4_3_141() {
    assert_has_artha_taddhita("palASa", TasyaVikara, T::aY, &["pAlASa"]);
    assert_has_artha_taddhita("palASa", TasyaVikara, T::aR, &["pAlASa"]);
    assert_has_artha_taddhita("Kadira", TasyaVikara, T::aY, &["KAdira"]);
    assert_has_artha_taddhita("Kadira", TasyaVikara, T::aR, &["KAdira"]);
    assert_has_artha_taddhita("yavAsa", TasyaVikara, T::aY, &["yAvAsa"]);
    assert_has_artha_taddhita("yavAsa", TasyaVikara, T::aR, &["yAvAsa"]);
}

#[test]
fn sutra_4_3_142() {
    assert_has_artha_taddhita("SamI", TasyaVikara, T::wlaY, &["SAmIla"]);
}

#[test]
fn sutra_4_3_144() {
    assert_has_artha_taddhita("Amra", TasyaVikara, T::mayaw, &["Amramaya"]);
    assert_has_artha_taddhita("SAla", TasyaVikara, T::mayaw, &["SAlamaya"]);
    assert_has_artha_taddhita("SAka", TasyaVikara, T::mayaw, &["SAkamaya"]);
    // SarAdi
    assert_has_artha_taddhita("Sara", TasyaVikara, T::mayaw, &["Saramaya"]);
    assert_has_artha_taddhita("darBa", TasyaVikara, T::mayaw, &["darBamaya"]);
    assert_has_artha_taddhita("mfd", TasyaVikara, T::mayaw, &["mfnmaya"]);
}

#[test]
fn sutra_4_3_145() {
    assert_has_artha_taddhita("go", TasyaVikara, T::mayaw, &["gomaya"]);
    // purIza?
    assert_has_artha_taddhita("go", TasyaVikara, T::yat, &["gavya"]);
}

#[test]
fn sutra_4_3_146() {
    assert_has_artha_taddhita("pizwa", TasyaVikara, T::mayaw, &["pizwamaya"]);
}

#[test]
fn sutra_4_3_147() {
    assert_has_artha_taddhita("pizwa", TasyaVikara, T::kan, &["pizwaka"]);
}

#[test]
fn sutra_4_3_157() {
    assert_has_artha_taddhita("uzwra", TasyaVikara, T::vuY, &["Ozwraka"]);
}

#[test]
fn sutra_4_3_158() {
    assert_has_artha_taddhita("umA", TasyaVikara, T::vuY, &["Omaka"]);
    assert_has_artha_taddhita("umA", TasyaVikara, T::aR, &["Oma"]);
    assert_has_artha_taddhita("UrRA", TasyaVikara, T::vuY, &["OrRaka"]);
    assert_has_artha_taddhita("UrRA", TasyaVikara, T::aY, &["OrRa"]);
}

#[test]
fn sutra_4_3_159() {
    assert_has_artha_taddhita("eRI", TasyaVikara, T::QaY, &["EReya"]);
    assert_has_artha_taddhita("eRI", TasyaVikara, T::aY, &["ERa"]);
}

#[test]
fn sutra_4_3_160() {
    assert_has_artha_taddhita("go", TasyaVikara, T::yat, &["gavya"]);
    assert_has_artha_taddhita("payas", TasyaVikara, T::yat, &["payasya"]);
}

#[test]
fn sutra_4_3_161() {
    assert_has_artha_taddhita("dru", TasyaVikara, T::yat, &["dravya"]);
}

#[test]
fn sutra_4_3_162() {
    assert_has_artha_taddhita("dru", TasyaVikara, T::vaya, &["druvaya"]);
}

#[test]
fn sutra_4_3_164() {
    assert_has_artha_taddhita("plakza", TasyaVikara, T::aR, &["plAkza"]);
    assert_has_artha_taddhita("nyagroDa", TasyaVikara, T::aR, &["nEyagroDa"]);
}
