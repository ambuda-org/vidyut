extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha;
use vidyut_prakriya::args::TaddhitaArtha::*;

fn assert_blocked(text: &str, artha: TaddhitaArtha, t: T) {
    assert_has_artha_taddhita(text, artha, t, &[]);
}

#[ignore]
#[test]
fn sutra_5_3_3() {
    assert_has_taddhita("idam", T::ha, &["iha"]);
}

#[ignore]
#[test]
fn sutra_5_3_4() {
    assert_has_taddhita("idam", T::rhil, &["etarhi"]);
    assert_has_taddhita("idam", T::Tamu, &["itTam"]);
}

#[test]
fn sutra_5_3_5() {
    assert_has_taddhita("etad", T::tasil, &["atas"]);
    assert_has_taddhita("etad", T::tral, &["atra"]);
}

#[test]
fn sutra_5_3_6() {
    assert_has_taddhita("sarva", T::dA, &["sarvadA", "sadA"]);
}

#[test]
fn sutra_5_3_7() {
    assert_has_taddhita("kim", T::tasil, &["kutas"]);
    assert_has_taddhita("yad", T::tasil, &["yatas"]);
    assert_has_taddhita("tad", T::tasil, &["tatas"]);
    assert_has_taddhita("bahu", T::tasil, &["bahutas"]);
}

#[test]
fn sutra_5_3_9() {
    assert_has_taddhita("pari", T::tasil, &["paritas"]);
    assert_has_taddhita("aBi", T::tasil, &["aBitas"]);
}

#[test]
fn sutra_5_3_10() {
    assert_has_taddhita("kim", T::tral, &["kutra"]);
    assert_has_taddhita("yad", T::tral, &["yatra"]);
    assert_has_taddhita("tad", T::tral, &["tatra"]);
    assert_has_taddhita("bahu", T::tral, &["bahutra"]);
    // But, not for idam
    assert_has_taddhita("idam", T::tral, &[]);
}

#[ignore]
#[test]
fn sutra_5_3_11() {
    assert_has_taddhita("idam", T::ha, &["iha"]);
}

#[test]
fn sutra_5_3_12() {
    assert_has_taddhita("kim", T::at, &["kva"]);
}

#[test]
fn sutra_5_3_13() {
    assert_has_taddhita("kim", T::ha, &["kuha"]);
}

#[test]
fn sutra_5_3_15() {
    assert_has_taddhita("sarva", T::dA, &["sarvadA", "sadA"]);
    assert_has_taddhita("eka", T::dA, &["ekadA"]);
    assert_has_taddhita("anya", T::dA, &["anyadA"]);
    assert_has_taddhita("kim", T::dA, &["kadA"]);
    assert_has_taddhita("yad", T::dA, &["yadA"]);
    assert_has_taddhita("tad", T::dA, &["tadA"]);
    assert_has_taddhita("idam", T::dA, &[]);
}

#[test]
fn sutra_5_3_16() {
    assert_has_taddhita("idam", T::rhil, &["etarhi"]);
}

#[ignore]
#[test]
fn sutra_5_3_18() {
    assert_has_taddhita("idam", T::dAnIm, &["idAnIm"]);
}

#[test]
fn sutra_5_3_19() {
    assert_has_taddhita("tad", T::dAnIm, &["tadAnIm"]);
}

#[ignore]
#[test]
fn sutra_5_3_23() {
    assert_has_taddhita("idam", T::Tamu, &["itTam"]);
}

#[test]
fn sutra_5_3_24() {
    assert_has_taddhita("kim", T::Tamu, &["kaTam"]);
}

#[test]
fn sutra_5_3_25() {
    assert_has_taddhita("tad", T::TAl, &["taTA"]);
}

#[test]
fn sutra_5_3_28() {
    assert_has_artha_taddhita("dakziRa", DigDeshaKala, T::atasuc, &["dakziRatas"]);
    assert_has_artha_taddhita("uttara", DigDeshaKala, T::atasuc, &["uttaratas"]);
}

#[test]
fn sutra_5_3_29() {
    assert_has_artha_taddhita("para", DigDeshaKala, T::atasuc, &["paratas"]);
    assert_has_artha_taddhita("para", DigDeshaKala, T::astAti, &["parastAt"]);
    assert_has_artha_taddhita("avara", DigDeshaKala, T::atasuc, &["avaratas"]);
    assert_has_artha_taddhita("avara", DigDeshaKala, T::astAti, &["avarastAt"]);
}

