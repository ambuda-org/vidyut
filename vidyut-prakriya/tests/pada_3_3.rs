extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_3_3_10() {
    let d = Dhatu::new;
    assert_has_krdanta(&[], &d("Bu\\ja~", Gana::Rudhadi), Krt::tumun, &["Boktum"]);
}
