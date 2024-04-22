//! Derivations that aren't captured in our other tests.
extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Rule;

#[test]
fn ambibat() {
    assert_has_tip(&[], &nic(&d("abi~", Bhvadi)), Lun, &["Ambibat"]);
}

#[test]
fn aucicchat() {
    assert_has_tip(&[], &nic(&d("uCI~", Tudadi)), Lun, &["OcicCat"]);
}

#[test]
fn urjijiayizati() {
    // Given as Urj in sk.
    assert_has_tip(&[], &san(&d("Urja~", Curadi)), Lat, &["Urjijayizati"]);
}

#[test]
fn titikzizati() {
    assert_has_ta(&[], &san(&d("tija~\\", Bhvadi)), Lat, &["titikzizate"]);
}

#[test]
fn praipsat() {
    // Not a real regression, just want to make sure this works consistently.
    let aap = d("A\\px~", Svadi);
    assert_has_tip(&["pra"], &san(&aap), Lan, &["prEpsat"]);
}

#[test]
fn irshy_san_lan() {
    assert_has_tip(
        &[],
        &san(&d("Irzya~", Bhvadi)),
        Lan,
        &["Erzyiyizat", "Erzyizizat"],
    );
}

/// Fixes https://github.com/ambuda-org/vidyut/issues/118
///
/// This test verifies the following:
/// - We correctly apply 6.4.108.
/// - We lengthen the dhatu's vowel with 6.4.2.
#[test]
fn jiyat_prakriya() {
    let jya = d("jyA\\", Kryadi);
    let args = Tinanta::builder()
        .dhatu(jya)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::AshirLin)
        .build()
        .unwrap();
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "jIyAt").unwrap();

    use Rule::Ashtadhyayi as A;

    assert_matches_prakriya(
        p,
        &[
            (A("1.3.1"), vec!["jyA\\"]),
            (A("3.4.116"), vec!["jyA", "ti"]),
            (A("6.1.16"), vec!["jiA", "yAs", "st"]),
            (A("6.1.108"), vec!["ji", "yAs", "st"]),
            (A("6.4.2"), vec!["jI", "yAs", "st"]),
            (A("8.4.56"), vec!["jI", "yA", "t"]),
        ],
    );
}
