extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;

#[ignore]
#[test]
fn sutra_5_3_3() {
    assert_has_taddhitanta("idam", T::ha, &["iha"]);
}

#[ignore]
#[test]
fn sutra_5_3_4() {
    assert_has_taddhitanta("idam", T::rhil, &["etarhi"]);
    assert_has_taddhitanta("idam", T::Tamu, &["itTam"]);
}

#[test]
fn sutra_5_3_5() {
    assert_has_taddhitanta("etad", T::tasil, &["atas"]);
    assert_has_taddhitanta("etad", T::tral, &["atra"]);
}

#[test]
fn sutra_5_3_6() {
    assert_has_taddhitanta("sarva", T::dA, &["sarvadA", "sadA"]);
}

#[test]
fn sutra_5_3_7() {
    assert_has_taddhitanta("kim", T::tasil, &["kutas"]);
    assert_has_taddhitanta("yad", T::tasil, &["yatas"]);
    assert_has_taddhitanta("tad", T::tasil, &["tatas"]);
    assert_has_taddhitanta("bahu", T::tasil, &["bahutas"]);
}

#[test]
fn sutra_5_3_9() {
    assert_has_taddhitanta("pari", T::tasil, &["paritas"]);
    assert_has_taddhitanta("aBi", T::tasil, &["aBitas"]);
}

#[test]
fn sutra_5_3_10() {
    assert_has_taddhitanta("kim", T::tral, &["kutra"]);
    assert_has_taddhitanta("yad", T::tral, &["yatra"]);
    assert_has_taddhitanta("tad", T::tral, &["tatra"]);
    assert_has_taddhitanta("bahu", T::tral, &["bahutra"]);
    // But, not for idam
    assert_has_taddhitanta("idam", T::tral, &[]);
}

#[ignore]
#[test]
fn sutra_5_3_11() {
    assert_has_taddhitanta("idam", T::ha, &["iha"]);
}

#[test]
fn sutra_5_3_12() {
    assert_has_taddhitanta("kim", T::at, &["kva"]);
}

#[test]
fn sutra_5_3_13() {
    assert_has_taddhitanta("kim", T::ha, &["kuha"]);
}

#[test]
fn sutra_5_3_15() {
    assert_has_taddhitanta("sarva", T::dA, &["sarvadA", "sadA"]);
    assert_has_taddhitanta("eka", T::dA, &["ekadA"]);
    assert_has_taddhitanta("anya", T::dA, &["anyadA"]);
    assert_has_taddhitanta("kim", T::dA, &["kadA"]);
    assert_has_taddhitanta("yad", T::dA, &["yadA"]);
    assert_has_taddhitanta("tad", T::dA, &["tadA"]);
    assert_has_taddhitanta("idam", T::dA, &[]);
}

#[test]
fn sutra_5_3_16() {
    assert_has_taddhitanta("idam", T::rhil, &["etarhi"]);
}

#[ignore]
#[test]
fn sutra_5_3_18() {
    assert_has_taddhitanta("idam", T::dAnIm, &["idAnIm"]);
}

#[test]
fn sutra_5_3_19() {
    assert_has_taddhitanta("tad", T::dAnIm, &["tadAnIm"]);
}

#[ignore]
#[test]
fn sutra_5_3_23() {
    assert_has_taddhitanta("idam", T::Tamu, &["itTam"]);
}

#[test]
fn sutra_5_3_24() {
    assert_has_taddhitanta("kim", T::Tamu, &["kaTam"]);
}

#[test]
fn sutra_5_3_25() {
    assert_has_taddhitanta("tad", T::TAl, &["taTA"]);
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
    assert_has_taddhitanta("eka", T::DA, &["ekaDA", "EkaDyam"]);
    // dvEDam/trEDam are from 5.3.45.
    // dveDA/treDA are from 5.3.46.
    assert_has_taddhitanta("dvi", T::DA, &["dviDA", "dvEDam", "dveDA"]);
    assert_has_taddhitanta("tri", T::DA, &["triDA", "trEDam", "treDA"]);
    assert_has_taddhitanta("catur", T::DA, &["caturDA"]);
    assert_has_taddhitanta("paYcan", T::DA, &["paYcaDA"]);
}

#[test]
fn sutra_5_3_44() {
    assert_has_taddhitanta("eka", T::DA, &["ekaDA", "EkaDyam"]);
}

#[test]
fn sutra_5_3_45_to_sutra_5_3_56() {
    assert_has_taddhitanta("dvi", T::DA, &["dviDA", "dvEDam", "dveDA"]);
    assert_has_taddhitanta("tri", T::DA, &["triDA", "trEDam", "treDA"]);
}

#[test]
fn sutra_5_3_47() {
    assert_has_taddhitanta("vEyAkaraRa", T::pASap, &["vEyAkaraRapASa"]);
    assert_has_taddhitanta("yAjYika", T::pASap, &["yAjYikapASa"]);
}

#[test]
fn sutra_5_3_52() {
    assert_has_taddhitanta("eka", T::Akinic, &["ekAkin"]);
}

