extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt;
use vidyut_prakriya::args::BaseKrt::*;
use vidyut_prakriya::args::Dhatu;
use vidyut_prakriya::args::Gana::*;

fn assert_has_krt(prefixes: &[&str], dhatu: &Dhatu, krt: BaseKrt, expected: &[&str]) {
    assert_has_krdanta(prefixes, dhatu, krt, expected)
}

#[test]
fn skip_sk_2829_to_sk_2833() {}

#[test]
fn sk_2834() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_krt(&[], &edh, tavya, &["eDitavya"]);
    assert_has_krt(&[], &edh, anIyar, &["eDanIya"]);

    let vas = d("va\\sa~", Bhvadi);
    assert_has_krt(&[], &vas, tavyat, &["vastavya", "vAstavya"]);

    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krt(&[], &pac, kelimar, &["pacelima"]);
    assert_has_krt(&[], &pac, tavya, &["paktavya"]);

    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_krt(&[], &bhid, tavya, &["Bettavya"]);
}

#[test]
fn sk_2847() {
    let shak = d("Sa\\kx~", Svadi);
    assert_has_krt(&[], &shak, yat, &["Sakya"]);

    let sah = d("zaha~\\", Bhvadi);
    assert_has_krt(&[], &sah, yat, &["sahya"]);
}

#[test]
fn sk_2860() {
    let x = d("Kanu~^", Bhvadi);
    assert_has_krt(&[], &x, kyap, &["Keya"]);
}
