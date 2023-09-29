/*!
Various test utils.

vidyut-prakriya has thousands of assert statements. This module contains various functions to
manage the boilerplate required for these assertions.
*/
extern crate vidyut_prakriya;

use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;
use vidyut_prakriya::Prakriya;

// Derivation helpers
// ------------------

/// Sanitizes our test results by making them deterministic and predictable.
fn sanitize_results(mut results: Vec<Prakriya>) -> Vec<Prakriya> {
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());

    results
        .into_iter()
        .filter(|p| {
            let text = p.text();
            !text.ends_with('d') && !text.ends_with('q') && !text.ends_with('g')
        })
        .collect()
}

/// Derives tinantas from the given initial conditions.
fn derive_tinantas(dhatu: &Dhatu, args: &TinantaArgs) -> Vec<Prakriya> {
    let a = Ashtadhyayi::new();
    let results = a.derive_tinantas(dhatu, args);
    sanitize_results(results)
}

/// Derives krdantas from the given initial conditions.
fn derive_krdantas(dhatu: &Dhatu, args: KrdantaArgs) -> Vec<Prakriya> {
    let a = Ashtadhyayi::new();
    let mut results = a.derive_krdantas(dhatu, &args);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    results
}

/// Derives taddhitantas from the given initial conditions.
fn derive_taddhitantas(p: &Pratipadika, t: Taddhita, a: Option<Artha>) -> Vec<Prakriya> {
    let args = if let Some(a) = a {
        TaddhitantaArgs::builder()
            .taddhita(t)
            .artha(a)
            .build()
            .unwrap()
    } else {
        TaddhitantaArgs::builder().taddhita(t).build().unwrap()
    };
    let a = Ashtadhyayi::new();
    let results = a.derive_taddhitantas(p, &args);
    sanitize_results(results)
}

fn derive_subantas(pratipadika: &Pratipadika, args: SubantaArgs) -> Vec<Prakriya> {
    let a = Ashtadhyayi::new();
    let results = a.derive_subantas(pratipadika, &args);
    sanitize_results(results)
}

/// Derives vakyas from the given initial conditions.
fn derive_vakyas(first: &str, second: &str) -> Vec<Prakriya> {
    let a = Ashtadhyayi::new();
    let mut results = a.derive_vakyas(&first, &second);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    results
}

/// Derives the prathama-eka forms of the given lakara.
fn derive_lakara(prefixes: &[&str], dhatu: &Dhatu, lakara: Lakara) -> Vec<Prakriya> {
    let dhatu = dhatu.clone().with_prefixes(prefixes);
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Eka)
        .lakara(lakara)
        .build()
        .unwrap();

    derive_tinantas(&dhatu, &args)
}

/// Derives the karmani-prathama-eka forms of the given lakara.
fn derive_karmani(prefixes: &[&str], dhatu: &Dhatu, lakara: Lakara) -> Vec<Prakriya> {
    let dhatu = dhatu.clone().with_prefixes(prefixes);
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Karmani)
        .purusha(Purusha::Prathama)
        .vacana(Eka)
        .lakara(lakara)
        .build()
        .unwrap();
    derive_tinantas(&dhatu, &args)
}

/// Nicely prints out the given prakriyas.
fn print_all_prakriyas(prakriyas: &[Prakriya]) {
    for p in prakriyas {
        for step in p.history() {
            println!("{} --> {}", step.rule(), step.result());
        }
        println!("{:?}", p.rule_choices());
        println!();
    }
}

// Heavy assert helpers
// --------------------
// These functions are too heavy for regular use. Instead, use the smaller assert functions below.

/// Asserts that the given `prakriyas` produce the `expected` results.
///
/// If there is any difference, this function will nicely print out each prakriya in `prakriyas`.
fn assert_padas(prakriyas: Vec<Prakriya>, expected: &[&str]) {
    let actuals: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();

    if actuals.len() != expected.len() {
        print_all_prakriyas(&prakriyas);
    }

    assert_eq!(
        actuals.len(),
        expected.len(),
        "expected: {expected:?}, actual: {actuals:?}"
    );

    let mut expected = Vec::from(expected);
    expected.sort();
    expected.dedup();

    for (i, p) in prakriyas.iter().enumerate() {
        let actual = p.text();

        if actual != expected[i] {
            print_all_prakriyas(&prakriyas);
        }
        assert_eq!(
            actual, expected[i],
            "expected: {expected:?}, actual: {actuals:?}"
        );
    }
}

