extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

fn prati(text: &str) -> Pratipadika {
    Pratipadika::builder().text(text).build().unwrap()
}

#[test]
fn sutra_5_1_119() {
    assert_has_taddhitanta(&prati("aSva"), T::tva, &["aSvatva"]);
    assert_has_taddhitanta(&prati("aSva"), T::tal, &["aSvatA"]);
    assert_has_taddhitanta(&prati("go"), T::tva, &["gotva"]);
    assert_has_taddhitanta(&prati("go"), T::tal, &["gotA"]);
}
