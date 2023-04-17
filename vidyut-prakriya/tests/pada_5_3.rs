extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

fn prati(text: &str) -> Pratipadika {
    Pratipadika::builder().text(text).build().unwrap()
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