/// Checks the given combination of `dhatu` and `prefixes` produces the `expected` results given
/// this `lakara`/`purusha`/`vacana` combination.
pub fn assert_has_tinanta(
    prefixes: &[&str],
    dhatu: &Dhatu,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    expected: &[&str],
) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(purusha)
        .vacana(vacana)
        .lakara(lakara)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

/// Checks parasmaipada + the given lakara/purusha/vacana
fn assert_has_parasmai_tinanta(
    prefixes: &[&str],
    dhatu: &Dhatu,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    expected: &[&str],
) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(purusha)
        .vacana(vacana)
        .lakara(lakara)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

/// Checks atmanepada + the given lakara/purusha/vacana
fn assert_has_atmane_tinanta(
    prefixes: &[&str],
    dhatu: &Dhatu,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    expected: &[&str],
) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(purusha)
        .vacana(vacana)
        .lakara(lakara)
        .pada(Pada::Atmane)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

/// Checks karmani + the given lakara/purusha/vacana
pub fn assert_has_karmani_tinanta(
    prefixes: &[&str],
    dhatu: &Dhatu,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    expected: &[&str],
) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Karmani)
        .purusha(purusha)
        .vacana(vacana)
        .lakara(lakara)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_sandhi(first: &str, second: &str, expected: &[&str]) {
    let prakriyas = derive_vakyas(&first, &second);

    let actuals: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    if actuals.len() != expected.len() {
        print_all_prakriyas(&prakriyas);
    }

    assert_eq!(
        actuals.len(),
        expected.len(),
        "expected: {expected:?}, actual: {actuals:?}"
    );

    let mut expected: Vec<_> = expected.iter().map(|text| text.replace(" ", "")).collect();
    expected.sort();
    expected.dedup();

    for (i, p) in prakriyas.iter().enumerate() {
        let actual = p.text();

        if actual != expected[i] {
            print_all_prakriyas(&prakriyas);
        }
        assert_eq!(
            actual, expected[i],
            "expected: {expected:?}, actual: {actuals:?}"
        );
    }
}

// Dhatu helpers
// -------------

/// Shorthand for `Dhatu::new`.
///
/// Our tests create dhatus thousands of times, so we defined this function to save some typing and
/// make our tests easier to scan.
pub fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

/// Marks a dhatu as taking san-pratyaya.
pub fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

/// Marks a dhatu as taking yaN-pratyaya.
pub fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan])
}

/// Marks a dhatu as taking Ric-pratyaya.
pub fn nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic])
}

/// Marks a dhatu as taking Nic-pratyaya followed by san-pratyaya.
pub fn nic_san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic, Sanadi::San])
}

/// Marks a dhatu as taking yaN-pratyaya with luk.
pub fn yan_luk(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::YanLuk])
}

// Pratipadika helpers
// -------------------

// Shorthand for building a pratipadika.
pub fn prati(text: &str) -> Pratipadika {
    Pratipadika::new(text)
}

// Shorthand for building a pratipadika that ends with NI/Ap.
pub fn nyap(text: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(text)
        .is_nyap(true)
        .build()
        .unwrap()
}

// Tinanta helpers (based on final suffix)
// ---------------------------------------

pub fn assert_has_tip(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, la, Purusha::Prathama, Eka, expected);
}

pub fn assert_has_tas(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, la, Purusha::Prathama, Dvi, expected);
}

pub fn assert_has_jhi(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, la, Purusha::Prathama, Bahu, expected);
}

pub fn assert_has_sip(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, la, Purusha::Madhyama, Eka, expected);
}

pub fn assert_has_thas(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, la, Purusha::Madhyama, Dvi, expected);
}

pub fn assert_has_tha(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, la, Purusha::Madhyama, Bahu, expected);
}