#[test]
fn sutra_5_3_42() {
    // EkaDyam is from 5.3.44.
    assert_has_taddhita("eka", T::DA, &["ekaDA", "EkaDyam"]);
    // dvEDam/trEDam are from 5.3.45.
    // dveDA/treDA are from 5.3.46.
    assert_has_taddhita("dvi", T::DA, &["dviDA", "dvEDam", "dveDA"]);
    assert_has_taddhita("tri", T::DA, &["triDA", "trEDam", "treDA"]);
    assert_has_taddhita("catur", T::DA, &["caturDA"]);
    assert_has_taddhita("paYcan", T::DA, &["paYcaDA"]);
}

#[test]
fn sutra_5_3_44() {
    assert_has_taddhita("eka", T::DA, &["ekaDA", "EkaDyam"]);
}

#[test]
fn sutra_5_3_45_to_sutra_5_3_56() {
    assert_has_taddhita("dvi", T::DA, &["dviDA", "dvEDam", "dveDA"]);
    assert_has_taddhita("tri", T::DA, &["triDA", "trEDam", "treDA"]);
}

#[test]
fn sutra_5_3_47() {
    assert_has_taddhita("vEyAkaraRa", T::pASap, &["vEyAkaraRapASa"]);
    assert_has_taddhita("yAjYika", T::pASap, &["yAjYikapASa"]);
}

#[test]
fn sutra_5_3_52() {
    assert_has_taddhita("eka", T::Akinic, &["ekAkin"]);
}

#[test]
fn sutra_5_3_53() {
    assert_has_taddhita("AQya", T::caraw, &["AQyacara"]);
    assert_has_taddhita("sukumAra", T::caraw, &["sukumAracara"]);

    let adhyacara = create_taddhitanta("AQyacara", "AQya", T::caraw);
    assert_has_sup_1s(&adhyacara, Stri, &["AQyacarI"]);
}

#[test]
fn sutra_5_3_54() {
    assert_has_taddhita("devadatta", T::rUpya, &["devadattarUpya"]);
    assert_has_taddhita("devadatta", T::caraw, &["devadattacara"]);
}

#[test]
fn sutra_5_3_55() {
    assert_has_taddhita("AQya", T::tamap, &["AQyatama"]);
    assert_has_taddhita("darSanIya", T::tamap, &["darSanIyatama"]);
    assert_has_taddhita("sukumAra", T::tamap, &["sukumAratama"]);
    assert_has_taddhita("pawu", T::izWan, &["pawizWa"]);
    assert_has_taddhita("laGu", T::izWan, &["laGizWa"]);
    assert_has_taddhita("guru", T::izWan, &["garizWa"]);
    assert_has_taddhita("SrezWa", T::tamap, &["SrezWatama"]);
}

#[test]
fn sutra_5_3_57() {
    assert_has_taddhita("AQya", T::tarap, &["AQyatara"]);
    assert_has_taddhita("sukumAra", T::tarap, &["sukumAratara"]);
    assert_has_taddhita("pawu", T::Iyasun, &["pawIyas"]);
    // TODO: others
}

#[test]
fn sutra_5_3_60_to_sutra_5_3_61() {
    assert_has_taddhita("praSasya", T::Iyasun, &["Sreyas", "jyAyas"]);
    assert_has_taddhita("praSasya", T::izWan, &["SrezWa", "jyezWa"]);
}

#[test]
fn sutra_5_3_62() {
    assert_has_taddhita("vfdDa", T::Iyasun, &["jyAyas", "varzIyas"]);
    assert_has_taddhita("vfdDa", T::izWan, &["jyezWa", "varzizWa"]);
}

#[test]
fn sutra_5_3_63() {
    assert_has_taddhita("antika", T::izWan, &["nedizWa"]);
    assert_has_taddhita("antika", T::Iyasun, &["nedIyas"]);
    assert_has_taddhita("bAQa", T::izWan, &["sADizWa"]);
    assert_has_taddhita("bAQa", T::Iyasun, &["sADIyas"]);
}

#[test]
fn sutra_5_3_64() {
    assert_has_taddhita("yuvan", T::izWan, &["kanizWa", "yavizWa"]);
    assert_has_taddhita("yuvan", T::Iyasun, &["kanIyas", "yavIyas"]);
    assert_has_taddhita("alpa", T::izWan, &["kanizWa", "alpizWa"]);
    assert_has_taddhita("alpa", T::Iyasun, &["kanIyas", "alpIyas"]);
}