#[test]
fn sutra_5_3_53() {
    assert_has_taddhitanta("AQya", T::caraw, &["AQyacara"]);
    assert_has_taddhitanta("sukumAra", T::caraw, &["sukumAracara"]);

    let adhyacara = create_taddhitanta("AQyacara", "AQya", T::caraw);
    assert_has_sup_1s(&adhyacara, Stri, &["AQyacarI"]);
}

#[test]
fn sutra_5_3_54() {
    assert_has_taddhitanta("devadatta", T::rUpya, &["devadattarUpya"]);
    assert_has_taddhitanta("devadatta", T::caraw, &["devadattacara"]);
}

#[test]
fn sutra_5_3_55() {
    assert_has_taddhitanta("AQya", T::tamap, &["AQyatama"]);
    assert_has_taddhitanta("darSanIya", T::tamap, &["darSanIyatama"]);
    assert_has_taddhitanta("sukumAra", T::tamap, &["sukumAratama"]);
    assert_has_taddhitanta("pawu", T::izWan, &["pawizWa"]);
    assert_has_taddhitanta("laGu", T::izWan, &["laGizWa"]);
    assert_has_taddhitanta("guru", T::izWan, &["garizWa"]);
    assert_has_taddhitanta("SrezWa", T::tamap, &["SrezWatama"]);
}

#[test]
fn sutra_5_3_57() {
    assert_has_taddhitanta("AQya", T::tarap, &["AQyatara"]);
    assert_has_taddhitanta("sukumAra", T::tarap, &["sukumAratara"]);
    assert_has_taddhitanta("pawu", T::Iyasun, &["pawIyas"]);
    // TODO: others
}

#[test]
fn sutra_5_3_60_to_sutra_5_3_61() {
    assert_has_taddhitanta("praSasya", T::Iyasun, &["Sreyas", "jyAyas"]);
    assert_has_taddhitanta("praSasya", T::izWan, &["SrezWa", "jyezWa"]);
}

#[test]
fn sutra_5_3_62() {
    assert_has_taddhitanta("vfdDa", T::Iyasun, &["jyAyas", "varzIyas"]);
    assert_has_taddhitanta("vfdDa", T::izWan, &["jyezWa", "varzizWa"]);
}

#[test]
fn sutra_5_3_63() {
    assert_has_taddhitanta("antika", T::izWan, &["nedizWa"]);
    assert_has_taddhitanta("antika", T::Iyasun, &["nedIyas"]);
    assert_has_taddhitanta("bAQa", T::izWan, &["sADizWa"]);
    assert_has_taddhitanta("bAQa", T::Iyasun, &["sADIyas"]);
}

#[test]
fn sutra_5_3_64() {
    assert_has_taddhitanta("yuvan", T::izWan, &["kanizWa", "yavizWa"]);
    assert_has_taddhitanta("yuvan", T::Iyasun, &["kanIyas", "yavIyas"]);
    assert_has_taddhitanta("alpa", T::izWan, &["kanizWa", "alpizWa"]);
    assert_has_taddhitanta("alpa", T::Iyasun, &["kanIyas", "alpIyas"]);
}

#[test]
fn sutra_5_3_66() {
    assert_has_taddhitanta("vEyAkaraRa", T::rUpap, &["vEyAkaraRarUpa"]);
    assert_has_taddhitanta("yAjYika", T::rUpap, &["yAjYikarUpa"]);
    assert_has_taddhitanta("cora", T::rUpap, &["corarUpa"]);
    assert_has_taddhitanta("dasyu", T::rUpap, &["dasyurUpa"]);
    // TODO: pacatirUpam, etc.
}

#[test]
fn sutra_5_3_67() {
    assert_has_taddhitanta("pawu", T::kalpap, &["pawukalpa"]);
    assert_has_taddhitanta("pawu", T::deSya, &["pawudeSya"]);
    assert_has_taddhitanta("pawu", T::deSIyar, &["pawudeSIya"]);

    assert_has_taddhitanta("mfdu", T::kalpap, &["mfdukalpa"]);
    assert_has_taddhitanta("mfdu", T::deSya, &["mfdudeSya"]);
    assert_has_taddhitanta("mfdu", T::deSIyar, &["mfdudeSIya"]);
    // TODO: pacatikalpam, etc.
}

#[test]
fn sutra_5_3_68() {
    assert_has_taddhitanta("pawu", T::bahuc, &["bahupawu"]);
    assert_has_taddhitanta("mfdu", T::bahuc, &["bahumfdu"]);
    assert_has_taddhitanta("guqa", T::bahuc, &["bahuguqa"]);
}

#[test]
fn sutra_5_3_69() {
    assert_has_taddhitanta("pawu", T::jAtIyar, &["pawujAtIya"]);
    assert_has_taddhitanta("mfdu", T::jAtIyar, &["mfdujAtIya"]);
    assert_has_taddhitanta("darSanIya", T::jAtIyar, &["darSanIyajAtIya"]);
}
