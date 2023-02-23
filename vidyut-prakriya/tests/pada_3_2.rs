extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_3_2_123() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("qupa\\ca~^z", Gana::Bhvadi), &["pacati"]);
    assert_has_lat_p(&[], &d("paWa~", Gana::Bhvadi), &["paWati"]);
}