#[test]
fn sutra_5_3_66() {
    assert_has_taddhita("vEyAkaraRa", T::rUpap, &["vEyAkaraRarUpa"]);
    assert_has_taddhita("yAjYika", T::rUpap, &["yAjYikarUpa"]);
    assert_has_taddhita("cora", T::rUpap, &["corarUpa"]);
    assert_has_taddhita("dasyu", T::rUpap, &["dasyurUpa"]);
    // TODO: pacatirUpam, etc.
}

#[test]
fn sutra_5_3_67() {
    assert_has_taddhita("pawu", T::kalpap, &["pawukalpa"]);
    assert_has_taddhita("pawu", T::deSya, &["pawudeSya"]);
    assert_has_taddhita("pawu", T::deSIyar, &["pawudeSIya"]);

    assert_has_taddhita("mfdu", T::kalpap, &["mfdukalpa"]);
    assert_has_taddhita("mfdu", T::deSya, &["mfdudeSya"]);
    assert_has_taddhita("mfdu", T::deSIyar, &["mfdudeSIya"]);
    // TODO: pacatikalpam, etc.
}

#[test]
fn sutra_5_3_68() {
    assert_has_taddhita("pawu", T::bahuc, &["bahupawu"]);
    assert_has_taddhita("mfdu", T::bahuc, &["bahumfdu"]);
    assert_has_taddhita("guqa", T::bahuc, &["bahuguqa"]);
}

#[test]
fn sutra_5_3_69() {
    assert_has_taddhita("pawu", T::jAtIyar, &["pawujAtIya"]);
    assert_has_taddhita("mfdu", T::jAtIyar, &["mfdujAtIya"]);
    assert_has_taddhita("darSanIya", T::jAtIyar, &["darSanIyajAtIya"]);
}

#[test]
fn sutra_5_3_70() {
    assert_has_artha_taddhita("aSva", Ajnate, T::ka, &["aSvaka"]);
    assert_has_artha_taddhita("gardaBa", Ajnate, T::ka, &["gardaBaka"]);
}

#[test]
fn sutra_5_3_75() {
    assert_has_artha_taddhita("SUdra", Kutsite, T::kan, &["SUdraka"]);
    assert_has_artha_taddhita("DAra", Kutsite, T::kan, &["DAraka"]);
    assert_has_artha_taddhita("pUrRa", Kutsite, T::kan, &["pUrRaka"]);
}

#[test]
fn sutra_5_3_76() {
    assert_has_artha_taddhita("putra", Anukampayam, T::ka, &["putraka"]);
    assert_has_artha_taddhita("vatsa", Anukampayam, T::ka, &["vatsaka"]);
    assert_has_artha_taddhita("durbala", Anukampayam, T::ka, &["durbalaka"]);
}

#[test]
fn sutra_5_3_86() {
    assert_has_artha_taddhita("vfkza", Hrasve, T::ka, &["vfkzaka"]);
    assert_has_artha_taddhita("plakza", Hrasve, T::ka, &["plakzaka"]);
    assert_has_artha_taddhita("stamBa", Hrasve, T::ka, &["stamBaka"]);
}

#[test]
fn sutra_5_3_87() {
    assert_has_artha_taddhita("vaMSa", Hrasve, T::ka, &["vaMSaka"]);
    assert_has_artha_taddhita("veRu", Hrasve, T::ka, &["veRuka"]);
    assert_has_artha_taddhita("daRqa", Hrasve, T::ka, &["daRqaka"]);
}

#[test]
fn sutra_5_3_88() {
    assert_has_artha_taddhita("kuwI", Hrasve, T::ra, &["kuwIra"]);
    assert_has_artha_taddhita("SamI", Hrasve, T::ra, &["SamIra"]);
    assert_has_artha_taddhita("SuRqA", Hrasve, T::ra, &["SuRqAra"]);
    assert_blocked("kuwI", Hrasve, T::ka);
}

#[test]
fn sutra_5_3_89() {
    assert_has_artha_taddhita("kutU", Hrasve, T::qupac, &["kutupa"]);
    assert_blocked("kutU", Hrasve, T::ka);
}

#[test]
fn sutra_5_3_90() {
    let kasutara = create_artha_taddhita("kAsUtara", "kAsU", Hrasve, T::zwarac);
    let gonitara = create_artha_taddhita("goRItara", "goRI", Hrasve, T::zwarac);
    assert_has_sup_1s(&kasutara, Stri, &["kAsUtarI"]);
    assert_has_sup_1s(&gonitara, Stri, &["goRItarI"]);
    assert_blocked("kAsU", Hrasve, T::ka);
}