pub fn assert_has_mip(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, la, Purusha::Uttama, Eka, expected);
}

pub fn assert_has_vas(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, la, Purusha::Uttama, Dvi, expected);
}

pub fn assert_has_mas(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, la, Purusha::Uttama, Bahu, expected);
}

pub fn assert_has_ta(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_atmane_tinanta(prefixes, dhatu, la, Purusha::Prathama, Eka, expected);
}

pub fn assert_has_aataam(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_atmane_tinanta(prefixes, dhatu, la, Purusha::Prathama, Dvi, expected);
}

pub fn assert_has_jha(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_atmane_tinanta(prefixes, dhatu, la, Purusha::Prathama, Bahu, expected);
}

pub fn assert_has_thaas(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_atmane_tinanta(prefixes, dhatu, la, Purusha::Madhyama, Eka, expected);
}

pub fn assert_has_aathaam(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_atmane_tinanta(prefixes, dhatu, la, Purusha::Madhyama, Dvi, expected);
}

pub fn assert_has_dhvam(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_atmane_tinanta(prefixes, dhatu, la, Purusha::Madhyama, Bahu, expected);
}

pub fn assert_has_iw(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_atmane_tinanta(prefixes, dhatu, la, Purusha::Uttama, Eka, expected);
}

pub fn assert_has_vahi(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_atmane_tinanta(prefixes, dhatu, la, Purusha::Uttama, Dvi, expected);
}

pub fn assert_has_mahin(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_atmane_tinanta(prefixes, dhatu, la, Purusha::Uttama, Bahu, expected);
}

// Tinanta helpers (based on lakara suffix)
// ----------------------------------------

pub fn assert_has_lat(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lat);
    assert_padas(actual, expected);
}

pub fn assert_has_lat_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::Lat, expected);
}

pub fn assert_has_lat_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, dhatu, Lakara::Lat, expected);
}

pub fn assert_has_lat_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_karmani(prefixes, dhatu, Lakara::Lat);
    assert_padas(actual, expected);
}

pub fn assert_has_lit(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lit);
    assert_padas(actual, expected);
}

pub fn assert_has_lit_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::Lit, expected);
}

pub fn assert_has_lit_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_karmani(prefixes, dhatu, Lakara::Lit);
    assert_padas(actual, expected);
}

pub fn assert_has_lut(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lut);
    assert_padas(actual, expected);
}

pub fn assert_has_lut_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::Lut, expected);
}

pub fn assert_has_lut_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_karmani(prefixes, dhatu, Lakara::Lut);
    assert_padas(actual, expected);
}

pub fn assert_has_lrt(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lrt);
    assert_padas(actual, expected);
}

pub fn assert_has_lrt_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::Lrt, expected);
}

pub fn assert_has_lrt_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, dhatu, Lakara::Lrt, expected);
}

pub fn assert_has_lrt_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_karmani(prefixes, dhatu, Lakara::Lrt);
    assert_padas(actual, expected);
}

pub fn assert_has_lot(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lot);
    assert_padas(actual, expected);
}

pub fn assert_has_lot_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::Lot, expected);
}

pub fn assert_has_lot_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, dhatu, Lakara::Lot, expected);
}

pub fn assert_has_lan(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lan);
    assert_padas(actual, expected);
}

pub fn assert_has_lan_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::Lan, expected);
}

pub fn assert_has_lan_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, dhatu, Lakara::Lan, expected);
}

pub fn assert_has_lan_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_karmani(prefixes, dhatu, Lakara::Lan);
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::AshirLin);
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::AshirLin, expected);
}

pub fn assert_has_ashirlin_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, dhatu, Lakara::AshirLin, expected);
}

pub fn assert_has_ashirlin_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_karmani(prefixes, dhatu, Lakara::AshirLin);
    assert_padas(actual, expected);
}

pub fn assert_has_vidhilin(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::VidhiLin);
    assert_padas(actual, expected);
}

pub fn assert_has_vidhilin_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::VidhiLin, expected);
}

pub fn assert_has_vidhilin_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, dhatu, Lakara::VidhiLin, expected);
}

pub fn assert_has_lun(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lun);
    assert_padas(actual, expected);
}

