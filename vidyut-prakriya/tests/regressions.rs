/*
Tests to verify that a regression has been fixed.

These tests catch gaps in our test suite based on the Kashika-vrtti. So far, these tests come from
the Siddhanta-kaumudi.
*/
extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Vacana::*;

#[ignore]
#[test]
fn sk_2308() {
    // Udit dhatus with Tal-pratyaya
    let gupu = d("gupU~", Bhvadi);
    assert_has_tinanta(&[], &gupu, Lit, Madhyama, Eka, &["jugopTa"]);
    assert_has_tinanta(&[], &gupu, Lit, Uttama, Dvi, &["jugupva"]);
    assert_has_tinanta(&[], &gupu, Lit, Uttama, Bahu, &["jugupma"]);
}

#[test]
fn sk_2397() {
    // akAravAn dhatus with Tal-pratyaya
    let skand = d("ska\\ndi~r", Bhvadi);
    assert_has_sip(&[], &skand, Lit, &["caskandiTa", "caskanTa", "caskantTa"]);
}

#[test]
fn sk_2447() {
    let urnu = d("UrRuY", Adadi);
    assert_has_sip(&[], &urnu, Lit, &["UrRunaviTa", "UrRunuviTa"]);
}

#[test]
fn sk_2574() {
    // ac-Adi dhatus with can-pratyaya and ak-lopa
    // TODO: why does the SK have AndaDat and not AndiDat?
    assert_has_lun_p(&[], &d("anDa", Curadi), &["AndaDat"]);
    assert_has_lun_p(&[], &d("anka", Curadi), &["AYcakat"]);
    assert_has_lun_p(&[], &d("anga", Curadi), &["AYjagat"]);
    assert_has_lun_p(&[], &d("vyaya", Curadi), &["avavyayat"]);
}

#[test]
fn sk_2630() {
    assert_has_lit(
        &[],
        &yan(&d("BU", Bhvadi)),
        &["boBUyAYcakre", "boBUyAmAsa", "boBUyAmbaBUva"],
    );
}

#[test]
fn sk_2758() {
    assert_has_lit(
        &[],
        &san(&d("BU", Bhvadi)),
        &["buBUzAYcakAra", "buBUzAmAsa", "buBUzAmbaBUva"],
    );
}
