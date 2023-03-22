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
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

#[ignore]
#[test]
fn sk_2308() {
    // Udit dhatus with Tal-pratyaya
    let gupu = d("gupU~", Bhvadi);
    assert_has_parasmai_tinanta(&[], &gupu, Lit, Madhyama, Eka, &["jugopTa"]);
    assert_has_parasmai_tinanta(&[], &gupu, Lit, Uttama, Dvi, &["jugupva"]);
    assert_has_parasmai_tinanta(&[], &gupu, Lit, Uttama, Bahu, &["jugupma"]);
}

#[test]
fn sk_2397() {
    // akAravAn dhatus with Tal-pratyaya
    let skand = d("ska\\ndi~r", Bhvadi);
    assert_has_parasmai_tinanta(
        &[],
        &skand,
        Lit,
        Madhyama,
        Eka,
        &["caskandiTa", "caskanTa", "caskantTa"],
    );
}
