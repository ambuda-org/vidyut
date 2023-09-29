extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Artha::*;
use vidyut_prakriya::args::Taddhita as T;

#[ignore]
#[test]
fn sutra_5_3_3() {
    assert_has_taddhitanta(&prati("idam"), T::ha, &["iha"]);
}

#[ignore]
#[test]
fn sutra_5_3_4() {
    assert_has_taddhitanta(&prati("idam"), T::rhil, &["etarhi"]);
    assert_has_taddhitanta(&prati("idam"), T::Tamu, &["itTam"]);
}

#[test]
fn sutra_5_3_5() {
    assert_has_taddhitanta(&prati("etad"), T::tasil, &["atas"]);
    assert_has_taddhitanta(&prati("etad"), T::tral, &["atra"]);
}

#[test]
fn sutra_5_3_6() {
    assert_has_taddhitanta(&prati("sarva"), T::dA, &["sarvadA", "sadA"]);
}

#[test]
fn sutra_5_3_7() {
    assert_has_taddhitanta(&prati("kim"), T::tasil, &["kutas"]);
    assert_has_taddhitanta(&prati("yad"), T::tasil, &["yatas"]);
    assert_has_taddhitanta(&prati("tad"), T::tasil, &["tatas"]);
    assert_has_taddhitanta(&prati("bahu"), T::tasil, &["bahutas"]);
}

#[test]
fn sutra_5_3_9() {
    assert_has_taddhitanta(&prati("pari"), T::tasil, &["paritas"]);
    assert_has_taddhitanta(&prati("aBi"), T::tasil, &["aBitas"]);
}

#[test]
fn sutra_5_3_10() {
    assert_has_taddhitanta(&prati("kim"), T::tral, &["kutra"]);
    assert_has_taddhitanta(&prati("yad"), T::tral, &["yatra"]);
    assert_has_taddhitanta(&prati("tad"), T::tral, &["tatra"]);
    assert_has_taddhitanta(&prati("bahu"), T::tral, &["bahutra"]);
    // But, not for idam
    assert_has_taddhitanta(&prati("idam"), T::tral, &[]);
}

#[ignore]
#[test]
fn sutra_5_3_11() {
    assert_has_taddhitanta(&prati("idam"), T::ha, &["iha"]);
}

#[test]
fn sutra_5_3_12() {
    assert_has_taddhitanta(&prati("kim"), T::at, &["kva"]);
}

#[test]
fn sutra_5_3_13() {
    assert_has_taddhitanta(&prati("kim"), T::ha, &["kuha"]);
}

#[test]
fn sutra_5_3_15() {
    assert_has_taddhitanta(&prati("sarva"), T::dA, &["sarvadA", "sadA"]);
    assert_has_taddhitanta(&prati("eka"), T::dA, &["ekadA"]);
    assert_has_taddhitanta(&prati("anya"), T::dA, &["anyadA"]);
    assert_has_taddhitanta(&prati("kim"), T::dA, &["kadA"]);
    assert_has_taddhitanta(&prati("yad"), T::dA, &["yadA"]);
    assert_has_taddhitanta(&prati("tad"), T::dA, &["tadA"]);
    assert_has_taddhitanta(&prati("idam"), T::dA, &[]);
}

#[test]
fn sutra_5_3_16() {
    assert_has_taddhitanta(&prati("idam"), T::rhil, &["etarhi"]);
}

#[ignore]
#[test]
fn sutra_5_3_18() {
    assert_has_taddhitanta(&prati("idam"), T::dAnIm, &["idAnIm"]);
}

#[test]
fn sutra_5_3_19() {
    assert_has_taddhitanta(&prati("tad"), T::dAnIm, &["tadAnIm"]);
}

#[ignore]
#[test]
fn sutra_5_3_23() {
    assert_has_taddhitanta(&prati("idam"), T::Tamu, &["itTam"]);
}

#[test]
fn sutra_5_3_24() {
    assert_has_taddhitanta(&prati("kim"), T::Tamu, &["kaTam"]);
}

#[test]
fn sutra_5_3_25() {
    assert_has_taddhitanta(&prati("tad"), T::TAl, &["taTA"]);
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
    assert_has_taddhitanta(&prati("eka"), T::DA, &["ekaDA"]);
    assert_has_taddhitanta(&prati("dvi"), T::DA, &["dviDA"]);
    assert_has_taddhitanta(&prati("tri"), T::DA, &["triDA"]);
    assert_has_taddhitanta(&prati("catur"), T::DA, &["caturDA"]);
    assert_has_taddhitanta(&prati("paYcan"), T::DA, &["paYcaDA"]);
}

#[test]
fn sutra_5_3_55() {
    assert_has_taddhitanta(&prati("AQya"), T::tamap, &["AQyatama"]);
    assert_has_taddhitanta(&prati("darSanIya"), T::tamap, &["darSanIyatama"]);
    assert_has_taddhitanta(&prati("sukumAra"), T::tamap, &["sukumAratama"]);
    assert_has_taddhitanta(&prati("pawu"), T::izWan, &["pawizWa"]);
    assert_has_taddhitanta(&prati("laGu"), T::izWan, &["laGizWa"]);
    assert_has_taddhitanta(&prati("guru"), T::izWan, &["garizWa"]);
    assert_has_taddhitanta(&prati("SrezWa"), T::tamap, &["SrezWatama"]);
}

#[test]
fn sutra_5_3_57() {
    assert_has_taddhitanta(&prati("AQya"), T::tarap, &["AQyatara"]);
    assert_has_taddhitanta(&prati("sukumAra"), T::tarap, &["sukumAratara"]);
    assert_has_taddhitanta(&prati("pawu"), T::Iyasun, &["pawIyas"]);
    // TODO: others
}

#[test]
fn sutra_5_3_66() {
    assert_has_taddhitanta(&prati("vEyAkaraRa"), T::rUpap, &["vEyAkaraRarUpa"]);
    assert_has_taddhitanta(&prati("yAjYika"), T::rUpap, &["yAjYikarUpa"]);
    assert_has_taddhitanta(&prati("cora"), T::rUpap, &["corarUpa"]);
    assert_has_taddhitanta(&prati("dasyu"), T::rUpap, &["dasyurUpa"]);
    // TODO: pacatirUpam, etc.
}

#[test]
fn sutra_5_3_67() {
    assert_has_taddhitanta(&prati("pawu"), T::kalpap, &["pawukalpa"]);
    assert_has_taddhitanta(&prati("pawu"), T::deSya, &["pawudeSya"]);
    assert_has_taddhitanta(&prati("pawu"), T::deSIyar, &["pawudeSIya"]);

    assert_has_taddhitanta(&prati("mfdu"), T::kalpap, &["mfdukalpa"]);
    assert_has_taddhitanta(&prati("mfdu"), T::deSya, &["mfdudeSya"]);
    assert_has_taddhitanta(&prati("mfdu"), T::deSIyar, &["mfdudeSIya"]);
    // TODO: pacatikalpam, etc.
}
