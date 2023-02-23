extern crate vidyut_prakriya;

use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;
use vidyut_prakriya::Prakriya;

pub fn derive_tinantas(dhatu: &Dhatu, args: &TinantaArgs) -> Vec<Prakriya> {
    let a = Ashtadhyayi::new();
    let mut results = a.derive_tinantas(dhatu, args);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());

    results
        .into_iter()
        .filter(|p| !(p.text().ends_with('d') || p.text().ends_with('q')))
        .collect()
}

pub fn derive_krdantas(dhatu: &Dhatu, krt: Krt) -> Vec<Prakriya> {
    let args = KrdantaArgs::builder().krt(krt).build().unwrap();

    let a = Ashtadhyayi::new();
    let mut results = a.derive_krdantas(dhatu, &args);
    results.sort_by_key(|p| p.text());

    results
        .into_iter()
        .filter(|p| !p.text().ends_with('d'))
        .collect()
}

pub fn derive_lakara(prefixes: &[&str], dhatu: &Dhatu, lakara: Lakara) -> Vec<Prakriya> {
    let dhatu = dhatu.clone().with_prefixes(prefixes);
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(lakara)
        .build()
        .unwrap();

    derive_tinantas(&dhatu, &args)
}

pub fn derive_parasmai(prefixes: &[&str], dhatu: &Dhatu, lakara: Lakara) -> Vec<Prakriya> {
    let dhatu = dhatu.clone().with_prefixes(prefixes);
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(lakara)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();

    derive_tinantas(&dhatu, &args)
}

pub fn derive_atmane(prefixes: &[&str], dhatu: &Dhatu, lakara: Lakara) -> Vec<Prakriya> {
    let dhatu = dhatu.clone().with_prefixes(prefixes);
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(lakara)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    derive_tinantas(&dhatu, &args)
}

fn print_all_prakriyas(prakriyas: &[Prakriya]) {
    for p in prakriyas {
        for step in p.history() {
            println!("{} --> {}", step.rule(), step.result());
        }
        println!("{:?}", p.rule_choices());
        println!();
    }
}

pub fn assert_padas(prakriyas: Vec<Prakriya>, expected: &[&str]) {
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

/// Checks the given lakara/purusha/vacana
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
pub fn assert_has_parasmai_tinanta(
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
pub fn assert_has_atmane_tinanta(
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

pub fn assert_has_lat(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lat);
    assert_padas(actual, expected);
}

pub fn assert_has_lat_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::Lat);
    assert_padas(actual, expected);
}

pub fn assert_has_lat_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_atmane(prefixes, dhatu, Lakara::Lat);
    assert_padas(actual, expected);
}

pub fn assert_has_lat_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Karmani)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();

    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lit(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lit);
    assert_padas(actual, expected);
}

pub fn assert_has_lit_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::Lit);
    assert_padas(actual, expected);
}

pub fn assert_has_lut(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lut);
    assert_padas(actual, expected);
}

pub fn assert_has_lut_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::Lut);
    assert_padas(actual, expected);
}

pub fn assert_has_lrt(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lrt);
    assert_padas(actual, expected);
}

pub fn assert_has_lrt_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::Lrt);
    assert_padas(actual, expected);
}

pub fn assert_has_lot(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lot);
    assert_padas(actual, expected);
}

pub fn assert_has_lot_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::Lot);
    assert_padas(actual, expected);
}

pub fn assert_has_lan(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lan);
    assert_padas(actual, expected);
}

pub fn assert_has_lan_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::Lan);
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::AshirLin);
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::AshirLin);
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_atmane(prefixes, dhatu, Lakara::AshirLin);
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Karmani)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::AshirLin)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_vidhilin_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::VidhiLin);
    assert_padas(actual, expected);
}

pub fn assert_has_lun(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lun);
    assert_padas(actual, expected);
}

pub fn assert_has_lun_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::Lun);
    assert_padas(actual, expected);
}

pub fn assert_has_lun_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_atmane(prefixes, dhatu, Lakara::Lun);
    assert_padas(actual, expected);
}

pub fn assert_has_lun_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Karmani)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lun)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lrn(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(prefixes, dhatu, Lakara::Lrn);
    assert_padas(actual, expected);
}

pub fn assert_has_lrn_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(prefixes, dhatu, Lakara::Lrn);
    assert_padas(actual, expected);
}

pub fn assert_has_krdanta(prefixes: &[&str], dhatu: &Dhatu, krt: Krt, expected: &[&str]) {
    assert_padas(
        derive_krdantas(&dhatu.clone().with_prefixes(prefixes), krt),
        expected,
    );
}

pub fn assert_has_subantas(
    text: &str,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
    expected: &[&str],
) {
    let a = Ashtadhyayi::new();
    let pratipadika = Pratipadika::new(text);
    let args = SubantaArgs::builder()
        .linga(linga)
        .vacana(vacana)
        .vibhakti(vibhakti)
        .build()
        .unwrap();

    let mut results = a.derive_subantas(&pratipadika, &args);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    let actual: Vec<_> = results
        .into_iter()
        .filter(|p| !(p.text().ends_with('d') || p.text().ends_with('q')))
        .collect();
    assert_padas(actual, expected);
}
