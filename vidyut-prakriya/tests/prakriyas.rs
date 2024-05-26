//! Tests that verify that a prakriya has a specific format.
//!
//! TODO: add tests from the पाणिनीयव्याकरणोदाहरणकोषः
extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Rule;

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
