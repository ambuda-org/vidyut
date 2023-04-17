extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

fn prati(text: &str) -> Pratipadika {
    Pratipadika::builder().text(text).build().unwrap()
}

#[test]
fn sutra_5_2_94() {
    assert_has_taddhitanta(&prati("go"), T::matup, &["gomat"]);
    assert_has_taddhitanta(&prati("vfkza"), T::matup, &["vfkzavat"]);
    assert_has_taddhitanta(&prati("yava"), T::matup, &["yavamat"]);
    assert_has_taddhitanta(&prati("plakza"), T::matup, &["plakzavat"]);
}

#[test]
fn sutra_5_2_121() {
    assert_has_taddhitanta(&prati("yaSas"), T::vini, &["yaSasvin"]);
    assert_has_taddhitanta(&prati("payas"), T::vini, &["payasvin"]);
    assert_has_taddhitanta(&prati("mAyA"), T::vini, &["mAyAvin"]);
    assert_has_taddhitanta(&prati("meDA"), T::vini, &["meDAvin"]);
    assert_has_taddhitanta(&prati("sraj"), T::vini, &["sragvin"]);
}