pub fn assert_has_lun_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::Lun, expected);
}

pub fn assert_has_lun_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, dhatu, Lakara::Lun, expected);
}

pub fn assert_has_lun_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_karmani(prefixes, dhatu, Lakara::Lun);
    assert_padas(actual, expected);
}

pub fn assert_has_lrn(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lrn);
    assert_padas(actual, expected);
}

pub fn assert_has_lrn_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, dhatu, Lakara::Lrn, expected);
}

pub fn assert_has_lrn_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, dhatu, Lakara::Lrn, expected);
}

pub fn assert_has_lrn_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_karmani(prefixes, dhatu, Lakara::Lrn);
    assert_padas(actual, expected);
}

// Krdanta helpers
// ---------------

pub fn assert_has_krdanta(prefixes: &[&str], dhatu: &Dhatu, krt: Krt, expected: &[&str]) {
    let args = KrdantaArgs::builder().krt(krt).build().unwrap();
    assert_padas(
        derive_krdantas(&dhatu.clone().with_prefixes(prefixes), args),
        expected,
    );
}

pub fn assert_has_upapada_krdanta(
    upapada: &str,
    prefixes: &[&str],
    dhatu: &Dhatu,
    krt: Krt,
    expected: &[&str],
) {
    let args = KrdantaArgs::builder()
        .krt(krt)
        .upapada(Upapada::make_subanta(upapada))
        .build()
        .unwrap();
    assert_padas(
        derive_krdantas(&dhatu.clone().with_prefixes(prefixes), args),
        expected,
    );
}

pub fn assert_has_upapada_krdanta_raw(
    upapada: Upapada,
    prefixes: &[&str],
    dhatu: &Dhatu,
    krt: Krt,
    expected: &[&str],
) {
    let args = KrdantaArgs::builder()
        .krt(krt)
        .upapada(upapada)
        .build()
        .unwrap();
    assert_padas(
        derive_krdantas(&dhatu.clone().with_prefixes(prefixes), args),
        expected,
    );
}

// Taddhitanta helpers
// -------------------

pub fn assert_has_taddhitanta(prati: &Pratipadika, t: Taddhita, expected: &[&str]) {
    assert_padas(derive_taddhitantas(prati, t, None), expected);
}

pub fn assert_has_artha_taddhita(
    prati: &str,
    requested_artha: Artha,
    t: Taddhita,
    expected: &[&str],
) {
    let pratipadika = Pratipadika::new(prati);
    let mut prakriyas = derive_taddhitantas(&pratipadika, t, Some(requested_artha));
    prakriyas.retain(|p| {
        if let Some(prakriya_artha) = p.artha() {
            requested_artha.is_type_of(prakriya_artha)
        } else {
            false
        }
    });
    assert_padas(prakriyas, expected);
}

// Subanta helpers
// ---------------

pub fn assert_has_subantas_p(
    pratipadika: &Pratipadika,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
    expected: &[&str],
) {
    let args = SubantaArgs::builder()
        .linga(linga)
        .vacana(vacana)
        .vibhakti(vibhakti)
        .build()
        .unwrap();

    let actual = derive_subantas(pratipadika, args);
    assert_padas(actual, expected);
}

pub fn assert_has_subantas(
    pratipadika_text: &str,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
    expected: &[&str],
) {
    let pratipadika = Pratipadika::new(pratipadika_text);
    assert_has_subantas_p(&pratipadika, linga, vibhakti, vacana, expected);
}

/// Like `assert_has_subantas_p` but without any filtering on the last sound.
/// (Needed for 8.4.56.)
pub fn assert_has_subantas_raw(
    pratipadika_text: &str,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
    expected: &[&str],
) {
    let pratipadika = Pratipadika::new(pratipadika_text);
    let a = Ashtadhyayi::new();
    let args = SubantaArgs::builder()
        .linga(linga)
        .vacana(vacana)
        .vibhakti(vibhakti)
        .build()
        .unwrap();

    let mut results = a.derive_subantas(&pratipadika, &args);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    let actual: Vec<_> = results.into_iter().collect();
    assert_padas(actual, expected);
}