#[test]
fn sutra_5_3_91() {
    assert_has_artha_taddhita("vatsa", Tanutve, T::zwarac, &["vatsatara"]);
    assert_has_artha_taddhita("ukzan", Tanutve, T::zwarac, &["ukzatara"]);
    assert_has_artha_taddhita("aSva", Tanutve, T::zwarac, &["aSvatara"]);
    assert_has_artha_taddhita("fzaBa", Tanutve, T::zwarac, &["fzaBatara"]);
}

#[test]
fn sutra_5_3_92() {
    assert_has_artha_taddhita("kim", DvayorEka, T::qatarac, &["katara"]);
    assert_has_artha_taddhita("yad", DvayorEka, T::qatarac, &["yatara"]);
    assert_has_artha_taddhita("tad", DvayorEka, T::qatarac, &["tatara"]);
}

#[test]
fn sutra_5_3_93() {
    assert_has_artha_taddhita("kim", BahunamEka, T::qatamac, &["katama"]);
    assert_has_artha_taddhita("yad", BahunamEka, T::qatamac, &["yatama"]);
    assert_has_artha_taddhita("tad", BahunamEka, T::qatamac, &["tatama"]);
}

#[ignore]
#[test]
fn sutra_5_3_94() {
    assert_has_artha_taddhita("eka", DvayorEka, T::qatarac, &["ekatara"]);
    assert_has_artha_taddhita("eka", BahunamEka, T::qatamac, &["ekatama"]);
}

#[test]
fn sutra_5_3_96() {
    assert_has_artha_taddhita("aSva", IvePratikrtau, T::kan, &["aSvaka"]);
    assert_has_artha_taddhita("uzwra", IvePratikrtau, T::kan, &["uzwraka"]);
    assert_has_artha_taddhita("gardaBa", IvePratikrtau, T::kan, &["gardaBaka"]);
}

#[test]
fn sutra_5_3_101() {
    let vasteya = create_artha_taddhita("vAsteya", "vasti", IvePratikrtau, T::QaY);
    assert_has_sup_1s(&vasteya, Pum, &["vAsteyaH"]);
    assert_has_sup_1s(&vasteya, Stri, &["vAsteyI"]);
}

#[test]
fn sutra_5_3_103() {
    assert_has_artha_taddhita("SAKA", IvePratikrtau, T::yat, &["SAKya"]);
    assert_has_artha_taddhita("muKa", IvePratikrtau, T::yat, &["muKya"]);
    assert_has_artha_taddhita("jaGana", IvePratikrtau, T::yat, &["jaGanya"]);
}

#[test]
fn sutra_5_3_105() {
    assert_has_artha_taddhita("kuSAgra", IvePratikrtau, T::Ca, &["kuSAgrIya"]);
}

#[test]
fn sutra_5_3_107() {
    assert_has_artha_taddhita("SarkarA", IvePratikrtau, T::aR, &["SArkara"]);
    assert_has_artha_taddhita("kapAlikA", IvePratikrtau, T::aR, &["kApAlika"]);
}

#[test]
fn sutra_5_3_108() {
    assert_has_artha_taddhita("aNguli", IvePratikrtau, T::Wak, &["ANgulika"]);
    assert_has_artha_taddhita("Baruja", IvePratikrtau, T::Wak, &["BArujika"]);
}

#[test]
fn sutra_5_3_109() {
    assert_has_artha_taddhita("ekaSAlA", IvePratikrtau, T::Wac, &["ekaSAlika"]);
    assert_has_artha_taddhita("ekaSAlA", IvePratikrtau, T::Wak, &["EkaSAlika"]);
}

#[test]
fn sutra_5_3_110() {
    assert_has_artha_taddhita("karka", IvePratikrtau, T::Ikak, &["kArkIka"]);
    assert_has_artha_taddhita("lohita", IvePratikrtau, T::Ikak, &["lOhitIka"]);
}

#[ignore]
#[test]
fn sutra_5_3_115() {
    let artha = AyudhaJiviSangha;
    let varkenya = create_artha_taddhita("vArkeRya", "vfka", artha, T::weRyaR);
    assert_has_sup_1s(&varkenya, Pum, &["vArkeRyaH"]);
    assert_has_sup_1d(&varkenya, Pum, &["vArkeRyO"]);
    assert_has_sup_1p(&varkenya, Pum, &["vfkAH"]);
}
