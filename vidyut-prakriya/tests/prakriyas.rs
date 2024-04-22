//! Tests that verify that a prakriya has a specific format.
//!
//! TODO: add tests from the पाणिनीयव्याकरणोदाहरणकोषः
extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Rule;

#[test]
fn bhavadi() {
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
