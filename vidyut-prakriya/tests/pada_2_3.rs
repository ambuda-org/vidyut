extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti as V;

#[test]
fn sutra_2_3_49() {
    assert_has_subantas("pawu", Pum, V::Sambodhana, Eka, &["pawo"]);
    assert_has_subantas("devadatta", Pum, V::Sambodhana, Eka, &["devadatta"]);
}
