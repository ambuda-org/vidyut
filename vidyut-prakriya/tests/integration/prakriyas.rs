//! Tests to verify that a prakriya has a specific format.
//!
//! TODO: add tests from the पाणिनीयव्याकरणोदाहरणकोषः
extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as K;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Rule;

fn tip_args(dhatu: Dhatu, la: Lakara) -> Tinanta {
    Tinanta::builder()
        .dhatu(dhatu)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(la)
        .build()
        .unwrap()
}

fn subanta(
    pratipadika: impl Into<Pratipadika>,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
) -> Subanta {
    Subanta::builder()
        .pratipadika(pratipadika.into())
        .linga(linga)
        .vibhakti(vibhakti)
        .vacana(vacana)
        .build()
        .unwrap()
}

// Sample test for `Bavati`.
#[test]
fn bhavati() {
    use Rule::Ashtadhyayi as A;

    let args = tip_args(d("BU", Bhvadi), Lat);
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "Bavati").unwrap();

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
    use Rule::Ashtadhyayi as A;

    let args = tip_args(d("jyA\\", Kryadi), AshirLin);
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "jIyAt").unwrap();

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
    use Rule::Ashtadhyayi as A;

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
    let p = ps.iter().find(|p| p.text() == "pAsparDyate").unwrap();

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
    use Rule::Ashtadhyayi as A;

    let args = tip_args(d("akzU~", Bhvadi), Lat);
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "akzRoti").unwrap();

    assert_matches_prakriya(p, &[(A("8.4.1"), vec!["akz", "Ro", "ti"])]);
}

// Test to make sure 8.4.2 applies in when r/z and R are intervened by at, ku, etc.
#[test]
fn krinaati() {
    use Rule::Ashtadhyayi as A;

    let args = tip_args(d("qukrI\\Y", Kryadi), Lat);
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "krIRAti").unwrap();

    assert_matches_prakriya(p, &[(A("8.4.2"), vec!["krI", "RA", "ti"])]);
}

// Fixes https://github.com/ambuda-org/vidyut/issues/125
//
// This test verifies the following:
// - We correctly apply 8.4.54 with 7.4.73
#[test]
fn babhuva() {
    use Rule::Ashtadhyayi as A;

    let args = tip_args(d("BU", Bhvadi), Lit);
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);
    let p = ps.iter().find(|p| p.text() == "baBUva").unwrap();

    assert_matches_prakriya(
        p,
        &[
            (A("7.4.73"), vec!["Ba", "BU", "v", "a"]),
            (A("8.4.54"), vec!["ba", "BU", "v", "a"]),
        ],
    );
}

// Fixes https://github.com/ambuda-org/vidyut/issues/126
//
// This test verifies the following:
// - we use 7.4.92 for carkarti, etc.
#[test]
fn carkarti_etc() {
    use Rule::Ashtadhyayi as A;

    let args = tip_args(yan_luk(&d("qukf\\Y", Tanadi)), Lat);
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);

    let p = ps.iter().find(|p| p.text() == "carkarti").unwrap();
    assert_matches_prakriya(p, &[(A("7.4.92:ruk"), vec!["ca", "ru~k", "kf", ""])]);

    let p = ps.iter().find(|p| p.text() == "carikarti").unwrap();
    assert_matches_prakriya(p, &[(A("7.4.92:rik"), vec!["ca", "rik", "kf", ""])]);

    let p = ps.iter().find(|p| p.text() == "carIkarti").unwrap();
    assert_matches_prakriya(p, &[(A("7.4.92:rIk"), vec!["ca", "rIk", "kf", ""])]);
}

// Fixes https://github.com/ambuda-org/vidyut/issues/127
//
// This test verifies the following:
// - we use 6.1.49 after guna.
#[test]
fn sadhayati() {
    use Rule::Ashtadhyayi as A;

    let args = tip_args(nic(&d("zi\\Du~", Divadi)), Lat);
    let t = Tester::default();
    let ps = t.derive_tinantas(&args);

    let p = ps.iter().find(|p| p.text() == "sADayati").unwrap();
    assert_matches_prakriya(
        p,
        &[
            (A("7.3.86"), vec!["seD", "i"]),
            (A("6.1.49"), vec!["sAD", "i"]),
        ],
    );
}

// Verifies that `nlp_mode` works.
#[test]
fn nlp_mode() {
    let t = Tester::with_nlp_mode();
    t.assert_has_tas(&[], &d("BU", Bhvadi), Lat, &["Bavatas"]);
    t.assert_has_sup_1s("dvAr", Linga::Pum, &["dvAr"]);
}

// Fixes a bug reported by Neelesh B.
#[test]
fn edha() {
    use Rule::Ashtadhyayi as A;

    let args = subanta(
        krdanta(&[], &d("eDa~\\", Tanadi), K::GaY),
        Linga::Stri,
        Vibhakti::Prathama,
        Vacana::Eka,
    );
    let t = Tester::default();
    let ps = t.derive_subantas(&args);

    let p = ps.iter().find(|p| p.text() == "eDA").unwrap();
    assert_matches_prakriya(p, &[(A("6.1.101"), vec!["eD", "A", "", ""])]);
}
