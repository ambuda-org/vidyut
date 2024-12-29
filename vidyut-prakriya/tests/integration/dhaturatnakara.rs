//! Supplemental tests for cases not covered in the KV or SK.

extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn drk_ksham_yan_luk() {
    let ksham = yan_luk(&d("kzamU~\\z", Bhvadi));
    assert_has_tip(&[], &ksham, Lat, &["caNkzamIti", "caNkzanti"]);
    assert_has_tas(&[], &ksham, Lat, &["caNkzAntaH"]);
    assert_has_jhi(&[], &ksham, Lat, &["caNkzamati"]);
    assert_has_sip(&[], &ksham, Lat, &["caNkzamIzi", "caNkzaMsi"]);
    assert_has_thas(&[], &ksham, Lat, &["caNkzAnTaH"]);
    assert_has_tha(&[], &ksham, Lat, &["caNkzAnTa"]);
    // DRK has typo here -- Ratva is correct.
    assert_has_mip(&[], &ksham, Lat, &["caNkzamImi", "caNkzaRmi"]);
    assert_has_vas(&[], &ksham, Lat, &["caNkzaRvaH"]);
    assert_has_mas(&[], &ksham, Lat, &["caNkzaRmaH"]);
}
