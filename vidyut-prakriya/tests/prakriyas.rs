//! Tests to verify that a prakriya has a specific format.
//!
//! TODO: add tests from the पाणिनीयव्याकरणोदाहरणकोषः
extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Rule;

// Sample test for `Bavati`.
#[test]
fn bhavati() {
    let bhu = d("BU", Bhvadi);
    let args = Tinanta::builder()
        .dhatu(bhu)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "Bavati").unwrap();

    use Rule::Ashtadhyayi as A;

    assert_matches_prakriya(
        p,
        &[
            (A("1.3.1"), vec!["BU"]),
            (A("3.4.78"), vec!["BU", "tip"]),
            (A("3.1.68"), vec!["BU", "Sap", "ti"]),
            (A("7.3.84"), vec!["Bo", "a", "ti"]),
            (A("6.1.78"), vec!["Bav", "a", "ti"]),
        ],
    );
}

/// Fixes https://github.com/ambuda-org/vidyut/issues/118
///
/// This test verifies the following:
/// - We correctly apply 6.4.108.
/// - We lengthen the dhatu's vowel with 6.4.2.
#[test]
fn jiyat() {
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

/// Fixes https://github.com/ambuda-org/vidyut/issues/119
///
/// This test verifies the following:
/// - We correctly apply 6.4.49.
#[test]
fn paspardhyate() {
    let spardh = d("sparDa~\\", Bhvadi);
    let args = Tinanta::builder()
        .dhatu(spardh.with_sanadi(&[Sanadi::yaN]))
        .prayoga(Prayoga::Karmani)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    for p in &ps {
        println!("{}", p.text());
    }
    let p = ps.iter().find(|p| p.text() == "pAsparDyate").unwrap();

    use Rule::Ashtadhyayi as A;

    assert_matches_prakriya(
        p,
        &[
            (A("1.3.1"), vec!["sparDa~\\"]),
            (A("1.2.4"), vec!["pA", "sparD", "ya", "ya", "te"]),
            (A("6.4.49"), vec!["pA", "sparD", "a", "ya", "te"]),
            (A("6.4.48"), vec!["pA", "sparD", "", "ya", "te"]),
            (A("8.4.68"), vec!["pA", "sparD", "", "ya", "te"]),
        ],
    );
}

// Test to make sure 8.4.1 applies in akzRoti (i.e when R immediately follows r/z)
#[test]
fn akshnoti() {
    let akz = d("akzU~", Bhvadi);
    let args = Tinanta::builder()
        .dhatu(akz)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "akzRoti").unwrap();

    use Rule::Ashtadhyayi as A;

    assert_matches_prakriya(p, &[(A("8.4.1"), vec!["ak", "Ro", "ti"])]);
}

// Test to make sure 8.4.2 applies in when r/z and R are intervened by at, ku, etc.
#[test]
fn krinaati() {
    let krii = d("qukrI\\Y", Kryadi);
    let args = Tinanta::builder()
        .dhatu(krii)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "krIRAti").unwrap();

    use Rule::Ashtadhyayi as A;

    assert_matches_prakriya(p, &[(A("8.4.2"), vec!["krI", "RA", "ti"])]);
}

// Fixes https://github.com/ambuda-org/vidyut/issues/125
//
// This test verifies the following:
// - We correctly apply 8.4.54 with 7.4.73
#[test]
fn babhuva() {
    let bhu = d("BU", Bhvadi);
    let args = Tinanta::builder()
        .dhatu(bhu)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lit)
        .build()
        .unwrap();
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "baBUva").unwrap();

    use Rule::Ashtadhyayi as A;

    assert_matches_prakriya(
        p,
        &[
            (A("7.4.73"), vec!["Ba", "BU", "v", "a"]),
            (A("8.4.54"), vec!["ba", "BU", "v", "a"]),
        ],
    );
}
