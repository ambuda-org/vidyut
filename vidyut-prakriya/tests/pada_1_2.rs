extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_1_2_1() {
    assert_has_lun_a(
        &["aDi"],
        &Dhatu::new("i\\N", Gana::Adadi),
        &["aDyagIzwa", "aDyEzwa"],
    );

    let kuw = Dhatu::new("kuwa~", Gana::Tudadi).with_antargana(Some(Antargana::Kutadi));

    assert_has_krdanta(&[], &kuw, Krt::tfc, &["kuwitf"]);
    assert_has_krdanta(&[], &kuw, Krt::tumun, &["kuwitum"]);
    assert_has_krdanta(&[], &kuw, Krt::tavya, &["kuwitavya"]);

    let puw = Dhatu::new("puwa~", Gana::Tudadi).with_antargana(Some(Antargana::Kutadi));
    assert_has_krdanta(&["ud"], &puw, Krt::tfc, &["utpuwitf"]);
    assert_has_krdanta(&["ud"], &puw, Krt::tumun, &["utpuwitum"]);
    assert_has_krdanta(&["ud"], &puw, Krt::tavya, &["utpuwitavya"]);

    let kuw_nic = kuw.clone().with_sanadi(&[Sanadi::Nic]);
    assert_has_lat_p(&["ud"], &kuw_nic, &["utkowayati"]);
    assert_has_lit_p(&["ud"], &kuw, &["uccukowa"]);
    assert_has_krdanta(&["ud"], &kuw, Krt::Rvul, &["utkowaka"]);
    assert_has_krdanta(&["ud"], &kuw, Krt::GaY, &["utkowa"]);
    // TODO: varttika
}

#[ignore]
#[test]
fn sutra_1_2_2() {
    let vij = Dhatu::new("o~vijI~\\", Gana::Tudadi);
    assert_has_krdanta(&["ud"], &vij, Krt::kta, &["udvijita"]);
    assert_has_krdanta(&["ud"], &vij, Krt::tumun, &["udvijitum"]);
    assert_has_krdanta(&["ud"], &vij, Krt::tavya, &["udvijitavya"]);

    assert_has_krdanta(&["ud"], &vij, Krt::lyuw, &["udvejana"]);
    assert_has_krdanta(&["ud"], &vij, Krt::anIyar, &["udvejanIya"]);
}
